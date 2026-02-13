use leptos::prelude::*;

#[derive(Clone, Debug)]
struct Metric {
    name: String,
    value: f64,
    rating: String,
}

#[component]
pub fn WebsitePerformance() -> impl IntoView {
    stylance::import_style!(style, "style/website_performance.module.scss");

    let lcp = RwSignal::new(None::<Metric>);
    let cls = RwSignal::new(None::<Metric>);
    let inp = RwSignal::new(None::<Metric>);
    let fcp = RwSignal::new(None::<Metric>);
    let ttfb = RwSignal::new(None::<Metric>);

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        use serde::Deserialize;
        use web_sys::{window, CustomEvent};

        #[derive(Deserialize)]
        struct MetricData {
            name: String,
            value: f64,
            rating: String,
        }

        Effect::new(move |_| {
            if let Some(window) = window() {
                let callback = Closure::wrap(Box::new(move |event: CustomEvent| {
                    if let Ok(detail) = serde_wasm_bindgen::from_value::<MetricData>(event.detail()) {
                        let metric = Metric {
                            name: detail.name.clone(),
                            value: detail.value,
                            rating: detail.rating,
                        };

                        match detail.name.as_str() {
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
                    callback.as_ref().unchecked_ref()
                );

                callback.forget();
            }
        });
    }

    view! {
            <div class=style::website_performance>
                <h1 class=style::website_performance_title>"Structure & Performance"</h1>
                <p class=style::website_performance_intro>
                        "J’ai pensé ce site avec l’idée "
                        <span class=style::website_performance_highlight>"« Light is Right »"</span>
                        " de Colin Chapman, et j’ai choisi ces technologies pour rester fidèle à cette philosophie."
                </p>
                <div class=style::website_performance_story>
                    <div class=style::website_performance_tech_badge>
                        <span class=style::website_performance_badge_label>"Conçu avec"</span>
                        <div class=style::website_performance_badge_stack>
                           <a href="https://leptos.dev/"><span>"Leptos (SSR)"</span></a>
                            <br/>
                            <a href="https://rust-lang.org/fr/"><span>"Rust"</span></a>
                            <br/>
                            <a href="https://github.com/tokio-rs/axum"><span>"Axum"</span></a>
                            <br/>
                            <a href="https://github.com/launchbadge/sqlx"><span>"SQLx (SQLite)"</span></a>
                            <br/>
                            <a href="https://github.com/basro/stylance-rs"><span>"Stylance"</span></a>
                        </div>
                    </div>
                    <p class=style::website_performance_narrative>
                        "Imaginez un site qui se charge presque instantanément. "
                        "Le plus gros élément visuel apparaît en "
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
            </div>
    }
}

#[component]
fn MetricInline(
    metric: RwSignal<Option<Metric>>,
    unit: &'static str,
) -> impl IntoView {
    stylance::import_style!(style, "style/website_performance_metric.module.scss");

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
                        format!("{:.0}{} ({})", m.value, unit,m.name)
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