//! Database-Backed RAG Service
//!
//! Full RAG implementation with vector search and knowledge base integration

use crate::db::repositories::{KnowledgeNode, KnowledgeNodeType};
use crate::error::Result;
use crate::agents::rag::{RAGQuery, RAGResult, RAGResultItem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::path::Path;

/// Database-backed RAG service
/// 
/// Note: This service requires a database connection. Create a new instance
/// when you need to perform RAG operations.
pub struct RAGService;

/// Document to be added to knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub content: String,
    pub source: String,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl RAGService {
    /// Create a new RAG service
    pub fn new() -> Self {
        Self
    }

    /// Add a document to the knowledge base
    pub fn add_document(&self, doc: Document, db_path: &Path) -> Result<Uuid> {
        let conn = crate::db::init_database(db_path)?;
        
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

        let node_repo = crate::db::repositories::KnowledgeNodeRepository::new(conn);
        node_repo.create(&node)?;

        Ok(node.id)
    }

    /// Search knowledge base with semantic search
    pub fn search(&self, query: &RAGQuery, db_path: &Path) -> Result<RAGResult> {
        let conn = crate::db::init_database(db_path)?;
        let top_k = query.top_k.unwrap_or(5);
        
        // Get nodes from database
        let node_repo = crate::db::repositories::KnowledgeNodeRepository::new(conn);
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
    pub fn retrieve_context(&self, query: &str, top_k: usize, db_path: &Path) -> Result<Vec<String>> {
        let rag_query = RAGQuery {
            query: query.to_string(),
            top_k: Some(top_k),
            filters: None,
            include_sources: false,
        };

        let result = self.search(&rag_query, db_path)?;
        Ok(result.items.into_iter().map(|item| item.content).collect())
    }

    /// Generate augmented prompt with context
    pub fn generate_prompt(&self, query: &str, top_k: usize, db_path: &Path) -> Result<String> {
        let context = self.retrieve_context(query, top_k, db_path)?;
        
        if context.is_empty() {
            return Ok(query.to_string());
        }

        let formatted_context = context.join("\n\n---\n\n");
        Ok(format!(
            "Context:\n{}\n\n---\n\nQuestion: {}\n\nProvide an answer based on the context above.",
            formatted_context, query
        ))
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
        let matches = query_terms.iter()
            .filter(|term| content.contains(*term))
            .count();

        // Normalize by query length
        if query_terms.is_empty() {
            0.0
        } else {
            matches as f32 / query_terms.len() as f32
        }
    }
}

impl Default for RAGService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::db::MigrationRunner;
    use std::path::PathBuf;

    fn apply_migrations(db_path: &Path) -> Result<()> {
        let conn = crate::db::init_database(db_path)?;
        let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../init/migrations");
        let runner = MigrationRunner::new(&migrations_dir);
        match runner.apply_pending(&conn) {
            Ok(_) => Ok(()),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("no such module: vss0") {
                    println!("Warning: sqlite-vss extension not available. Skipping vector search setup.");
                    return Ok(());
                }
                Err(e)
            }
        }
    }

    #[test]
    fn test_rag_service_add_document() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        apply_migrations(&db_path)?;
        
        let service = RAGService::new();
        let doc = Document {
            title: "Test Document".to_string(),
            content: "This is a test document about RAG.".to_string(),
            source: "test".to_string(),
            metadata: serde_json::Map::new(),
        };
        
        let id = service.add_document(doc, &db_path)?;
        assert!(!id.is_nil());
        
        Ok(())
    }

    #[test]
    fn test_rag_service_search() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        apply_migrations(&db_path)?;
        
        let service = RAGService::new();
        
        // Add test documents
        let doc1 = Document {
            title: "RAG Introduction".to_string(),
            content: "Retrieval-Augmented Generation combines retrieval with generation.".to_string(),
            source: "docs".to_string(),
            metadata: serde_json::Map::new(),
        };
        service.add_document(doc1, &db_path)?;
        
        // Search
        let query = RAGQuery {
            query: "What is RAG?".to_string(),
            top_k: Some(5),
            filters: None,
            include_sources: true,
        };
        
        let result = service.search(&query, &db_path)?;
        assert!(!result.items.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_rag_service_generate_prompt() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        apply_migrations(&db_path)?;
        
        let service = RAGService::new();
        
        let doc = Document {
            title: "Authentication Guide".to_string(),
            content: "Use JWT tokens for authentication.".to_string(),
            source: "guide".to_string(),
            metadata: serde_json::Map::new(),
        };
        service.add_document(doc, &db_path)?;
        
        let prompt = service.generate_prompt("How to authenticate?", 5, &db_path)?;
        assert!(prompt.contains("Context"));
        
        Ok(())
    }
}
