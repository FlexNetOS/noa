//! Inference server control component.

use dioxus::prelude::*;

use crate::chat::hooks::use_api_client;

/// Server status information.
#[derive(Clone, Debug, Default)]
pub struct ServerStatus {
    pub running: bool,
    pub url: String,
    pub port: u16,
    pub model: Option<String>,
    pub last_check: Option<String>,
}

/// Server control panel for local inference.
#[component]
pub fn ServerControl() -> Element {
    let client = use_api_client();
    let mut status = use_signal(ServerStatus::default);
    let mut is_checking = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    
    // Check server health on mount and periodically
    use_effect(move || {
        let client = client.clone();
        spawn(async move {
            loop {
                is_checking.set(true);
                
                match client.health().await {
                    Ok(health) => {
                        status.set(ServerStatus {
                            running: health.status == "healthy",
                            url: "http://localhost:3001".to_string(),
                            port: 3001,
                            model: None,
                            last_check: Some(chrono::Utc::now().format("%H:%M:%S").to_string()),
                        });
                        error.set(None);
                    }
                    Err(e) => {
                        status.set(ServerStatus {
                            running: false,
                            url: "http://localhost:3001".to_string(),
                            port: 3001,
                            model: None,
                            last_check: Some(chrono::Utc::now().format("%H:%M:%S").to_string()),
                        });
                        error.set(Some(e.to_string()));
                    }
                }
                
                is_checking.set(false);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    });
    
    let status_badge_class = if status.read().running {
        "badge badge-success gap-2"
    } else {
        "badge badge-error gap-2"
    };
    
    let status_text = if status.read().running {
        "Running"
    } else {
        "Offline"
    };
    
    rsx! {
        div {
            class: "card bg-base-200 shadow-xl",
            
            div {
                class: "card-body",
                
                // Header
                div {
                    class: "flex items-center justify-between",
                    
                    h3 {
                        class: "card-title flex items-center gap-2",
                        
                        // Server icon
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2"
                            }
                        }
                        "Inference Server"
                    }
                    
                    // Status badge
                    div {
                        class: status_badge_class,
                        if status.read().running {
                            svg {
                                class: "w-4 h-4",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                    clip_rule: "evenodd"
                                }
                            }
                        } else {
                            svg {
                                class: "w-4 h-4",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }
                        {status_text}
                    }
                }
                
                // Server details
                div {
                    class: "mt-4 space-y-2 text-sm",
                    
                    div {
                        class: "flex justify-between",
                        span { class: "text-base-content/70", "URL" }
                        code { class: "font-mono", {status.read().url.clone()} }
                    }
                    
                    div {
                        class: "flex justify-between",
                        span { class: "text-base-content/70", "Port" }
                        span { {status.read().port.to_string()} }
                    }
                    
                    if let Some(ref model) = status.read().model {
                        div {
                            class: "flex justify-between",
                            span { class: "text-base-content/70", "Model" }
                            span { {model.clone()} }
                        }
                    }
                    
                    if let Some(ref last) = status.read().last_check {
                        div {
                            class: "flex justify-between",
                            span { class: "text-base-content/70", "Last Check" }
                            span { {last.clone()} }
                        }
                    }
                }
                
                // Error display
                if let Some(ref err) = *error.read() {
                    div {
                        class: "alert alert-warning mt-4 text-sm",
                        span { {err.clone()} }
                    }
                }
                
                // Refresh button
                div {
                    class: "card-actions justify-end mt-4",
                    
                    button {
                        class: "btn btn-ghost btn-sm gap-2",
                        disabled: *is_checking.read(),
                        
                        svg {
                            class: if *is_checking.read() { "w-4 h-4 animate-spin" } else { "w-4 h-4" },
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                            }
                        }
                        "Refresh"
                    }
                }
            }
        }
    }
}
