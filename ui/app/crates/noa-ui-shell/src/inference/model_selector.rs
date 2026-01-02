//! Model selector component.

use dioxus::prelude::*;
use noa_api_client::{Model, ModelsResponse};
use std::sync::Arc;

use crate::chat::hooks::use_api_client;

/// Model selector for choosing inference models.
#[component]
pub fn ModelSelector(
    provider_id: String,
    selected_model: Option<String>,
    on_select: EventHandler<String>,
) -> Element {
    let client = use_api_client();
    let mut models = use_signal(Vec::<Model>::new);
    let mut is_loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    
    // Fetch models when provider changes
    use_effect(move || {
        let client = client.clone();
        let provider = provider_id.clone();
        
        spawn(async move {
            is_loading.set(true);
            
            match client.list_models(&provider).await {
                Ok(response) => {
                    models.set(response.models);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            }
            
            is_loading.set(false);
        });
    });
    
    let current_name = selected_model
        .as_ref()
        .and_then(|id| models.read().iter().find(|m| &m.id == id).map(|m| m.name.clone()))
        .unwrap_or_else(|| "Select Model".to_string());
    
    rsx! {
        div {
            class: "model-selector",
            
            // Label
            label {
                class: "label",
                span { class: "label-text", "Model" }
            }
            
            // Select dropdown
            div {
                class: "dropdown w-full",
                
                label {
                    tabindex: "0",
                    class: "btn btn-outline w-full justify-between",
                    
                    span { {current_name} }
                    
                    if *is_loading.read() {
                        span { class: "loading loading-spinner loading-xs" }
                    } else {
                        svg {
                            class: "w-4 h-4",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M19 9l-7 7-7-7"
                            }
                        }
                    }
                }
                
                ul {
                    tabindex: "0",
                    class: "dropdown-content menu p-2 shadow bg-base-100 rounded-box w-full max-h-60 overflow-y-auto z-50",
                    
                    if *is_loading.read() {
                        li {
                            class: "disabled",
                            span { "Loading models..." }
                        }
                    } else if models.read().is_empty() {
                        li {
                            class: "disabled",
                            span { "No models available" }
                        }
                    } else {
                        for model in models.read().iter() {
                            ModelItem {
                                model: model.clone(),
                                is_selected: selected_model.as_ref() == Some(&model.id),
                                on_select: on_select.clone(),
                            }
                        }
                    }
                }
            }
            
            // Error
            if let Some(ref err) = *error.read() {
                div {
                    class: "text-error text-xs mt-1",
                    {err.clone()}
                }
            }
        }
    }
}

/// Individual model item.
#[component]
fn ModelItem(
    model: Model,
    is_selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let id = model.id.clone();
    
    rsx! {
        li {
            a {
                class: if is_selected { "active" } else { "" },
                onclick: move |_| on_select.call(id.clone()),
                
                div {
                    class: "flex flex-col",
                    
                    span { class: "font-medium", {model.name.clone()} }
                    
                    div {
                        class: "text-xs opacity-70 flex gap-2",
                        
                        if let Some(ref size) = model.size {
                            span { {size.clone()} }
                        }
                        
                        if let Some(ctx) = model.context_length {
                            span { "Context: {ctx}" }
                        }
                    }
                }
            }
        }
    }
}
