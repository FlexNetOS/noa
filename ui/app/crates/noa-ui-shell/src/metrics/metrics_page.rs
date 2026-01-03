//! Metrics page component.

use dioxus::prelude::*;

use super::{StatusCard, MetricChart};

/// System status.
#[derive(Clone, Debug)]
pub struct SystemStatus {
    pub api_healthy: bool,
    pub llama_healthy: bool,
    pub p2p_peers: u32,
    pub active_agents: u32,
}

/// Resource usage.
#[derive(Clone, Debug)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
    pub gpu_percent: Option<f32>,
}

/// Inference stats.
#[derive(Clone, Debug)]
pub struct InferenceStats {
    pub total_requests: u64,
    pub avg_latency_ms: f32,
    pub tokens_per_second: f32,
    pub cache_hit_rate: f32,
}

/// Metrics page with real-time monitoring.
#[component]
pub fn MetricsPage() -> Element {
    let status = use_signal(|| SystemStatus {
        api_healthy: true,
        llama_healthy: true,
        p2p_peers: 0,
        active_agents: 2,
    });
    
    let resources = use_signal(|| ResourceUsage {
        cpu_percent: 25.0,
        memory_percent: 42.0,
        disk_percent: 35.0,
        gpu_percent: Some(15.0),
    });
    
    let inference = use_signal(|| InferenceStats {
        total_requests: 1247,
        avg_latency_ms: 125.3,
        tokens_per_second: 42.5,
        cache_hit_rate: 0.85,
    });
    
    rsx! {
        div {
            class: "metrics-page p-6 space-y-6 overflow-y-auto",
            
            // Header
            h1 {
                class: "text-2xl font-bold",
                "System Metrics"
            }
            
            // Status cards
            div {
                class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                
                StatusCard {
                    title: "API Server".to_string(),
                    status: (if status.read().api_healthy { "healthy" } else { "error" }).to_string(),
                    value: (if status.read().api_healthy { "Online" } else { "Offline" }).to_string(),
                    icon: "🌐".to_string(),
                }
                
                StatusCard {
                    title: "LLM Engine".to_string(),
                    status: (if status.read().llama_healthy { "healthy" } else { "error" }).to_string(),
                    value: (if status.read().llama_healthy { "Running" } else { "Stopped" }).to_string(),
                    icon: "🧠".to_string(),
                }
                
                StatusCard {
                    title: "P2P Peers".to_string(),
                    status: (if status.read().p2p_peers > 0 { "healthy" } else { "warning" }).to_string(),
                    value: status.read().p2p_peers.to_string(),
                    icon: "🔗".to_string(),
                }
                
                StatusCard {
                    title: "Active Agents".to_string(),
                    status: "healthy".to_string(),
                    value: status.read().active_agents.to_string(),
                    icon: "🤖".to_string(),
                }
            }
            
            // Resource usage
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h2 {
                        class: "card-title",
                        "Resource Usage"
                    }
                    
                    div {
                        class: "grid grid-cols-2 md:grid-cols-4 gap-6 mt-4",
                        
                        ResourceBar {
                            label: "CPU".to_string(),
                            value: resources.read().cpu_percent,
                            color: "primary".to_string(),
                        }
                        
                        ResourceBar {
                            label: "Memory".to_string(),
                            value: resources.read().memory_percent,
                            color: "secondary".to_string(),
                        }
                        
                        ResourceBar {
                            label: "Disk".to_string(),
                            value: resources.read().disk_percent,
                            color: "accent".to_string(),
                        }
                        
                        if let Some(gpu) = resources.read().gpu_percent {
                            ResourceBar {
                                label: "GPU".to_string(),
                                value: gpu,
                                color: "success".to_string(),
                            }
                        }
                    }
                }
            }
            
            // Inference stats
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h2 {
                        class: "card-title",
                        "Inference Performance"
                    }
                    
                    div {
                        class: "stats stats-vertical md:stats-horizontal shadow w-full mt-4",
                        
                        div {
                            class: "stat",
                            
                            div { class: "stat-title", "Total Requests" }
                            div { class: "stat-value", "{inference.read().total_requests}" }
                            div { class: "stat-desc", "Since startup" }
                        }
                        
                        div {
                            class: "stat",
                            
                            div { class: "stat-title", "Avg Latency" }
                            div { class: "stat-value", "{inference.read().avg_latency_ms:.1}ms" }
                            div { class: "stat-desc", "Response time" }
                        }
                        
                        div {
                            class: "stat",
                            
                            div { class: "stat-title", "Throughput" }
                            div { class: "stat-value", "{inference.read().tokens_per_second:.1}" }
                            div { class: "stat-desc", "Tokens/second" }
                        }
                        
                        div {
                            class: "stat",
                            
                            div { class: "stat-title", "Cache Hit Rate" }
                            div { class: "stat-value", "{inference.read().cache_hit_rate * 100.0:.0}%" }
                            div { class: "stat-desc", "KV cache efficiency" }
                        }
                    }
                }
            }
            
            // Charts placeholder
            div {
                class: "card bg-base-200",
                
                div {
                    class: "card-body",
                    
                    h2 {
                        class: "card-title",
                        "Performance History"
                    }
                    
                    MetricChart {}
                }
            }
        }
    }
}

/// Resource usage bar.
#[component]
fn ResourceBar(label: String, value: f32, color: String) -> Element {
    let status = if value > 90.0 { "error" } else if value > 70.0 { "warning" } else { &color };
    
    rsx! {
        div {
            class: "space-y-2",
            
            div {
                class: "flex justify-between text-sm",
                
                span { {label} }
                span { class: "font-mono", "{value:.1}%" }
            }
            
            progress {
                class: "progress progress-{status} w-full",
                value: "{value}",
                max: "100",
            }
        }
    }
}
