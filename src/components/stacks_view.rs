use crate::models::stack::StackStructure;
use leptos::prelude::*;

#[component]
pub fn Stack() -> impl IntoView {
    stylance::import_style!(style, "style/stack.module.scss");

    let stacks = RwSignal::new(vec![
        StackStructure {
            logo: vec![
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
            title: "Front-End",
            description: "Ce que l'utilisateur voit et avec quoi il interagit sur un site",
        },
        StackStructure {
            logo: vec![
                "protocol/rest-api-icon.svg",
                "protocol/webrtc-icon.svg",
                "protocol/websocket-icon.svg",
            ],
            title: "Protocole",
            description:
                "Les règles qui permettent aux applications et sites de communiquer entre eux",
        },
        StackStructure {
            logo: vec![
                "database/mysql-icon.svg",
                "database/postgresql-icon.svg",
                "database/sqlite-icon.svg",
            ],
            title: "Base de donnée",
            description: "Gestion et stockage des données d'une application ou d'un site web",
        },
        StackStructure {
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
        StackStructure {
            logo: vec!["platform/linux-icon.svg", "platform/windows-icon.svg"],
            title: "Plateforme",
            description: "Les plateformes sont les systèmes qui exécutent les applications",
        },
        StackStructure {
            logo: vec!["devops_and_infrastructure/github-icon.svg",
                       "devops_and_infrastructure/gitlab-icon.svg",
                       "devops_and_infrastructure/postman-icon.svg",
                       "devops_and_infrastructure/jenkins-icon.svg"],
            title: "DevOps & Infrastructure",
            description: "Tout ce qui permet de déployer, faire tourner, automatiser et surveiller les applications en production",
        },
    ]);

    view! {
        <div class=style::technical_stack>
            <h1 class=style::technical_stack_title>"Mon Environnement Technique"</h1>
            <div class=style::stack_container>
                {move ||
                    stacks.get().into_iter()
                    .map(|stack : StackStructure|
                        view! {
                            <StackCard stack=stack/>
                        }
                    )
                    .collect::<Vec<_>>()
                }
            </div>
       </div>
    }
}

#[component]
fn StackCard(stack: StackStructure) -> impl IntoView {
    stylance::import_style!(style, "style/stack_card.module.scss");

    view! {
       <div class=style::stack_card>
            <h2 class=style::stack_card_title>{stack.title}</h2>
            <div class=style::stack_card_content>
                <p class=style::stack_card_description>{stack.description}</p>
                <div class=style::stack_card_logos>
                    {
                        stack.logo.into_iter()
                        .map(|logo |{
                            view! {
                                <img src=format!("/logo/programming_language/{}",logo) alt=format!("Icône pour {}", logo)/>
                            }
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
       </div>
    }
}
