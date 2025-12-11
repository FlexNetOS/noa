//! ToolkenGPT Token Registry
//!
//! T658: Implement ToolkenGPT token registry
//! US2: Registry for tool tokens and embeddings

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Tool token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolToken {
    pub id: Uuid,
    pub tool_name: String,
    pub token_id: u32,
    pub embedding: Vec<f32>,
    pub description: String,
    pub capabilities: Vec<String>,
}

/// ToolkenGPT token registry
#[derive(Clone)]
pub struct ToolkenGptRegistry {
    tokens: Arc<RwLock<HashMap<String, ToolToken>>>,
    token_id_counter: Arc<RwLock<u32>>,
}

impl Clone for ToolkenGptRegistry {
    fn clone(&self) -> Self {
        Self {
            tokens: Arc::clone(&self.tokens),
            token_id_counter: Arc::clone(&self.token_id_counter),
        }
    }
}

impl ToolkenGptRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
            token_id_counter: Arc::new(RwLock::new(10000)), // Start after standard vocab
        }
    }

    /// Register a tool token
    pub async fn register_tool(
        &self,
        tool_name: String,
        description: String,
        capabilities: Vec<String>,
        embedding: Vec<f32>,
    ) -> Result<Uuid> {
        let mut counter = self.token_id_counter.write().await;
        let token_id = *counter;
        *counter += 1;

        let token = ToolToken {
            id: Uuid::new_v4(),
            tool_name: tool_name.clone(),
            token_id,
            embedding,
            description,
            capabilities,
        };

        let mut tokens = self.tokens.write().await;
        tokens.insert(tool_name, token.clone());

        Ok(token.id)
    }

    /// Get tool token by name
    pub async fn get_tool(&self, tool_name: &str) -> Option<ToolToken> {
        let tokens = self.tokens.read().await;
        tokens.get(tool_name).cloned()
    }

    /// List all registered tools
    pub async fn list_tools(&self) -> Vec<ToolToken> {
        let tokens = self.tokens.read().await;
        tokens.values().cloned().collect()
    }

    /// Find tools by capability
    pub async fn find_by_capability(&self, capability: &str) -> Vec<ToolToken> {
        let tokens = self.tokens.read().await;
        tokens
            .values()
            .filter(|token| token.capabilities.contains(&capability.to_string()))
            .cloned()
            .collect()
    }
}

impl Default for ToolkenGptRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_tool() {
        let registry = ToolkenGptRegistry::new();
        let embedding = vec![0.1; 384];
        let id = registry
            .register_tool(
                "test_tool".to_string(),
                "Test tool".to_string(),
                vec!["test".to_string()],
                embedding,
            )
            .await
            .unwrap();
        assert!(!id.to_string().is_empty());
    }
}
