use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::{NoaError, Result};

/// Mount specification persisted with a kernel mode change.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MountSpec {
    pub name: String,
    pub host_path: String,
    pub guest_path: String,
    #[serde(default = "default_mount_mode")]
    pub mode: String,
}

fn default_mount_mode() -> String {
    "ro".to_string()
}

/// Checkpoint written whenever the kernel mode changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelCheckpoint {
    pub timestamp: String,
    pub source_mode: String,
    pub target_mode: String,
    pub reason: String,
    pub capability_policy: Option<String>,
    #[serde(default)]
    pub mounts: Vec<MountSpec>,
    #[serde(default)]
    pub status: String,
}

pub struct CheckpointWriter {
    path: PathBuf,
}

impl CheckpointWriter {
    /// Create a writer that targets the `.kernel-switch-state.json` file.
    pub fn new(root: Option<PathBuf>) -> Self {
        let root = resolve_root(root);
        let path = root.join(".kernel-switch-state.json");
        Self { path }
    }

    /// Write a checkpoint to disk.
    pub fn write(&self, checkpoint: &KernelCheckpoint) -> Result<()> {
        let file = std::fs::File::create(&self.path)?;
        serde_json::to_writer_pretty(file, checkpoint)?;
        Ok(())
    }

    /// Convenience helper to build and write a checkpoint in one call.
    pub fn record_transition(
        &self,
        source_mode: impl Into<String>,
        target_mode: impl Into<String>,
        reason: impl Into<String>,
        capability_policy: Option<String>,
        mounts: Vec<MountSpec>,
        status: impl Into<String>,
    ) -> Result<KernelCheckpoint> {
        let checkpoint = KernelCheckpoint {
            timestamp: Utc::now().to_rfc3339(),
            source_mode: source_mode.into(),
            target_mode: target_mode.into(),
            reason: reason.into(),
            capability_policy,
            mounts,
            status: status.into(),
        };

        self.write(&checkpoint)?;
        Ok(checkpoint)
    }

    /// File location used for the checkpoint.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn resolve_root(root: Option<PathBuf>) -> PathBuf {
    if let Some(root) = root {
        return root;
    }

    if let Ok(env_root) = std::env::var("NOA_ROOT") {
        return PathBuf::from(env_root);
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
