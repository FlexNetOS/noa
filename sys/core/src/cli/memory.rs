//! Memory CLI Commands
//!
//! T145-T148: Implement memory CLI commands
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use crate::db::init_database;
use crate::db::repositories::MemoryRepository;
use crate::db::vector_search::VectorSearch;
use crate::error::Result;
use crate::services::{MemoryService, SearchService};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use uuid::Uuid;

/// Memory management commands
#[derive(Debug, Args)]
pub struct MemoryArgs {
    #[command(subcommand)]
    pub command: MemoryCommand,
}

#[derive(Debug, Subcommand)]
pub enum MemoryCommand {
    /// Create a new memory entry
    Create {
        /// Memory type: interaction, decision, learning, or artifact
        #[arg(long)]
        r#type: String,
        /// Memory content
        #[arg(long)]
        content: String,
        /// Source agent ID (optional)
        #[arg(long)]
        agent: Option<String>,
        /// Parent memory ID (optional)
        #[arg(long)]
        parent: Option<String>,
        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
    },
    /// Search memories
    Search {
        /// Search query
        query: String,
        /// Search type: semantic, keyword, or hybrid
        #[arg(long, default_value = "hybrid")]
        search_type: String,
        /// Maximum results
        #[arg(long, default_value = "10")]
        limit: u32,
        /// Minimum similarity score (for semantic search)
        #[arg(long, default_value = "0.7")]
        threshold: f32,
    },
    /// List memories
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u64,
        /// Limit for pagination
        #[arg(long, default_value = "20")]
        limit: u64,
    },
    /// Get memory by ID
    Get {
        /// Memory ID
        id: String,
    },
}

/// Execute memory command
pub async fn execute_memory(args: MemoryArgs, db_path: PathBuf) -> Result<()> {
    let conn1 = init_database(&db_path)?;
    let conn2 = init_database(&db_path)?;
    let conn3 = init_database(&db_path)?;
    let conn4 = init_database(&db_path)?;
    let memory_repo = MemoryRepository::new(conn1);
    let vector_search = VectorSearch::new(conn2)?;

    let memory_service = MemoryService::new(conn3, conn4);
    let search_service = SearchService::new(memory_repo, vector_search);

    match args.command {
        MemoryCommand::Create {
            r#type,
            content,
            agent,
            parent,
            tags,
        } => {
            let memory_type = match r#type.as_str() {
                "interaction" => crate::memory::MemoryType::Interaction,
                "decision" => crate::memory::MemoryType::Decision,
                "learning" => crate::memory::MemoryType::Learning,
                "artifact" => crate::memory::MemoryType::Artifact,
                _ => {
                    eprintln!("Invalid memory type: {}. Must be: interaction, decision, learning, or artifact", r#type);
                    return Ok(());
                }
            };

            let agent_id = agent.map(|s| Uuid::parse_str(&s)).transpose().map_err(|e| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "agent",
                    format!("Invalid UUID: {}", e),
                    "INVALID_UUID",
                ))
            })?;

            let parent_id = parent.map(|s| Uuid::parse_str(&s)).transpose().map_err(|e| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "parent",
                    format!("Invalid UUID: {}", e),
                    "INVALID_UUID",
                ))
            })?;

            let tags_set: std::collections::HashSet<String> = tags
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_default();

            let id = memory_service
                .create(memory_type, content, None, agent_id, parent_id, tags_set)
                .await?;

            println!("Created memory: {}", id);
            Ok(())
        }
        MemoryCommand::Search {
            query,
            search_type,
            limit,
            threshold,
        } => {
            let results = match search_type.as_str() {
                "semantic" => search_service.search_semantic(&query, limit, threshold).await?,
                "keyword" => search_service.search_keyword(&query, limit)?,
                "hybrid" => search_service.search_hybrid(&query, limit, threshold).await?,
                _ => {
                    eprintln!(
                        "Invalid search type: {}. Must be: semantic, keyword, or hybrid",
                        search_type
                    );
                    return Ok(());
                }
            };

            println!("Found {} results:", results.len());
            for (i, result) in results.iter().enumerate() {
                println!(
                    "\n{}. Memory {} (score: {:.3})",
                    i + 1,
                    result.memory.id,
                    result.score
                );
                println!("   Type: {:?}", result.memory.memory_type);
                println!(
                    "   Content: {}",
                    if result.memory.content.len() > 100 {
                        &result.memory.content[..100]
                    } else {
                        &result.memory.content
                    }
                );
            }
            Ok(())
        }
        MemoryCommand::List { offset, limit } => {
            let memories = memory_service.list(offset, limit)?;
            println!("Found {} memories:", memories.len());
            for memory in memories {
                println!("\nMemory {} ({:?})", memory.id, memory.memory_type);
                println!("  Created: {}", memory.created_at);
                println!(
                    "  Content: {}",
                    if memory.content.len() > 80 {
                        &memory.content[..80]
                    } else {
                        &memory.content
                    }
                );
            }
            Ok(())
        }
        MemoryCommand::Get { id } => {
            let memory_id = Uuid::parse_str(&id).map_err(|e| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "id",
                    format!("Invalid UUID: {}", e),
                    "INVALID_UUID",
                ))
            })?;

            match memory_service.get(&memory_id)? {
                Some(memory) => {
                    println!("Memory: {}", memory.id);
                    println!("Type: {:?}", memory.memory_type);
                    println!("Created: {}", memory.created_at);
                    println!("Updated: {}", memory.updated_at);
                    println!("Content: {}", memory.content);
                    if let Some(ref metadata) = memory.metadata {
                        println!(
                            "Metadata: {}",
                            serde_json::to_string_pretty(metadata).map_err(|e| {
                                crate::error::NoaError::Serialization(e.to_string())
                            })?
                        );
                    }
                    if !memory.tags.is_empty() {
                        println!("Tags: {:?}", memory.tags);
                    }
                    if let Some(agent) = memory.source_agent {
                        println!("Source Agent: {}", agent);
                    }
                    if let Some(parent) = memory.parent_id {
                        println!("Parent: {}", parent);
                    }
                    println!("Checksum: {}", memory.checksum);
                    Ok(())
                }
                None => {
                    eprintln!("Memory not found: {}", id);
                    Ok(())
                }
            }
        }
    }
}
