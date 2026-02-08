use gloo_timers::future::sleep;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use rand::Rng;
use std::time::Duration;

#[component]
pub fn Introduction() -> impl IntoView {
    stylance::import_style!(style, "style/introduction.module.scss");

    let developper_types = vec!["Front-End.", "Back-End.", "Full-stack."];
    let hobbies = vec!["Motard.", "Joueur.", "Powerlifter.", "Plongeur."];
    let social_medias: Vec<SocialMedia> = vec![
        SocialMedia {
            url: "https://github.com/The-Pac",
            image: "/logo/programming_language/devops_and_infrastructure/github-icon.svg",
            alt: "",
        },
        SocialMedia {
            url: "https://fr.linkedin.com/in/baptiste-arsac",
            image: "/logo/social_media/linkedin-icon.svg",
            alt: "",
        },
    ];

    let developper_type_text = RwSignal::new(String::new());
    let hobbie_text = RwSignal::new(String::new());
    let intro_hobby_text = RwSignal::new(String::new());
    let is_scrolled = RwSignal::new(false);

    let dev_types = developper_types.clone();

    Effect::new(move |_| {
        let dev_types = dev_types.clone();
        let hobbies = hobbies.clone();
        spawn_local(async move {
            for (i, dev_type) in dev_types.iter().enumerate() {
                write_like_human(dev_type.to_string(), developper_type_text).await;
                sleep(Duration::from_millis(2000)).await;
                if i < dev_types.len() - 1 {
                    clear_text(developper_type_text).await;
                }
            }

            write_like_human("Mais aussi un ".to_string(), intro_hobby_text).await;

            for (i, hobby) in hobbies.iter().enumerate() {
                write_like_human(hobby.to_string(), hobbie_text).await;
                sleep(Duration::from_millis(2000)).await;
                if i < hobbies.len() - 1 {
                    clear_text(hobbie_text).await;
                }
            }
        });

        let handle_scroll = move || {
            let scroll_y = window().scroll_y().unwrap_or(0.0);
            is_scrolled.set(scroll_y > 50.0);
        };

        let window = window();
        let closure =
            leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                handle_scroll();
            }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    });


    view! {
        <div
            class=style::intro_container
            class=(style::scrolled, move || is_scrolled.get())
        >
            <div class=style::social_media_container>
                {
                    social_medias.into_iter()
                    .map(|social_media |{
                        view! {
                            <a href=social_media.url>
                                <img src=social_media.image alt=format!("Icône pour {}", social_media.alt)/>
                            </a>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
            <div class=style::intro_content>
                <b>"Salut,"</b>
                <h1 class=style::intro_line_1>"Je suis Baptiste."</h1>
                <h2 class=style::intro_line_2>"Un développeur "{move || developper_type_text}</h2>
                <Show when=move || developper_type_text.get() == developper_types[developper_types.len()-1]>
                    <h2 class=style::intro_line_3>{move || intro_hobby_text}{move || hobbie_text}</h2>
                </Show>
            </div>
        </div>
    }
}

struct SocialMedia {
    url: &'static str,
    image: &'static str,
    alt: &'static str,
}

async fn write_like_human(text: String, signal: RwSignal<String>) {
    let mut rng = rand::rng();

    for char in text.chars() {
        signal.update(move |current_value| current_value.push(char));
        let delay = rng.random_range(50..150);
        sleep(Duration::from_millis(delay)).await;
    }
}

async fn clear_text(signal: RwSignal<String>) {
    let mut rng = rand::rng();

    while !signal.get_untracked().is_empty() {
        signal.update(move |current_value| {
            current_value.pop();
        });
        let delay = rng.random_range(25..85);
        sleep(Duration::from_millis(delay)).await;
    }
}
