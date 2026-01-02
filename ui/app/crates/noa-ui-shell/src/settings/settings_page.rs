//! Main settings page.

use dioxus::prelude::*;

use super::{ProviderSettings, AppearanceSettings, AboutPage};

/// Settings page with tabs.
#[component]
pub fn SettingsPage() -> Element {
    let mut active_tab = use_signal(|| "providers".to_string());
    
    let tabs = [
        ("providers", "Providers", "🔌"),
        ("appearance", "Appearance", "🎨"),
        ("about", "About", "ℹ️"),
    ];
    
    rsx! {
        div {
            class: "settings-page p-6",
            
            h1 {
                class: "text-2xl font-bold mb-6",
                "Settings"
            }
            
            // Tab navigation
            div {
                class: "tabs tabs-boxed mb-6",
                
                for (id, label, icon) in tabs.iter() {
                    button {
                        class: if *active_tab.read() == *id { "tab tab-active" } else { "tab" },
                        onclick: {
                            let id = id.to_string();
                            move |_| active_tab.set(id.clone())
                        },
                        span { class: "mr-2", {*icon} }
                        {*label}
                    }
                }
            }
            
            // Tab content
            div {
                class: "settings-content",
                
                match active_tab.read().as_str() {
                    "providers" => rsx! { ProviderSettings {} },
                    "appearance" => rsx! { AppearanceSettings {} },
                    "about" => rsx! { AboutPage {} },
                    _ => rsx! { ProviderSettings {} },
                }
            }
        }
    }
}
