use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Resource Graph (DRG)
///
/// Maps resource allocation and consumption across components.
pub struct DynamicResourceGraph {
    graph: DomainGraph,
}

impl DynamicResourceGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("drg", GraphKind::Resource),
        }
    }

    pub fn track_resource(&mut self, id: &str, capacity: f64, metadata: Value) {
        self.graph.upsert_node(
            id,
            "resource",
            serde_json::json!({
                "capacity": capacity,
                "metadata": metadata
            }),
        );
    }

    pub fn allocate(&mut self, resource: &str, consumer: &str, amount: f64) {
        self.graph.upsert_node(consumer, "consumer", serde_json::json!({}));
        self.graph.link(
            resource,
            consumer,
            "allocates",
            amount,
            serde_json::json!({ "amount": amount }),
        );
    }
}

impl Default for DynamicResourceGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicResourceGraph {
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
