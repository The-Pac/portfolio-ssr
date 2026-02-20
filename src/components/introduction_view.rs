use gloo_timers::future::sleep;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use std::time::Duration;
use crate::components::svg_icon::SvgIcon;

#[component]
pub fn Introduction() -> impl IntoView {
    stylance::import_style!(style, "style/introduction.module.scss");


    let developper_types = vec!["Front-End.", "Back-End.", "Full-stack."];
    let hobbies = vec!["Motard.", "Joueur.", "Powerlifter.", "Plongeur."];

    let social_medias: Vec<SocialMedia> = vec![
        SocialMedia {
            url: "https://github.com/The-Pac",
            image: "/logo/programming_language/devops_and_infrastructure/github-icon.svg",
            alt: "GitHub de Baptiste Arsac",
        },
        SocialMedia {
            url: "https://fr.linkedin.com/in/baptiste-arsac",
            image: "/logo/social_media/linkedin-icon.svg",
            alt: "Profil LinkedIn de Baptiste Arsac",
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
                sleep(Duration::from_millis(1000)).await;
                if i < dev_types.len() - 1 {
                    clear_text(developper_type_text).await;
                }
            }

            write_like_human("Mais aussi un ".to_string(), intro_hobby_text).await;

            for (i, hobby) in hobbies.iter().enumerate() {
                write_like_human(hobby.to_string(), hobbie_text).await;
                sleep(Duration::from_millis(1000)).await;
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
            class=style::introduction
            class=(style::scrolled, move || is_scrolled.get())
        >
            <div class=style::introduction_social_media>
                {
                    social_medias.into_iter()
                    .map(|social_media |{
                        view! {
                            <a class=style::social_media_link href=social_media.url>
                                <SvgIcon class=style::social_media_img.to_string() src=social_media.image.to_string()/>
                            </a>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
            <div class=style::introduction_content>
                <strong class=style::introduction_greeting>"Salut,"</strong>
                <h1 class=style::introduction_line_1>
                    "Je suis Baptiste Arsac."<br/>
                    <span class=style::introduction_line_2>
                        "Un développeur "{move || developper_type_text}
                    </span>
                </h1>
                <Show when=move || developper_type_text.get() == developper_types[developper_types.len()-1]>
                    <p class=style::introduction_line_3>{move || intro_hobby_text}{move || hobbie_text}</p>
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
    for char in text.chars() {
        signal.update(move |current_value| current_value.push(char));
        let delay = random_range_js(50,150);
        sleep(Duration::from_millis(delay)).await;
    }
}

async fn clear_text(signal: RwSignal<String>) {
    while !signal.get_untracked().is_empty() {
        signal.update(move |current_value| {
            current_value.pop();
        });
        let delay = random_range_js(25,85);
        sleep(Duration::from_millis(delay)).await;
    }
}

fn random_range_js(min: u64, max: u64) -> u64 {
    let random = js_sys::Math::random();
    min + (random * (max - min) as f64) as u64
}