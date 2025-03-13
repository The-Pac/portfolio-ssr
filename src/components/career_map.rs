use leptos::either::Either;
use leptos::ev::{MouseEvent, TouchEvent, WheelEvent};
use leptos::html::Div;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;
use leptos::logging::error;

const RADIUS_DISTANCE_FROM_PARENT: f64 = 500.0;
const ZOOM_FACTOR: f64 = 0.1;
const MIN_SCALE: f64 = 0.5;
const MAX_SCALE: f64 = 5.0;

#[component]
pub fn CareerMap() -> impl IntoView {
    stylance::import_style!(style, "style/career_map.module.scss");
    let load_career_ressource = OnceResource::new(load_career());

    let is_dragging = RwSignal::new(false);
    let drag_start_x = RwSignal::new(0);
    let drag_start_y = RwSignal::new(0);
    let container_left = RwSignal::new(0);
    let container_top = RwSignal::new(0);
    let scale = RwSignal::new(1.0);
    let animation_frame_id = RwSignal::new(-1);
    let is_focus = RwSignal::new(false);

    let map_content_ref: NodeRef<Div> = NodeRef::new();

    let disable_scroll = move || {
        if let Some(document) = document().body() {
            let _ = document.style().set_property("overflow", "hidden");
        }
    };

    let enable_scroll = move || {
        if let Some(document) = document().body() {
            let _ = document.style().set_property("overflow", "auto");
        }
    };

    let update_transform = move || {
        if let Some(container) = map_content_ref.get_untracked() {
            let transform_value = format!(
                "translate({}px, {}px) scale({})",
                container_left.get_untracked(),
                container_top.get_untracked(),
                scale.get_untracked()
            );
            container.style(format!("transform : {}", &transform_value));
        }
    };

    let handle_zoom = move |event: WheelEvent| {
        if is_focus.get() {
            event.prevent_default();

            let delta = -event.delta_y() * ZOOM_FACTOR * 0.01;
            let old_scale = scale.get();
            let new_scale = (old_scale + delta).clamp(MIN_SCALE, MAX_SCALE);

            if (new_scale - old_scale).abs() > 0.001 {
                scale.set(new_scale);

                request_animation_frame(move || {
                    update_transform();
                });
            }
        }
    };

    let handle_drag_start = move |event: MouseEvent| {
        event.prevent_default();
        is_dragging.set(true);
        drag_start_x.set(event.client_x() - container_left.get());
        drag_start_y.set(event.client_y() - container_top.get());

        if let Some(container) = map_content_ref.get() {
            let _ = container.style("cursor : grabbing");
        }
    };

    let handle_drag_move = move |event: MouseEvent| {
        if is_dragging.get() {
            let new_left = event.client_x() - drag_start_x.get();
            let new_top = event.client_y() - drag_start_y.get();

            if animation_frame_id.get() == -1 {
                animation_frame_id.set(1);
                request_animation_frame(move || {
                    container_left.set(new_left);
                    container_top.set(new_top);
                    update_transform();
                    animation_frame_id.set(-1);
                });
            }
        }
    };

    let handle_drag_end = move |_| {
        is_dragging.set(false);
        if let Some(container) = map_content_ref.get() {
            let _ = container.style("cursor : grab");
        }
    };

    let handle_touch_start = move |event: TouchEvent| {
        event.prevent_default();
        disable_scroll();

        if let Some(touch) = event.touches().item(0) {
            is_dragging.set(true);
            is_focus.set(true);

            drag_start_x.set(touch.client_x() - container_left.get());
            drag_start_y.set(touch.client_y() - container_top.get());

            if let Some(container) = map_content_ref.get() {
                let _ = container.style("cursor : grabbing");
            }
        }
    };

    let handle_touch_move = move |event: TouchEvent| {
        if is_dragging.get() && event.touches().length() == 1 {
            if let Some(touch) = event.touches().item(0) {
                let new_left = touch.client_x() - drag_start_x.get();
                let new_top = touch.client_y() - drag_start_y.get();

                if animation_frame_id.get() == -1 {
                    animation_frame_id.set(1);
                    request_animation_frame(move || {
                        container_left.set(new_left);
                        container_top.set(new_top);
                        update_transform();
                        animation_frame_id.set(-1);
                    });
                }
            }
        } else if event.touches().length() > 1 {
            if let (Some(touch1), Some(touch2)) = (event.touches().item(0), event.touches().item(1))
            {
                let distance = ((touch1.client_x() - touch2.client_x()).pow(2)
                    + (touch1.client_y() - touch2.client_y()).pow(2))
                .isqrt();

                // Logique de zoom à implémenter
                // Vous devrez stocker la distance initiale entre les doigts et comparer
            }
        }
    };

    let handle_touch_end = move |event: TouchEvent| {
        event.prevent_default();
        is_dragging.set(false);
        is_focus.set(false);
        enable_scroll();

        if let Some(container) = map_content_ref.get() {
            let _ = container.style("cursor : grab");
        }
    };

    let handle_touch_zoom = move |event: TouchEvent| {
        event.prevent_default();

        if event.touches().length() > 1 {
            if let (Some(touch1), Some(touch2)) = (event.touches().item(0), event.touches().item(1))
            {
                let current_distance = ((touch1.client_x() - touch2.client_x()).pow(2)
                    + (touch1.client_y() - touch2.client_y()).pow(2))
                .isqrt();

                // Implémentation du zoom multi-touch
                // Vous aurez besoin de stocker une distance initiale et de comparer
                // Exemple de calcul de zoom basique :
                let old_scale = scale.get();
                let new_scale =
                    (old_scale * (current_distance as f64 / 100.0)).clamp(MIN_SCALE, MAX_SCALE);

                if (new_scale - old_scale).abs() > 0.001 {
                    scale.set(new_scale);

                    request_animation_frame(move || {
                        update_transform();
                    });
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
                    on:mouseleave=handle_drag_end
                    on:wheel=handle_zoom
                    on:mouseup=handle_drag_end
                    on:touchstart=handle_touch_start
                    on:touchmove=handle_touch_move
                    on:touchend=handle_touch_end
                    on:touchcancel=handle_touch_end
                    on:gesturestart=handle_touch_zoom
                    role="button"
                    tabindex="0">
                        <Suspense fallback=move || view! { <p>"Loading career..."</p> }>
                            {move || {
                                load_career_ressource.get().map(|career| {
                                    match career {
                                         Ok(career_data) => {
                                            Either::Left(
                                                career_data.roots.values().map(|root| {
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
                                                    <p>"Failed to get data."</p>
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

    let children = RwSignal::new(node.children.clone());

    let num_children = children.get_untracked().len();
    let angle_delta = if num_children > 1 {
        140.0 / (num_children - 1) as f64
    } else {
        70.0
    };

    view! {
        <svg class=style::connection_svg>
            <g>
                <Show when=move || !children.get().is_empty() fallback=|| view! {}>
                    {move ||
                        children.get().into_iter()
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
                </Show>
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
                        href=node.logo_url.clone()
                        x="-25"
                        y="-25"
                        width="50"
                        height="50"
                    />
                </g>
            </g>
        </svg>
    }.into_any()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CareerNode {
    pub id: i32,
    pub logo_url: String,
    pub title: String,
    pub year: i32,
    pub parent_id: Option<i32>,
    pub children: HashMap<i32, CareerNode>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct CareerNodeTree {
    pub roots: HashMap<i32, CareerNode>,
}

impl CareerNodeTree {
    pub fn build_from_flat_data(nodes: Vec<CareerNode>) -> Self {
        let mut node_map: HashMap<i32, CareerNode> =
            nodes.into_iter().map(|node| (node.id, node)).collect();

        let root_id = node_map
            .values()
            .find(|node| node.parent_id.is_none())
            .expect("Aucun root node trouvé. Un nœud sans parent_id est nécessaire.")
            .id;

        let mut root = node_map
            .remove(&root_id)
            .expect("Impossible de trouver le root après extraction.");

        let mut unprocessed = Vec::new();
        while !node_map.is_empty() {
            for (id, node) in node_map.drain() {
                if let Some(parent_id) = node.parent_id {
                    if let Some(parent_node) = Self::find_node_mut(&mut root, parent_id) {
                        parent_node.children.insert(node.id, node);
                    } else {
                        unprocessed.push((id, node));
                    }
                }
            }

            for (id, node) in unprocessed.drain(..) {
                node_map.insert(id, node);
            }
        }

        CareerNodeTree {
            roots: HashMap::from([(root_id, root)]),
        }
    }

    fn find_node_mut<'a>(node: &'a mut CareerNode, id: i32) -> Option<&'a mut CareerNode> {
        if node.id == id {
            return Some(node);
        }
        for child in node.children.values_mut() {
            if let Some(found) = Self::find_node_mut(child, id) {
                return Some(found);
            }
        }
        None
    }
}

fn calculate_coordinates(rayon: f64, angle_deg: f64) -> (f64, f64) {
    let angle_rad = (angle_deg * PI) / 180.0;

    let x = rayon * angle_rad.cos();
    let y = rayon * angle_rad.sin();

    (x, y)
}

#[server(LoadCareer)]
pub async fn load_career() -> Result<CareerNodeTree, ServerFnError> {
    match get_career_nodes().await {
        Ok(career_nodes) => {
            Ok(CareerNodeTree::build_from_flat_data(career_nodes))
        }
        Err(errors) => {
            error!("Career loading error: {:?}", errors);
            Err(errors)
        }
    }
}

#[cfg(feature = "ssr")]
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for CareerNode {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        Ok(Self {
            id: row.try_get("id")?,
            logo_url: row.try_get("logo_url")?,
            title: row.try_get("title")?,
            year: row.try_get("year")?,
            parent_id: row.try_get("parent_id")?,
            children: HashMap::new(),
            metadata: None,
        })
    }
}

#[server]
pub async fn get_career_nodes() -> Result<Vec<CareerNode>, ServerFnError> {
    let mut connection = crate::libs::database::ssr::db().await?;

    let career_nodes = sqlx::query_as::<_, CareerNode>("SELECT * FROM careers")
        .fetch_all(&mut connection)
        .await?;

    Ok(career_nodes)
}
