//! About page component.

use dioxus::prelude::*;

/// About page with version and system info.
#[component]
pub fn AboutPage() -> Element {
    rsx! {
        div {
            class: "about-page space-y-6",
            
            // Logo and title
            div {
                class: "text-center py-8",
                
                div {
                    class: "text-6xl mb-4",
                    "🧠"
                }
                
                h1 {
                    class: "text-3xl font-bold",
                    "NOA"
                }
                
                p {
                    class: "text-base-content/70",
                    "Neural Operating Architecture"
                }
            }
            
            // Version info
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm mb-4",
                        "Version Information"
                    }
                    
                    div {
                        class: "grid grid-cols-2 gap-4 text-sm",
                        
                        div {
                            class: "text-base-content/70",
                            "UI Version"
                        }
                        div {
                            class: "font-mono",
                            "0.1.0"
                        }
                        
                        div {
                            class: "text-base-content/70",
                            "API Version"
                        }
                        div {
                            class: "font-mono",
                            "1.0.0"
                        }
                        
                        div {
                            class: "text-base-content/70",
                            "Dioxus Version"
                        }
                        div {
                            class: "font-mono",
                            "0.7.2"
                        }
                        
                        div {
                            class: "text-base-content/70",
                            "Build Date"
                        }
                        div {
                            class: "font-mono",
                            "2026-01-02"
                        }
                    }
                }
            }
            
            // Features
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm mb-4",
                        "Features"
                    }
                    
                    ul {
                        class: "space-y-2",
                        
                        li {
                            class: "flex items-center gap-2",
                            span { class: "text-success", "✓" }
                            "Local-first AI inference"
                        }
                        li {
                            class: "flex items-center gap-2",
                            span { class: "text-success", "✓" }
                            "Multi-provider support"
                        }
                        li {
                            class: "flex items-center gap-2",
                            span { class: "text-success", "✓" }
                            "P2P state synchronization"
                        }
                        li {
                            class: "flex items-center gap-2",
                            span { class: "text-success", "✓" }
                            "Agent sandbox isolation"
                        }
                        li {
                            class: "flex items-center gap-2",
                            span { class: "text-success", "✓" }
                            "Cross-platform (Web, Desktop)"
                        }
                    }
                }
            }
            
            // Links
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm mb-4",
                        "Links"
                    }
                    
                    div {
                        class: "flex flex-wrap gap-2",
                        
                        a {
                            class: "btn btn-sm btn-ghost",
                            href: "https://github.com/flexnetos/noa",
                            target: "_blank",
                            "📦 GitHub"
                        }
                        
                        a {
                            class: "btn btn-sm btn-ghost",
                            href: "https://noa.flexnetos.com/docs",
                            target: "_blank",
                            "📚 Documentation"
                        }
                        
                        a {
                            class: "btn btn-sm btn-ghost",
                            href: "https://github.com/flexnetos/noa/issues",
                            target: "_blank",
                            "🐛 Report Issue"
                        }
                    }
                }
            }
            
            // License
            div {
                class: "text-center text-sm text-base-content/50 py-4",
                "© 2026 FlexNetOS. MIT License."
            }
        }
    }
}
