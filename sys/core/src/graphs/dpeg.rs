use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Performance Graph (DPeG)
///
/// Tracks performance signals between components.
pub struct DynamicPerformanceGraph {
    graph: DomainGraph,
}

impl DynamicPerformanceGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dpeg", GraphKind::Performance),
        }
    }

    pub fn record_latency(&mut self, producer: &str, consumer: &str, latency_ms: f64) {
        self.graph.link(
            producer,
            consumer,
            "latency",
            latency_ms,
            serde_json::json!({ "latency_ms": latency_ms }),
        );
    }

    pub fn record_throughput(&mut self, component: &str, value: f64) {
        self.graph.record_metric(component.to_string(), "throughput", value);
    }
}

impl Default for DynamicPerformanceGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicPerformanceGraph {
    fn kind(&self) -> GraphKind {
        self.graph.kind()
    }

    fn name(&self) -> &str {
        self.graph.graph_name()
    }

    fn version(&self) -> u64 {
        self.graph.graph_version()
    }

    fn snapshot(&self) -> GraphSnapshot {
        self.graph.snapshot()
    }

    fn record_event(&mut self, event: GraphEvent) {
        self.graph.record_event(event);
    }

    fn prune_stale(&mut self, max_age: Duration) {
        self.graph.prune_stale(max_age);
    }
}
