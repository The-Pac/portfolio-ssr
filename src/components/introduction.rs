use std::time::Duration;
use gloo_timers::future::sleep;
use leptos::prelude::*;
use leptos::task::spawn_local;
use rand::Rng;

#[component]
pub fn Introduction() -> impl IntoView {
    stylance::import_style!(style, "style/introduction.module.scss");

    let developper_types = vec!["Front-End.", "Back-End.", "Full-stack."];
    let hobbies = vec!["Motard.", "Joueur.", "Powerlifter.", "Plongeur."];

    let developper_type_text = RwSignal::new(String::new());
    let hobbie_text = RwSignal::new(String::new());
    let intro_hobby_text  = RwSignal::new(String::new());

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
    });

    view! {
        <div class=style::intro_container>
            <b>"Salut,"</b>
            <h1 class=style::intro_line_1>"Je suis Baptiste."</h1>
            <h2 class=style::intro_line_2>"Un développeur "{move || developper_type_text}</h2>
            <Show when=move || developper_type_text.get() == developper_types[developper_types.len()-1]>
                <h2 class=style::intro_line_3>{move || intro_hobby_text}{move || hobbie_text}</h2>
            </Show>
        </div>
    }
}

async fn write_like_human(text: String, signal: RwSignal<String>) {
    let mut rng = rand::thread_rng();

    for char in text.chars() {
        signal.update(|current_value| current_value.push(char));
        let delay = rng.gen_range(50..150);
        sleep(Duration::from_millis(delay)).await;
    }
}

async fn clear_text(signal: RwSignal<String>) {
    let mut rng = rand::thread_rng();

    while !signal.get_untracked().is_empty() { 
        signal.update(|current_value| {
            current_value.pop();
        });
        let delay = rng.gen_range(25..85);
        sleep(Duration::from_millis(delay)).await;
    }
}