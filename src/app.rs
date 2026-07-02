use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Script, Style, Stylesheet, Title};
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
                <Meta name="description" content="Développeur Full Stack à Montpellier, je crée des applications modernes et des sites web sur mesure. Disponible en CDI, freelance ou pour votre projet digital."/>
        
                <Meta http_equiv="Content-Type" content="charset=utf-8"/>
                <Meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Meta name="author" content="Arsac Baptiste"/>
                <Meta name="theme-color" content="#214f46"/>
                <Meta name="robots" content="index, follow"/>

                <Meta property="og:title" content="Portfolio – Arsac Baptiste"/>
                <Meta property="og:description" content="Développeur Full Stack à Montpellier, \
                spécialisé en applications web modernes et sites sur mesure. Découvrez mes projets." />
                <Meta property="og:type" content="website"/>
                <Meta property="og:url" content="https://www.arsac-baptiste.dev"/>
                <Meta property="og:image" content="https://www.arsac-baptiste.dev/image/me.webp"/>

                <Link rel="icon" href="/favicon.ico" type_="image/x-icon"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <Script type_="application/ld+json">
                    {r#"
                    [
                        {
                            "@context": "https://schema.org",
                            "@type": "Person",
                            "name": "Arsac Baptiste",
                            "url": "https://arsac-baptiste.dev",
                            "image": "https://arsac-baptiste.dev/image/me.webp",
                            "jobTitle": "Développeur Full Stack",
                            "address": {
                                "@type": "PostalAddress",
                                "addressLocality": "Montpellier",
                                "addressCountry": "FR"
                            },
                            "sameAs": [
                                "https://fr.linkedin.com/in/baptiste-arsac",
                                "https://github.com/baptiste-arsac"
                            ]
                        },
                        {
                          "@context": "https://schema.org",
                          "@type": "Organization",
                          "name": "Arsac Baptiste",
                          "url": "https://arsac-baptiste.dev",
                          "logo": {
                            "@type": "ImageObject",
                            "url": "https://arsac-baptiste.dev/favicon.ico"
                          },
                          "founder": {
                            "@type": "Person",
                            "name": "Arsac Baptiste"
                          },
                          "sameAs": [
                            "https://fr.linkedin.com/in/baptiste-arsac",
                            "https://github.com/baptiste-arsac"
                          ]
                        },
                        {
                            "@context": "https://schema.org",
                            "@type": "WebSite",
                            "name": "Portfolio de Arsac Baptiste",
                            "url": "https://arsac-baptiste.dev",
                            "author": {
                                "@type": "Person",
                                "name": "Arsac Baptiste"
                            },
                            "publisher": {
                                "@type": "Organization",
                                "name": "Arsac Baptiste",
                                "url": "https://arsac-baptiste.dev",
                                "logo": {
                                    "@type": "ImageObject",
                                    "url": "https://arsac-baptiste.dev/favicon.ico"
                                }
                            }
                        },
                        {
                            "@context": "https://schema.org",
                            "@type": "BreadcrumbList",
                            "itemListElement": [
                                {
                                    "@type": "ListItem",
                                    "position": 1,
                                    "name": "Accueil",
                                    "item": "https://arsac-baptiste.dev"
                                }
                            ]
                        }
                    ]
                    "#}
                </Script>
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
        <Style>{include_str!("critical.css")}</Style>
        <Script type_="module">{include_str!("web-vitals-init.js")}</Script>
        <Script defer="true" src="https://arsac-baptiste.dev/umami/script.js" attr:data-website-id="b34ac937-30e8-4bb2-b908-ac27455ad56e"></Script>
        <Title text="Baptiste Arsac - Développeur Full-Stack"/>
        <Link
            rel="canonical"
            href="https://arsac-baptiste.dev/"
        />
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
    use crate::components::technical_stacks_view::TechnicalStack;
    use crate::components::website_performances_view::WebsitePerformance;
    use crate::components::projects_view::Project;
    use crate::components::recommendations_view::Recommendation;

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