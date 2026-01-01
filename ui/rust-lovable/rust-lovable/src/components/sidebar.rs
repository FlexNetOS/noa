use dioxus::prelude::*;
use std::collections::HashMap;

use crate::core::ui_generator::ComponentType;

#[component]
pub fn Sidebar() -> Element {
    let mut expanded_sections = use_signal(HashMap::<String, bool>::new);

    rsx! {
        div {
            class: "sidebar",

            // Project info
            div {
                class: "project-info",

                h3 { "My Project" }

                div {
                    class: "project-stats",

                    div {
                        class: "stat",
                        span { class: "stat-label", "Components: " }
                        span { class: "stat-value", "12" }
                    }

                    div {
                        class: "stat",
                        span { class: "stat-label", "Pages: " }
                        span { class: "stat-value", "3" }
                    }
                }
            }

            // Component Library
            SidebarSection {
                title: "Components",
                expanded: *expanded_sections.read().get("components").unwrap_or(&true),
                on_toggle: move |_| {
                    let mut sections = expanded_sections.read().clone();
                    sections.insert("components".to_string(), !sections.get("components").unwrap_or(&true));
                    expanded_sections.set(sections);
                },

                ComponentLibrary {}
            }

            // Pages
            SidebarSection {
                title: "Pages",
                expanded: *expanded_sections.read().get("pages").unwrap_or(&true),
                on_toggle: move |_| {
                    let mut sections = expanded_sections.read().clone();
                    sections.insert("pages".to_string(), !sections.get("pages").unwrap_or(&true));
                    expanded_sections.set(sections);
                },

                PageList {}
            }

            // Assets
            SidebarSection {
                title: "Assets",
                expanded: *expanded_sections.read().get("assets").unwrap_or(&false),
                on_toggle: move |_| {
                    let mut sections = expanded_sections.read().clone();
                    sections.insert("assets".to_string(), !sections.get("assets").unwrap_or(&false));
                    expanded_sections.set(sections);
                },

                AssetList {}
            }
        }
    }
}

#[component]
pub fn SidebarSection(
    title: String,
    expanded: bool,
    on_toggle: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "sidebar-section",

            div {
                class: "section-header",
                onclick: move |_| on_toggle.call(()),

                h4 { "{title}" }

                div {
                    class: if expanded { "expand-icon expanded" } else { "expand-icon" },
                    "▼"
                }
            }

            if expanded {
                div {
                    class: "section-content",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn ComponentLibrary() -> Element {
    let component_categories = vec![
        (
            "Layout",
            vec![
                ("Container", ComponentType::Container),
                ("Flex", ComponentType::Flex),
                ("Grid", ComponentType::Grid),
                ("Stack", ComponentType::Stack),
            ],
        ),
        (
            "Basic",
            vec![
                ("Text", ComponentType::Text),
                ("Button", ComponentType::Button),
                ("Image", ComponentType::Image),
                ("Icon", ComponentType::Icon),
            ],
        ),
        (
            "Forms",
            vec![
                ("Input", ComponentType::Input),
                ("TextArea", ComponentType::TextArea),
                ("Select", ComponentType::Select),
                ("Checkbox", ComponentType::Checkbox),
            ],
        ),
        (
            "Navigation",
            vec![
                ("Navbar", ComponentType::Navbar),
                ("Sidebar", ComponentType::Sidebar),
                ("Tabs", ComponentType::Tabs),
                ("Menu", ComponentType::Menu),
            ],
        ),
    ];

    rsx! {
        div {
            class: "component-library",

            for (category, components) in component_categories {
                div {
                    class: "component-category",

                    h5 { "{category}" }

                    for (name, component_type) in components {
                        DraggableComponent {
                            name: name.to_string(),
                            component_type: component_type.clone()
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DraggableComponent(name: String, component_type: ComponentType) -> Element {
    rsx! {
        div {
            class: "draggable-component",
            draggable: "true",
            ondragstart: move |_event| {
                // TODO(web): set drag data via DataTransfer when supported by the active renderer.
                let _ = &name;
            },

            div {
                class: "component-icon",
                // Icon based on component type
                match component_type {
                    ComponentType::Button => "🔘",
                    ComponentType::Text => "📝",
                    ComponentType::Input => "📋",
                    ComponentType::Container => "📦",
                    ComponentType::Flex => "↔️",
                    ComponentType::Grid => "▦",
                    ComponentType::Image => "🖼️",
                    ComponentType::Navbar => "📱",
                    _ => "◆",
                }
            }

            div {
                class: "component-name",
                "{name}"
            }
        }
    }
}

#[component]
pub fn PageList() -> Element {
    let pages = vec![
        ("Home", "/", true),
        ("About", "/about", false),
        ("Contact", "/contact", false),
    ];

    rsx! {
        div {
            class: "page-list",

            for (name, _path, is_active) in pages {
                div {
                    class: if is_active { "page-item active" } else { "page-item" },

                    div {
                        class: "page-icon",
                        "📄"
                    }

                    div {
                        class: "page-name",
                        "{name}"
                    }

                    if is_active {
                        div {
                            class: "page-status",
                            "✓"
                        }
                    }
                }
            }

            button {
                class: "add-page-button",
                onclick: move |_| {
                    // Add new page
                },
                "+ Add Page"
            }
        }
    }
}

#[component]
pub fn AssetList() -> Element {
    let assets = vec![
        ("logo.png", "Image", "2.1 MB"),
        ("hero-bg.jpg", "Image", "1.8 MB"),
        ("styles.css", "Stylesheet", "15 KB"),
    ];

    rsx! {
        div {
            class: "asset-list",

            for (name, asset_type, size) in assets {
                div {
                    class: "asset-item",

                    div {
                        class: "asset-icon",
                        match asset_type {
                            "Image" => "🖼️",
                            "Stylesheet" => "🎨",
                            _ => "📁",
                        }
                    }

                    div {
                        class: "asset-info",

                        div {
                            class: "asset-name",
                            "{name}"
                        }

                        div {
                            class: "asset-details",
                            "{asset_type} • {size}"
                        }
                    }
                }
            }

            button {
                class: "add-asset-button",
                onclick: move |_| {
                    // Upload new asset
                },
                "+ Upload Asset"
            }
        }
    }
}
