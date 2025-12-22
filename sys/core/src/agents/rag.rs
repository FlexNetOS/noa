use crate::agents::base::BaseAgent;
use crate::error::{NoaError, Result};
use crate::db::repositories::{KnowledgeNodeRepository, EmbeddingRepository};
use crate::db::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
pub struct RAGAgent {
    conn: Option<Arc<Connection>>,
    embedding_model: String,
}

impl RAGAgent {
    /// Create a new RAG agent without database connection (limited functionality)
    pub fn new() -> Self {
        Self {
            conn: None,
            embedding_model: "default".into(),
        }
    }

    /// Create RAG agent with database connection for full functionality
    pub fn with_connection(conn: Arc<Connection>) -> Self {
        Self {
            conn: Some(conn),
            embedding_model: "default".into(),
        }
    }

    /// Set the embedding model to use
    pub fn with_embedding_model(mut self, model: String) -> Self {
        self.embedding_model = model;
        self
    }

    /// Execute a RAG query
    pub fn query(&self, query: RAGQuery) -> Result<RAGResult> {
        let conn = self.conn.as_ref().ok_or_else(|| {
            NoaError::Internal {
                message: "RAGAgent requires database connection".into(),
                source: None,
            }
        })?;

        let top_k = query.top_k.unwrap_or(5);
        
        // Retrieve relevant knowledge nodes
        let node_repo = KnowledgeNodeRepository::new(conn);
        let nodes = node_repo.search(&query.query, top_k)?;

        let mut items = Vec::new();
        for node in nodes {
            items.push(RAGResultItem {
                content: node.content.clone(),
                score: 0.85, // Placeholder score
                source: Some(node.source_ref.clone()),
                metadata: Some(node.metadata.clone()),
            });
        }

        Ok(RAGResult {
            total_found: items.len(),
            items,
            query: query.query,
        })
    }

    /// Retrieve context for a query (simplified version without embeddings)
    pub fn retrieve_context(&self, query: &str, top_k: usize) -> Result<Vec<String>> {
        let rag_query = RAGQuery {
            query: query.to_string(),
            top_k: Some(top_k),
            filters: None,
            include_sources: false,
        };

        let result = self.query(rag_query)?;
        Ok(result.items.into_iter().map(|item| item.content).collect())
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

    /// Full RAG pipeline: retrieve + generate
    pub fn execute_rag(&self, query: &str, top_k: usize) -> Result<String> {
        // Retrieve relevant context
        let context = self.retrieve_context(query, top_k)?;
        
        if context.is_empty() {
            return Ok(format!("No relevant context found for: {}", query));
        }

        // Generate response with context
        let response = self.generate_with_context(query, &context)?;
        Ok(response)
    }

    /// Add a document to the knowledge base
    pub fn add_document(&self, content: String, source: String, metadata: serde_json::Value) -> Result<i64> {
        let conn = self.conn.as_ref().ok_or_else(|| {
            NoaError::Internal {
                message: "RAGAgent requires database connection".into(),
                source: None,
            }
        })?;

        let node_repo = KnowledgeNodeRepository::new(conn);
        let node_id = node_repo.create_node(
            "document".to_string(),
            content,
            source,
            metadata,
        )?;

        Ok(node_id)
    }

    /// Search knowledge base
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        let conn = self.conn.as_ref().ok_or_else(|| {
            NoaError::Internal {
                message: "RAGAgent requires database connection".into(),
                source: None,
            }
        })?;

        let node_repo = KnowledgeNodeRepository::new(conn);
        let nodes = node_repo.search(query, limit)?;
        
        Ok(nodes.into_iter().map(|n| n.content).collect())
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
        "Retrieval-augmented generation: searches knowledge base and generates contextual responses"
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "retrieve".into(),
            "generate".into(),
            "search".into(),
            "index".into(),
            "embed".into(),
        ]
    }

    fn execute(&self, task: &str) -> Result<String> {
        // Try to parse as JSON query
        match serde_json::from_str::<RAGQuery>(task) {
            Ok(query) => {
                let result = self.query(query)?;
                Ok(serde_json::to_string(&result)?)
            }
            Err(_) => {
                // Fallback: treat as simple search query
                if self.conn.is_some() {
                    let result = self.execute_rag(task, 5)?;
                    Ok(result)
                } else {
                    Ok(format!(
                        "RAGAgent processed query '{}' (limited mode: no database connection)",
                        task
                    ))
                }
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
}

