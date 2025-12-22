use crate::agents::base::BaseAgent;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// RAG query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGQuery {
    pub query: String,
    pub top_k: Option<usize>,
    pub filters: Option<serde_json::Value>,
    pub include_sources: bool,
}

/// RAG result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGResultItem {
    pub content: String,
    pub score: f32,
    pub source: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// RAG query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGResult {
    pub items: Vec<RAGResultItem>,
    pub total_found: usize,
    pub query: String,
}

/// Retrieval-Augmented Generation Agent
/// 
/// This agent provides retrieval-augmented generation capabilities.
/// For full functionality with database integration, use the separate
/// RAG service that can maintain database connections.
pub struct RAGAgent {
    embedding_model: String,
}

impl RAGAgent {
    /// Create a new RAG agent
    pub fn new() -> Self {
        Self {
            embedding_model: "default".into(),
        }
    }

    /// Set the embedding model to use
    pub fn with_embedding_model(mut self, model: String) -> Self {
        self.embedding_model = model;
        self
    }

    /// Generate response with retrieved context
    pub fn generate_with_context(&self, query: &str, context: &[String]) -> Result<String> {
        // This is a simplified version - in a full implementation,
        // this would call the neural inference engine
        
        let formatted_context = context.join("\n\n---\n\n");
        let prompt = format!(
            "Context:\n{}\n\nQuestion: {}\n\nAnswer based on the context above:",
            formatted_context, query
        );

        Ok(prompt)
    }

    /// Format a RAG query for processing
    pub fn format_query(&self, query: &RAGQuery) -> String {
        format!(
            "Query: {}\nTop-K: {}\nModel: {}",
            query.query,
            query.top_k.unwrap_or(5),
            self.embedding_model
        )
    }
}

impl Default for RAGAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseAgent for RAGAgent {
    fn name(&self) -> &str {
        "rag"
    }

    fn description(&self) -> &str {
        "Retrieval-augmented generation agent"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["retrieve".into(), "generate".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        // Try to parse as JSON query
        match serde_json::from_str::<RAGQuery>(task) {
            Ok(query) => {
                Ok(format!(
                    "RAGAgent: {}\n(Full database integration available via RAG service)",
                    self.format_query(&query)
                ))
            }
            Err(_) => {
                // Fallback: treat as simple query
                Ok(format!(
                    "RAGAgent would retrieve context for: '{}'\n(Full database integration available via RAG service)",
                    task
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_agent_creation() {
        let agent = RAGAgent::new();
        assert_eq!(agent.name(), "rag");
        assert!(agent.capabilities().contains(&"retrieve".to_string()));
    }

    #[test]
    fn test_generate_with_context() {
        let agent = RAGAgent::new();
        let context = vec![
            "The capital of France is Paris.".to_string(),
            "Paris is known for the Eiffel Tower.".to_string(),
        ];
        let result = agent.generate_with_context("What is the capital of France?", &context);
        assert!(result.is_ok());
        let prompt = result.unwrap();
        assert!(prompt.contains("Paris"));
        assert!(prompt.contains("Context"));
    }

    #[test]
    fn test_format_query() {
        let agent = RAGAgent::new();
        let query = RAGQuery {
            query: "test query".into(),
            top_k: Some(10),
            filters: None,
            include_sources: true,
        };
        let formatted = agent.format_query(&query);
        assert!(formatted.contains("test query"));
        assert!(formatted.contains("10"));
    }
}
