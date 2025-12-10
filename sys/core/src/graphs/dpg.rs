use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Process Graph (DPG)
///
/// Tracks running processes and inter-process relationships.
pub struct DynamicProcessGraph {
    graph: DomainGraph,
}

impl DynamicProcessGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dpg", GraphKind::Process),
        }
    }

    pub fn observe_process(&mut self, pid: &str, command: &str, metadata: Value) {
        self.graph.upsert_node(
            pid,
            "process",
            serde_json::json!({
                "command": command,
                "metadata": metadata
            }),
        );
    }

    pub fn link_processes(&mut self, parent: &str, child: &str) {
        self.graph
            .link(parent, child, "spawns", 1.0, serde_json::json!({ "type": "spawn" }));
    }
}

impl Default for DynamicProcessGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicProcessGraph {
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
