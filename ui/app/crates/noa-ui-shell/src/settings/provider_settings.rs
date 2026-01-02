//! Provider settings component.

use dioxus::prelude::*;
use noa_api_client::Provider;

use crate::chat::hooks::{use_api_client, fetch_providers};

/// Provider configuration settings.
#[component]
pub fn ProviderSettings() -> Element {
    let client = use_api_client();
    let mut providers = use_signal(Vec::<Provider>::new);
    let mut is_loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    
    // Load providers
    {
        let client = client.clone();
        use_effect(move || {
            let client = client.clone();
            spawn(async move {
                match fetch_providers(client).await {
                    Ok(p) => {
                        providers.set(p);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        is_loading.set(false);
                    }
                }
            });
        });
    }
    
    rsx! {
        div {
            class: "provider-settings space-y-6",
            
            // Header
            div {
                class: "flex items-center justify-between",
                
                h2 {
                    class: "text-xl font-semibold",
                    "AI Providers"
                }
                
                button {
                    class: "btn btn-sm btn-ghost",
                    onclick: move |_| {
                        is_loading.set(true);
                        let client = client.clone();
                        spawn(async move {
                            if let Ok(p) = fetch_providers(client).await {
                                providers.set(p);
                            }
                            is_loading.set(false);
                        });
                    },
                    "🔄 Refresh"
                }
            }
            
            // Error
            if let Some(ref err) = *error.read() {
                div {
                    class: "alert alert-error",
                    {err.clone()}
                }
            }
            
            // Loading
            if *is_loading.read() {
                div {
                    class: "flex justify-center p-8",
                    span { class: "loading loading-spinner loading-lg" }
                }
            } else {
                // Provider cards
                div {
                    class: "grid gap-4",
                    
                    for provider in providers.read().iter() {
                        ProviderCard { provider: provider.clone() }
                    }
                }
            }
            
            // Priority info
            div {
                class: "card bg-base-200 mt-6",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm",
                        "Provider Priority"
                    }
                    
                    p {
                        class: "text-sm text-base-content/70",
                        "Providers are used in priority order. Local providers are preferred for privacy and speed. Cloud providers are used as fallback."
                    }
                    
                    div {
                        class: "mt-2 text-sm",
                        
                        ol {
                            class: "list-decimal list-inside space-y-1",
                            li { "Local (llama.cpp) - Always available offline" }
                            li { "Hybrid (Cursor) - IDE context awareness" }
                            li { "Cloud (Claude, OpenAI) - Complex reasoning" }
                        }
                    }
                }
            }
        }
    }
}

/// Individual provider card.
#[component]
fn ProviderCard(provider: Provider) -> Element {
    let status_badge = match provider.status.as_str() {
        "available" => ("badge-success", "Available"),
        "starting" => ("badge-warning", "Starting"),
        _ => ("badge-error", "Unavailable"),
    };
    
    let type_badge = match provider.provider_type.as_str() {
        "local" => ("badge-success", "Local"),
        "hybrid" => ("badge-info", "Hybrid"),
        _ => ("badge-warning", "Cloud"),
    };
    
    rsx! {
        div {
            class: "card bg-base-100 shadow border border-base-300",
            
            div {
                class: "card-body",
                
                div {
                    class: "flex items-center justify-between",
                    
                    div {
                        class: "flex items-center gap-3",
                        
                        // Provider icon
                        span {
                            class: "text-2xl",
                            match provider.provider_type.as_str() {
                                "local" => "🖥️",
                                "hybrid" => "🔀",
                                _ => "☁️",
                            }
                        }
                        
                        div {
                            h3 { class: "font-semibold", {provider.name.clone()} }
                            p { class: "text-sm text-base-content/70", {provider.id.clone()} }
                        }
                    }
                    
                    div {
                        class: "flex gap-2",
                        
                        span {
                            class: "badge {type_badge.0}",
                            {type_badge.1}
                        }
                        
                        span {
                            class: "badge {status_badge.0}",
                            {status_badge.1}
                        }
                    }
                }
                
                if let Some(priority) = provider.priority {
                    div {
                        class: "mt-2 text-sm text-base-content/70",
                        "Priority: {priority}"
                    }
                }
            }
        }
    }
}
