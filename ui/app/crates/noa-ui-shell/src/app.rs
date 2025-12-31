use dioxus::prelude::*;

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

    // Dioxus reference styleguide (kept generic; no legacy project naming)
    #[layout(StyleguideLayout)]
    #[route("/styleguide")]
    StyleguideHome {},
    #[route("/styleguide/blog/:id")]
    StyleguideBlog { id: i32 },
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

            header {
                class: "app-header",
                h1 { "NOA UI - Conversational Builder" }
                Toolbar {}
            }

            main {
                class: "main-layout",

                Sidebar {}

                div { class: "canvas-section", UICanvas {} }

                div { class: "chat-section", ChatInterface {} }
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
        }
    }
}

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "settings-page",
            h1 { "Settings" }
        }
    }
}

#[component]
pub fn StyleguideLayout() -> Element {
    rsx! { crate::styleguide::demo::StyleguideLayout {} }
}

#[component]
pub fn StyleguideHome() -> Element {
    rsx! { crate::styleguide::demo::StyleguideHome {} }
}

#[component]
pub fn StyleguideBlog(id: i32) -> Element {
    rsx! { crate::styleguide::demo::StyleguideBlog { id } }
}
