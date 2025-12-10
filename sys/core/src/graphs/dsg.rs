use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Software Graph (DSG)
///
/// Tracks services, versions, and their runtime relationships.
pub struct DynamicSoftwareGraph {
    graph: DomainGraph,
}

impl DynamicSoftwareGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dsg", GraphKind::Software),
        }
    }

    pub fn register_service(&mut self, name: &str, version: &str, metadata: Value) {
        self.graph.upsert_node(
            name,
            "service",
            serde_json::json!({
                "version": version,
                "metadata": metadata
            }),
        );
    }

    pub fn connect(&mut self, caller: &str, callee: &str, latency_ms: f64) {
        self.graph
            .link(caller, callee, "calls", latency_ms, serde_json::json!({ "latency_ms": latency_ms }));
    }
}

impl Default for DynamicSoftwareGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicSoftwareGraph {
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
