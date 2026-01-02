use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    let mut is_building = use_signal(|| false);
    let active_project = use_signal(|| String::from("My Project"));

    rsx! {
        div {
            class: "toolbar",

            // Left section - Project controls
            div {
                class: "toolbar-left",

                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // New project
                    },
                    title: "New Project",
                    "📝"
                }

                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // Open project
                    },
                    title: "Open Project",
                    "📁"
                }

                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // Save project
                    },
                    title: "Save Project",
                    "💾"
                }

                div {
                    class: "toolbar-separator"
                }

                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // Undo
                    },
                    title: "Undo",
                    "↶"
                }

                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // Redo
                    },
                    title: "Redo",
                    "↷"
                }
            }

            // Center section - Project info
            div {
                class: "toolbar-center",

                h2 {
                    class: "project-title",
                    "{active_project.read()}"
                }

                div {
                    class: "project-status",
                    span { class: "status-indicator ready", "●" }
                    span { "Ready" }
                }
            }

            // Right section - Build and deploy
            div {
                class: "toolbar-right",

                // Platform selector
                div {
                    class: "platform-selector",

                    select {
                        onchange: move |_event| {
                            // Handle platform change
                        },

                        option { value: "web", "Web" }
                        option { value: "desktop", "Desktop" }
                        option { value: "mobile", "Mobile" }
                        option { value: "universal", "Universal" }
                    }
                }

                div {
                    class: "toolbar-separator"
                }

                button {
                    class: "toolbar-button build-button",
                    onclick: move |_| {
                        is_building.set(true);

                        // Move a handle into the async task.
                        let mut is_building = is_building;
                        // Simulate build process
                        spawn(async move {
                            // Note: For WASM compatibility, skip tokio delay
                            // In production, this would track actual build progress
                            #[cfg(target_arch = "wasm32")]
                            {
                                // Skip delay for WASM
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                // Skip simulated delay for now
                            }
                            is_building.set(false);
                        });
                    },
                    disabled: *is_building.read(),

                    if *is_building.read() {
                        "Building..."
                    } else {
                        "Build"
                    }
                }

                button {
                    class: "toolbar-button deploy-button",
                    onclick: move |_| {
                        // Deploy project
                    },
                    "Deploy"
                }

                div {
                    class: "toolbar-separator"
                }

                // Settings
                button {
                    class: "toolbar-button",
                    onclick: move |_| {
                        // Open settings
                    },
                    title: "Settings",
                    "⚙️"
                }
            }
        }
    }
}

#[component]
pub fn ProjectSelector() -> Element {
    let projects = vec!["My Project", "Landing Page", "Dashboard", "Mobile App"];

    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            class: "project-selector",

            button {
                class: "project-selector-button",
                onclick: move |_| {
                    let open = *is_open.read();
                    is_open.set(!open);
                },

                span { "My Project" }
                span { class: if *is_open.read() { "dropdown-icon open" } else { "dropdown-icon" }, "▼" }
            }

            if *is_open.read() {
                div {
                    class: "project-dropdown",

                    for project in projects {
                        div {
                            class: "project-option",
                            onclick: move |_| {
                                // Select project
                                is_open.set(false);
                            },
                            "{project}"
                        }
                    }

                    div {
                        class: "project-option new-project",
                        onclick: move |_| {
                            // Create new project
                            is_open.set(false);
                        },
                        "+ New Project"
                    }
                }
            }
        }
    }
}
