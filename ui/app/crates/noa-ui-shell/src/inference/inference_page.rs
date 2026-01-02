//! Main inference page component.

use dioxus::prelude::*;
use noa_api_client::Provider;
use std::sync::Arc;

use crate::chat::hooks::{use_api_client, use_providers_state, fetch_providers};
use crate::chat::ProviderSelector;
use super::{ServerControl, ModelSelector, CompletionPanel};

/// Full inference page with server control, model selection, and completion panel.
#[component]
pub fn InferencePage() -> Element {
    let client = use_api_client();
    let mut providers_state = use_providers_state();
    let mut selected_model = use_signal(|| None::<String>);
    
    // Load providers on mount
    use_effect(move || {
        let client = client.clone();
        spawn(async move {
            providers_state.write().is_loading = true;
            
            match fetch_providers(client).await {
                Ok(providers) => {
                    let mut state = providers_state.write();
                    state.providers = providers.clone();
                    state.is_loading = false;
                    
                    // Select first local provider if available
                    if state.selected_provider.is_none() {
                        if let Some(p) = providers.iter().find(|p| p.provider_type == "local" && p.status == "available") {
                            state.selected_provider = Some(p.id.clone());
                        } else if let Some(p) = providers.iter().find(|p| p.status == "available") {
                            state.selected_provider = Some(p.id.clone());
                        }
                    }
                }
                Err(e) => {
                    let mut state = providers_state.write();
                    state.error = Some(e);
                    state.is_loading = false;
                }
            }
        });
    });
    
    // Handle provider selection
    let handle_provider_select = move |provider_id: String| {
        providers_state.write().selected_provider = Some(provider_id);
        selected_model.set(None); // Reset model when provider changes
    };
    
    // Handle model selection
    let handle_model_select = move |model_id: String| {
        selected_model.set(Some(model_id));
    };
    
    rsx! {
        div {
            class: "inference-page p-6 space-y-6",
            
            // Header
            div {
                class: "flex items-center justify-between",
                
                h1 {
                    class: "text-2xl font-bold",
                    "Inference"
                }
                
                // Provider selector
                ProviderSelector {
                    providers: providers_state.read().providers.clone(),
                    selected_provider: providers_state.read().selected_provider.clone(),
                    is_loading: providers_state.read().is_loading,
                    on_select: handle_provider_select,
                }
            }
            
            // Provider error
            if let Some(ref error) = providers_state.read().error {
                div {
                    class: "alert alert-warning",
                    span { "Provider error: {error}" }
                }
            }
            
            // Main content grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                
                // Left column: Server status and model selection
                div {
                    class: "space-y-6",
                    
                    // Server control
                    ServerControl {}
                    
                    // Model selector (if provider selected)
                    if let Some(ref provider_id) = providers_state.read().selected_provider {
                        div {
                            class: "card bg-base-200 shadow-xl",
                            
                            div {
                                class: "card-body",
                                
                                h3 {
                                    class: "card-title",
                                    "Model Selection"
                                }
                                
                                ModelSelector {
                                    provider_id: provider_id.clone(),
                                    selected_model: selected_model.read().clone(),
                                    on_select: handle_model_select,
                                }
                            }
                        }
                    }
                }
                
                // Right column (2 cols): Completion panel
                div {
                    class: "lg:col-span-2",
                    
                    CompletionPanel {
                        provider: providers_state.read().selected_provider.clone(),
                        model: selected_model.read().clone(),
                    }
                }
            }
            
            // Provider priority info
            div {
                class: "card bg-base-100 shadow",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm",
                        "Provider Priority"
                    }
                    
                    div {
                        class: "overflow-x-auto",
                        
                        table {
                            class: "table table-xs",
                            
                            thead {
                                tr {
                                    th { "Priority" }
                                    th { "Provider" }
                                    th { "Type" }
                                    th { "Status" }
                                }
                            }
                            
                            tbody {
                                for (idx, provider) in providers_state.read().providers.iter().enumerate() {
                                    tr {
                                        td { {(idx + 1).to_string()} }
                                        td { {provider.name.clone()} }
                                        td {
                                            span {
                                                class: match provider.provider_type.as_str() {
                                                    "local" => "badge badge-success badge-xs",
                                                    "hybrid" => "badge badge-info badge-xs",
                                                    _ => "badge badge-warning badge-xs",
                                                },
                                                {provider.provider_type.clone()}
                                            }
                                        }
                                        td {
                                            span {
                                                class: match provider.status.as_str() {
                                                    "available" => "text-success",
                                                    "starting" => "text-warning",
                                                    _ => "text-error",
                                                },
                                                {provider.status.clone()}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
