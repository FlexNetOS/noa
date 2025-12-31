use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Busy,
    Error,
}

pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn register_agent(&mut self, agent: Agent) -> Result<()> {
        self.agents.insert(agent.id.clone(), agent);
        Ok(())
    }

    pub fn get_agent(&self, id: &str) -> Option<Agent> {
        self.agents.get(id).cloned()
    }

    pub fn list_agents(&self) -> Vec<Agent> {
        self.agents.values().cloned().collect()
    }
}