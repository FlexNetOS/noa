// Rust Lovable - Conversational UI Builder
// Dioxus 0.7 compatible - Updated: 2026-01-01
use dioxus_logger::tracing::{info, Level};

// Use the App component from app_ui (the Dioxus component)
use rust_lovable::app_ui::App;

fn main() {
    // Initialize logging
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting Rust Lovable - A conversational UI builder");

    // For dioxus 0.7, we use the unified launch approach
    // The CLI handles fullstack server setup automatically when needed
    dioxus::launch(App);
}
