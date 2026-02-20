use leptos::either::Either;
use crate::models::database_error::ProjectError;
use crate::models::recommandation::RecommendationWithLogo;
use leptos::ev::{WheelEvent, TouchEvent};
use leptos::prelude::*;

#[component]
pub fn Recommendation() -> impl IntoView {
    stylance::import_style!(style, "style/recommendation.module.scss");

    let recommendations_resource: LocalResource<Result<Vec<RecommendationWithLogo>, ProjectError>> =
        LocalResource::new(|| async {
            crate::server::recommendation::get_all_recommendations().await
        });

    let current_index = RwSignal::new(0);
    let is_scrolling = RwSignal::new(false);
    let touch_start_x = RwSignal::new(0.0);
    let is_swiping = RwSignal::new(false);
    let is_mobile = RwSignal::new(false);

    Effect::new(move || {
        #[cfg(not(feature = "ssr"))]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                let width = window.inner_width().ok()
                    .and_then(|w| w.as_f64())
                    .unwrap_or(1024.0);
                is_mobile.set(width <= 768.0);

                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    if let Some(window) = web_sys::window() {
                        let width = window.inner_width().ok()
                            .and_then(|w| w.as_f64())
                            .unwrap_or(1024.0);
                        is_mobile.set(width <= 768.0);
                    }
                }) as Box<dyn Fn()>);

                let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    });

    let on_wheel = move |ev: WheelEvent| {
        ev.prevent_default();

        if is_scrolling.get() {
            return;
        }

        if let Some(Ok(recommendations)) = recommendations_resource.get() {
            is_scrolling.set(true);

            let delta_y = ev.delta_y();

            current_index.update(|index| {
                if delta_y > 0.0 {
                    *index = (*index + 1) % recommendations.len();
                } else if delta_y < 0.0 {
                    *index = if *index == 0 {
                        recommendations.len() - 1
                    } else {
                        *index - 1
                    };
                }
            });

            set_timeout(
                move || {
                    is_scrolling.set(false);
                },
                std::time::Duration::from_millis(500),
            );
        }
    };

    let on_touch_start = move |ev: TouchEvent| {
        if let Some(touch) = ev.touches().get(0) {
            touch_start_x.set(touch.client_x() as f64);
            is_swiping.set(true);
        }
    };

    let on_touch_end = move |ev: TouchEvent| {
        if !is_swiping.get() {
            return;
        }

        if let Some(touch) = ev.changed_touches().get(0) {
            if let Some(Ok(recommendations)) = recommendations_resource.get() {
                let diff = touch.client_x() as f64 - touch_start_x.get();

                if diff.abs() > 50.0 {
                    current_index.update(|index| {
                        if diff < 0.0 {
                            *index = (*index + 1) % recommendations.len();
                        } else {
                            *index = if *index == 0 {
                                recommendations.len() - 1
                            } else {
                                *index - 1
                            };
                        }
                    });
                }
            }
        }

        is_swiping.set(false);
        touch_start_x.set(0.0);
    };

    view! {
        <div class=style::recommendation
            on:wheel=on_wheel
            on:touchstart=on_touch_start
            on:touchend=on_touch_end
        >
            <div class=style::recommendation_carousel_track>
                <Suspense fallback=move || view! { <p>"Chargement des recommendations..."</p> }>
                    {move || {
                        recommendations_resource.get().map(|result| {
                            match result {
                                Ok(recommendations) => {
                                    Either::Left(
                                        recommendations.iter().enumerate().map(|(idx, recommendation)| {
                                            let offset = move || {
                                                let current = current_index.get() as i32;
                                                let item_idx = idx as i32;
                                                item_idx - current
                                            };

                                            let is_selected = move || offset() == 0;

                                            let logo_path = recommendation.logo.path.clone();
                                            let author = recommendation.author.clone();
                                            let texte = recommendation.texte.clone();

                                            view! {
                                                <div
                                                    class=style::carousel_item
                                                    class=(style::selected, is_selected)
                                                    style:transform=move || {
                                                        let off = offset();
                                                        let scale = if off == 0 { 1.0 } else { 0.7 };
                                                        let translate_x = if is_mobile.get() {
                                                            off as f64 * 10.0
                                                        } else {
                                                            off as f64 * 30.0
                                                        };
                                                        format!("translateX({}rem) scale({})", translate_x, scale)
                                                    }
                                                    style:opacity=move || {
                                                        let off = offset().abs();
                                                        if off > 2 { "0" }
                                                        else if off == 0 { "1" }
                                                        else { "0.5" }
                                                    }
                                                    style:z-index=move || (100 - offset().abs()).to_string()
                                                >
                                                    <div class=style::item_content>
                                                        <img
                                                            src=logo_path.clone()
                                                            alt={format!("une recommendation de la part de {}",author.clone().unwrap_or_default())}
                                                            class=style::logo
                                                            loading="lazy"
                                                            decoding="async"
                                                        />
                                                        {
                                                            let author_clone = author.clone();
                                                            let texte_clone = texte.clone();
                                                            move || {
                                                                if is_selected() {
                                                                    if let Some(text) = texte_clone.clone() {
                                                                        view! {
                                                                            <div class=style::recommendation>
                                                                                <p>{text}</p>
                                                                                <h3 class=style::card_author>{author_clone.clone().unwrap_or_default()}</h3>
                                                                            </div>
                                                                        }.into_any()
                                                                    } else {
                                                                        view! { <></> }.into_any()
                                                                    }
                                                                } else {
                                                                    view! { <></> }.into_any()
                                                                }
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            }
                                    }).collect::<Vec<_>>()
                                    )
                                }
                                Err(e) => {
                                    Either::Right(view! {
                                        <p class="error">"Erreur lors du chargement des recommendations: " {e.to_string()}</p>
                                    })
                                }
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}