use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{NoaError, Result, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleRecord {
    pub id: String,
    pub path: String,
    pub size_bytes: u64,
    pub checksum: Option<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// KIDX_CAP: CAS index + blob metadata.
pub struct CapsuleIndex {
    records: HashMap<String, CapsuleRecord>,
}

impl CapsuleIndex {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        size_bytes: u64,
        checksum: Option<String>,
        metadata: serde_json::Value,
    ) -> Result<CapsuleRecord> {
        let id = id.into();
        let path = path.into();
        if self.records.contains_key(&id) {
            return Err(NoaError::Validation(ValidationError::new(
                "capsule_id",
                format!("Capsule {} already registered", id),
                "DUPLICATE_CAPSULE",
            )));
        }

        let record = CapsuleRecord {
            id: id.clone(),
            path,
            size_bytes,
            checksum,
            created_at: Utc::now(),
            metadata,
        };
        self.records.insert(id, record.clone());
        Ok(record)
    }

    pub fn get(&self, id: &str) -> Option<&CapsuleRecord> {
        self.records.get(id)
    }

    pub fn list(&self) -> Vec<CapsuleRecord> {
        self.records.values().cloned().collect()
    }

    pub fn update_checksum(&mut self, id: &str, checksum: String) -> Result<()> {
        let record = self.records.get_mut(id).ok_or_else(|| NoaError::NotFound {
            resource: "capsule".to_string(),
            id: id.to_string(),
        })?;
        record.checksum = Some(checksum);
        Ok(())
    }
}

impl Default for CapsuleIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_and_retrieves_capsule() {
        let mut index = CapsuleIndex::new();
        let record = index
            .register(
                "cap-1",
                "/tmp/capsule",
                1024,
                Some("abc123".into()),
                serde_json::json!({"kind": "test"}),
            )
            .unwrap();
        assert_eq!(record.id, "cap-1");
        assert!(index.get("cap-1").is_some());
    }

    #[test]
    fn prevents_duplicate_registration() {
        let mut index = CapsuleIndex::new();
        index.register("cap-1", "/tmp/c1", 1, None, serde_json::json!({})).unwrap();
        let duplicate = index.register("cap-1", "/tmp/c2", 2, None, serde_json::json!({}));
        assert!(duplicate.is_err());
    }

    #[test]
    fn updates_checksum() {
        let mut index = CapsuleIndex::new();
        index.register("cap-1", "/tmp/c1", 1, None, serde_json::json!({})).unwrap();
        index.update_checksum("cap-1", "newsum".into()).expect("should update checksum");
        let updated = index.get("cap-1").unwrap();
        assert_eq!(updated.checksum.as_deref(), Some("newsum"));
    }
}
