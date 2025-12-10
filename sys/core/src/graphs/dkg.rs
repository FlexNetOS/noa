use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Dynamic Knowledge Graph (DKG)
///
/// Captures knowledge entities and relationships produced by the digest pipeline.
pub struct DynamicKnowledgeGraph {
    graph: DomainGraph,
}

impl DynamicKnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("dkg", GraphKind::Knowledge),
        }
    }

    pub fn upsert_concept(&mut self, id: &str, label: &str, metadata: Value) {
        self.graph.upsert_node(id, label, metadata);
    }

    pub fn relate(&mut self, source: &str, target: &str, relation: &str) {
        self.graph.link(
            source,
            target,
            relation,
            1.0,
            serde_json::json!({ "relation": relation }),
        );
    }
}

impl Default for DynamicKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for DynamicKnowledgeGraph {
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
