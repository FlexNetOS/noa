use crate::error::Result;
use crate::governance::snapshot::{SnapshotRecord, SnapshotService};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Validation result for a rollback path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackAssessment {
    pub snapshot_id: Uuid,
    pub valid: bool,
    pub missing_artifacts: Vec<PathBuf>,
    pub message: String,
    pub snapshot: Option<SnapshotRecord>,
}

/// Validates rollback feasibility before execution.
pub struct RollbackValidator {
    snapshots: SnapshotService,
}

impl RollbackValidator {
    pub fn new(snapshots: SnapshotService) -> Self {
        Self { snapshots }
    }

    pub fn validate(&self, snapshot_id: &Uuid) -> Result<RollbackAssessment> {
        let snapshot = self.snapshots.load(snapshot_id)?;
        if snapshot.is_none() {
            return Ok(RollbackAssessment {
                snapshot_id: *snapshot_id,
                valid: false,
                missing_artifacts: Vec::new(),
                message: "Snapshot not found".to_string(),
                snapshot: None,
            });
        }

        let snapshot = snapshot.unwrap();
        let mut missing = Vec::new();

        for artifact in &snapshot.artifacts {
            if !artifact.path.exists() {
                missing.push(artifact.path.clone());
            }
        }

        let valid = missing.is_empty();
        let message = if valid {
            "Rollback path validated".to_string()
        } else {
            format!("Missing {} artifact(s)", missing.len())
        };

        Ok(RollbackAssessment {
            snapshot_id: *snapshot_id,
            valid,
            missing_artifacts: missing,
            message,
            snapshot: Some(snapshot),
        })
    }

    /// Validate a snapshot record directly (bypasses lookup).
    pub fn validate_record(&self, snapshot: &SnapshotRecord) -> Result<RollbackAssessment> {
        let mut missing = Vec::new();
        for artifact in &snapshot.artifacts {
            if !artifact.path.exists() {
                missing.push(artifact.path.clone());
            } else if let Some(expected) = &artifact.hash {
                let actual = super::snapshot::hash_file(&artifact.path)?;
                if &actual != expected {
                    missing.push(artifact.path.clone());
                }
            }
        }

        Ok(RollbackAssessment {
            snapshot_id: snapshot.id,
            valid: missing.is_empty(),
            missing_artifacts: missing,
            message: "Inline validation".to_string(),
            snapshot: Some(snapshot.clone()),
        })
    }
}
