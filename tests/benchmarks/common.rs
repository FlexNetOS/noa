use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BenchmarkStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub id: String,
    pub description: String,
    pub duration_ms: u128,
    pub target_ms: u128,
    pub status: BenchmarkStatus,
    pub notes: Option<String>,
}

impl BenchmarkResult {
    pub fn new(id: &str, description: &str, duration: Duration, target_ms: u128) -> Self {
        let passed = duration.as_millis() <= target_ms;
        Self {
            id: id.to_string(),
            description: description.to_string(),
            duration_ms: duration.as_millis(),
            target_ms,
            status: if passed {
                BenchmarkStatus::Passed
            } else {
                BenchmarkStatus::Failed
            },
            notes: None,
        }
    }

    pub fn skipped(id: &str, description: &str, note: &str, target_ms: u128) -> Self {
        Self {
            id: id.to_string(),
            description: description.to_string(),
            duration_ms: 0,
            target_ms,
            status: BenchmarkStatus::Skipped,
            notes: Some(note.to_string()),
        }
    }

    pub fn with_notes(mut self, note: impl Into<String>) -> Self {
        self.notes = Some(note.into());
        self
    }

    pub fn passed(&self) -> bool {
        self.status == BenchmarkStatus::Passed
    }
}

pub fn record_result(result: &BenchmarkResult) -> PathBuf {
    let dir = sc_results_dir();
    fs::create_dir_all(&dir).expect("failed to create sc-benchmarks directory");

    let file = format!(
        "{}-{}.json",
        result.id,
        result
            .description
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .trim_matches('_')
    );

    let path = dir.join(file);
    let payload =
        serde_json::to_string_pretty(result).expect("failed to serialize benchmark result");
    fs::write(&path, payload).expect("failed to write benchmark result");
    path
}

pub fn repo_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or(manifest)
}

pub fn sc_results_dir() -> PathBuf {
    repo_root().join("test-results").join("sc-benchmarks")
}
