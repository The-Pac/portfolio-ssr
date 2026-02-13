use crate::models::recommandation::Recommandation;
use leptos::ev::WheelEvent;
use leptos::prelude::*;

#[component]
pub fn Recommendation() -> impl IntoView {
    stylance::import_style!(style, "style/recommendation.module.scss");

    let recommandations = [
        Recommandation {
            src: "logo/company/technopli.png",
            author: "Technopli",
            texte: Some("Great company to work with!"),
        },
        Recommandation {
            src: "logo/company/irouicome.png",
            author: "Irouicome",
            texte: Some("Reliable and professional service."),
        },
        Recommandation {
            src: "logo/company/astree_software.png",
            author: "Astree Software",
            texte: None,
        },
    ];

    let current_index = RwSignal::new(0);
    let is_scrolling = RwSignal::new(false);

    let on_wheel = move |ev: WheelEvent| {
        ev.prevent_default();

        if is_scrolling.get() {
            return;
        }

        is_scrolling.set(true);

        let delta_y = ev.delta_y();

        current_index.update(|index| {
            if delta_y > 0.0 {
                *index = (*index + 1) % recommandations.len();
            } else if delta_y < 0.0 {
                *index = if *index == 0 {
                    recommandations.len() - 1
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
    };

    view! {
        <div class=style::recommendation_wrapper on:wheel=on_wheel>
            <div class=style::carousel_track>
                {
                    recommandations.iter().enumerate().map(|(idx, recommendation)| {
                        let logo_src = recommendation.src;
                        let author = recommendation.author;
                        let text = recommendation.texte;

                        let offset = move || {
                            let current = current_index.get() as i32;
                            let item_idx = idx as i32;
                            item_idx - current
                        };

                        let is_selected = move || offset() == 0;

                        view! {
                            <div
                                class=style::carousel_item
                                class=(style::selected, is_selected)
                                style:transform=move || {
                                    let off = offset();
                                    let scale = if off == 0 { 1.0 } else { 0.7 };
                                    let translate_x = off as f64 * 30.0;
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
                                    <img src=logo_src alt=author class=style::logo />
                                    {
                                        move || {
                                            if is_selected() {
                                                if let Some(text) = text {
                                                    view! {
                                                        <div class=style::recommendation>
                                                            <p>{text}</p>
                                                            <h3 class=style::card_author>{author}</h3>
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
                }
            </div>
        </div>
    }
}
