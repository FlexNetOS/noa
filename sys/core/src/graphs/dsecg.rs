use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Security Graph (DSeCG)
///
/// Links controls, detections, and assets for continuous assurance.
pub struct DynamicSecurityGraph {
    graph: DomainGraph,
}

impl DynamicSecurityGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dsecg", GraphKind::Security),
        }
    }

    pub fn register_control(&mut self, control: &str, coverage: f64, metadata: Value) {
        self.graph.upsert_node(
            control,
            "control",
            serde_json::json!({
                "coverage": coverage,
                "metadata": metadata
            }),
        );
    }

    pub fn attach_asset(&mut self, control: &str, asset: &str) {
        self.graph
            .upsert_node(asset, "asset", serde_json::json!({}));
        self.graph.link(control, asset, "protects", 1.0, serde_json::json!({}));
    }
}

impl Default for DynamicSecurityGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicSecurityGraph {
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
