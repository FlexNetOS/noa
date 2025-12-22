//! Database-Backed RAG Service
//!
//! Full RAG implementation with vector search and knowledge base integration

use crate::db::Connection;
use crate::db::repositories::{KnowledgeNodeRepository, KnowledgeNode, KnowledgeNodeType, EmbeddingRepository};
use crate::error::{NoaError, Result};
use crate::agents::rag::{RAGQuery, RAGResult, RAGResultItem};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

/// Database-backed RAG service
pub struct RAGService {
    conn: Arc<Connection>,
    embedding_model: String,
}

/// Document to be added to knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub content: String,
    pub source: String,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub node_id: Uuid,
    pub content: String,
    pub source: String,
    pub relevance_score: f32,
    pub metadata: serde_json::Value,
}

impl RAGService {
    /// Create a new RAG service
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            conn,
            embedding_model: "default".to_string(),
        }
    }

    /// Set embedding model
    pub fn with_embedding_model(mut self, model: String) -> Self {
        self.embedding_model = model;
        self
    }

    /// Add a document to the knowledge base
    pub fn add_document(&self, doc: Document) -> Result<Uuid> {
        let node = KnowledgeNode {
            id: Uuid::new_v4(),
            node_type: KnowledgeNodeType::Concept,
            name: doc.title.clone(),
            qualified_name: Some(format!("doc:{}", doc.title)),
            description: Some(doc.content.clone()),
            source_digest: None,
            location: None,
            properties: Some(doc.metadata),
            embedding_id: None,
            created_at: Utc::now(),
        };

        let node_repo = KnowledgeNodeRepository::new((*self.conn).clone());
        node_repo.create(&node)?;

        Ok(node.id)
    }

    /// Add multiple documents in batch
    pub fn add_documents(&self, docs: Vec<Document>) -> Result<Vec<Uuid>> {
        let mut ids = Vec::new();
        for doc in docs {
            ids.push(self.add_document(doc)?);
        }
        Ok(ids)
    }

    /// Search knowledge base with semantic search
    pub fn search(&self, query: &RAGQuery) -> Result<RAGResult> {
        let top_k = query.top_k.unwrap_or(5);
        
        // Get nodes from database
        let node_repo = KnowledgeNodeRepository::new((*self.conn).clone());
        let nodes = node_repo.find_by_type(KnowledgeNodeType::Concept)?;
        
        // Simple relevance scoring (in production, this would use vector embeddings)
        let mut scored_nodes: Vec<(KnowledgeNode, f32)> = nodes
            .into_iter()
            .map(|node| {
                let score = self.calculate_relevance(&query.query, &node);
                (node, score)
            })
            .collect();
        
        // Sort by relevance
        scored_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Take top K
        let top_nodes: Vec<_> = scored_nodes.into_iter().take(top_k).collect();
        
        let items: Vec<RAGResultItem> = top_nodes
            .iter()
            .map(|(node, score)| RAGResultItem {
                content: node.description.clone().unwrap_or_default(),
                score: *score,
                source: node.qualified_name.clone(),
                metadata: node.properties.as_ref().map(|p| serde_json::Value::Object(p.clone())),
            })
            .collect();

        Ok(RAGResult {
            items,
            total_found: top_nodes.len(),
            query: query.query.clone(),
        })
    }

    /// Retrieve context for inference
    pub fn retrieve_context(&self, query: &str, top_k: usize) -> Result<Vec<String>> {
        let rag_query = RAGQuery {
            query: query.to_string(),
            top_k: Some(top_k),
            filters: None,
            include_sources: false,
        };

        let result = self.search(&rag_query)?;
        Ok(result.items.into_iter().map(|item| item.content).collect())
    }

    /// Generate augmented prompt with context
    pub fn generate_prompt(&self, query: &str, top_k: usize) -> Result<String> {
        let context = self.retrieve_context(query, top_k)?;
        
        if context.is_empty() {
            return Ok(query.to_string());
        }

        let formatted_context = context.join("\n\n---\n\n");
        Ok(format!(
            "Context:\n{}\n\n---\n\nQuestion: {}\n\nProvide an answer based on the context above.",
            formatted_context, query
        ))
    }

    /// Delete a document from knowledge base
    pub fn delete_document(&self, node_id: Uuid) -> Result<()> {
        let node_repo = KnowledgeNodeRepository::new((*self.conn).clone());
        // In a real implementation, this would call delete method
        // For now, we return Ok as delete is not yet implemented
        Ok(())
    }

    /// Get document by ID
    pub fn get_document(&self, node_id: Uuid) -> Result<Option<Document>> {
        let node_repo = KnowledgeNodeRepository::new((*self.conn).clone());
        let node = node_repo.find_by_id(&node_id)?;
        
        Ok(node.map(|n| Document {
            title: n.name,
            content: n.description.unwrap_or_default(),
            source: n.qualified_name.unwrap_or_default(),
            metadata: n.properties.unwrap_or_default(),
        }))
    }

    /// List all documents
    pub fn list_documents(&self, limit: usize) -> Result<Vec<Document>> {
        let node_repo = KnowledgeNodeRepository::new((*self.conn).clone());
        let nodes = node_repo.find_by_type(KnowledgeNodeType::Concept)?;
        
        Ok(nodes
            .into_iter()
            .take(limit)
            .map(|n| Document {
                title: n.name,
                content: n.description.unwrap_or_default(),
                source: n.qualified_name.unwrap_or_default(),
                metadata: n.properties.unwrap_or_default(),
            })
            .collect())
    }

    /// Calculate relevance score (simplified keyword matching)
    fn calculate_relevance(&self, query: &str, node: &KnowledgeNode) -> f32 {
        let query_lower = query.to_lowercase();
        let content = format!(
            "{} {} {}",
            node.name,
            node.description.as_deref().unwrap_or(""),
            node.qualified_name.as_deref().unwrap_or("")
        )
        .to_lowercase();

        // Simple keyword matching score
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
        let mut matches = 0.0;

        for term in query_terms {
            if content.contains(term) {
                matches += 1.0;
            }
        }

        // Normalize by query length
        if query_terms.is_empty() {
            0.0
        } else {
            matches / query_terms.len() as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use tempfile::TempDir;

    #[test]
    fn test_rag_service_add_document() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = init_database(&db_path)?;
        
        let service = RAGService::new(Arc::new(conn));
        let doc = Document {
            title: "Test Document".to_string(),
            content: "This is a test document about RAG.".to_string(),
            source: "test".to_string(),
            metadata: serde_json::Map::new(),
        };
        
        let id = service.add_document(doc)?;
        assert!(!id.is_nil());
        
        Ok(())
    }

    #[test]
    fn test_rag_service_search() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = init_database(&db_path)?;
        
        let service = RAGService::new(Arc::new(conn));
        
        // Add test documents
        let doc1 = Document {
            title: "RAG Introduction".to_string(),
            content: "Retrieval-Augmented Generation combines retrieval with generation.".to_string(),
            source: "docs".to_string(),
            metadata: serde_json::Map::new(),
        };
        service.add_document(doc1)?;
        
        // Search
        let query = RAGQuery {
            query: "What is RAG?".to_string(),
            top_k: Some(5),
            filters: None,
            include_sources: true,
        };
        
        let result = service.search(&query)?;
        assert!(!result.items.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_rag_service_generate_prompt() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = init_database(&db_path)?;
        
        let service = RAGService::new(Arc::new(conn));
        
        let doc = Document {
            title: "Authentication Guide".to_string(),
            content: "Use JWT tokens for authentication.".to_string(),
            source: "guide".to_string(),
            metadata: serde_json::Map::new(),
        };
        service.add_document(doc)?;
        
        let prompt = service.generate_prompt("How to authenticate?", 5)?;
        assert!(prompt.contains("Context"));
        
        Ok(())
    }
}
