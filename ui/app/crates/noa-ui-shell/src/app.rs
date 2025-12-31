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

    // Migrated reference UI (from `ui/lilrep`)
    #[layout(LilrepLayout)]
    #[route("/lilrep")]
    LilrepHome {},
    #[route("/lilrep/blog/:id")]
    LilrepBlog { id: i32 },
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
pub fn LilrepLayout() -> Element {
    rsx! { crate::styleguide::lilrep::LilrepLayout {} }
}

#[component]
pub fn LilrepHome() -> Element {
    rsx! { crate::styleguide::lilrep::LilrepHome {} }
}

#[component]
pub fn LilrepBlog(id: i32) -> Element {
    rsx! { crate::styleguide::lilrep::LilrepBlog { id } }
}
