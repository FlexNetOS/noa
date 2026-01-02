//! Main chat page component.

use dioxus::prelude::*;
use noa_api_client::ChatMessage;

use super::hooks::{use_api_client, use_chat_state, use_providers_state, send_message, fetch_providers};
use super::{ChatMessages, ChatInput, ProviderSelector};

/// Full chat page with provider selection and message history.
#[component]
pub fn ChatPage() -> Element {
    // Get API client
    let client = use_api_client();
    
    // State
    let mut chat_state = use_chat_state();
    let mut providers_state = use_providers_state();
    
    // Load providers on mount
    {
        let client = client.clone();
        use_effect(move || {
            let client = client.clone();
            spawn(async move {
                providers_state.write().is_loading = true;
                
                match fetch_providers(client).await {
                    Ok(providers) => {
                        let mut state = providers_state.write();
                        state.providers = providers.clone();
                        state.is_loading = false;
                        
                        // Select first available provider
                        if state.selected_provider.is_none() {
                            if let Some(p) = providers.iter().find(|p| p.status == "available") {
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
    }
    
    // Handle sending messages
    let handle_send = {
        let client = client.clone();
        move |message: String| {
            let client = client.clone();
            let provider = providers_state.read().selected_provider.clone();
            let model = providers_state.read().selected_model.clone();
            
            // Add user message
            {
                let mut state = chat_state.write();
                state.messages.push(ChatMessage {
                    role: "user".to_string(),
                    content: message.clone(),
                });
                state.is_loading = true;
                state.error = None;
            }
            
            // Send to API
            spawn(async move {
                let history = chat_state.read().messages.clone();
                
                match send_message(client, message, history, provider, model).await {
                    Ok(response) => {
                        let mut state = chat_state.write();
                        state.messages.push(ChatMessage {
                            role: "assistant".to_string(),
                            content: response.content,
                        });
                        state.is_loading = false;
                        state.current_provider = Some(response.provider);
                        state.current_model = response.model;
                    }
                    Err(e) => {
                        let mut state = chat_state.write();
                        state.error = Some(e);
                        state.is_loading = false;
                    }
                }
            });
        }
    };
    
    // Handle provider selection
    let handle_provider_select = move |provider_id: String| {
        providers_state.write().selected_provider = Some(provider_id);
    };
    
    rsx! {
        div {
            class: "chat-page flex flex-col h-full bg-base-100",
            
            // Header with provider selector
            div {
                class: "chat-header flex items-center justify-between p-4 border-b border-base-300",
                
                h2 {
                    class: "text-xl font-bold",
                    "AI Chat"
                }
                
                div {
                    class: "flex items-center gap-4",
                    
                    // Current provider indicator
                    if let Some(ref provider) = chat_state.read().current_provider {
                        span {
                            class: "text-sm text-base-content/70",
                            "Using: {provider}"
                            if let Some(ref model) = chat_state.read().current_model {
                                " ({model})"
                            }
                        }
                    }
                    
                    // Provider selector
                    ProviderSelector {
                        providers: providers_state.read().providers.clone(),
                        selected_provider: providers_state.read().selected_provider.clone(),
                        is_loading: providers_state.read().is_loading,
                        on_select: handle_provider_select,
                    }
                }
            }
            
            // Error display
            if let Some(ref error) = chat_state.read().error {
                div {
                    class: "alert alert-error mx-4 mt-4",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        }
                    }
                    span { {error.clone()} }
                }
            }
            
            // Provider loading/error
            if let Some(ref error) = providers_state.read().error {
                div {
                    class: "alert alert-warning mx-4 mt-4",
                    span { "Provider error: {error}" }
                }
            }
            
            // Messages area
            ChatMessages {
                messages: chat_state.read().messages.clone(),
                is_loading: chat_state.read().is_loading,
            }
            
            // Input area
            ChatInput {
                placeholder: "Ask the AI assistant...".to_string(),
                disabled: chat_state.read().is_loading || providers_state.read().providers.is_empty(),
                on_send: handle_send,
            }
        }
    }
}
