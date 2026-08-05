use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContactForm {
    name: String,
    surname: String,
    email: String,
    company: String,
    message: String,
}
#[component]
pub fn ContactForm() -> impl IntoView {
    stylance::import_style!(
        #[allow(dead_code)]
        style, "style/contact.module.scss");



    let name = RwSignal::new(String::new());
    let surname = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let company = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let status: RwSignal<Option<String>> = RwSignal::new(None);

    let is_ready_to_send = Memo::new(move |_| {
        !name.get().trim().is_empty()
            && !surname.get().trim().is_empty()
            && !email.get().trim().is_empty()
            && email.get().contains('@')
            && !message.get().trim().is_empty()
    });

    let send_form = Action::new(|contact_form: &ContactForm| {
        let contact_form = contact_form.clone();
        async move { send_contact_form(contact_form).await }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email_value = email.get();

        if !email_value.contains('@') {
            log!("Invalid email");
            return;
        }

        let contact_form = ContactForm {
            name: name.get(),
            surname: surname.get(),
            email: email_value,
            company: company.get(),
            message: message.get(),
        };
        send_form.dispatch(contact_form);
    };

    Effect::new(move || {
        if let Some(result) = send_form.value().get() {
            match result {
                Ok(_) => {
                    status.set(Some("Message sent!".to_string()));
                    name.set(String::new());
                    surname.set(String::new());
                    email.set(String::new());
                    message.set(String::new());
                    company.set(String::new());
                }
                Err(_) => {
                    status.set(Some("Failed to send message".to_string()));
                }
            }
        }
    });

    view! {
        <section class=style::contact>
            <h2 class=style::contact_title>Contactez-moi</h2>
            <form class=style::contact_form on:submit=on_submit>
                <input
                    id=0
                    placeholder="Nom"
                    class=style::contact_form_name_input
                    type="text"
                    bind:value=name
                    required=true
                />

                <input
                    id=1
                    placeholder="Prénom"
                    class=style::contact_form_surname_input
                    type="text"
                    bind:value=surname
                    required=true
                />

                <input
                    id=2
                    placeholder="Email"
                    class=style::contact_form_email_input
                    type="email"
                    bind:value=email
                    required=true
                />

                <input
                    id=3
                    placeholder="Entreprise"
                    class=style::contact_form_company_input
                    type="text"
                    bind:value=company
                    required=false
                />

                <textarea
                    id=4
                    placeholder="Dites-moi comment je peux vous être utile !"
                    class=style::contact_form_message_textarea
                    bind:value=message
                    required=true>
                </textarea>

                <button id=5 type="submit" aria-label="Envoyer le fomulaire de contact" disabled=move || !is_ready_to_send.get() || send_form.pending().get() class=style::contact_form_send_button>
                    <b>"Envoyer"</b>
                </button>
            </form>
        </section>
    }
}

#[derive(Serialize)]
struct DiscordWebhook {
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
    fields: Vec<DiscordField>,
}

#[derive(Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool,
}

#[server(SendContactForm, "/api")]
pub async fn send_contact_form(contact_form: ContactForm) -> Result<(), ServerFnError> {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set");
    let embed = DiscordEmbed {
        title: "New message".to_string(),
        description: "A new message from the contact form".to_string(),
        color: 0x5865F2,
        fields: vec![
            DiscordField {
                name: "Name : ".to_string(),
                value: format!(
                    "{} {}",
                    contact_form.name.clone(),
                    contact_form.surname.clone()
                ),
                inline: true,
            },
            DiscordField {
                name: "Email : ".to_string(),
                value: contact_form.email.clone(),
                inline: true,
            },
            DiscordField {
                name: "Message : ".to_string(),
                value: contact_form.message.clone(),
                inline: false,
            },
            DiscordField {
                name: "Company : ".to_string(),
                value: contact_form.company.clone(),
                inline: false,
            },
        ],
    };

    let payload = DiscordWebhook {
        content: None,
        embeds: vec![embed],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string());

    if !response.unwrap().status().is_success() {
        return Err(ServerFnError::ServerError(
            "Failed to send to Discord".to_string(),
        ));
    }

    Ok(())
}
