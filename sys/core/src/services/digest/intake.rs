//! Intake Service (Stage 1)
//!
//! T153: Implement IntakeService
//! §3.4: Digest Everything Pipeline - Stage 1: Discover
//! US4: Digest Everything Pipeline

use crate::db::repositories::DigestRepository;
use crate::db::Connection;
use crate::error::{NoaError, Result, ValidationError};
use uuid::Uuid;

/// GitHub domain patterns for URI detection
const GITHUB_HTTPS_PREFIX: &str = "https://github.com/";
const GITHUB_HTTP_PREFIX: &str = "http://github.com/";
const HTTPS_PREFIX: &str = "https://";
const HTTP_PREFIX: &str = "http://";

/// Intake service for discovering and registering digest sources
pub struct IntakeService {
    digest_repo: DigestRepository,
}

impl IntakeService {
    /// Create a new intake service
    pub fn new(conn: Connection) -> Self {
        Self {
            digest_repo: DigestRepository::new(conn),
        }
    }

    /// Discover and register a new digest source
    ///
    /// This is Stage 1 of the digest pipeline - discovering sources
    /// to be digested (repositories, files, APIs, documents)
    ///
    /// # Arguments
    /// * `uri` - URI or path to the source (repository URL, file path, API endpoint, document URI)
    /// * `source_type` - Type of source (Repository, File, Api, Document)
    ///
    /// # Returns
    /// Returns the UUID of the created or existing digest source
    ///
    /// # Errors
    /// Returns an error if:
    /// - URI is empty or invalid
    /// - Database operation fails
    /// - Source registration fails
    pub async fn discover_source(
        &self,
        uri: &str,
        source_type: crate::db::repositories::DigestSourceType,
    ) -> Result<Uuid> {
        // Validate URI is not empty
        if uri.trim().is_empty() {
            return Err(NoaError::Validation(ValidationError::new(
                "uri",
                "URI cannot be empty. Provide a valid repository URL, file path, API endpoint, or document URI.",
                "EMPTY_URI",
            )));
        }

        // Check if source already exists
        if let Some(existing) = self.digest_repo.find_by_uri(uri).map_err(|e| {
            NoaError::Validation(ValidationError::new(
                "database",
                format!(
                    "Failed to check for existing source: {}. Ensure database is accessible and operational.",
                    e
                ),
                "DB_QUERY_FAILED",
            ))
        })? {
            return Ok(existing.id);
        }

        // Extract name from URI
        let name = Self::extract_name_from_uri(uri);

        // Create new digest source
        let source = crate::db::repositories::DigestSource {
            id: Uuid::new_v4(),
            source_type,
            uri: uri.to_string(),
            name,
            status: crate::db::repositories::DigestStatus::Pending,
            last_digest: None,
            version: None,
            profile: None,
            sbom: None,
            security_report: None,
            stats: None,
        };

        self.digest_repo.create(&source).map_err(|e| {
            NoaError::Validation(ValidationError::new(
                "database",
                format!(
                    "Failed to create digest source: {}. Check database permissions and schema.",
                    e
                ),
                "DB_CREATE_FAILED",
            ))
        })?;
        Ok(source.id)
    }

    /// Extract a human-readable name from a URI
    ///
    /// # Arguments
    /// * `uri` - URI or path string
    ///
    /// # Returns
    /// A human-readable name extracted from the URI
    fn extract_name_from_uri(uri: &str) -> String {
        // Try to extract name from common URI patterns
        if uri.starts_with(GITHUB_HTTPS_PREFIX) || uri.starts_with(GITHUB_HTTP_PREFIX) {
            // Extract repo name from GitHub URL
            uri.split('/').last().unwrap_or(uri).trim_end_matches(".git").to_string()
        } else if uri.starts_with(HTTPS_PREFIX) || uri.starts_with(HTTP_PREFIX) {
            // Extract domain or path component
            uri.split('/')
                .nth(2)
                .or_else(|| uri.split('/').last())
                .unwrap_or(uri)
                .to_string()
        } else {
            // For file paths, use the file/directory name
            std::path::Path::new(uri)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(uri)
                .to_string()
        }
    }
}
