#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::body::Body;
    use axum::http::{Request, Response};
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio_ssr::app::*;
    use portfolio_ssr::libs::database::init_database;
    use std::path::PathBuf;
    use std::time::Duration;
    use tower_http::compression::CompressionLayer;
    use tower_http::services::ServeDir;
    use tower_http::trace::TraceLayer;
    use tracing::Span;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    init_database()
        .await
        .expect("problem during initialization of the database");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback_service(
            ServeDir::new(leptos_options.site_root.as_ref())
                .precompressed_br()
                .precompressed_gzip(),
        )
        .with_state(leptos_options)
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request<_>| {
                    let path = req.uri().path();

                    let is_asset = path.starts_with("/logo/")
                        || path.starts_with("/pkg/")
                        || path.ends_with(".svg")
                        || path.ends_with(".webp")
                        || path.ends_with(".js")
                        || path.ends_with(".css")
                        || path.ends_with(".wasm");

                    if is_asset {
                        return Span::none();
                    }

                    let ip = req
                        .headers()
                        .get("x-forwarded-for")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info_span!(
                        "http",
                        method = %req.method(),
                        uri = %path,
                        ip = %ip,
                    )
                })
                .on_response(|res: &Response<_>, latency: Duration, _span: &Span| {
                    let millis = latency.as_millis();
                    if millis > 1000 {
                        tracing::warn!(status = %res.status(), latency_ms = millis, "slow request");
                    } else {
                        tracing::info!(status = %res.status(), latency_ms = millis);
                    }
                }),
        );

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
#[cfg(not(feature = "ssr"))]
pub fn main() {}
