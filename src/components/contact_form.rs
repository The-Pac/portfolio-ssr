use leptos::ev::SubmitEvent;
use leptos::logging::log;
use std::rc::Rc;
use leptos::prelude::*;
#[derive(Clone)]
struct FormField {
    placeholder: &'static str,
    required: bool,
    class: &'static str,
    input_type: &'static str,
}

#[component]
pub fn ContactForm() -> impl IntoView {
    stylance::import_style!(style, "style/contact_form.module.scss");

    let fields = vec![
        ("name", FormField {
            placeholder: "Nom",
            required: true,
            class: style::name_input,
            input_type: "text",
        }),
        ("surname", FormField {
            placeholder: "Prénom",
            required: true,
            class: style::surname_input,
            input_type: "text",
        }),
        ("email", FormField {
            placeholder: "Email",
            required: true,
            class: style::email_input,
            input_type: "email",
        }),
        ("company", FormField {
            placeholder: "Entreprise",
            required: false,
            class: style::company_input,
            input_type: "text",
        }),
    ];

    let form_data: Rc<Vec<(&str, RwSignal<String>)>> = Rc::new(
        fields
            .iter()
            .map(|(name, _)| (*name, RwSignal::new("".to_string())))
            .collect()
    );

    let (message,set_message) = signal("".to_string());

    let submit_data = form_data.clone();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email = submit_data.iter()
            .find(|(name, _)| *name == "email")
            .map(|(_, signal)| signal.get())
            .unwrap_or_default();

        if !email.contains('@') {
            log!("Email invalide");
            return;
        }

        send_contact_form(
            submit_data.iter().find(|(name, _)| *name == "name").map(|(_, signal)| signal.get()).unwrap_or_default(),
            submit_data.iter().find(|(name, _)| *name == "surname").map(|(_, signal)| signal.get()).unwrap_or_default(),
            email,
            submit_data.iter().find(|(name, _)| *name == "company").map(|(_, signal)| signal.get()).unwrap_or_default(),
            message.get(),
        );
    };

    let fields = Rc::new(fields);

    view! {
        <form class=style::contact_form_container on:submit=on_submit>
            {fields.iter().map(|(name, field)| {
                let form_data = form_data.clone();
                let value = form_data
                    .iter()
                    .find(|(n, _)| *n == *name)
                    .map(|(_, signals)| *signals)
                    .unwrap();

                view! {
                    <input
                        prop:placeholder=field.placeholder
                        class=field.class
                        type=field.input_type
                        bind:value=value
                        prop:required=field.required
                    />
                }
            }).collect::<Vec<_>>()}

            <textarea
                placeholder="Dites-moi comment je peux vous aider ! 💬"
                class=style::message_textarea
                bind:value=(message,set_message)
                required=true>
            </textarea>

            <button type="submit" class=style::send_button>
                <b>"Envoyer"</b>
            </button>
        </form>
    }
}

fn send_contact_form(name: String, surname: String, email: String, company: String, message: String) {
    log!(
        "Sending contact form:\nName: {} {}\nEmail: {}\nCompany: {}\nMessage: {}",
        name,
        surname,
        email,
        company,
        message
    );
}