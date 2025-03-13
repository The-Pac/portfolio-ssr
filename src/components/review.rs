use leptos::prelude::*;
#[derive(Clone, Copy)]
struct Logo {
    id: i32,
    src: &'static str,
    author: &'static str,
    recommendation: Option<&'static str>,
}

#[component]
pub fn Recommendation() -> impl IntoView {
    stylance::import_style!(style, "style/review.module.scss");

    let logos = RwSignal::new(vec![
        Logo {
            id: 1,
            src: "/static/logo/company/technopli.png",
            author: "Technopli",
            recommendation: Some("Great company to work with!"),
        },
        Logo {
            id: 2,
            src: "/static/logo/company/irouicome.png",
            author: "Irouicome",
            recommendation: Some("Reliable and professional service."),
        },
        Logo {
            id: 3,
            src: "/static/logo/company/astree_software.png",
            author: "Astree Software",
            recommendation: Some("Innovative and customer-focused."),
        },
    ]);

    let current_logo_index = RwSignal::new(0);
    let current_logo = RwSignal::new(logos.get_untracked()[current_logo_index.get_untracked()]);

    view! {
       <div class=style::recommendation_container on:scroll=move |_event| {
            current_logo_index.update(|index| *index +=1);
            current_logo.update(|current_logo| *current_logo = logos.get_untracked()[current_logo_index.get_untracked()]);
        }>
                {
                    logos.get_untracked().iter().map(|logo| {
                    let logo_clone = *logo;
                        view! {
                            <div class=style::logo_recommandation_container>
                                <img src=logo.src alt=logo.author class=style::logo />
                                <Show when=move || { current_logo.get().recommendation.is_some() && logo_clone.id == current_logo.get().id }>
                                    <div class=style::recommendation>
                                        <p >{current_logo.get().recommendation}</p>
                                        <h3 class=style::card_author>{current_logo.get().author}</h3>
                                    </div>
                                </Show>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }
        </div>
    }
}
