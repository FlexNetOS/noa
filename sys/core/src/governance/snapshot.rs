use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Snapshot artifact metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotArtifact {
    pub path: PathBuf,
    pub hash: Option<String>,
}

/// Snapshot record persisted to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotRecord {
    pub id: Uuid,
    pub description: String,
    pub actor: String,
    pub created_at: DateTime<Utc>,
    pub checksum: String,
    pub artifacts: Vec<SnapshotArtifact>,
}

/// Snapshot service for self-modification safeguards.
pub struct SnapshotService {
    root: PathBuf,
}

impl SnapshotService {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }

    pub fn ensure_root(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    /// Create a snapshot manifest from payload metadata and tracked files.
    pub fn create_snapshot(
        &self,
        description: &str,
        actor: &str,
        payload: &serde_json::Value,
        tracked_files: &[PathBuf],
    ) -> Result<SnapshotRecord> {
        self.ensure_root()?;

        let serialized = serde_json::to_vec(payload)?;
        let checksum = format!("{:x}", Sha256::digest(&serialized));
        let mut artifacts = Vec::new();

        for path in tracked_files {
            let hash = if path.exists() && path.is_file() {
                Some(hash_file(path)?)
            } else {
                None
            };
            artifacts.push(SnapshotArtifact {
                path: path.clone(),
                hash,
            });
        }

        let record = SnapshotRecord {
            id: Uuid::new_v4(),
            description: description.to_string(),
            actor: actor.to_string(),
            created_at: Utc::now(),
            checksum,
            artifacts,
        };

        let path = self.root.join(format!("{}.json", record.id));
        let mut file = fs::File::create(path)?;
        serde_json::to_writer_pretty(&mut file, &record)?;
        file.write_all(b"\n")?;

        Ok(record)
    }

    /// Load a snapshot by id.
    pub fn load(&self, id: &Uuid) -> Result<Option<SnapshotRecord>> {
        let path = self.root.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }

        let file = fs::File::open(path)?;
        let record: SnapshotRecord = serde_json::from_reader(file)?;
        Ok(Some(record))
    }

    /// List recent snapshots (best-effort).
    pub fn list(&self, limit: usize) -> Result<Vec<SnapshotRecord>> {
        self.ensure_root()?;
        let mut records = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            if let Ok(file) = fs::File::open(entry.path()) {
                if let Ok(record) = serde_json::from_reader::<_, SnapshotRecord>(file) {
                    records.push(record);
                }
            }
        }

        records.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        records.truncate(limit);
        Ok(records)
    }
}

pub(crate) fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}
