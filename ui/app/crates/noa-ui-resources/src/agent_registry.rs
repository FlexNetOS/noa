//! Agent registry for managing AI agents

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An AI agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Busy,
    Error,
}

/// Registry for managing agents
pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
}

impl AgentRegistry {
    /// Create a new AgentRegistry
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Initialize the registry
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Register an agent
    pub fn register_agent(&mut self, agent: Agent) -> Result<()> {
        self.agents.insert(agent.id.clone(), agent);
        Ok(())
    }

    /// Get an agent by ID
    pub fn get_agent(&self, id: &str) -> Option<Agent> {
        self.agents.get(id).cloned()
    }

    /// List all agents
    pub fn list_agents(&self) -> Vec<Agent> {
        self.agents.values().cloned().collect()
    }

    /// Update agent status
    pub fn update_status(&mut self, id: &str, status: AgentStatus) -> Result<()> {
        if let Some(agent) = self.agents.get_mut(id) {
            agent.status = status;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    /// Remove an agent
    pub fn remove_agent(&mut self, id: &str) -> Option<Agent> {
        self.agents.remove(id)
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
