/*use leptos::prelude::*;


#[derive(Clone)]
pub enum ProjectStatus {
    Idea,        // Just an idea
    Planned,     // Project is planned but not started
    InProgress,  // Work is currently ongoing
    OnHold,      // Temporarily paused
    Cancelled,   // Project was cancelled
    Completed,   // Project is finished
    Pending,     // Waiting on something before proceeding
    Archived,    // No longer active, stored for reference
}
#[derive(Clone)]
struct ProjectStructure {
    stacks : Vec<&'static str>,
    title: &'static str,
    description: &'static str,
    status: ProjectStatus,
    url : Option<&'static str>
}

#[component]
pub fn Project() -> impl IntoView {
    stylance::import_style!(style, "style/project.module.scss");

    let projects = RwSignal::new(vec![
        ProjectStructure {
            stacks: vec![
                "front_end/angular-icon.svg",
                "front_end/javascript-icon.svg",
                "front_end/reactjs-icon.svg",
                "front_end/sass-icon.svg",
                "front_end/svelte-icon.svg",
                "front_end/typescript-icon.svg",
                "front_end/vuejs-icon.svg",
                "front_end/css-icon.svg",
                "front_end/html5-icon.svg",
            ],
            title: "",
            description: "",
            status : ProjectStatus::,
            url : Some("https://github.com/The-Pac/portfolio-ssr"),
        },
        ProjectStructure {
            logo: vec![
                "protocol/rest-api-icon.svg",
                "protocol/webrtc-icon.svg",
                "protocol/websocket-icon.svg",
            ],
            title: "Protocole",
            description: "Les règles qui permettent aux applications et sites de communiquer entre eux",
        },
        ProjectStructure {
            logo: vec![
                "database/mysql-icon.svg",
                "database/postgresql-icon.svg",
                "database/sqlite-icon.svg",
            ],
            title: "Base de donnée",
            description: "Gestion et stockage des données d'une application ou d'un site web",
        },
        ProjectStructure {
            logo: vec![
                "back_end/java-icon.svg",
                "back_end/laravel-icon.svg",
                "back_end/leptos-icon.svg",
                "back_end/lua-icon.svg",
                "back_end/nodejs-icon.svg",
                "back_end/php-icon.svg",
                "back_end/rust-icon.svg",
                "back_end/spring-icon.svg",
                "back_end/tauri-icon.svg",
            ],
            title: "Back-End",
            description: "Le back-end gère les données et le fonctionnement en arrière-plan",
        },
        ProjectStructure {
            logo: vec!["platform/linux-icon.svg", "platform/windows-icon.svg"],
            title: "Plateforme",
            description: "Les plateformes sont les systèmes qui exécutent les applications",
        },
    ]);

    view! {
       <h1 class=style::title>Les technologies qui me permettent de construire des projets</h1>
       <div class=style::project_container>
            {move ||
                projects.get().into_iter()
                .map(|stack : ProjectStructure|
                    view! {
                        <ProjectCard project=project/>
                    }
                )
                .collect::<Vec<_>>()
            }
       </div>
    }
}

#[component]
fn ProjectCard(project: ProjectStructure) -> impl IntoView {
    stylance::import_style!(style, "style/project_card.module.scss");

    view! {
       <div class=style::project_card>
            <div class=style::card_inner>
                <div class=style::card_front>
                    <h2>{project.title}</h2>
                    <p>{project.description}</p>
                </div>
            </div>
       </div>
    }
}
*/
