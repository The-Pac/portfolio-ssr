use leptos::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Metric {
    name: String,
    value: f64,
    rating: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ResourceBreakdown {
    #[serde(rename = "type")]
    resource_type: String,
    #[serde(rename = "sizeKo")]
    size_ko: f64,
}

#[component]
pub fn WebsitePerformance() -> impl IntoView {
    stylance::import_style!(
        #[allow(dead_code)]
        style, "style/website_performance.module.scss");

    let lcp = RwSignal::new(None::<Metric>);
    let cls = RwSignal::new(None::<Metric>);
    let inp = RwSignal::new(None::<Metric>);
    let fcp = RwSignal::new(None::<Metric>);
    let ttfb = RwSignal::new(None::<Metric>);
    let total_size_ko = RwSignal::new(None::<Metric>);
    let total_size_breakdown: RwSignal<Option<Vec<ResourceBreakdown>>> = RwSignal::new(None);


    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use web_sys::{window, CustomEvent};

        #[derive(Deserialize, Debug)]
        struct SizeData {
            name: String,
            #[serde(rename = "sizeKo")]
            size_ko: f64,
            breakdown: Vec<ResourceBreakdown>,
        }

        #[derive(Deserialize, Debug)]
        struct MetricData {
            name: String,
            value: f64,
            rating: String,
        }

        Effect::new(move |_| {
            if let Some(window) = window() {
                let callback = Closure::wrap(Box::new(move |event: CustomEvent| {
                    if let Ok(size_data) =
                        serde_wasm_bindgen::from_value::<SizeData>(event.detail())
                    {
                        if size_data.name == "TOTAL_SIZE" {
                            total_size_ko.set(Some( Metric { name: "Taille".to_string(), value :size_data.size_ko, rating: "good".to_string() }));
                            total_size_breakdown.set(Some(size_data.breakdown));
                            return;
                        }
                    }

                    if let Ok(metric_data) =
                        serde_wasm_bindgen::from_value::<MetricData>(event.detail())
                    {
                        let metric = Metric {
                            name: metric_data.name.clone(),
                            value: metric_data.value,
                            rating: metric_data.rating,
                        };

                        match metric_data.name.as_str() {
                            "LCP" => lcp.set(Some(metric)),
                            "CLS" => cls.set(Some(metric)),
                            "INP" => inp.set(Some(metric)),
                            "FCP" => fcp.set(Some(metric)),
                            "TTFB" => ttfb.set(Some(metric)),
                            _ => {}
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = window.add_event_listener_with_callback(
                    "webvitals",
                    callback.as_ref().unchecked_ref(),
                );

                callback.forget();
            }
        });
    }

    view! {
        <div class=style::website_performance>
            <h2 class=style::website_performance_title>"Structure & Performance"</h2>
            <div class=style::website_performance_introduction>
                <p>"J'ai pensé ce site avec l'idée "</p>
                <q class=style::website_performance_highlight cite="https://fr.wikipedia.org/wiki/Colin_Chapman#:~:text=%C2%AB%20Ce%20qui%20est%20l%C3%A9ger%20est%20bien%20%C2%BB%20(%C2%AB%20Light%20is%20Right%20%C2%BB)%20.">
                  "Light is Right"
                </q>
                <p class=style::website_performance_citation>
                    "- Colin Chapman, "
                    <cite>"Philosophie du Design Automobile"</cite>
                </p>
                <p>"et j'ai choisi ces technologies pour rester fidèle à cette philosophie."</p>
            </div>
            <div class=style::website_performance_story>
                <div class=style::website_performance_tech_and_narrative>
                    <div class=style::website_performance_tech_badge>
                        <h3 class=style::website_performance_badge_label>"Conçu avec"</h3>
                        <div class=style::website_performance_badge_stack>
                            <a href="https://leptos.dev/">"Leptos (SSR)"</a>
                            <a href="https://rust-lang.org/fr/">"Rust"</a>
                            <a href="https://github.com/tokio-rs/axum">"Axum"</a>
                            <a href="https://github.com/launchbadge/sqlx">"SQLx"</a>
                            <a href="https://github.com/basro/stylance-rs">"Stylance"</a>
                        </div>
                    </div>
                    <p class=style::website_performance_narrative>
                        "Imaginez un site de "
                        <MetricInline metric=total_size_ko unit="ko"/>
                        " qui se charge presque instantanément. Le plus gros élément visuel apparaît en "
                        <MetricInline metric=lcp unit="ms"/>
                        ". "
                        "Rien ne bouge pendant que vous lisez "
                        <MetricInline metric=cls unit=""/>
                        " de décalage seulement. "
                        "Chaque clic répond en "
                        <MetricInline metric=inp unit="ms"/>
                        ". "
                        "Le premier pixel s'affiche après "
                        <MetricInline metric=fcp unit="ms"/>
                        ", et le serveur répond en "
                        <MetricInline metric=ttfb unit="ms"/>
                        "."
                    </p>
                </div>
                <Show
                    when=move || total_size_breakdown.get().is_some()
                    fallback=|| ()
                >
                    {move || {
                        let breakdown = total_size_breakdown.get().unwrap();

                        view! {
                            <div class=style::website_performance_breakdown>
                                <For
                                    each=move || breakdown.clone()
                                    key=|item| item.resource_type.clone()
                                    children=move |item| {
                                        view! {
                                            <div class=style::breakdown_item>
                                                <span>{item.resource_type.clone()}</span>
                                                <strong>{item.size_ko.clone()} "ko"</strong>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        }.into_any()
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
fn MetricInline(metric: RwSignal<Option<Metric>>, unit: &'static str) -> impl IntoView {
    stylance::import_style!(
        #[allow(dead_code)]
        style, "style/website_performance_metric.module.scss");

    view! {
        <span class=style::metric_inline>
            {move || {
                metric.get().map(|m| {
                    let rating_class = match m.rating.as_str() {
                        "good" => style::metric_good,
                        "needs-improvement" => style::metric_warning,
                        _ => style::metric_poor,
                    };

                    let formatted = if unit.is_empty() {
                        format!("{:.3} ({})", m.value, m.name)
                    } else {
                        format!("{:.0}{} ({})", m.value, unit, m.name)
                    };

                    view! {
                        <span class=rating_class>{formatted}</span>
                    }.into_any()
                }).unwrap_or_else(|| view! {
                    <span class=style::metric_loading>"..."</span>
                }.into_any())
            }}
        </span>
    }
}
