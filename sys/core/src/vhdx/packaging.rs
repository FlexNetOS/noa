use chrono::{DateTime, Utc};
use std::path::PathBuf;

use crate::error::Result;

/// Metadata describing a VHDX package.
#[derive(Debug, Clone)]
pub struct VhdxPackage {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub artifacts: Vec<PathBuf>,
    pub size_bytes: u64,
}

/// Packages artifacts into a VHDX stack descriptor (simulation).
pub struct VhdxPackager;

impl VhdxPackager {
    pub fn new() -> Self {
        Self
    }

    pub fn package(&self, artifacts: Vec<PathBuf>) -> Result<VhdxPackage> {
        let total_size = artifacts
            .iter()
            .filter_map(|p| std::fs::metadata(p).ok())
            .map(|m| m.len())
            .sum();

        Ok(VhdxPackage {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            artifacts,
            size_bytes: total_size,
        })
    }
}

impl Default for VhdxPackager {
    fn default() -> Self {
        Self::new()
    }
}
