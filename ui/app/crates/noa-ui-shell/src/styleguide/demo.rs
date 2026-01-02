use dioxus::prelude::*;

/// A small Dioxus styleguide area.
#[component]
pub fn StyleguideLayout() -> Element {
    rsx! {
        div {
            class: "styleguide-layout",
            
            // Simple navbar
            nav {
                class: "navbar bg-base-200",
                div {
                    class: "navbar-start",
                    Link { to: crate::app::Route::StyleguideHome {}, class: "btn btn-ghost", "Home" }
                    Link { to: crate::app::Route::StyleguideBlog { id: 1 }, class: "btn btn-ghost", "Blog" }
                }
            }

            Outlet::<crate::app::Route> {}
        }
    }
}

#[component]
pub fn StyleguideHome() -> Element {
    rsx! {
        div {
            class: "hero min-h-96 bg-base-200",
            div {
                class: "hero-content text-center",
                div {
                    class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "NOA Styleguide" }
                    p { class: "py-6", "Component library and design system for the NOA platform." }
                }
            }
        }
    }
}

#[component]
pub fn StyleguideBlog(id: i32) -> Element {
    rsx! {
        div {
            class: "p-6",

            // Content
            h1 { class: "text-3xl font-bold mb-4", "This is blog #{id}!" }
            p { class: "mb-4", "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            div {
                class: "flex gap-4",
                Link {
                    to: crate::app::Route::StyleguideBlog { id: id - 1 },
                    class: "btn btn-outline",
                    "Previous"
                }
                Link {
                    to: crate::app::Route::StyleguideBlog { id: id + 1 },
                    class: "btn btn-outline",
                    "Next"
                }
            }
        }
    }
}
