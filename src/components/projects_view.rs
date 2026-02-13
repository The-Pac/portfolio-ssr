use crate::models;
use crate::models::database_error::ProjectError;
use crate::models::project::ProjectStatus;
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn Project() -> impl IntoView {
    stylance::import_style!(style, "style/project.module.scss");

    let projects_resource: LocalResource<Result<Vec<models::project::Project>, ProjectError>> =
        LocalResource::new(|| async { crate::server::project::get_all_projects().await });

    view! {
        <div class=style::project>
            <h1 class=style::project_title>"Mes Projets"</h1>
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
                on:click=move |_| is_flipped.update(|f| *f = !*f)
                aria-label="Retourner la carte"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
                </svg>
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
                        <h2 class=style::project_card_title>{project.title}</h2>
                        <span class={ move || {
                            format!("{} {}", style::project_card_status, status_class(project.status.clone()))
                        }}>
                            {format!("{:?}",project.status)}
                        </span>
                    </div>
                    <div class=style::project_card_body>
                        <p class=style::project_card_description>
                            {project.description.as_ref()
                                .map(|description| description.clone().chars().take(150).collect::<String>())
                                .unwrap_or_default()
                            }
                            "..."
                        </p>
                        {project.url_to_project.as_ref().map(|url|
                            view! {
                                <a class=style::project_card_link href=url.clone() target="_blank" rel="noopener noreferrer">
                                    <img
                                        src="/logo/programming_language/devops_and_infrastructure/github-icon.svg"
                                        alt="Icône GitHub"
                                    />
                                </a>
                            }
                        )}
                    </div>
                </div>
                <div class=style::project_card_back>
                    <div class=style::project_card_stack>
                        {project.technologies.iter().map(|tech: &models::technology::TechnologyWithLogo| {
                            view! {
                                <img
                                    src=format!("{}", tech.logo_path)
                                    alt=format!("Icône pour {}", tech.name)
                                    title=format!("{} ({})", tech.name, tech.category_title)
                                />
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
       </div>
    }
}
