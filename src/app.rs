use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Style, Stylesheet, Title};
use leptos_router::{components::{Route, Router, Routes}, path};


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    stylance::import_style!(style, "app.module.scss");
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio-ssr.css"/>
        <Style>{include_str!("../style/critical.scss")}</Style>

        <Title text="Baptiste Portfolio"/>

        <Router>
            <main class=style::main>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                }>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use crate::components::introduction::Introduction;
    use crate::components::review::Recommendation;
    use crate::components::contact_form::ContactForm;
    use crate::components::about_me::AboutMe;
    use crate::components::career_map::CareerMap;
    use crate::components::stack::Stack;

    view! {
        <Introduction/>
        <AboutMe/>
        <CareerMap/>
        <Stack/>
        <Recommendation/>
        <ContactForm/>
    }
}