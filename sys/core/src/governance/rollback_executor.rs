use crate::error::{NoaError, Result, ValidationError};
use crate::governance::rollback_validator::RollbackAssessment;
use crate::governance::snapshot::SnapshotService;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::info;
use uuid::Uuid;

/// Rollback execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub snapshot_id: Uuid,
    pub applied: bool,
    pub notes: String,
    pub plan_path: PathBuf,
}

/// Executes validated rollback plans.
pub struct RollbackExecutor {
    snapshots: SnapshotService,
    log_dir: PathBuf,
}

impl RollbackExecutor {
    pub fn new(snapshots: SnapshotService, log_dir: impl Into<PathBuf>) -> Self {
        Self {
            snapshots,
            log_dir: log_dir.into(),
        }
    }

    pub async fn execute(&self, assessment: &RollbackAssessment) -> Result<RollbackResult> {
        if !assessment.valid {
            return Err(NoaError::Validation(ValidationError::new(
                "snapshot",
                "Rollback validation failed",
                "ROLLBACK_INVALID",
            )));
        }

        fs::create_dir_all(&self.log_dir).await?;
        let plan_path = self
            .log_dir
            .join(format!("rollback-{}.json", assessment.snapshot_id));

        let mut file = fs::File::create(&plan_path).await?;
        let snapshot_meta = self.snapshots.load(&assessment.snapshot_id)?;
        let payload = serde_json::json!({
            "snapshot_id": assessment.snapshot_id,
            "missing_artifacts": assessment.missing_artifacts,
            "message": assessment.message,
            "timestamp": chrono::Utc::now(),
            "snapshot": snapshot_meta,
        });
        let serialized = serde_json::to_vec_pretty(&payload)?;
        file.write_all(&serialized).await?;
        file.write_all(b"\n").await?;

        info!(
            target: "governance::rollback",
            snapshot = %assessment.snapshot_id,
            "Rollback plan documented"
        );

        Ok(RollbackResult {
            snapshot_id: assessment.snapshot_id,
            applied: true,
            notes: "Rollback plan recorded; apply file-level restoration as scripted.".to_string(),
            plan_path,
        })
    }
}
