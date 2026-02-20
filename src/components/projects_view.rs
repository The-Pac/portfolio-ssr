use crate::models;
use crate::models::database_error::ProjectError;
use crate::models::project::ProjectStatus;
use leptos::either::Either;
use leptos::prelude::*;
use crate::components::svg_icon::SvgIcon;

#[component]
pub fn Project() -> impl IntoView {
    stylance::import_style!(style, "style/project.module.scss");

    let projects_resource: LocalResource<Result<Vec<models::project::Project>, ProjectError>> =
        LocalResource::new(|| async { crate::server::project::get_all_projects().await });

    view! {
        <div class=style::project>
            <h2 class=style::project_title>"Mes Projets"</h2>
            <div class=style::project_cards>
                <Suspense fallback=move || view! { <p>"Chargement des projets..."</p> }>
                    {move || {
                        projects_resource.get().map(|result| {
                            match result {
                                Ok(projects) => {
                                    Either::Left(
                                        projects.into_iter()
                                            .map(|project| view! { <ProjectCard project=project/> })
                                            .collect_view()
                                    )
                                }
                                Err(e) => {
                                    Either::Right(view! {
                                        <p class="error">"Erreur lors du chargement des projets: " {e.to_string()}</p>
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

#[component]
fn ProjectCard(project: models::project::Project) -> impl IntoView {
    stylance::import_style!(style, "style/project_card.module.scss");
    let is_flipped = RwSignal::new(false);

    let status_class = move |status| match status {
        ProjectStatus::Idea => style::status_idea,
        ProjectStatus::InProgress => style::status_in_progress,
        ProjectStatus::Cancelled => style::status_cancelled,
        ProjectStatus::Completed => style::status_completed,
        ProjectStatus::Pending => style::status_pending,
        ProjectStatus::Archived => style::status_archived,
    };

    view! {
       <div class=style::project_card>
            <button
                class={move || {
                    if is_flipped.get() {
                        format!("{} {}", style::project_card_flip_button, style::flipped)
                    } else {
                        style::project_card_flip_button.to_string()
                    }
                }}
                on:click=move |_| is_flipped.update(|flipped| *flipped = !*flipped)
                aria-label="Retourner la carte"
            >
                <SvgIcon src="/logo/mdi/flip-forward-icon.svg".to_string()/>
            </button>
            <div class={move || {
                if is_flipped.get() {
                    format!("{} {}", style::project_card_inner, style::flipped)
                } else {
                    style::project_card_inner.to_string()
                }
            }}>
                <div class=style::project_card_front>
                    <div class=style::project_card_header>
                        <h3 class=style::project_card_title>{project.title}</h3>
                        <p class={ move || {
                            format!("{} {}", style::project_card_status, status_class(project.status.clone()))
                        }}>
                            {project.status.to_string()}
                        </p>
                    </div>
                    <div class=style::project_card_body>
                        <details class=style::project_card_description>
                            <summary>
                                "Voir la description"
                            </summary>
                            <p>
                               {
                                    project.description.as_ref()
                                        .map(|description| {
                                            if description.len() > 150 {
                                                format!("{}...", description.chars().take(150).collect::<String>())
                                            } else {
                                                description.clone()
                                            }
                                        })
                                        .unwrap_or_default()
                                }
                            </p>
                        </details>

                        {project.url_to_project.as_ref().map(|url|
                            view! {
                                <a
                                    class=style::project_card_link
                                    href=url.clone()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    <SvgIcon src="/logo/programming_language/devops_and_infrastructure/github-icon.svg".to_string()/>
                                </a>
                            }
                        )}
                    </div>

                </div>
                <div class=style::project_card_back>
                    <div class=style::project_card_stack>
                        {project.technologies
                            .iter()
                            .map(|tech: &models::technology::TechnologyWithLogo| {
                                if tech.logo_path.ends_with(".svg") {
                                    view! {
                                        <SvgIcon
                                            src=tech.logo_path.clone()
                                             alt=format!("{} icon", tech.name)
                                        />
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <img
                                            src=format!("{}", tech.logo_path)
                                            alt=format!("Icône pour {}", tech.name)
                                            title=format!("{}", tech.name)
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
       </div>
    }
}
