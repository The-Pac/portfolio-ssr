use crate::components::svg_icon::SvgIcon;
use crate::models::database_error::TechnologyError;
use crate::models::technology::{TechnologyCategory, TechnologyWithLogo};
use leptos::either::Either;
use leptos::prelude::*;
use std::collections::HashMap;

type TechnologiesResourceType = LocalResource<(
    Result<Vec<TechnologyCategory>, TechnologyError>,
    Result<HashMap<String, Vec<TechnologyWithLogo>>, TechnologyError>,
)>;

#[component]
pub fn TechnicalStack() -> impl IntoView {
    stylance::import_style!(style, "style/technical_stack.module.scss");

    let technologies_resource: TechnologiesResourceType = LocalResource::new(|| async {
        (
            crate::server::technology::get_all_technology_categories().await,
            crate::server::technology::get_technologies_by_category().await,
        )
    });

    view! {
        <div class=style::technical_stack>
            <h2 class=style::technical_stack_title>"Mon Environnement Technique"</h2>
            <div class=style::technical_stack_container>
                <Suspense fallback=move || view! { <p>"Chargement des technologies..."</p> }>
                    {move ||
                        technologies_resource.get().map(|result| {
                           match result {
                                (Ok(technology_categories), Ok(technology_group_by_category)) => {
                                    Either::Left(
                                        technology_categories
                                            .into_iter()
                                            .map(|technology_category| {
                                                let technologies = technology_group_by_category
                                                    .get(&technology_category.title)
                                                    .cloned()
                                                    .unwrap_or_default();

                                                view! {
                                                    <TechnicalStackCard
                                                        technology_category=technology_category
                                                        technologies_with_logo=technologies
                                                    />
                                                }
                                            })
                                            .collect_view()
                                    )
                                }

                                (Err(e), _) | (_, Err(e)) => {
                                    Either::Right(view! {
                                        <p class="error">
                                            "Erreur lors du chargement des technologies: "
                                            {e.to_string()}
                                        </p>
                                    })
                                }
                            }
                        })
                    }
                </Suspense>
            </div>
       </div>
    }
}

#[component]
fn TechnicalStackCard(
    technology_category: TechnologyCategory,
    technologies_with_logo: Vec<TechnologyWithLogo>,
) -> impl IntoView {
    stylance::import_style!(style, "style/technical_stack_card.module.scss");

    let is_open = RwSignal::new(false);


    view! {
       <div
            class={move || {
                if is_open.get() {
                    format!("{} {}", style::stack_card, style::open)
                } else {
                    style::stack_card.to_string()
                }
            }}
            on:click=move |_| is_open.update(|open| *open = !*open)
       >
            <h3 class=style::stack_card_title>
                {technology_category.title}
            </h3>
            <div class=style::stack_card_content>
                <p class=style::stack_card_description>{technology_category.description}</p>
                <div class=style::stack_card_logos>
                    {
                        technologies_with_logo.into_iter()
                        .map(|technology_with_logo: TechnologyWithLogo| {
                            if technology_with_logo.logo_path.ends_with(".svg") {
                                view! {
                                    <SvgIcon
                                        src=technology_with_logo.logo_path.clone()
                                    />
                                }
                                .into_any()
                            } else {
                                view! {
                                    <img
                                        src=format!("{}", technology_with_logo.logo_path)
                                        alt=format!("Icône pour {}", technology_with_logo.name)
                                        title=format!("{} ({})", technology_with_logo.name, technology_with_logo.category_title)
                                        loading="lazy"
                                        decoding="async"
                                    />
                                }
                                .into_any()
                            }
                        })
                        .collect_view()
                    }
                </div>
            </div>
       </div>
    }
}
