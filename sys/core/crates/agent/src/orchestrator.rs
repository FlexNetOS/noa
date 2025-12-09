//! CECCA - Chief Executive Command & Control Agent
//!
//! Root orchestrator for all NOA agent activities.

use noa_common::{EntityId, Priority, Result};
use std::collections::HashMap;

use crate::stack::MicroAgentStack;

/// CECCA Orchestrator - decomposes goals and routes to agents
pub struct Orchestrator {
    /// Active MicroAgentStacks by ID
    stacks: HashMap<EntityId, MicroAgentStack>,
}

impl Orchestrator {
    /// Create a new orchestrator
    pub fn new() -> Self {
        Self {
            stacks: HashMap::new(),
        }
    }

    /// Decompose a goal into tasks
    pub async fn decompose_goal(&self, goal: &str) -> Result<Vec<Task>> {
        // Placeholder implementation
        // Will use LLM to decompose goals into actionable tasks
        Ok(vec![Task {
            id: noa_common::new_id(),
            description: goal.to_string(),
            priority: Priority::Normal,
        }])
    }

    /// Create a new MicroAgentStack for an objective
    pub fn create_stack(&mut self, objective: &str, reusable: bool) -> EntityId {
        let stack = if reusable {
            MicroAgentStack::reusable("custom", objective)
        } else {
            MicroAgentStack::disposable(objective)
        };
        let id = stack.metadata.id;
        self.stacks.insert(id, stack);
        id
    }

    /// Get a stack by ID
    pub fn get_stack(&self, id: EntityId) -> Option<&MicroAgentStack> {
        self.stacks.get(&id)
    }

    /// Get mutable stack by ID
    pub fn get_stack_mut(&mut self, id: EntityId) -> Option<&mut MicroAgentStack> {
        self.stacks.get_mut(&id)
    }

    /// Route a task to the appropriate agent/stack
    pub async fn route_task(&mut self, task: Task) -> Result<EntityId> {
        // Placeholder: create a disposable stack for each task
        let stack_id = self.create_stack(&task.description, false);
        Ok(stack_id)
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// A task to be executed by agents
#[derive(Debug, Clone)]
pub struct Task {
    pub id: EntityId,
    pub description: String,
    pub priority: Priority,
}

