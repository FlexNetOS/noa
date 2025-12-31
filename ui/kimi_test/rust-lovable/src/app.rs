use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::canvas::UICanvas;
use crate::components::chat::ChatInterface;
use crate::components::sidebar::Sidebar;
use crate::components::toolbar::Toolbar;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/project/:id")]
    Project { id: String },
    #[route("/settings")]
    Settings {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Home() -> Element {
    let theme = use_signal(|| String::from("dark"));

    rsx! {
        div {
            class: "app-container {theme.read()}",

            // App Header
            header {
                class: "app-header",
                h1 { "Rust Lovable - Conversational UI Builder" }
                Toolbar {}
            }

            // Main Layout
            main {
                class: "main-layout",

                // Left Sidebar - Project Explorer
                Sidebar {}

                // Center - UI Canvas
                div {
                    class: "canvas-section",
                    UICanvas {}
                }

                // Right Panel - Chat Interface
                div {
                    class: "chat-section",
                    ChatInterface {}
                }
            }
        }
    }
}

#[component]
pub fn Project(id: String) -> Element {
    rsx! {
        div {
            class: "project-page",
            h1 { "Project: {id}" }
            // Project-specific content here
        }
    }
}

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "settings-page",
            h1 { "Settings" }
            // Settings content here
        }
    }
}
