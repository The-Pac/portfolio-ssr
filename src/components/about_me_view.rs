use leptos::prelude::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    stylance::import_style!(pub style, "style/about_me.module.scss");

    view! {
        <div class=style::about_me>
            <img class=style::about_me_img src="/image/me.jpg"/>
            <div class=style::introduction>
                <span class=style::introduction_tag>"À propos de moi"</span>
                <h2 class=style::introduction_heading>
                    "Développeur"<br/>
                    <em>"Junior"</em><br/>
                    "motivé."
                </h2>

                <p class=style::introduction_body>
                  "J'ai eu l'opportunité de participer à divers projets et d'apprendre
                  aux côtés de personnes talentueuses."
                </p>
                <p class=style::introduction_body>
                  "Curieux et déterminé, je suis toujours prêt à relever de nouveaux défis
                  et à perfectionner mes compétences en développement."
                </p>
            </div>
        </div>
    }
}
