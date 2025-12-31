use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/styleguide/main.css");
const BLOG_CSS: Asset = asset!("/assets/styleguide/blog.css");

/// A small Dioxus styleguide area (derived from the default Dioxus workspace template).
#[component]
pub fn StyleguideLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        noa_ui_styleguide_ui::Navbar {
            Link { to: crate::app::Route::StyleguideHome {}, "Home" }
            Link { to: crate::app::Route::StyleguideBlog { id: 1 }, "Blog" }
        }

        Outlet::<crate::app::Route> {}
    }
}

#[component]
pub fn StyleguideHome() -> Element {
    rsx! {
        noa_ui_styleguide_ui::Hero {}
        noa_ui_styleguide_ui::Echo {}
    }
}

#[component]
pub fn StyleguideBlog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: crate::app::Route::StyleguideBlog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: crate::app::Route::StyleguideBlog { id: id + 1 },
                "Next"
            }
        }
    }
}
