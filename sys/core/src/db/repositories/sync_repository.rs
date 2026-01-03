//! SyncState Repository
//!
//! T238: §3.8 Implement SyncState repository
//! US6: P2P Hive-Mind Device Federation
//! FR-020: P2P state synchronization

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use uuid::Uuid;

/// SyncState entity representing P2P synchronization state
#[derive(Debug, Clone)]
pub struct SyncState {
    pub id: Uuid,
    pub device_id: Uuid,
    pub entity_type: String,
    pub last_sync: Option<DateTime<Utc>>,
    pub local_version: i64,
    pub remote_version: Option<i64>,
    pub pending_ops: Option<Map<String, Value>>,
    pub conflicts: Option<Map<String, Value>>,
}

/// SyncState repository for CRUD operations
pub struct SyncRepository {
    conn: Connection,
}

impl SyncRepository {
    /// Create a new sync repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new sync state
    pub fn create(&self, sync_state: &SyncState) -> Result<Uuid> {
        let pending_ops_json = sync_state
            .pending_ops
            .as_ref()
            .map(|p| serde_json::to_string(p))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize pending_ops".to_string(),
                    error: e.to_string(),
                })
            })?;

        let conflicts_json = sync_state
            .conflicts
            .as_ref()
            .map(|c| serde_json::to_string(c))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize conflicts".to_string(),
                    error: e.to_string(),
                })
            })?;

        self.conn
            .execute(
                r#"
                INSERT INTO sync_state (
                    id, device_id, entity_type, last_sync,
                    local_version, remote_version, pending_ops, conflicts
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    sync_state.id.to_string(),
                    sync_state.device_id.to_string(),
                    sync_state.entity_type,
                    sync_state.last_sync.map(|d| d.to_rfc3339()),
                    sync_state.local_version,
                    sync_state.remote_version,
                    pending_ops_json,
                    conflicts_json,
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(sync_state.id)
    }

    /// Find sync state by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<SyncState>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, device_id, entity_type, last_sync,
                       local_version, remote_version, pending_ops, conflicts
                FROM sync_state
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_sync_state(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(sync_state)) => Ok(Some(sync_state)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM sync_state".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find sync state by device and entity type
    pub fn find_by_device_and_type(
        &self,
        device_id: &Uuid,
        entity_type: &str,
    ) -> Result<Option<SyncState>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, device_id, entity_type, last_sync,
                       local_version, remote_version, pending_ops, conflicts
                FROM sync_state
                WHERE device_id = ?1 AND entity_type = ?2
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(
                params![device_id.to_string(), entity_type],
                |row| self.row_to_sync_state(row),
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(sync_state)) => Ok(Some(sync_state)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM sync_state".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// List all sync states for a device
    pub fn list_by_device(&self, device_id: &Uuid) -> Result<Vec<SyncState>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, device_id, entity_type, last_sync,
                       local_version, remote_version, pending_ops, conflicts
                FROM sync_state
                WHERE device_id = ?1
                ORDER BY entity_type
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![device_id.to_string()], |row| self.row_to_sync_state(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut sync_states = Vec::new();
        for row in rows {
            sync_states.push(row?);
        }
        Ok(sync_states)
    }

    /// Update sync state
    pub fn update(&self, sync_state: &SyncState) -> Result<()> {
        let pending_ops_json = sync_state
            .pending_ops
            .as_ref()
            .map(|p| serde_json::to_string(p))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize pending_ops".to_string(),
                    error: e.to_string(),
                })
            })?;

        let conflicts_json = sync_state
            .conflicts
            .as_ref()
            .map(|c| serde_json::to_string(c))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize conflicts".to_string(),
                    error: e.to_string(),
                })
            })?;

        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET device_id = ?2, entity_type = ?3, last_sync = ?4,
                    local_version = ?5, remote_version = ?6,
                    pending_ops = ?7, conflicts = ?8
                WHERE id = ?1
                "#,
                params![
                    sync_state.id.to_string(),
                    sync_state.device_id.to_string(),
                    sync_state.entity_type,
                    sync_state.last_sync.map(|d| d.to_rfc3339()),
                    sync_state.local_version,
                    sync_state.remote_version,
                    pending_ops_json,
                    conflicts_json,
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Update versions
    pub fn update_versions(
        &self,
        id: &Uuid,
        local_version: i64,
        remote_version: Option<i64>,
    ) -> Result<()> {
        self.conn
            .execute(
                r#"
                UPDATE sync_state
                SET local_version = ?2, remote_version = ?3, last_sync = ?4
                WHERE id = ?1
                "#,
                params![
                    id.to_string(),
                    local_version,
                    remote_version,
                    Utc::now().to_rfc3339(),
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Delete sync state
    pub fn delete(&self, id: &Uuid) -> Result<()> {
        self.conn
            .execute("DELETE FROM sync_state WHERE id = ?1", params![id.to_string()])
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "DELETE FROM sync_state".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Convert database row to SyncState
    fn row_to_sync_state(&self, row: &Row) -> rusqlite::Result<SyncState> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "UUID".to_string(), rusqlite::types::Type::Text)
        })?;

        let device_id_str: String = row.get(1)?;
        let device_id = Uuid::parse_str(&device_id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "UUID".to_string(), rusqlite::types::Type::Text)
        })?;

        let entity_type: String = row.get(2)?;

        let last_sync_str: Option<String> = row.get(3)?;
        let last_sync = last_sync_str
            .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(3, "timestamp".to_string(), rusqlite::types::Type::Text)
            })?;

        let local_version: i64 = row.get(4)?;
        let remote_version: Option<i64> = row.get(5)?;

        let pending_ops_json: Option<String> = row.get(6)?;
        let pending_ops = pending_ops_json
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(6, "JSON".to_string(), rusqlite::types::Type::Text)
            })?;

        let conflicts_json: Option<String> = row.get(7)?;
        let conflicts = conflicts_json
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(7, "JSON".to_string(), rusqlite::types::Type::Text)
            })?;

        Ok(SyncState {
            id,
            device_id,
            entity_type,
            last_sync,
            local_version,
            remote_version,
            pending_ops,
            conflicts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS sync_state (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                last_sync TEXT,
                local_version INTEGER NOT NULL,
                remote_version INTEGER,
                pending_ops TEXT,
                conflicts TEXT
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_sync_state_create_and_find() {
        let conn = setup_test_db();
        let repo = SyncRepository::new(conn);

        let sync_state = SyncState {
            id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: "memory".to_string(),
            last_sync: Some(Utc::now()),
            local_version: 1,
            remote_version: Some(1),
            pending_ops: None,
            conflicts: None,
        };

        let id = repo.create(&sync_state).unwrap();
        assert_eq!(id, sync_state.id);

        let found = repo.find_by_id(&sync_state.id).unwrap().unwrap();
        assert_eq!(found.entity_type, "memory");
        assert_eq!(found.local_version, 1);
    }

    #[test]
    fn test_sync_state_find_by_device_and_type() {
        let conn = setup_test_db();
        let repo = SyncRepository::new(conn);

        let device_id = Uuid::new_v4();
        
        let sync_state = SyncState {
            id: Uuid::new_v4(),
            device_id,
            entity_type: "task".to_string(),
            last_sync: None,
            local_version: 5,
            remote_version: Some(3),
            pending_ops: None,
            conflicts: None,
        };

        repo.create(&sync_state).unwrap();

        let found = repo.find_by_device_and_type(&device_id, "task").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().local_version, 5);
    }

    #[test]
    fn test_sync_state_find_nonexistent() {
        let conn = setup_test_db();
        let repo = SyncRepository::new(conn);

        let result = repo.find_by_id(&Uuid::new_v4()).unwrap();
        assert!(result.is_none());
    }
}

