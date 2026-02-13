use std::path::PathBuf;
use dotenv::dotenv;
use portfolio_ssr::libs::database::init_database;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio_ssr::app::*;

    dotenv().ok();

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
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let use_tls = std::env::var("USE_TLS")
        .unwrap_or_else(|_| "false".to_string()) == "true";

    if use_tls {
        use axum_server::tls_rustls::RustlsConfig;

        let cert_path = std::env::var("TLS_CERT_PATH")
            .expect("TLS_CERT_PATH not found");
        let key_path = std::env::var("TLS_KEY_PATH")
            .expect("TLS_KEY_PATH not found");

        let tls_config = RustlsConfig::from_pem_file(
            PathBuf::from(cert_path),
            PathBuf::from(key_path),
        )
            .await
            .expect("failed to open TLS certificats ");

        println!("listening on https://{}", addr);
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}
#[cfg(not(feature = "ssr"))]
pub fn main() {

}