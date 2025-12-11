//! Configuration Lineage and Provenance Tracking
//!
//! Tracks configuration changes, versions, and provenance for audit and rollback.
//! T048: Config lineage/provenance tracking
//! §3.5: Transparent & Auditable

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration lineage entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigLineage {
    /// Configuration version
    pub version: String,

    /// Parent version this derived from
    pub parent_version: Option<String>,

    /// Type of change
    pub change_type: ChangeType,

    /// Reason for the change
    pub change_reason: String,

    /// Source file path if derived
    pub source_file: Option<String>,

    /// SHA-256 content hash
    pub hash: String,

    /// Timestamp of change
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of configuration change
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    /// New configuration created
    Create,

    /// Configuration updated
    Update,

    /// Configuration derived from another
    Derive,

    /// Configuration merged from multiple sources
    Merge,
}

/// Configuration lineage tracker
pub struct LineageTracker {
    lineage_file: std::path::PathBuf,
    history: Vec<ConfigLineage>,
}

impl LineageTracker {
    /// Create a new lineage tracker
    pub fn new(noa_root: &Path) -> Result<Self> {
        let lineage_file = noa_root.join("config").join("lineage.json");
        let history = if lineage_file.exists() {
            let content = std::fs::read_to_string(&lineage_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        Ok(Self {
            lineage_file,
            history,
        })
    }

    /// Record a configuration change
    pub fn record_change(
        &mut self,
        version: String,
        change_type: ChangeType,
        change_reason: String,
        content: &str,
        source_file: Option<String>,
    ) -> Result<()> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        let parent_version = self.history.last().map(|e| e.version.clone());

        let entry = ConfigLineage {
            version,
            parent_version,
            change_type,
            change_reason,
            source_file,
            hash,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        self.history.push(entry);
        self.save()?;

        Ok(())
    }

    /// Get lineage history
    pub fn history(&self) -> &[ConfigLineage] {
        &self.history
    }

    /// Get latest version
    pub fn latest_version(&self) -> Option<&ConfigLineage> {
        self.history.last()
    }

    /// Save lineage to file
    fn save(&self) -> Result<()> {
        if let Some(parent) = self.lineage_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self.history)?;
        std::fs::write(&self.lineage_file, content)?;

        Ok(())
    }
}
