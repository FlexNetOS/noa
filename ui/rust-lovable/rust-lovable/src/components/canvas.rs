use dioxus::prelude::*;

use crate::core::conversational_ai::{PlatformTarget, ViewMode};
use crate::core::ui_generator::UIComponent;

#[component]
pub fn UICanvas() -> Element {
    let mut selected_component = use_signal(|| None::<String>);
    let mut view_mode = use_signal(|| ViewMode::Design);
    let mut platform_target = use_signal(|| PlatformTarget::Universal);
    let components = use_signal(Vec::<UIComponent>::new);

    rsx! {
        div {
            class: "ui-canvas",

            // Canvas toolbar
            div {
                class: "canvas-toolbar",

                // View mode selector
                div {
                    class: "view-mode-selector",

                    button {
                        class: if *view_mode.read() == ViewMode::Design { "active" } else { "" },
                        onclick: move |_| view_mode.set(ViewMode::Design),
                        "Design"
                    }

                    button {
                        class: if *view_mode.read() == ViewMode::Code { "active" } else { "" },
                        onclick: move |_| view_mode.set(ViewMode::Code),
                        "Code"
                    }

                    button {
                        class: if *view_mode.read() == ViewMode::Split { "active" } else { "" },
                        onclick: move |_| view_mode.set(ViewMode::Split),
                        "Split"
                    }

                    button {
                        class: if *view_mode.read() == ViewMode::Preview { "active" } else { "" },
                        onclick: move |_| view_mode.set(ViewMode::Preview),
                        "Preview"
                    }
                }

                // Platform target selector
                div {
                    class: "platform-selector",

                    select {
                        onchange: move |event| {
                            let target = match event.value().as_str() {
                                "web" => PlatformTarget::Web,
                                "desktop" => PlatformTarget::Desktop,
                                "mobile" => PlatformTarget::Mobile,
                                _ => PlatformTarget::Universal,
                            };
                            platform_target.set(target);
                        },

                        option { value: "universal", "Universal" }
                        option { value: "web", "Web" }
                        option { value: "desktop", "Desktop" }
                        option { value: "mobile", "Mobile" }
                    }
                }
            }

            // Canvas content based on view mode
            div {
                class: "canvas-content",

                match *view_mode.read() {
                    ViewMode::Design => rsx! {
                        DesignView {
                            components: components.read().clone(),
                            selected_component: selected_component.read().clone(),
                            on_select_component: move |id| selected_component.set(Some(id))
                        }
                    },
                    ViewMode::Code => rsx! {
                        CodeView {
                            components: components.read().clone(),
                            platform_target: *platform_target.read()
                        }
                    },
                    ViewMode::Split => rsx! {
                        div {
                            class: "split-view",

                            div {
                                class: "split-pane design-pane",
                                DesignView {
                                    components: components.read().clone(),
                                    selected_component: selected_component.read().clone(),
                                    on_select_component: move |id| selected_component.set(Some(id))
                                }
                            }

                            div {
                                class: "split-pane code-pane",
                                CodeView {
                                    components: components.read().clone(),
                                    platform_target: *platform_target.read()
                                }
                            }
                        }
                    },
                    ViewMode::Preview => rsx! {
                        PreviewView {
                            components: components.read().clone(),
                            platform_target: *platform_target.read()
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn DesignView(
    components: Vec<UIComponent>,
    selected_component: Option<String>,
    on_select_component: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "design-view",

            if components.is_empty() {
                div {
                    class: "empty-canvas",

                    div {
                        class: "empty-state-content",

                        h3 { "Start Building Your UI" }

                        p { "Use the chat interface to describe what you want to create, or drag components from the sidebar." }

                        div {
                            class: "quick-start-buttons",

                            button {
                                onclick: move |_| {
                                    // Add a sample component
                                },
                                "Add Button"
                            }

                            button {
                                onclick: move |_| {
                                    // Add a sample component
                                },
                                "Add Text"
                            }

                            button {
                                onclick: move |_| {
                                    // Add a sample component
                                },
                                "Add Container"
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "component-tree",

                    for component in components {
                        ComponentNode {
                            component: component.clone(),
                            selected: selected_component.as_ref() == Some(&component.id),
                            on_select: move |id| on_select_component.call(id)
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CodeView(components: Vec<UIComponent>, platform_target: PlatformTarget) -> Element {
    let code = generate_code_preview(&components, platform_target);

    rsx! {
        div {
            class: "code-view",

            div {
                class: "code-header",

                h4 { "Generated Code" }

                div {
                    class: "code-actions",

                    button {
                        onclick: move |_| {
                            // Copy code to clipboard
                        },
                        "Copy"
                    }

                    button {
                        onclick: move |_| {
                            // Export code
                        },
                        "Export"
                    }
                }
            }

            pre {
                class: "code-content",
                code {
                    "{code}"
                }
            }
        }
    }
}

#[component]
pub fn PreviewView(components: Vec<UIComponent>, platform_target: PlatformTarget) -> Element {
    // Placeholder implementation for now.
    let _ = (&components, platform_target);

    rsx! {
        div {
            class: "preview-view",

            div {
                class: "preview-header",

                h4 { "Live Preview" }

                div {
                    class: "preview-controls",

                    select {
                        onchange: move |_event| {
                            // Handle device preview change
                        },

                        option { value: "desktop", "Desktop" }
                        option { value: "tablet", "Tablet" }
                        option { value: "mobile", "Mobile" }
                    }

                    button {
                        onclick: move |_| {
                            // Refresh preview
                        },
                        "Refresh"
                    }
                }
            }

            div {
                class: "preview-frame",

                // This would render the actual components in a preview iframe
                // For now, we'll show a placeholder
                div {
                    class: "preview-placeholder",
                    "Live preview will appear here"
                }
            }
        }
    }
}

#[component]
pub fn ComponentNode(
    component: UIComponent,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let indent = component.id.len() * 10; // Simple indent calculation

    rsx! {
        div {
            class: if selected { "component-node selected" } else { "component-node" },
            style: "margin-left: {indent}px",
            onclick: move |_| on_select.call(component.id.clone()),

            div {
                class: "component-icon",
                // Component type icon would go here
                "◆"
            }

            div {
                class: "component-info",

                div {
                    class: "component-name",
                    {format!("{:?}", component.component_type)}
                }

                if let Some(text) = component.properties.get("text").and_then(|v| v.as_str()) {
                    div {
                        class: "component-preview",
                        "{text}"
                    }
                }
            }

            // Render children recursively
            for child in &component.children {
                ComponentNode {
                    component: child.clone(),
                    selected: false,
                    on_select: move |id| on_select.call(id)
                }
            }
        }
    }
}

fn generate_code_preview(components: &[UIComponent], platform_target: PlatformTarget) -> String {
    // TODO: platform-specific codegen
    let _ = platform_target;

    let mut code = String::new();

    code.push_str("use dioxus::prelude::*;\n\n");
    code.push_str("#[component]\n");
    code.push_str("pub fn GeneratedUI() -> Element {\n");
    code.push_str("    rsx! {\n");

    for component in components {
        if let Some(generated_code) = &component.generated_code {
            code.push_str(&format!("        {}\n", generated_code));
        }
    }

    code.push_str("    }\n");
    code.push_str("}\n");

    code
}
