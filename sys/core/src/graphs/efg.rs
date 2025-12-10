use chrono::Duration;
use serde_json::Value;

use crate::graphs::{DomainGraph, DynamicGraph, GraphEvent, GraphKind, GraphSnapshot};

/// Environment Function Graph (EFG)
///
/// Captures how environmental inputs map to functions and components.
pub struct EnvironmentFunctionGraph {
    graph: DomainGraph,
}

impl EnvironmentFunctionGraph {
    pub fn new() -> Self {
        Self {
            graph: DomainGraph::new("efg", GraphKind::Environment),
        }
    }

    /// Record a function and the component providing it.
    pub fn observe_function(&mut self, function: &str, component: &str, metadata: Value) {
        self.graph.upsert_node(function, "function", metadata.clone());
        self.graph.upsert_node(
            component,
            "component",
            serde_json::json!({ "role": "provider" }),
        );
        self.graph.link(component, function, "provides", 1.0, metadata);
    }

    /// Relate two functions through a dependency edge.
    pub fn relate_functions(&mut self, from: &str, to: &str, weight: f64) {
        self.graph.upsert_node(from, "function", serde_json::json!({}));
        self.graph.upsert_node(to, "function", serde_json::json!({}));
        self.graph.link(
            from,
            to,
            "depends_on",
            weight,
            serde_json::json!({ "type": "dependency" }),
        );
    }
}

impl Default for EnvironmentFunctionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicGraph for EnvironmentFunctionGraph {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relates_functions() {
        let mut graph = EnvironmentFunctionGraph::new();
        graph.relate_functions("ingest", "normalize", 0.8);
        let snap = graph.snapshot();
        assert_eq!(snap.nodes.len(), 2);
        assert_eq!(snap.edges.len(), 1);
        assert_eq!(snap.edges[0].relation, "depends_on");
    }
}
