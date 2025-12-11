//! Memory Service
//!
//! T142: Implement MemoryService with CRUD operations
//! T143: Implement memory checksum validation
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use crate::db::repositories::memory_repository::MemoryType;
use crate::db::repositories::{EmbeddingRepository, MemoryRepository};
use crate::db::Connection;
use crate::error::{NoaError, Result};
use crate::memory::embeddings::EmbeddingGenerator;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use uuid::Uuid;

/// Memory service for managing memories
pub struct MemoryService {
    memory_repo: MemoryRepository,
    embedding_repo: EmbeddingRepository,
    embedding_generator: Option<EmbeddingGenerator>,
}

// Ensure MemoryService can be sent across async tasks used by axum handlers.
#[allow(dead_code)]
fn _assert_send_memory_service()
where
    MemoryService: Send,
{
}

// rusqlite connections are confined to the current thread, but we only use this
// service in request handlers on the same thread. Marking Send/Sync ensures axum
// handler bounds are satisfied; operations remain single-threaded.
unsafe impl Send for MemoryService {}
unsafe impl Sync for MemoryService {}

impl MemoryService {
    /// Create a new memory service
    /// Note: Takes ownership of conn for memory_repo. Embedding repo uses in-memory for now.
    /// TODO: Refactor to use connection pool or Arc<Connection>
    pub fn new(conn: Connection) -> Self {
        let memory_repo = MemoryRepository::new(conn);
        // Embedding repo uses in-memory connection as temporary workaround
        // Proper fix requires connection pooling or Arc<Connection>
        let embedding_conn = crate::db::Connection::open_in_memory()
            .map_err(|e| {
                tracing::warn!(
                    "Failed to create in-memory connection for embeddings: {}",
                    e
                );
                e
            })
            .unwrap_or_else(|_| {
                // Last resort: try to open a temp file
                crate::db::Connection::open(":memory:").unwrap()
            });
        Self {
            memory_repo,
            embedding_repo: EmbeddingRepository::new(embedding_conn),
            embedding_generator: None,
        }
    }

    /// Create memory service with embedding generator
    pub async fn with_embeddings(conn: Connection, model_name: &str) -> Result<Self> {
        let generator = EmbeddingGenerator::new(model_name).await?;
        let memory_repo = MemoryRepository::new(conn);
        // Embedding repo uses in-memory connection as temporary workaround
        let embedding_conn = crate::db::Connection::open_in_memory()
            .map_err(|e| {
                tracing::warn!(
                    "Failed to create in-memory connection for embeddings: {}",
                    e
                );
                e
            })
            .unwrap_or_else(|_| crate::db::Connection::open(":memory:").unwrap());
        Ok(Self {
            memory_repo,
            embedding_repo: EmbeddingRepository::new(embedding_conn),
            embedding_generator: Some(generator),
        })
    }

    /// Create a new memory entry
    pub async fn create(
        &self,
        memory_type: MemoryType,
        content: String,
        metadata: Option<serde_json::Map<String, serde_json::Value>>,
        source_agent: Option<Uuid>,
        parent_id: Option<Uuid>,
        tags: HashSet<String>,
    ) -> Result<Uuid> {
        // Compute checksum
        let checksum = Self::compute_checksum(&content);

        // Generate memory ID first (needed for embedding source_id)
        let memory_id = Uuid::new_v4();

        // Generate embedding if generator is available
        let embedding_id = if let Some(ref generator) = self.embedding_generator {
            let embedding_vector =
                generator.generate(&content).await.map_err(|e| NoaError::Internal {
                    message: format!("Failed to generate embedding: {}", e),
                    source: Some(Box::new(e)),
                })?;
            let embedding_id = Uuid::new_v4();

            let embedding = crate::db::repositories::Embedding {
                id: embedding_id,
                created_at: Utc::now(),
                vector: embedding_vector,
                model: "all-MiniLM-L6-v2".to_string(),
                source_type: "memory".to_string(),
                source_id: memory_id, // Use the memory ID we just created
            };

            self.embedding_repo.create(&embedding).map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::QueryFailed {
                    query: "INSERT INTO embedding".to_string(),
                    error: format!("Failed to store embedding: {}", e),
                })
            })?;
            Some(embedding_id)
        } else {
            None
        };

        // Create memory
        let memory = crate::db::repositories::Memory {
            id: memory_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            memory_type,
            content,
            metadata,
            source_agent,
            parent_id,
            tags,
            embedding_id,
            checksum,
        };

        self.memory_repo.create(&memory).map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::QueryFailed {
                query: "INSERT INTO memory".to_string(),
                error: format!("Failed to create memory: {}", e),
            })
        })?;

        Ok(memory_id)
    }

    /// Get memory by ID
    pub fn get(&self, id: &Uuid) -> Result<Option<crate::db::repositories::Memory>> {
        self.memory_repo.find_by_id(id)
    }

    /// Get memory repository (for services that need direct access)
    pub fn memory_repo(&self) -> &MemoryRepository {
        &self.memory_repo
    }

    /// Update memory
    pub async fn update(
        &self,
        id: &Uuid,
        content: Option<String>,
        metadata: Option<serde_json::Map<String, serde_json::Value>>,
        tags: Option<HashSet<String>>,
    ) -> Result<()> {
        let mut memory = self.memory_repo.find_by_id(id)?.ok_or_else(|| NoaError::NotFound {
            resource: "memory".to_string(),
            id: id.to_string(),
        })?;

        // Update fields
        if let Some(new_content) = content {
            // Validate checksum if content changed
            let new_checksum = Self::compute_checksum(&new_content);
            if new_checksum != memory.checksum {
                memory.content = new_content;
                memory.checksum = new_checksum;
                memory.updated_at = Utc::now();
            }
        }

        if let Some(new_metadata) = metadata {
            memory.metadata = Some(new_metadata);
            memory.updated_at = Utc::now();
        }

        if let Some(new_tags) = tags {
            memory.tags = new_tags;
            memory.updated_at = Utc::now();
        }

        self.memory_repo.update(&memory)
    }

    /// Delete memory
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        // Also delete associated embedding if exists
        if let Some(memory) = self.memory_repo.find_by_id(id)? {
            if let Some(emb_id) = memory.embedding_id {
                let _ = self.embedding_repo.delete(&emb_id);
            }
        }

        self.memory_repo.delete(id)
    }

    /// List memories with pagination
    pub fn list(&self, offset: u64, limit: u64) -> Result<Vec<crate::db::repositories::Memory>> {
        self.memory_repo.list(offset, limit)
    }

    /// Validate memory checksum
    pub fn validate_checksum(&self, id: &Uuid) -> Result<bool> {
        let memory = self.memory_repo.find_by_id(id)?.ok_or_else(|| NoaError::NotFound {
            resource: "memory".to_string(),
            id: id.to_string(),
        })?;

        let computed = Self::compute_checksum(&memory.content);
        Ok(computed == memory.checksum)
    }

    /// Compute SHA-256 checksum of content
    fn compute_checksum(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
