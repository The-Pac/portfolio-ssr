use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Script, Style, Stylesheet, Title};
use leptos_router::components::RoutingProgress;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::time::Duration;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta name="description" content="Portfolio de Baptiste Arsac - Jeune développeur Full \
                Stack motivé basé à Montpellier. Créateur d'applications web modernes et sites sur mesure.\
                 Disponible pour CDI,freelance ou création de votre projet digital. Explorez mes réalisations !"/>
                <meta http_equiv="Content-Type" content="charset=utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="author" content="Arsac Baptiste"/>
                <meta name="theme-color" content="#214f46"/>

                <meta property="og:title" content="Portfolio – Arsac Baptiste"/>
                <meta property="og:description" content="Développeur Full Stack à Montpellier, \
                spécialisé en applications web modernes et sites sur mesure. Découvrez mes projets." />
                <meta property="og:type" content="website"/>
                <meta property="og:url" content="https://www.arsac-baptiste.dev"/>
                <meta property="og:image" content="https://www.arsac-baptiste.dev/image/me.jpg"/>

                <Link rel="icon" href="/favicon.ico" type_="image/x-icon"/>
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

    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio-ssr.css"/>
        <Style>{include_str!("critical.scss")}</Style>
        <Script type_="module">{include_str!("web-vitals-init.js")}</Script>

        <Title text="Baptiste Portfolio"/>

        <Router set_is_routing>
            <div class=style::routing_progress>
                <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
            </div>
            <main>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use crate::components::about_me_view::AboutMe;
    use crate::components::career_map_view::CareerMap;
    use crate::components::contact_view::ContactForm;
    use crate::components::introduction_view::Introduction;
    use crate::components::recommendations_view::Recommendation;
    use crate::components::technical_stacks_view::TechnicalStack;
    use crate::components::website_performances_view::WebsitePerformance;
    use crate::components::projects_view::Project;
    stylance::import_style!(style, "app.module.scss");

    view! {
        <Introduction/>
        <AboutMe/>
        <WebsitePerformance/>
        <CareerMap/>
        <TechnicalStack/>
        <hr class=style::separator/>
        <Project/>
        <hr class=style::separator/>
        <ContactForm/>
        <Recommendation/>
    }
}