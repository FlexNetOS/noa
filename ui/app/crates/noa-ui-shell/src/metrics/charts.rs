//! Chart components for metrics visualization.

use dioxus::prelude::*;

/// Metric chart placeholder.
///
/// In a full implementation, this would use a charting library
/// like plotters or chart.js via web bindings.
#[component]
pub fn MetricChart() -> Element {
    rsx! {
        div {
            class: "metric-chart w-full h-64 bg-base-300 rounded-lg flex items-center justify-center",
            
            div {
                class: "text-center text-base-content/50",
                
                div {
                    class: "text-4xl mb-2",
                    "📊"
                }
                
                p {
                    "Performance chart"
                }
                
                p {
                    class: "text-sm mt-2",
                    "Real-time metrics visualization"
                }
                
                // Simple ASCII-art style chart placeholder
                div {
                    class: "font-mono text-xs mt-4 text-left inline-block",
                    
                    pre {
                        r#"
    100│    ╭─╮
       │   ╭╯ ╰╮    ╭──╮
    50 │  ╭╯   ╰────╯  ╰─╮
       │──╯              ╰──
     0 └────────────────────
        1m    5m   15m   1h
                        "#
                    }
                }
            }
        }
    }
}

/// Sparkline mini chart.
#[component]
pub fn Sparkline(values: Vec<f32>, height: u32) -> Element {
    if values.is_empty() {
        return rsx! { div { class: "w-full h-4" } };
    }
    
    let max = values.iter().cloned().fold(f32::MIN, f32::max);
    let min = values.iter().cloned().fold(f32::MAX, f32::min);
    let range = (max - min).max(1.0);
    
    let bar_width = 100.0 / values.len() as f32;
    
    rsx! {
        div {
            class: "sparkline flex items-end gap-px h-{height}",
            
            for (i, value) in values.iter().enumerate() {
                {
                    let normalized = ((value - min) / range * 100.0).max(5.0);
                    rsx! {
                        div {
                            class: "bg-primary rounded-t",
                            style: "width: {bar_width}%; height: {normalized}%;",
                        }
                    }
                }
            }
        }
    }
}
