use leptos::either::Either;
use leptos::ev::{MouseEvent, TouchEvent, WheelEvent};
use leptos::html::Div;
use leptos::prelude::*;
use std::f64::consts::PI;
use crate::models::career::{ CareerNode, InteractionState};

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

    let load_career_ressource = LocalResource::new(|| {
        crate::models::career::load_career()
    });

    let is_dragging = RwSignal::new(false);
    let drag_start_x = RwSignal::new(0.0);
    let drag_start_y = RwSignal::new(0.0);
    let container_left = RwSignal::new(0.0);
    let container_top = RwSignal::new(0.0);
    let scale = RwSignal::new(1.0);
    let is_focus = RwSignal::new(false);
    let is_zooming = RwSignal::new(false);
    let zoom_center_x = RwSignal::new(0.0);
    let zoom_center_y = RwSignal::new(0.0);
    let is_animation_pending = RwSignal::new(false);
    let last_interaction_time = RwSignal::new(0.0);
    let map_content_ref: NodeRef<Div> = NodeRef::new();
    let state = RwSignal::new(InteractionState::Idle);
    let initial_touch_distance = RwSignal::new(0.0);

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

            container.style(format!("transform: {}", &transform_value));
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
        if is_focus.get() {
            event.prevent_default();
            if matches!(state.get(), InteractionState::Idle) {
                state.set(InteractionState::Zooming);

                let delta = -event.delta_y() * ZOOM_FACTOR * 0.01;
                let old_scale = scale.get();
                let new_scale = (old_scale + delta).clamp(MIN_SCALE, MAX_SCALE);

                if (new_scale - old_scale).abs() > 0.001 {
                    is_zooming.set(true);

                    zoom_center_x.set(event.client_x() as f64);
                    zoom_center_y.set(event.client_y() as f64);

                    let point_x = zoom_center_x.get() - container_left.get();
                    let point_y = zoom_center_y.get() - container_top.get();

                    let normalized_point_x = point_x / old_scale;
                    let normalized_point_y = point_y / old_scale;

                    scale.set(new_scale);

                    let new_point_x = normalized_point_x * new_scale;
                    let new_point_y = normalized_point_y * new_scale;

                    let new_left = zoom_center_x.get() - new_point_x;
                    let new_top = zoom_center_y.get() - new_point_y;

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
        }
    };

    view! {
        <div class=style::career_map_container
            on:click=move |_| {
                is_focus.set(true);
                disable_scroll();
            }
            on:mouseleave=move |_| {
                is_focus.set(false);
                enable_scroll();
            }>
            <div class=style::career_map
                node_ref=map_content_ref
                on:mousedown=handle_drag_start
                on:mousemove=handle_drag_move
                on:mouseup=handle_drag_end
                on:mouseleave=handle_drag_leave
                on:wheel=handle_zoom
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
    }
}

#[component]
fn CareerNodeView(node: CareerNode, start_x: f64, start_y: f64, start_angle: f64) -> impl IntoView {
    stylance::import_style!(style, "style/career_card.module.scss");

    let children = node.children.clone();
    let num_children = children.len();

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

                            view! {
                                <line
                                    x1=start_x
                                    y1=start_y
                                    x2=absolute_child_x
                                    y2=absolute_child_y
                                    stroke="rgba(0,0,0,0.3)"
                                    stroke-width="2"
                                    stroke-dasharray="5,5"
                                    pointer-events="none"
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
                >
                    <circle
                        r="50"
                        fill="white"
                        stroke="black"
                        stroke-width="2"
                    />
                    <image
                        href=node.logo_path.clone()
                        x="-25"
                        y="-25"
                        width="50"
                        height="50"
                    />
                </g>
            </g>
        </svg>
    }
        .into_any()
}