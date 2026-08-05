use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SvgIcon(
    src: String,
    #[prop(optional)]
    alt: Option<String>,
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    style: Option<String>,
) -> impl IntoView {
    let svg_content = RwSignal::new(String::new());

    Effect::new({
        let src = src.clone();
        let class = class.clone();
        let style = style.clone();
        let alt = alt.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                use wasm_bindgen_futures::spawn_local;
                spawn_local({
                    let src = src.clone();
                    let class = class.clone();
                    let style = style.clone();
                    let alt = alt.clone();
                    async move {
                        if let Ok(content) = fetch_svg(&src).await {
                            let modified = inject_svg_attrs(
                                &content,
                                class,
                                style,
                                alt
                            );
                            svg_content.set(modified);
                        }
                    }
                });
            }
        }
    });

    view! {
        <span
            style="display: contents"
            inner_html=move || svg_content.get()
        />
    }
}

fn inject_svg_attrs(svg: &str, class: Option<String>, style: Option<String>, alt: Option<String>,) -> String {
    let mut result = svg.to_string();

    if let Some(ref a) = alt {
        if let (Some(start), Some(end)) = (result.find("<title>"), result.find("</title>")) {
            result.replace_range(start..end + 8, &format!("<title>{}</title>", a));
        }
    }

    if let Some(pos) = result.find("<svg") {
        let insert_pos = pos + 4;
        let mut attrs = String::new();

        if let Some(c) = class {
            attrs.push_str(&format!(r#" class="{}""#, c));
        }
        if let Some(s) = style {
            attrs.push_str(&format!(r#" style="{}""#, s));
        }

        if !attrs.is_empty() {
            result.insert_str(insert_pos, &attrs);
        }
    }

    result
}

#[cfg(feature = "hydrate")]
fn svg_cache() -> &'static std::sync::Mutex<std::collections::HashMap<String, String>> {
    use std::sync::OnceLock;
    static CACHE: OnceLock<std::sync::Mutex<std::collections::HashMap<String, String>>> = OnceLock::new();
    CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

#[cfg(feature = "hydrate")]
async fn fetch_svg(src: &str) -> Result<String, ()> {
    if let Ok(cache) = svg_cache().lock() {
        if let Some(cached) = cache.get(src) {
            return Ok(cached.clone());
        }
    }

    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::window;

    let win = window().ok_or(())?;
    let resp_value = JsFuture::from(win.fetch_with_str(src))
        .await
        .map_err(|_| ())?;
    let resp: web_sys::Response = resp_value.dyn_into().map_err(|_| ())?;
    let text = JsFuture::from(resp.text().map_err(|_| ())?)
        .await
        .map_err(|_| ())?;
    let content = text.as_string().ok_or(())?;

    if let Ok(mut cache) = svg_cache().lock() {
        cache.insert(src.to_string(), content.clone());
    }

    Ok(content)
}