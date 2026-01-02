//! Status card component.

use dioxus::prelude::*;

/// Status card for displaying service status.
#[component]
pub fn StatusCard(
    title: String,
    status: String,
    value: String,
    icon: String,
) -> Element {
    let status_color = match status.as_str() {
        "healthy" => "success",
        "warning" => "warning",
        "error" => "error",
        _ => "ghost",
    };
    
    let indicator_color = match status.as_str() {
        "healthy" => "bg-success",
        "warning" => "bg-warning",
        "error" => "bg-error",
        _ => "bg-base-300",
    };
    
    rsx! {
        div {
            class: "card bg-base-200 shadow-sm",
            
            div {
                class: "card-body p-4",
                
                div {
                    class: "flex items-center justify-between",
                    
                    div {
                        class: "flex items-center gap-3",
                        
                        span {
                            class: "text-2xl",
                            {icon}
                        }
                        
                        div {
                            h3 {
                                class: "font-medium text-sm text-base-content/70",
                                {title}
                            }
                            
                            p {
                                class: "text-xl font-bold",
                                {value}
                            }
                        }
                    }
                    
                    // Status indicator
                    div {
                        class: "w-3 h-3 rounded-full {indicator_color}",
                        title: "{status}",
                    }
                }
            }
        }
    }
}
