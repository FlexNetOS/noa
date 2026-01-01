use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "app",

            h1 { "Welcome to Rust Lovable!" }

            p { "This is a simple example of a UI built with Rust Lovable." }

            button {
                onclick: move |_| {
                    println!("Button clicked!");
                },
                "Click me"
            }
        }
    }
}
