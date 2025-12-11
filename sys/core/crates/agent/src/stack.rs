//! MicroAgentStack implementation

use noa_common::{AgentState, AgentType, EntityId, Metadata, Timestamp};
use serde::{Deserialize, Serialize};

/// MicroAgentStack - temporary agent groupings for bounded objectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroAgentStack {
    pub metadata: Metadata,
    /// Stack code: mas_* (reusable) or gen_mas (disposable)
    pub stack_code: String,
    /// Type of stack
    pub stack_type: AgentType,
    /// Current lifecycle state
    pub state: AgentState,
    /// Objective description
    pub objective: String,
    /// IDs of agents in this stack
    pub agent_ids: Vec<EntityId>,
    /// When the stack was activated
    pub activated_at: Option<Timestamp>,
    /// When the stack completed or terminated
    pub completed_at: Option<Timestamp>,
}

impl MicroAgentStack {
    /// Create a new reusable MicroAgentStack
    pub fn reusable(name: &str, objective: &str) -> Self {
        Self {
            metadata: Metadata::new(),
            stack_code: format!("mas_{}", name),
            stack_type: AgentType::ReusableStack,
            state: AgentState::Bootstrap,
            objective: objective.to_string(),
            agent_ids: Vec::new(),
            activated_at: None,
            completed_at: None,
        }
    }

    /// Create a new disposable MicroAgentStack (gen_mas)
    pub fn disposable(objective: &str) -> Self {
        Self {
            metadata: Metadata::new(),
            stack_code: "gen_mas".to_string(),
            stack_type: AgentType::DisposableStack,
            state: AgentState::Bootstrap,
            objective: objective.to_string(),
            agent_ids: Vec::new(),
            activated_at: None,
            completed_at: None,
        }
    }

    /// Transition to next lifecycle state
    pub fn transition(&mut self, next_state: AgentState) -> noa_common::Result<()> {
        use AgentState::*;

        let valid = match (self.state, next_state) {
            (Bootstrap, Execute) => true,
            (Execute, Validate) => true,
            (Validate, Package) => true,
            (Package, Archive) => true,
            // Also allow going back to Execute from Validate (retry)
            (Validate, Execute) => true,
            _ => false,
        };

        if valid {
            self.state = next_state;
            self.metadata.touch();

            if next_state == Execute && self.activated_at.is_none() {
                self.activated_at = Some(noa_common::now());
            }
            if next_state == Archive {
                self.completed_at = Some(noa_common::now());
            }
            Ok(())
        } else {
            Err(noa_common::NoaError::validation(format!(
                "Invalid state transition from {:?} to {:?}",
                self.state, next_state
            )))
        }
    }

    /// Check if this is a disposable stack
    pub fn is_disposable(&self) -> bool {
        self.stack_type == AgentType::DisposableStack
    }

    /// Add an agent to this stack
    pub fn add_agent(&mut self, agent_id: EntityId) {
        self.agent_ids.push(agent_id);
        self.metadata.touch();
    }
}
