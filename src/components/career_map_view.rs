use crate::models::career::{CareerNode, CareerNodeTree, InteractionState};
use crate::models::database_error::CareerError;
use leptos::either::Either;
use leptos::ev::{MouseEvent, TouchEvent, WheelEvent};
use leptos::html::Div;
use leptos::prelude::*;
use std::f64::consts::PI;

const RADIUS_DISTANCE_FROM_PARENT: f64 = 500.0;
const ZOOM_FACTOR: f64 = 0.1;
const MIN_SCALE: f64 = 0.5;
const MAX_SCALE: f64 = 5.0;
const MAP_SIZE: f64 = 50000.0;
const MAP_CENTER: f64 = MAP_SIZE / 2.0;
const DEFAULT_SCALE: f64 = 1.0;

fn calculate_coordinates(rayon: f64, angle_deg: f64) -> (f64, f64) {
    let angle_rad = (angle_deg * PI) / 180.0;

    let x = rayon * angle_rad.cos();
    let y = rayon * angle_rad.sin();

    (x, y)
}

fn get_touch_distance(touch1: &web_sys::Touch, touch2: &web_sys::Touch) -> f64 {
    let dx = (touch2.client_x() - touch1.client_x()) as f64;
    let dy = (touch2.client_y() - touch1.client_y()) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn get_touch_center(touch1: &web_sys::Touch, touch2: &web_sys::Touch) -> (f64, f64) {
    let center_x = ((touch1.client_x() + touch2.client_x()) as f64) / 2.0;
    let center_y = ((touch1.client_y() + touch2.client_y()) as f64) / 2.0;
    (center_x, center_y)
}

#[component]
pub fn CareerMap() -> impl IntoView {
    stylance::import_style!(style, "style/career_map.module.scss");

    let load_career_ressource: LocalResource<Result<CareerNodeTree, CareerError>> =
        LocalResource::new(|| async {
            let career_nodes = crate::server::career::load_career().await?;
            Ok(CareerNodeTree::build_from_flat_data(career_nodes))
        });

    let is_dragging = RwSignal::new(false);
    let center_map_with_viewport = RwSignal::new((0.0, 0.0));
    let drag_start_x = RwSignal::new(0.0);
    let drag_start_y = RwSignal::new(0.0);
    let container_left = RwSignal::new(0.0);
    let container_top = RwSignal::new(0.0);
    let scale = RwSignal::new(DEFAULT_SCALE);
    let is_zooming = RwSignal::new(false);
    let is_animation_pending = RwSignal::new(false);
    let last_interaction_time = RwSignal::new(0.0);
    let map_content_ref: NodeRef<Div> = NodeRef::new();
    let map_container_ref: NodeRef<Div> = NodeRef::new();
    let state = RwSignal::new(InteractionState::Idle);
    let initial_touch_distance = RwSignal::new(0.0);
    let is_fullscreen = RwSignal::new(false);
    let touch_start_y = RwSignal::new(0.0);
    let touch_start_x = RwSignal::new(0.0);
    let last_touch_center = RwSignal::new((0.0, 0.0));
    let is_pinching = RwSignal::new(false);

    let disable_scroll = move || {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(document) = document().body() {
                let _ = document.style().set_property("overflow", "hidden");
            }
        }
    };

    let enable_scroll = move || {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(document) = document().body() {
                let _ = document.style().set_property("overflow", "auto");
            }
        }
    };

    let update_transform = move || {
        if let Some(container) = map_content_ref.get_untracked() {
            let left = container_left.get_untracked();
            let top = container_top.get_untracked();
            let zoom = scale.get_untracked();

            let transform_value = format!(
                "translate3d({}px, {}px, 0px) scale3d({}, {}, 1)",
                left, top, zoom, zoom
            );

            container.style(format!("transform: {}", transform_value));
        }
    };

    Effect::new(move || {
        if let Some(map_container) = map_container_ref.get_untracked() {
            let rect = map_container.get_bounding_client_rect();

            let absolute_center_height = -MAP_CENTER + rect.height() / 2.0;
            let absolute_center_width = -MAP_CENTER + rect.width() / 2.0;

            center_map_with_viewport.set((absolute_center_width, absolute_center_height));

            container_left.set(absolute_center_width);
            container_top.set(absolute_center_height);
            update_transform();
        }
    });

    let enter_fullscreen = move |_| {
        is_fullscreen.set(true);
        disable_scroll();
    };

    let exit_fullscreen = move |_| {
        if !is_animation_pending.get() {
            is_animation_pending.set(true);
            request_animation_frame(move || {
                scale.set(DEFAULT_SCALE);
                container_left.set(center_map_with_viewport.get_untracked().0);
                container_top.set(center_map_with_viewport.get_untracked().1);

                update_transform();
                is_animation_pending.set(false);
            });
            is_fullscreen.set(false);
            enable_scroll();
        }
    };

    let handle_touch_start = move |event: TouchEvent| {
        event.prevent_default();

        let touches = event.touches();

        if touches.length() == 1 {
            if let Some(touch) = touches.get(0) {
                touch_start_y.set(touch.client_y() as f64);
                touch_start_x.set(touch.client_x() as f64);
                drag_start_x.set(touch.client_x() as f64 - container_left.get());
                drag_start_y.set(touch.client_y() as f64 - container_top.get());
                is_dragging.set(true);
                is_pinching.set(false);
                state.set(InteractionState::Dragging);
            }
        } else if touches.length() == 2 {
            if let (Some(touch1), Some(touch2)) = (touches.get(0), touches.get(1)) {
                is_dragging.set(false);
                is_pinching.set(true);
                state.set(InteractionState::Zooming);

                let distance = get_touch_distance(&touch1, &touch2);
                initial_touch_distance.set(distance);

                let center = get_touch_center(&touch1, &touch2);
                last_touch_center.set(center);
            }
        }
    };

    let handle_touch_move = move |event: TouchEvent| {
        event.prevent_default();

        let touches = event.touches();

        if touches.length() == 1 && is_dragging.get() && !is_pinching.get() {
            if let Some(touch) = touches.get(0) {
                let touch_y = touch.client_y() as f64;
                let touch_x = touch.client_x() as f64;

                if !is_fullscreen.get() {
                    let diff_y = touch_y - touch_start_y.get();
                    if diff_y > 100.0 {
                        exit_fullscreen(());
                        return;
                    }
                }

                let new_left = touch_x - drag_start_x.get();
                let new_top = touch_y - drag_start_y.get();

                if !is_animation_pending.get() {
                    is_animation_pending.set(true);
                    request_animation_frame(move || {
                        container_left.set(new_left);
                        container_top.set(new_top);
                        update_transform();
                        is_animation_pending.set(false);
                    });
                }
            }
        } else if touches.length() == 2 && is_pinching.get() {
            if let (Some(touch1), Some(touch2)) = (touches.get(0), touches.get(1)) {
                let current_distance = get_touch_distance(&touch1, &touch2);
                let initial_distance = initial_touch_distance.get();

                if initial_distance > 0.0 {
                    let scale_change = current_distance / initial_distance;
                    let current_scale = scale.get();
                    let new_scale = (current_scale * scale_change).clamp(MIN_SCALE, MAX_SCALE);

                    let current_center = get_touch_center(&touch1, &touch2);
                    let last_center = last_touch_center.get();

                    if let Some(container_elem) = map_container_ref.get() {
                        let rect = container_elem.get_bounding_client_rect();
                        let zoom_point_x = current_center.0 - rect.left();
                        let zoom_point_y = current_center.1 - rect.top();

                        let scale_delta = new_scale / current_scale;
                        let old_left = container_left.get();
                        let old_top = container_top.get();

                        let new_left = zoom_point_x - (zoom_point_x - old_left) * scale_delta;
                        let new_top = zoom_point_y - (zoom_point_y - old_top) * scale_delta;

                        let pan_x = current_center.0 - last_center.0;
                        let pan_y = current_center.1 - last_center.1;

                        if !is_animation_pending.get() {
                            is_animation_pending.set(true);
                            request_animation_frame(move || {
                                scale.set(new_scale);
                                container_left.set(new_left + pan_x);
                                container_top.set(new_top + pan_y);
                                update_transform();
                                is_animation_pending.set(false);
                            });
                        }
                    }

                    initial_touch_distance.set(current_distance);
                    last_touch_center.set(current_center);
                }
            }
        }
    };

    let handle_touch_end = move |event: TouchEvent| {
        event.prevent_default();

        #[cfg(not(feature = "ssr"))]
        {
            let now = web_sys::window()
                .unwrap_or_else(|| panic!("No window"))
                .performance()
                .unwrap_or_else(|| panic!("No performance"))
                .now();

            last_interaction_time.set(now);
        }

        is_dragging.set(false);
        is_pinching.set(false);
        state.set(InteractionState::Idle);
        initial_touch_distance.set(0.0);
    };

    let handle_drag_start = move |event: MouseEvent| {
        event.prevent_default();
        state.set(InteractionState::Dragging);
        is_dragging.set(true);
        drag_start_x.set(event.client_x() as f64 - container_left.get());
        drag_start_y.set(event.client_y() as f64 - container_top.get());
        if let Some(container) = map_content_ref.get() {
            let _ = container.style("cursor: grabbing");
            update_transform();
        }
    };

    let handle_drag_move = move |event: MouseEvent| {
        event.prevent_default();
        if is_dragging.get() {
            let new_left = event.client_x() as f64 - drag_start_x.get();
            let new_top = event.client_y() as f64 - drag_start_y.get();
            if !is_animation_pending.get() {
                is_animation_pending.set(true);
                request_animation_frame(move || {
                    container_left.set(new_left);
                    container_top.set(new_top);
                    update_transform();
                    is_animation_pending.set(false);
                });
            }
        }
    };

    let handle_drag_leave = move |event: MouseEvent| {
        if is_dragging.get() {
            event.prevent_default();
            state.set(InteractionState::Idle);
            is_dragging.set(false);
            if let Some(container) = map_content_ref.get() {
                let _ = container.style("cursor: grab");
                update_transform();
            }
        }
    };

    let handle_drag_end = move |event: MouseEvent| {
        event.prevent_default();

        #[cfg(not(feature = "ssr"))]
        {
            let now = web_sys::window()
                .unwrap_or_else(|| panic!("No window"))
                .performance()
                .unwrap_or_else(|| panic!("No performance"))
                .now();

            last_interaction_time.set(now);
        }

        is_dragging.set(false);
        state.set(InteractionState::Idle);

        if let Some(container) = map_content_ref.get() {
            let _ = container.style("cursor: grab");
            update_transform();
        }
    };

    let handle_zoom = move |event: WheelEvent| {
        if is_fullscreen.get() {
            event.prevent_default();
            is_zooming.set(true);

            let delta = if event.delta_y() > 0.0 { -ZOOM_FACTOR } else { ZOOM_FACTOR };
            let new_scale = (scale.get() + delta).clamp(MIN_SCALE, MAX_SCALE);

            if let Some(container) = map_container_ref.get() {
                let rect = container.get_bounding_client_rect();
                let mouse_x = event.client_x() as f64 - rect.left();
                let mouse_y = event.client_y() as f64 - rect.top();

                let scale_ratio = new_scale / scale.get();
                let old_left = container_left.get();
                let old_top = container_top.get();

                let new_left = mouse_x - (mouse_x - old_left) * scale_ratio;
                let new_top = mouse_y - (mouse_y - old_top) * scale_ratio;

                scale.set(new_scale);
                container_left.set(new_left);
                container_top.set(new_top);

                if !is_animation_pending.get() {
                    is_animation_pending.set(true);
                    request_animation_frame(move || {
                        update_transform();
                        is_animation_pending.set(false);
                        is_zooming.set(false);
                    });
                }
            }
        }
    };

    view! {
        <div
            class={move || {
                if is_fullscreen.get() {
                    format!("{} {}", style::career_map, style::fullscreen)
                } else {
                    style::career_map.to_string()
                }
            }}
        >
            <div
                node_ref=map_container_ref
                class=style::career_map_viewport
                on:click=move |_| {
                    if !is_fullscreen.get() {
                        enter_fullscreen(());
                    }
                }
                on:touchstart=move |_| {
                    if !is_fullscreen.get() {
                        enter_fullscreen(());
                    }
                }
            >
                <div class=style::career_map_canvas
                    node_ref=map_content_ref
                    on:mousedown=handle_drag_start
                    on:mousemove=handle_drag_move
                    on:mouseup=handle_drag_end
                    on:mouseleave=handle_drag_leave
                    on:wheel=handle_zoom
                    on:touchstart=handle_touch_start
                    on:touchmove=handle_touch_move
                    on:touchend=handle_touch_end
                    on:touchcancel=handle_touch_end
                    role="button"
                    aria-label="Canvas de la carte"
                    tabindex="0">
                    <Suspense fallback=move || view! { <p>"Chargement de ma carrière..."</p> }>
                        {move || {
                            load_career_ressource.get().map(|career| {
                                match career {
                                    Ok(career_data) => {
                                        Either::Left(
                                            career_data.roots.values().map(|root: &CareerNode| {
                                                view! {
                                                    <CareerNodeView node={root.clone()} start_x={MAP_SIZE/2.0} start_y={MAP_SIZE/2.0} start_angle=90.0/>
                                                }
                                            })
                                            .collect_view(),
                                        )
                                    }
                                    Err(_) => {
                                        Either::Right(
                                            view! {
                                                <p>"Failed to load career data. Please try again later."</p>
                                            },
                                        )
                                    }
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </div>

            {move || is_fullscreen.get().then(|| view! {
                <button
                    class=style::exit_fullscreen_btn
                    on:click=move |event:MouseEvent| {
                        event.stop_propagation();
                        exit_fullscreen(());
                    }
                    aria-label="Sortir du plein écran"
                >
                    "Sortir"
                </button>
            })}
        </div>
    }
}

#[component]
fn CareerNodeView(node: CareerNode, start_x: f64, start_y: f64, start_angle: f64) -> impl IntoView {
    stylance::import_style!(style, "style/career_card.module.scss");

    let show_tooltip = RwSignal::new(false);

    let children = node.children.clone();
    let num_children = children.len();
    let node_name = node.title.clone();
    let logo_path = node.logo_path.clone();

    let angle_delta = if num_children > 1 {
        140.0 / (num_children - 1) as f64
    } else {
        70.0
    };

    view! {
        <svg class=style::connection_svg>
            <g>
                {
                    children.into_iter()
                        .enumerate()
                        .map(|(index, (_child_node_id, child_node))| {
                            let child_angle = if num_children > 1 {
                                start_angle - 70.0 + index as f64 * angle_delta
                            } else {
                                start_angle
                            };
                            let (child_x, child_y) = calculate_coordinates(
                                RADIUS_DISTANCE_FROM_PARENT,
                                child_angle
                            );

                            let absolute_child_x = start_x + child_x;
                            let absolute_child_y = start_y + child_y;
                            let control_x = start_x + (absolute_child_x - start_x) / 2.0;
                            let control_y = start_y;
                            view! {
                                <path
                                    d=format!(
                                        "M {} {} Q {} {} {} {}",
                                        start_x, start_y,
                                        control_x, control_y,
                                        absolute_child_x, absolute_child_y
                                    )
                                    stroke="rgba(0,0,0,0.3)"
                                    stroke-width="2"
                                    fill="none"
                                    pointer-events="none"
                                    style="filter: drop-shadow(0px 2px 4px rgba(0, 0, 0, 0.1))"
                                />
                                <CareerNodeView
                                    node=child_node
                                    start_x=absolute_child_x
                                    start_y=absolute_child_y
                                    start_angle=child_angle
                                />
                            }
                        }).collect_view()
                }

                <g
                    class=style::career_card
                    style:transform=format!("translate3d({}px, {}px, 0px) scale3d(1, 1, 1)", start_x as i32, start_y as i32)
                    on:mouseenter=move |_| show_tooltip.set(true)
                    on:mouseleave=move |_| show_tooltip.set(false)
                >
                    <circle
                        class=style::halo
                        r="60"
                        fill="rgba(74, 144, 226, 0.2)"
                    />
                    <circle
                        r="50"
                        fill="#214f46"
                        stroke="black"
                        stroke-width="2"
                        style="filter: drop-shadow(0px 4px 12px rgba(0, 0, 0, 0.15))"
                    />
                    <image
                        href=logo_path.clone()
                        x="-25"
                        y="-25"
                        width="50"
                        height="50"
                    />

                    {move || show_tooltip.get().then(|| view! {
                        <g class=style::tooltip>
                            <rect
                                x="-60"
                                y="-90"
                                width="120"
                                height="30"
                                rx="5"
                                fill="rgba(0, 0, 0, 0.85)"
                            />
                            <text
                                x="0"
                                y="-70"
                                text-anchor="middle"
                                fill="white"
                                font-size="14"
                                font-weight="500"
                            >
                                {node_name.clone()}
                            </text>
                        </g>
                    })}
                </g>
            </g>
        </svg>
    }
        .into_any()
}