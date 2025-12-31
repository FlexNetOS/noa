use dioxus_logger::tracing::{info, Level};

use rust_lovable::App;

#[cfg(feature = "server")]
use dioxus::fullstack::prelude::*;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting Rust Lovable - A conversational UI builder");

    info!("Starting fullstack server...");

    // If the Dioxus CLI is running, it proxies fullstack into the main address; otherwise we bind localhost.
    // For container/deployment use-cases, allow overriding the bind address.
    let address: std::net::SocketAddr = std::env::var("RUST_LOVABLE_ADDRESS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(dioxus::cli_config::fullstack_address_or_localhost);

    // Set up the axum router. `serve_dioxus_application` adds a fallback route that serves your component and
    // server functions.
    let router = axum::Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("dist"))
        .serve_dioxus_application(ServeConfigBuilder::default(), App);

    info!("Server listening on http://{address}");
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "server"))]
fn main() {
    // Initialize logging
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting Rust Lovable - A conversational UI builder");

    // For desktop/mobile/web client builds we just launch the app.
    dioxus::launch(App);
}