//! Memory CLI Commands
//!
//! T145-T148: Implement memory CLI commands
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use crate::cli::CliContext;
use crate::db::vector_search::VectorSearch;
use crate::error::Result;
use crate::services::{MemoryService, SearchService};
use crate::db::repositories::MemoryRepository;
use clap::{Args, Subcommand};
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
pub async fn execute(ctx: &CliContext, args: MemoryArgs) -> Result<()> {
    let conn = ctx.db.get()?;

    let memory_repo = MemoryRepository::new(&conn);
    let vector_search = VectorSearch::new(&conn)?;
    let memory_service = MemoryService::new(&conn);
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
                    eprintln!(
                        "Invalid memory type: {}. Must be: interaction, decision, learning, or artifact",
                        r#type
                    );
                    return Ok(());
                }
            };

            let agent_id = agent
                .map(|s| Uuid::parse_str(&s))
                .transpose()
                .map_err(|e| {
                    crate::error::NoaError::Validation(crate::error::ValidationError::new(
                        "agent",
                        format!("Invalid UUID: {}", e),
                        "INVALID_UUID",
                    ))
                })?;

            let parent_id = parent
                .map(|s| Uuid::parse_str(&s))
                .transpose()
                .map_err(|e| {
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

            println!("✓ Created memory: {}", id);
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
                other => {
                    eprintln!("Invalid search_type: {}. Must be: semantic, keyword, hybrid", other);
                    return Ok(());
                }
            };

            for result in results {
                println!(
                    "{} ({:.3}) {}",
                    result.memory.id,
                    result.score,
                    result.memory.content
                );
            }
        }
        MemoryCommand::List { offset, limit } => {
            let memories = memory_service.list(offset, limit)?;
            for memory in memories {
                println!("{} {}", memory.id, memory.content);
            }
        }
        MemoryCommand::Get { id } => {
            let id = Uuid::parse_str(&id).map_err(|e| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "id",
                    format!("Invalid UUID: {}", e),
                    "INVALID_UUID",
                ))
            })?;

            if let Some(memory) = memory_service.get(&id)? {
                println!("{}\n{}", memory.id, memory.content);
            } else {
                println!("Memory not found");
            }
        }
    }

    Ok(())
}

