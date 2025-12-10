use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Hardware Graph (DHG)
///
/// Models hardware resources, accelerators, and device topology.
pub struct DynamicHardwareGraph {
    graph: DomainGraph,
}

impl DynamicHardwareGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dhg", GraphKind::Hardware),
        }
    }

    pub fn register_device(&mut self, id: &str, device_type: &str, metadata: Value) {
        self.graph.upsert_node(
            id,
            "device",
            serde_json::json!({
                "device_type": device_type,
                "metadata": metadata
            }),
        );
    }

    pub fn attach(&mut self, parent: &str, child: &str) {
        self.graph.link(parent, child, "attached", 1.0, serde_json::json!({}));
    }
}

impl Default for DynamicHardwareGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicHardwareGraph {
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
