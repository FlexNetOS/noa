//! NOA Digest Commands
//!
//! T185-T188: Implement noa digest commands
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use clap::{Args, Subcommand};
use std::path::PathBuf;
use tracing::info;

use crate::error::{NoaError, Result, ValidationError};
use crate::services::DigestService;

/// Default database path relative to NOA root
const DEFAULT_DB_PATH: &str = "data/noa.db";

/// Default output directory for digest artifacts
const DEFAULT_OUTPUT_DIR: &str = "./digest-output";

/// Default limit for knowledge graph search results
const DEFAULT_KNOWLEDGE_SEARCH_LIMIT: u64 = 10;

/// Arguments for digest commands
#[derive(Args, Debug)]
pub struct DigestArgs {
    #[command(subcommand)]
    pub command: DigestCommands,
}

#[derive(Subcommand, Debug)]
pub enum DigestCommands {
    /// Digest a source (repository, file, API, document)
    Digest {
        /// Source URI or path
        #[arg(value_name = "URI")]
        uri: String,

        /// Source type (repository, file, api, document)
        #[arg(short, long, default_value = "repository")]
        source_type: String,

        /// Output directory
        #[arg(short, long, default_value = DEFAULT_OUTPUT_DIR)]
        output: PathBuf,
    },

    /// Show digest job status
    Status {
        /// Job ID (optional, shows all if not provided)
        #[arg(value_name = "JOB_ID")]
        job_id: Option<String>,
    },

    /// Generate security report
    SecurityReport {
        /// Source ID
        #[arg(value_name = "SOURCE_ID")]
        source_id: String,
    },

    /// Search knowledge graph
    Knowledge {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Limit results
        #[arg(short, long, default_value_t = DEFAULT_KNOWLEDGE_SEARCH_LIMIT)]
        limit: u64,
    },
}

/// Execute digest command
pub async fn execute(args: DigestArgs) -> Result<()> {
    match args.command {
        DigestCommands::Digest {
            uri,
            source_type,
            output,
        } => {
            info!(uri = %uri, "Digesting source");
            execute_digest(uri, source_type, output).await
        }
        DigestCommands::Status { job_id } => {
            info!(?job_id, "Getting digest status");
            execute_status(job_id).await
        }
        DigestCommands::SecurityReport { source_id } => {
            info!(source_id = %source_id, "Generating security report");
            execute_security_report(source_id).await
        }
        DigestCommands::Knowledge { query, limit } => {
            info!(query = %query, limit = limit, "Searching knowledge graph");
            execute_knowledge_search(query, limit).await
        }
    }
}

/// Execute digest command for a source
///
/// # Arguments
/// * `uri` - Source URI or file path to digest
/// * `source_type` - Type of source (repository, file, api, document)
/// * `output` - Output directory for digest artifacts
///
/// # Errors
/// Returns an error if:
/// - URI is invalid or inaccessible
/// - Source type is invalid
/// - Database connection fails
/// - Digest pipeline fails at any stage
async fn execute_digest(uri: String, source_type: String, output: PathBuf) -> Result<()> {
    // Validate URI is not empty
    if uri.trim().is_empty() {
        return Err(NoaError::Validation(ValidationError::new(
            "uri",
            "URI cannot be empty. Provide a valid repository URL, file path, API endpoint, or document URI.",
            "EMPTY_URI",
        )));
    }

    // Validate output directory can be created
    if let Some(parent) = output.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NoaError::Validation(ValidationError::new(
                    "output",
                    format!(
                        "Cannot create output directory: {}. Error: {}. Ensure parent directory exists and is writable.",
                        parent.display(),
                        e
                    ),
                    "OUTPUT_DIR_CREATE_FAILED",
                ))
            })?;
        }
    }

    // Get database path from config or use default
    let db_path = PathBuf::from(DEFAULT_DB_PATH);

    // Validate database path exists or can be created
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NoaError::Validation(ValidationError::new(
                    "db_path",
                    format!(
                        "Cannot create database directory: {}. Error: {}. Ensure parent directory exists and is writable.",
                        parent.display(),
                        e
                    ),
                    "DB_DIR_CREATE_FAILED",
                ))
            })?;
        }
    }

    let digest_service = DigestService::new(&db_path);

    // Convert source_type string to enum with validation
    let source_type_enum = match source_type.as_str() {
        "repository" => crate::db::repositories::DigestSourceType::Repository,
        "file" => crate::db::repositories::DigestSourceType::File,
        "api" => crate::db::repositories::DigestSourceType::Api,
        "document" => crate::db::repositories::DigestSourceType::Document,
        _ => {
            return Err(NoaError::Validation(ValidationError::new(
                "source_type",
                format!(
                    "Invalid source type: '{}'. Valid types are: repository, file, api, document. Use --source-type to specify.",
                    source_type
                ),
                "INVALID_SOURCE_TYPE",
            )));
        }
    };

    println!("Digesting source: {}", uri);
    println!("Output directory: {}", output.display());

    match digest_service.digest_source(&uri, source_type_enum).await {
        Ok(source_id) => {
            println!("✓ Digest completed successfully");
            println!("  Source ID: {}", source_id);
            println!("  Output: {}", output.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Digest failed: {}", e);
            eprintln!(
                "  What: Failed to complete digest pipeline for source '{}'",
                uri
            );
            eprintln!("  Why: {}", e);
            eprintln!("  How to fix: Check that the source is accessible, database is operational, and all required tools are installed.");
            Err(e)
        }
    }
}

/// Execute status command to show digest job status
///
/// # Arguments
/// * `job_id` - Optional job ID. If provided, shows status for that job. If None, lists all jobs.
///
/// # Errors
/// Returns an error if:
/// - Job ID is invalid UUID format
/// - Database connection fails
/// - Job not found (when specific job_id provided)
async fn execute_status(job_id: Option<String>) -> Result<()> {
    // TODO: Implement status command using DigestJobQueue
    if let Some(id) = job_id {
        // Validate UUID format
        uuid::Uuid::parse_str(&id).map_err(|_| {
            NoaError::Validation(ValidationError::new(
                "job_id",
                format!(
                    "Invalid job ID format: '{}'. Expected UUID format (e.g., 550e8400-e29b-41d4-a716-446655440000).",
                    id
                ),
                "INVALID_UUID",
            ))
        })?;
        println!("Digest job status: {}", id);
        // Get job from queue and display status
    } else {
        println!("All digest jobs:");
        // List all jobs
    }
    Ok(())
}

/// Execute security report generation for a source
///
/// # Arguments
/// * `source_id` - UUID of the digest source
///
/// # Errors
/// Returns an error if:
/// - Source ID is invalid UUID format
/// - Source not found
/// - Security scanning tools unavailable
async fn execute_security_report(source_id: String) -> Result<()> {
    // Validate UUID format
    let source_uuid = uuid::Uuid::parse_str(&source_id).map_err(|_| {
        NoaError::Validation(ValidationError::new(
            "source_id",
            format!(
                "Invalid source ID format: '{}'. Expected UUID format (e.g., 550e8400-e29b-41d4-a716-446655440000).",
                source_id
            ),
            "INVALID_UUID",
        ))
    })?;

    // TODO: Implement security report generation
    println!("Generating security report for source: {}", source_uuid);
    Ok(())
}

/// Execute knowledge graph search
///
/// # Arguments
/// * `query` - Search query string
/// * `limit` - Maximum number of results to return
///
/// # Errors
/// Returns an error if:
/// - Query is empty
/// - Database connection fails
/// - Search index unavailable
async fn execute_knowledge_search(query: String, limit: u64) -> Result<()> {
    // Validate query is not empty
    if query.trim().is_empty() {
        return Err(NoaError::Validation(ValidationError::new(
            "query",
            "Search query cannot be empty. Provide a non-empty search string.",
            "EMPTY_QUERY",
        )));
    }

    // Validate limit is reasonable (prevent excessive memory usage)
    const MAX_SEARCH_LIMIT: u64 = 1000;
    if limit > MAX_SEARCH_LIMIT {
        return Err(NoaError::Validation(ValidationError::new(
            "limit",
            format!(
                "Search limit {} exceeds maximum of {}. Reduce limit to prevent excessive memory usage.",
                limit, MAX_SEARCH_LIMIT
            ),
            "LIMIT_TOO_LARGE",
        )));
    }

    // TODO: Implement knowledge graph search
    println!("Searching knowledge graph for: {}", query);
    println!("Limit: {}", limit);
    Ok(())
}
