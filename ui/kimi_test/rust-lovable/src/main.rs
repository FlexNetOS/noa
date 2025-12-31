use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod app;
mod components;
mod core;
mod utils;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting Rust Lovable - A conversational UI builder");

    #[cfg(feature = "web")]
    {
        info!("Starting web server...");
        // Launch web server for fullstack mode
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
        
        let app = axum::Router::new()
            .nest_service("/assets", tower_http::services::ServeDir::new("dist"))
            .fallback(dioxus::fullstack::render::axum_handler(App));

        info!("Server listening on http://localhost:8080");
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
    }

    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        info!("Launching desktop/mobile application...");
        dioxus::launch(App);
    }

    Ok(())
}