//! Completion panel for direct text completion.

use dioxus::prelude::*;
use noa_api_client::ChatRequest;

use crate::chat::hooks::use_api_client;

/// Panel for testing direct text completions.
#[component]
pub fn CompletionPanel(
    #[props(default = None)] provider: Option<String>,
    #[props(default = None)] model: Option<String>,
) -> Element {
    let client = use_api_client();
    let mut prompt = use_signal(String::new);
    let mut completion = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut stats = use_signal(|| None::<CompletionStats>);
    
    let handle_complete = {
        let client = client.clone();
        let provider = provider.clone();
        let model = model.clone();
        
        move |_| {
            let client = client.clone();
            let provider = provider.clone();
            let model = model.clone();
            let prompt_text = prompt.read().clone();
            
            if prompt_text.trim().is_empty() {
                return;
            }
            
            spawn(async move {
                is_loading.set(true);
                error.set(None);
                completion.set(String::new());
                
                let start = std::time::Instant::now();
                
                let request = ChatRequest {
                    message: prompt_text,
                    provider,
                    model,
                    history: None,
                    stream: false,
                };
                
                match client.chat(request).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        completion.set(response.content.clone());
                        stats.set(Some(CompletionStats {
                            provider: response.provider,
                            model: response.model,
                            latency_ms: elapsed.as_millis() as u64,
                            tokens: None, // API doesn't return token count yet
                        }));
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
                
                is_loading.set(false);
            });
        }
    };
    
    rsx! {
        div {
            class: "card bg-base-200 shadow-xl",
            
            div {
                class: "card-body",
                
                h3 {
                    class: "card-title",
                    "Text Completion"
                }
                
                // Prompt input
                div {
                    class: "form-control",
                    
                    label {
                        class: "label",
                        span { class: "label-text", "Prompt" }
                    }
                    
                    textarea {
                        class: "textarea textarea-bordered h-32 font-mono text-sm",
                        placeholder: "Enter your prompt here...",
                        value: "{prompt}",
                        oninput: move |e| prompt.set(e.value()),
                    }
                }
                
                // Complete button
                div {
                    class: "flex justify-end mt-4",
                    
                    button {
                        class: "btn btn-primary gap-2",
                        disabled: *is_loading.read() || prompt.read().trim().is_empty(),
                        onclick: handle_complete,
                        
                        if *is_loading.read() {
                            span { class: "loading loading-spinner loading-sm" }
                        } else {
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13 10V3L4 14h7v7l9-11h-7z"
                                }
                            }
                        }
                        "Complete"
                    }
                }
                
                // Error display
                if let Some(ref err) = *error.read() {
                    div {
                        class: "alert alert-error mt-4",
                        span { {err.clone()} }
                    }
                }
                
                // Completion output
                if !completion.read().is_empty() {
                    div {
                        class: "mt-4",
                        
                        label {
                            class: "label",
                            span { class: "label-text", "Completion" }
                        }
                        
                        div {
                            class: "bg-base-300 rounded-lg p-4 font-mono text-sm whitespace-pre-wrap",
                            {completion.read().clone()}
                        }
                        
                        // Stats
                        if let Some(ref s) = *stats.read() {
                            div {
                                class: "flex gap-4 mt-2 text-xs text-base-content/70",
                                
                                span {
                                    "Provider: "
                                    span { class: "font-semibold", {s.provider.clone()} }
                                }
                                
                                if let Some(ref model) = s.model {
                                    span {
                                        "Model: "
                                        span { class: "font-semibold", {model.clone()} }
                                    }
                                }
                                
                                span {
                                    "Latency: "
                                    span { class: "font-semibold", "{s.latency_ms}ms" }
                                }
                                
                                if let Some(tokens) = s.tokens {
                                    span {
                                        "Tokens: "
                                        span { class: "font-semibold", "{tokens}" }
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

/// Completion statistics.
#[derive(Clone, Debug)]
struct CompletionStats {
    provider: String,
    model: Option<String>,
    latency_ms: u64,
    tokens: Option<u32>,
}
