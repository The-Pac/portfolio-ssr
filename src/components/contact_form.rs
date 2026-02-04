use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn ContactForm() -> impl IntoView {
    stylance::import_style!(style, "style/contact_form.module.scss");

    let name = RwSignal::new("".to_string());
    let surname = RwSignal::new("".to_string());
    let email = RwSignal::new("".to_string());
    let company = RwSignal::new("".to_string());
    let message = RwSignal::new("".to_string());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email_value = email.get();

        if !email_value.contains('@') {
            log!("Email invalide");
            return;
        }

        send_contact_form(
            name.get(),
            surname.get(),
            email_value,
            company.get(),
            message.get(),
        );
    };

    view! {
        <form class=style::contact_form_container on:submit=on_submit>
            <input
                id=0
                placeholder="Nom"
                class=style::name_input
                type="text"
                bind:value=name
                required=true
            />

            <input
                id=1
                placeholder="Prénom"
                class=style::surname_input
                type="text"
                bind:value=surname
                required=true
            />

            <input
                id=2
                placeholder="Email"
                class=style::email_input
                type="email"
                bind:value=email
                required=true
            />

            <input
                id=3
                placeholder="Entreprise"
                class=style::company_input
                type="text"
                bind:value=company
                required=false
            />

            <textarea
                placeholder="Dites-moi comment je peux vous être utile ! 💬"
                class=style::message_textarea
                bind:value=message
                required=true>
            </textarea>

            <button type="submit" class=style::send_button>
                <b>"Envoyer"</b>
            </button>
        </form>
    }
}

fn send_contact_form(
    name: String,
    surname: String,
    email: String,
    company: String,
    message: String,
) {
    log!(
        "Sending contact form:\nName: {} {}\nEmail: {}\nCompany: {}\nMessage: {}",
        name,
        surname,
        email,
        company,
        message
    );
}
