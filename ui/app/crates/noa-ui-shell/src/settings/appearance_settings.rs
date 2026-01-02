//! Appearance settings component.

use dioxus::prelude::*;

/// Theme options.
const THEMES: [(&str, &str); 8] = [
    ("light", "Light"),
    ("dark", "Dark"),
    ("cupcake", "Cupcake"),
    ("bumblebee", "Bumblebee"),
    ("emerald", "Emerald"),
    ("corporate", "Corporate"),
    ("synthwave", "Synthwave"),
    ("retro", "Retro"),
];

/// Appearance settings.
#[component]
pub fn AppearanceSettings() -> Element {
    let mut theme = use_signal(|| "dark".to_string());
    let mut font_size = use_signal(|| 14);
    let mut compact_mode = use_signal(|| false);
    
    rsx! {
        div {
            class: "appearance-settings space-y-6",
            
            h2 {
                class: "text-xl font-semibold",
                "Appearance"
            }
            
            // Theme selection
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm",
                        "Theme"
                    }
                    
                    div {
                        class: "grid grid-cols-4 gap-2 mt-4",
                        
                        for (id, name) in THEMES.iter() {
                            button {
                                class: if *theme.read() == *id { 
                                    "btn btn-sm btn-primary" 
                                } else { 
                                    "btn btn-sm btn-ghost" 
                                },
                                onclick: {
                                    let id = id.to_string();
                                    move |_| theme.set(id.clone())
                                },
                                {*name}
                            }
                        }
                    }
                }
            }
            
            // Font size
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h3 {
                        class: "card-title text-sm",
                        "Font Size"
                    }
                    
                    div {
                        class: "flex items-center gap-4 mt-4",
                        
                        input {
                            r#type: "range",
                            class: "range range-primary",
                            min: "12",
                            max: "20",
                            value: "{font_size}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<i32>() {
                                    font_size.set(v);
                                }
                            }
                        }
                        
                        span {
                            class: "text-sm font-mono w-12",
                            "{font_size}px"
                        }
                    }
                    
                    // Preview
                    div {
                        class: "mt-4 p-4 bg-base-300 rounded-lg",
                        style: "font-size: {font_size}px",
                        "The quick brown fox jumps over the lazy dog."
                    }
                }
            }
            
            // Compact mode
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    div {
                        class: "flex items-center justify-between",
                        
                        div {
                            h3 {
                                class: "font-semibold",
                                "Compact Mode"
                            }
                            p {
                                class: "text-sm text-base-content/70",
                                "Reduce padding and spacing for more content"
                            }
                        }
                        
                        input {
                            r#type: "checkbox",
                            class: "toggle toggle-primary",
                            checked: *compact_mode.read(),
                            onchange: move |e| compact_mode.set(e.checked()),
                        }
                    }
                }
            }
            
            // Save button
            div {
                class: "flex justify-end",
                
                button {
                    class: "btn btn-primary",
                    "Save Preferences"
                }
            }
        }
    }
}
