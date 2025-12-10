//! Workflow DAG Engine
//!
//! T182: Implement workflow DAG engine
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::error::Result;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Workflow node representing a stage in the digest pipeline
#[derive(Debug, Clone)]
pub struct WorkflowNode {
    pub id: Uuid,
    pub name: String,
    pub stage: String,
    pub dependencies: Vec<Uuid>,
}

/// Workflow DAG engine for orchestrating digest pipeline stages
pub struct WorkflowDAGEngine {
    nodes: HashMap<Uuid, WorkflowNode>,
    edges: HashMap<Uuid, Vec<Uuid>>,
}

impl WorkflowDAGEngine {
    /// Create a new workflow DAG engine
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Add a node to the workflow
    pub fn add_node(&mut self, node: WorkflowNode) {
        self.nodes.insert(node.id, node.clone());
        self.edges.insert(node.id, node.dependencies.clone());
    }

    /// Get topological sort of nodes (execution order)
    pub fn topological_sort(&self) -> Result<Vec<Uuid>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                self.visit(*node_id, &mut visited, &mut temp_visited, &mut result)?;
            }
        }

        result.reverse();
        Ok(result)
    }

    fn visit(
        &self,
        node_id: Uuid,
        visited: &mut HashSet<Uuid>,
        temp_visited: &mut HashSet<Uuid>,
        result: &mut Vec<Uuid>,
    ) -> Result<()> {
        if temp_visited.contains(&node_id) {
            return Err(crate::error::NoaError::Validation(
                crate::error::ValidationError::new(
                    "workflow",
                    "Circular dependency detected in workflow DAG",
                    "CIRCULAR_DEPENDENCY",
                ),
            ));
        }

        if visited.contains(&node_id) {
            return Ok(());
        }

        temp_visited.insert(node_id);

        if let Some(dependencies) = self.edges.get(&node_id) {
            for dep_id in dependencies {
                self.visit(*dep_id, visited, temp_visited, result)?;
            }
        }

        temp_visited.remove(&node_id);
        visited.insert(node_id);
        result.push(node_id);

        Ok(())
    }

    /// Get nodes that can be executed in parallel (no dependencies between them)
    pub fn get_parallel_groups(&self) -> Vec<Vec<Uuid>> {
        let sorted = self.topological_sort().unwrap_or_default();
        let mut groups = Vec::new();
        let mut remaining: HashSet<Uuid> = sorted.iter().copied().collect();
        let mut completed: HashSet<Uuid> = HashSet::new();

        while !remaining.is_empty() {
            let mut current_group = Vec::new();

            for node_id in &remaining {
                let deps = self.edges.get(node_id).unwrap_or(&Vec::new());
                if deps.iter().all(|dep| completed.contains(dep)) {
                    current_group.push(*node_id);
                }
            }

            if current_group.is_empty() {
                // Should not happen if DAG is valid, but handle gracefully
                break;
            }

            for node_id in &current_group {
                completed.insert(*node_id);
                remaining.remove(node_id);
            }

            groups.push(current_group);
        }

        groups
    }
}

impl Default for WorkflowDAGEngine {
    fn default() -> Self {
        Self::new()
    }
}

