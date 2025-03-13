use leptos::prelude::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    stylance::import_style!(style, "style/about_me.module.scss");

    view! {
        <div class=style::about_me_container>
            <img class=style::me src="/static/image/me.jpg" />
            <div class=style::quick_introduction_container>
                <p>"Développeur junior avec une motivation sans limites, j’ai eu l’opportunité de participer à divers projets et d’apprendre aux côtés de personnes talentueuses."</p>
                <p>"Curieux et déterminé, je suis toujours prêt à relever de nouveaux défis et à perfectionner mes compétences en développement."</p>
            </div>
        </div>
    }
}