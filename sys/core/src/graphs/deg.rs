use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Error Graph (DEG)
///
/// Maps faults, their sources, and impact radius.
pub struct DynamicErrorGraph {
    graph: DomainGraph,
}

impl DynamicErrorGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("deg", GraphKind::Error),
        }
    }

    pub fn record_error(&mut self, id: &str, component: &str, metadata: Value) {
        self.graph.upsert_node(
            id,
            "error",
            serde_json::json!({
                "component": component,
                "metadata": metadata
            }),
        );
    }

    pub fn propagate(&mut self, from: &str, to: &str, severity: f64) {
        self.graph.link(
            from,
            to,
            "impacts",
            severity,
            serde_json::json!({ "severity": severity }),
        );
    }
}

impl Default for DynamicErrorGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicErrorGraph {
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
