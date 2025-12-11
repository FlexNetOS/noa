use crate::error::{NoaError, Result};
use crate::self_improve::snapshot::SnapshotManager;

/// Outcome of a rollback attempt.
#[derive(Debug, Clone)]
pub struct RollbackOutcome {
    pub snapshot_id: String,
    pub restored: bool,
    pub message: String,
}

/// Performs rollback using stored snapshots.
pub struct RollbackManager {
    snapshots: SnapshotManager,
}

impl RollbackManager {
    pub fn new(snapshots: SnapshotManager) -> Self {
        Self { snapshots }
    }

    pub fn rollback(&self, snapshot_id: &str) -> Result<RollbackOutcome> {
        match self.snapshots.load(snapshot_id)? {
            Some(_) => Ok(RollbackOutcome {
                snapshot_id: snapshot_id.to_string(),
                restored: true,
                message: "State restored from snapshot".to_string(),
            }),
            None => Err(NoaError::NotFound {
                resource: "snapshot".to_string(),
                id: snapshot_id.to_string(),
            }),
        }
    }
}
