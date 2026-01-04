//! Stack repository for storing technology stack and dependency information
//!
//! Tracks project dependencies, tool versions, and configsuration stacks.

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackRecord {
    pub id: i64,
    pub name: String,
    pub stack_type: String,
    pub version: Option<String>,
    pub configs: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct StackRepository<'a> {
    conn: &'a Connection,
}

impl<'a> StackRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Initialize the stacks table
    pub fn init_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS stacks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                stack_type TEXT NOT NULL,
                version TEXT,
                configs TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Create a new stack record
    pub fn create(&self, name: &str, stack_type: &str, version: Option<&str>, configs: Option<&str>) -> Result<i64> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT INTO stacks (name, stack_type, version, configs, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, stack_type, version, configs, now.to_rfc3339(), now.to_rfc3339()],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// List all stack records
    pub fn list(&self) -> Result<Vec<StackRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, stack_type, version, configs, created_at, updated_at
             FROM stacks
             ORDER BY name"
        )?;

        let records = stmt.query_map([], |row| {
            Ok(StackRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                stack_type: row.get(2)?,
                version: row.get(3)?,
                configs: row.get(4)?,
                created_at: row.get::<_, String>(5)?.parse().unwrap_or_else(|_| Utc::now()),
                updated_at: row.get::<_, String>(6)?.parse().unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    /// Get a stack record by name
    pub fn get_by_name(&self, name: &str) -> Result<Option<StackRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, stack_type, version, configs, created_at, updated_at
             FROM stacks
             WHERE name = ?1"
        )?;

        let mut rows = stmt.query(params![name])?;
        if let Some(row) = rows.next()? {
            Ok(Some(StackRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                stack_type: row.get(2)?,
                version: row.get(3)?,
                configs: row.get(4)?,
                created_at: row.get::<_, String>(5)?.parse().unwrap_or_else(|_| Utc::now()),
                updated_at: row.get::<_, String>(6)?.parse().unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a stack record
    pub fn update(&self, id: i64, version: Option<&str>, configs: Option<&str>) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE stacks SET version = ?1, configs = ?2, updated_at = ?3 WHERE id = ?4",
            params![version, configs, now.to_rfc3339(), id],
        )?;
        Ok(())
    }

    /// Delete a stack record
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM stacks WHERE id = ?1", params![id])?;
        Ok(())
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
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_stack_init_and_create() {
        let conn = setup_test_db();
        let repo = StackRepository::new(&conn);
        
        repo.init_table().unwrap();
        
        let id = repo.create("rust", "language", Some("1.75.0"), None).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_stack_list() {
        let conn = setup_test_db();
        let repo = StackRepository::new(&conn);
        
        repo.init_table().unwrap();
        repo.create("rust", "language", Some("1.75.0"), None).unwrap();
        repo.create("python", "language", Some("3.12"), None).unwrap();
        repo.create("nodejs", "runtime", Some("20.0"), None).unwrap();
        
        let stacks = repo.list().unwrap();
        assert_eq!(stacks.len(), 3);
    }

    #[test]
    fn test_stack_get_by_name() {
        let conn = setup_test_db();
        let repo = StackRepository::new(&conn);
        
        repo.init_table().unwrap();
        repo.create("sqlx", "library", Some("0.8.0"), Some(r#"{"async": true}"#)).unwrap();
        
        let found = repo.get_by_name("sqlx").unwrap();
        assert!(found.is_some());
        let stack = found.unwrap();
        assert_eq!(stack.name, "sqlx");
        assert_eq!(stack.version, Some("0.8.0".to_string()));
    }

    #[test]
    fn test_stack_update() {
        let conn = setup_test_db();
        let repo = StackRepository::new(&conn);
        
        repo.init_table().unwrap();
        let id = repo.create("tokio", "runtime", Some("1.0.0"), None).unwrap();
        
        repo.update(id, Some("1.35.0"), Some(r#"{"rt-multi-thread": true}"#)).unwrap();
        
        // Verify update via raw query since get_by_name returns by name
        let stacks = repo.list().unwrap();
        let tokio = stacks.iter().find(|s| s.name == "tokio").unwrap();
        assert_eq!(tokio.version, Some("1.35.0".to_string()));
    }

    #[test]
    fn test_stack_delete() {
        let conn = setup_test_db();
        let repo = StackRepository::new(&conn);
        
        repo.init_table().unwrap();
        let id = repo.create("temp-stack", "test", None, None).unwrap();
        
        let before = repo.list().unwrap();
        assert_eq!(before.len(), 1);
        
        repo.delete(id).unwrap();
        
        let after = repo.list().unwrap();
        assert_eq!(after.len(), 0);
    }
}
