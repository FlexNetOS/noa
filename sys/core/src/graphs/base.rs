use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Graph classification used to specialize behavior per domain.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GraphKind {
    Environment,
    Software,
    Hardware,
    Process,
    Resource,
    Security,
    Performance,
    Error,
    Knowledge,
}

/// Node representation within a dynamic graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: GraphKind,
    pub metadata: serde_json::Value,
    pub last_seen: DateTime<Utc>,
}

/// Edge representation between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub weight: f64,
    pub metadata: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

/// Event log for dynamic graph updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphEvent {
    NodeUpsert(GraphNode),
    EdgeUpsert(GraphEdge),
    Metric {
        node_id: String,
        key: String,
        value: f64,
        at: DateTime<Utc>,
    },
}

/// Snapshot of a graph's state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub name: String,
    pub kind: GraphKind,
    pub version: u64,
    pub updated_at: DateTime<Utc>,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Common behavior for all dynamic graphs.
pub trait DynamicGraph {
    fn kind(&self) -> GraphKind;
    fn name(&self) -> &str;
    fn version(&self) -> u64;
    fn snapshot(&self) -> GraphSnapshot;
    fn record_event(&mut self, event: GraphEvent);
    fn prune_stale(&mut self, max_age: Duration);
}

/// Shared graph implementation reused by specialized graphs.
#[derive(Debug, Clone)]
pub struct DomainGraph {
    name: String,
    kind: GraphKind,
    version: u64,
    nodes: HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
    events: Vec<GraphEvent>,
}

impl DomainGraph {
    pub fn new(name: impl Into<String>, kind: GraphKind) -> Self {
        Self {
            name: name.into(),
            kind,
            version: 0,
            nodes: HashMap::new(),
            edges: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn upsert_node(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        metadata: serde_json::Value,
    ) {
        let id = id.into();
        let label: String = label.into();
        let now = Utc::now();
        let entry = self.nodes.entry(id.clone()).or_insert(GraphNode {
            id: id.clone(),
            label: label.clone(),
            kind: self.kind,
            metadata: serde_json::json!({}),
            last_seen: now,
        });

        entry.label = label;
        entry.metadata = metadata;
        entry.last_seen = now;

        self.version += 1;
        self.events.push(GraphEvent::NodeUpsert(entry.clone()));
    }

    pub fn link(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        relation: impl Into<String>,
        weight: f64,
        metadata: serde_json::Value,
    ) {
        let edge = GraphEdge {
            from: from.into(),
            to: to.into(),
            relation: relation.into(),
            weight,
            metadata,
            updated_at: Utc::now(),
        };

        self.edges.push(edge.clone());
        self.version += 1;
        self.events.push(GraphEvent::EdgeUpsert(edge));
    }

    pub fn record_metric(
        &mut self,
        node_id: impl Into<String>,
        key: impl Into<String>,
        value: f64,
    ) {
        self.events.push(GraphEvent::Metric {
            node_id: node_id.into(),
            key: key.into(),
            value,
            at: Utc::now(),
        });
    }

    pub fn nodes(&self) -> &HashMap<String, GraphNode> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<GraphEdge> {
        &self.edges
    }

    pub fn kind(&self) -> GraphKind {
        self.kind
    }

    pub fn graph_name(&self) -> &str {
        &self.name
    }

    pub fn graph_version(&self) -> u64 {
        self.version
    }

    pub fn events(&self) -> &Vec<GraphEvent> {
        &self.events
    }
}

impl DynamicGraph for DomainGraph {
    fn kind(&self) -> GraphKind {
        self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn snapshot(&self) -> GraphSnapshot {
        GraphSnapshot {
            name: self.name.clone(),
            kind: self.kind,
            version: self.version,
            updated_at: Utc::now(),
            nodes: self.nodes.values().cloned().collect(),
            edges: self.edges.clone(),
        }
    }

    fn record_event(&mut self, event: GraphEvent) {
        match &event {
            GraphEvent::NodeUpsert(node) => {
                self.nodes.insert(node.id.clone(), node.clone());
                self.version += 1;
            }
            GraphEvent::EdgeUpsert(edge) => {
                self.edges.push(edge.clone());
                self.version += 1;
            }
            GraphEvent::Metric { .. } => {}
        }
        self.events.push(event);
    }

    fn prune_stale(&mut self, max_age: Duration) {
        let cutoff = Utc::now() - max_age;
        self.nodes.retain(|_, node| node.last_seen >= cutoff);
        self.edges.retain(|edge| {
            self.nodes.contains_key(&edge.from) && self.nodes.contains_key(&edge.to)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_upsert_updates_version() {
        let mut graph = DomainGraph::new("efg", GraphKind::Environment);
        graph.upsert_node("sensor-1", "Sensor", serde_json::json!({"health": "ok"}));
        graph.upsert_node("sensor-1", "Sensor", serde_json::json!({"health": "warn"}));
        assert_eq!(graph.version(), 2);
        assert_eq!(graph.nodes().len(), 1);
    }

    #[test]
    fn snapshot_contains_edges_and_nodes() {
        let mut graph = DomainGraph::new("dsg", GraphKind::Software);
        graph.upsert_node(
            "service-a",
            "Service A",
            serde_json::json!({"version": "1.0"}),
        );
        graph.upsert_node(
            "service-b",
            "Service B",
            serde_json::json!({"version": "1.1"}),
        );
        graph.link(
            "service-a",
            "service-b",
            "calls",
            1.0,
            serde_json::json!({}),
        );

        let snap = graph.snapshot();
        assert_eq!(snap.nodes.len(), 2);
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn prune_removes_stale_nodes() {
        let mut graph = DomainGraph::new("drg", GraphKind::Resource);
        let old_time = Utc::now() - Duration::days(2);

        graph.record_event(GraphEvent::NodeUpsert(GraphNode {
            id: "old".to_string(),
            label: "Old".to_string(),
            kind: GraphKind::Resource,
            metadata: serde_json::json!({}),
            last_seen: old_time,
        }));

        graph.prune_stale(Duration::hours(12));
        assert!(graph.nodes().is_empty());
    }
}
