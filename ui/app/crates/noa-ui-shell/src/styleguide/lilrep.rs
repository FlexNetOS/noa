use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/lilrep/main.css");
const BLOG_CSS: Asset = asset!("/assets/lilrep/blog.css");

/// A styleguide area migrated from `ui/lilrep`.
#[component]
pub fn LilrepLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        noa_ui_lilrep_ui::Navbar {
            Link { to: crate::app::Route::LilrepHome {}, "Home" }
            Link { to: crate::app::Route::LilrepBlog { id: 1 }, "Blog" }
        }

        Outlet::<crate::app::Route> {}
    }
}

#[component]
pub fn LilrepHome() -> Element {
    rsx! {
        noa_ui_lilrep_ui::Hero {}
        noa_ui_lilrep_ui::Echo {}
    }
}

#[component]
pub fn LilrepBlog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: crate::app::Route::LilrepBlog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: crate::app::Route::LilrepBlog { id: id + 1 },
                "Next"
            }
        }
    }
}
