use std::path::Path;

use crate::error::{NoaError, Result, ValidationError};
use crate::kernel::nkal::KernelMode;

use super::checkpoint::KernelCheckpoint;

/// Snapshot of the current/expected kernel state.
#[derive(Debug, Clone, Default)]
pub struct ModeState {
    pub expected_mode: Option<KernelMode>,
    pub checkpoint: Option<KernelCheckpoint>,
}

/// Verifies state after a kernel mode switch.
pub struct StateVerifier;

impl StateVerifier {
    /// Load a checkpoint from disk for validation.
    pub fn load_checkpoint(path: impl AsRef<Path>) -> Result<KernelCheckpoint> {
        let data = std::fs::read_to_string(path)?;
        let checkpoint: KernelCheckpoint = serde_json::from_str(&data)?;
        Ok(checkpoint)
    }

    /// Ensure the last checkpoint matches the expected mode.
    pub fn verify_mode(expected: KernelMode, checkpoint: &KernelCheckpoint) -> Result<()> {
        let target = checkpoint.target_mode.parse::<KernelMode>().map_err(|_| {
            NoaError::Validation(ValidationError::new(
                "target_mode",
                format!("Invalid mode '{}' in checkpoint", checkpoint.target_mode),
                "NKAL_STATE_INVALID_MODE",
            ))
        })?;

        if target != expected {
            return Err(NoaError::Validation(ValidationError::new(
                "target_mode",
                format!(
                    "Checkpoint target '{}' does not match expected '{}'",
                    checkpoint.target_mode, expected
                ),
                "NKAL_STATE_MISMATCH",
            )));
        }

        Ok(())
    }

    /// Verify that required mounts were included in the checkpoint.
    pub fn verify_mounts(checkpoint: &KernelCheckpoint, required: &[&str]) -> Result<()> {
        for mount_name in required {
            let found = checkpoint.mounts.iter().any(|m| m.name == *mount_name);
            if !found {
                return Err(NoaError::Validation(ValidationError::new(
                    "mounts",
                    format!("Missing required mount '{}' in checkpoint", mount_name),
                    "NKAL_MOUNT_MISSING",
                )));
            }
        }
        Ok(())
    }

    /// Ensure the checkpoint is marked as completed (status != pending).
    pub fn verify_status(checkpoint: &KernelCheckpoint) -> Result<()> {
        if checkpoint.status.to_lowercase().starts_with("pending") {
            return Err(NoaError::Validation(ValidationError::new(
                "status",
                "Kernel switch checkpoint still pending",
                "NKAL_STATE_PENDING",
            )));
        }
        Ok(())
    }
}
