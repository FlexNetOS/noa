//! Provider selector component.

use dioxus::prelude::*;
use noa_api_client::Provider;

/// Provider selector dropdown.
#[component]
pub fn ProviderSelector(
    providers: Vec<Provider>,
    selected_provider: Option<String>,
    is_loading: bool,
    on_select: EventHandler<String>,
) -> Element {
    let current_name = selected_provider
        .as_ref()
        .and_then(|id| providers.iter().find(|p| &p.id == id))
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "Select Provider".to_string());

    rsx! {
        div {
            class: "provider-selector",
            
            // Dropdown
            div {
                class: "dropdown dropdown-end",
                
                // Trigger button
                label {
                    tabindex: "0",
                    class: "btn btn-ghost btn-sm gap-2",
                    
                    // Provider icon
                    ProviderIcon { provider_type: selected_provider.as_ref().and_then(|id| {
                        providers.iter().find(|p| &p.id == id).map(|p| p.provider_type.clone())
                    }).unwrap_or_default() }
                    
                    span { {current_name} }
                    
                    // Chevron
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
                
                // Dropdown content
                ul {
                    tabindex: "0",
                    class: "dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52 z-50",
                    
                    if is_loading {
                        li {
                            class: "disabled",
                            span { "Loading providers..." }
                        }
                    } else if providers.is_empty() {
                        li {
                            class: "disabled",
                            span { "No providers available" }
                        }
                    } else {
                        for provider in providers.iter() {
                            ProviderItem {
                                provider: provider.clone(),
                                is_selected: selected_provider.as_ref() == Some(&provider.id),
                                on_select: on_select.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Individual provider item in dropdown.
#[component]
fn ProviderItem(
    provider: Provider,
    is_selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let status_class = match provider.status.as_str() {
        "available" => "badge-success",
        "starting" => "badge-warning",
        _ => "badge-error",
    };
    
    let id = provider.id.clone();
    
    rsx! {
        li {
            a {
                class: if is_selected { "active" } else { "" },
                onclick: move |_| on_select.call(id.clone()),
                
                div {
                    class: "flex items-center gap-2 w-full",
                    
                    ProviderIcon { provider_type: provider.provider_type.clone() }
                    
                    div {
                        class: "flex-1",
                        div { class: "font-medium", {provider.name.clone()} }
                        div {
                            class: "text-xs opacity-70 flex items-center gap-1",
                            span { class: "badge badge-xs {status_class}", {provider.status.clone()} }
                            if let Some(priority) = provider.priority {
                                span { "Priority: {priority}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Provider type icon.
#[component]
fn ProviderIcon(provider_type: String) -> Element {
    let (icon, color) = match provider_type.as_str() {
        "local" => ("🖥️", "text-green-500"),
        "hybrid" => ("🔀", "text-blue-500"),
        "cloud" => ("☁️", "text-purple-500"),
        _ => ("🤖", "text-gray-500"),
    };
    
    rsx! {
        span {
            class: "text-lg {color}",
            {icon}
        }
    }
}
