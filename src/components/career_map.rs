use crate::models::career::{CareerNode, InteractionState};
use leptos::either::Either;
use leptos::ev::{MouseEvent, TouchEvent, WheelEvent};
use leptos::html::Div;
use leptos::prelude::*;
use std::f64::consts::PI;

const RADIUS_DISTANCE_FROM_PARENT: f64 = 500.0;
const ZOOM_FACTOR: f64 = 0.1;
const MIN_SCALE: f64 = 0.5;
const MAX_SCALE: f64 = 5.0;

fn calculate_coordinates(rayon: f64, angle_deg: f64) -> (f64, f64) {
    let angle_rad = (angle_deg * PI) / 180.0;

    let x = rayon * angle_rad.cos();
    let y = rayon * angle_rad.sin();

    (x, y)
}

#[component]
pub fn CareerMap() -> impl IntoView {
    stylance::import_style!(style, "style/career_map.module.scss");

    let load_career_ressource = LocalResource::new(|| crate::models::career::load_career());

    let is_dragging = RwSignal::new(false);
    let drag_start_x = RwSignal::new(0.0);
    let drag_start_y = RwSignal::new(0.0);
    let container_left = RwSignal::new(0.0);
    let container_top = RwSignal::new(0.0);
    let scale = RwSignal::new(1.0);
    let is_zooming = RwSignal::new(false);
    let is_animation_pending = RwSignal::new(false);
    let last_interaction_time = RwSignal::new(0.0);
    let map_content_ref: NodeRef<Div> = NodeRef::new();
    let map_container_ref: NodeRef<Div> = NodeRef::new();
    let state = RwSignal::new(InteractionState::Idle);
    let initial_touch_distance = RwSignal::new(0.0);

    let is_fullscreen = RwSignal::new(false);
    let touch_start_y = RwSignal::new(0.0);

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
                left, top,zoom, zoom
            );

            container.style(format!("transform: {}", transform_value));
        }
    };

    let enter_fullscreen = move |_| {
        is_fullscreen.set(true);
        disable_scroll();
    };

    let exit_fullscreen = move |_| {
        is_fullscreen.set(false);
        enable_scroll();
    };

    let handle_touch_start = move |event: TouchEvent| {
        if let Some(touch) = event.touches().get(0) {
            touch_start_y.set(touch.client_y() as f64);
        }
    };

    let handle_touch_move = move |event: TouchEvent| {
        if !is_dragging.get() && is_fullscreen.get() {
            if let Some(touch) = event.touches().get(0) {
                let touch_y = touch.client_y() as f64;
                let diff = touch_y - touch_start_y.get();

                if diff > 100.0 {
                    exit_fullscreen(());
                }
            }
        }
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

            let parent_rect = map_container_ref
                .get()
                .expect("map_container_ref should be mounted")
                .get_bounding_client_rect();

            let mouse_viewport_x = event.client_x() as f64 - parent_rect.left();
            let mouse_viewport_y = event.client_y() as f64 - parent_rect.top();

            let old_scale = scale.get();

            let old_mouse_position_x = (mouse_viewport_x - container_left.get()) / old_scale;
            let old_mouse_position_y = (mouse_viewport_y - container_top.get()) / old_scale;

            let delta = -event.delta_y() * ZOOM_FACTOR * 0.01;
            let new_scale = (old_scale + delta).clamp(MIN_SCALE, MAX_SCALE);

            let new_left = mouse_viewport_x - (old_mouse_position_x * new_scale);
            let new_top = mouse_viewport_y - (old_mouse_position_y * new_scale);

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
    };

    view! {
        <div
            class={move || {
                if is_fullscreen.get() {
                    format!("{} {}", style::career_map_wrapper, style::fullscreen)
                } else {
                    style::career_map_wrapper.to_string()
                }
            }}
        >
            <div
                node_ref=map_container_ref
                class=style::career_map_container
                on:click=move |_| {
                    if !is_fullscreen.get() {
                        enter_fullscreen(());
                    }
                }
            >
                <div class=style::career_map
                    node_ref=map_content_ref
                    on:mousedown=handle_drag_start
                    on:mousemove=handle_drag_move
                    on:mouseup=handle_drag_end
                    on:mouseleave=handle_drag_leave
                    on:wheel=handle_zoom
                    on:touchstart=handle_touch_start
                    on:touchmove=handle_touch_move
                    role="button"
                    tabindex="0">
                    <Suspense fallback=move || view! { <p>"Loading career..."</p> }>
                        {move || {
                            load_career_ressource.get().map(|career| {
                                match career {
                                    Ok(career_data) => {
                                        Either::Left(
                                            career_data.roots.values().map(|root: &CareerNode| {
                                                view! {
                                                    <CareerNodeView node={root.clone()} start_x=1500.0 start_y=200.0 start_angle=90.0/>
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
                    on:click=move |_| exit_fullscreen(())
                    aria-label="Exit fullscreen"
                >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M19 9l-7 7-7-7"/>
                    </svg>
                    <span class=style::exit_text>"Sortir"</span>
                </button>
            })}
        {move || is_fullscreen.get().then(|| view! {
            <div class=style::swipe_indicator>
                <div class=style::swipe_line></div>
                <span class=style::swipe_text>"Glissez vers le bas"</span>
            </div>
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
                    style:transform=format!("translate({}px, {}px)", start_x as i32, start_y as i32)
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
                        fill="white"
                        stroke="black"
                        stroke-width="2"
                        style="filter: drop-shadow(0px 4px 12px rgba(0, 0, 0, 0.15))"
                    />
                    <image
                        href=node.logo_path.clone()
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
