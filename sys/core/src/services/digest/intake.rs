//! Intake Service (Stage 1)
//!
//! T153: Implement IntakeService
//! §3.4: Digest Everything Pipeline - Stage 1: Discover
//! US4: Digest Everything Pipeline

use crate::db::Connection;
use crate::db::repositories::DigestRepository;
use crate::error::Result;
use uuid::Uuid;

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
    pub async fn discover_source(
        &self,
        uri: &str,
        source_type: crate::db::repositories::DigestSourceType,
    ) -> Result<Uuid> {
        // Check if source already exists
        if let Some(existing) = self.digest_repo.find_by_uri(uri)? {
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

        self.digest_repo.create(&source)?;
        Ok(source.id)
    }

    /// Extract a human-readable name from a URI
    fn extract_name_from_uri(uri: &str) -> String {
        // Try to extract name from common URI patterns
        if uri.starts_with("https://github.com/") || uri.starts_with("http://github.com/") {
            // Extract repo name from GitHub URL
            uri.split('/')
                .last()
                .unwrap_or(uri)
                .trim_end_matches(".git")
                .to_string()
        } else if uri.starts_with("https://") || uri.starts_with("http://") {
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


