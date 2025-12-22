//! Database Migration Runner
//!
//! Applies and tracks database migrations.
//! §3.2: Database schema management

use std::path::{Path, PathBuf};
use std::fs;

use crate::error::{DatabaseError, Result};
use super::Connection;

/// Database migration
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: String,
    pub description: String,
    pub sql: String,
}

impl Migration {
    pub fn new(version: impl Into<String>, description: impl Into<String>, sql: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            description: description.into(),
            sql: sql.into(),
        }
    }
}

/// Migration runner for applying database migrations
pub struct MigrationRunner {
    migrations_dir: PathBuf,
}

impl MigrationRunner {
    /// Create a new migration runner
    pub fn new(migrations_dir: &Path) -> Self {
        Self {
            migrations_dir: migrations_dir.to_path_buf(),
        }
    }

    /// Get all available migrations from the migrations directory
    pub fn get_migrations(&self) -> Result<Vec<Migration>> {
        let mut migrations = Vec::new();

        if !self.migrations_dir.exists() {
            return Ok(migrations);
        }

        let mut entries: Vec<_> = fs::read_dir(&self.migrations_dir)
            .map_err(|e| DatabaseError::MigrationFailed {
                version: "discovery".to_string(),
                error: e.to_string(),
            })?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "sql")
                    .unwrap_or(false)
            })
            .collect();

        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

            // Parse version from filename (e.g., "001_initial" -> "001_initial")
            let version = filename.to_string();
            let description = version.split('_').skip(1).collect::<Vec<_>>().join(" ");

            let sql = fs::read_to_string(&path).map_err(|e| DatabaseError::MigrationFailed {
                version: version.clone(),
                error: e.to_string(),
            })?;

            migrations.push(Migration {
                version,
                description: if description.is_empty() {
                    "Migration".to_string()
                } else {
                    description
                },
                sql,
            });
        }

        Ok(migrations)
    }

    /// Get applied migrations from the database
    pub fn get_applied_migrations(&self, conn: &Connection) -> Result<Vec<String>> {
        // Ensure migrations table exists
        self.ensure_migrations_table(conn)?;

        let conn = conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT version FROM schema_migrations ORDER BY version")
            .map_err(|e| DatabaseError::QueryFailed {
                query: "SELECT versions".to_string(),
                error: e.to_string(),
            })?;

        let versions: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| DatabaseError::QueryFailed {
                query: "SELECT versions".to_string(),
                error: e.to_string(),
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(versions)
    }

    /// Apply pending migrations
    pub fn apply_pending(&self, conn: &Connection) -> Result<Vec<String>> {
        let all_migrations = self.get_migrations()?;
        let applied = self.get_applied_migrations(conn)?;
        let mut newly_applied = Vec::new();

        for migration in all_migrations {
            if !applied.contains(&migration.version) {
                self.apply_migration(conn, &migration)?;
                newly_applied.push(migration.version.clone());
                tracing::info!(
                    version = %migration.version,
                    description = %migration.description,
                    "Applied migration"
                );
            }
        }

        Ok(newly_applied)
    }

    /// Apply a single migration
    fn apply_migration(&self, conn: &Connection, migration: &Migration) -> Result<()> {
        let conn = conn.lock().unwrap();
        conn.execute_batch(&format!(
            "BEGIN TRANSACTION;
            {};
            INSERT OR REPLACE INTO schema_migrations (version, description, applied_at)
            VALUES ('{}', '{}', datetime('now'));
            COMMIT;",
            migration.sql,
            migration.version,
            migration.description.replace('\'', "''")
        ))
        .map_err(|e| {
            // Attempt rollback on failure
            let _ = conn.execute_batch("ROLLBACK;");
            DatabaseError::MigrationFailed {
                version: migration.version.clone(),
                error: e.to_string(),
            }
        })?;

        Ok(())
    }

    /// Ensure the schema_migrations table exists
    fn ensure_migrations_table(&self, conn: &Connection) -> Result<()> {
        conn.lock()
            .unwrap()
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                description TEXT,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#,
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "CREATE schema_migrations".to_string(),
                error: e.to_string(),
            })?;

        Ok(())
    }

    /// Get migration status
    pub fn status(&self, conn: &Connection) -> Result<MigrationStatus> {
        let all = self.get_migrations()?;
        let applied = self.get_applied_migrations(conn)?;

        let pending: Vec<_> = all
            .iter()
            .filter(|m| !applied.contains(&m.version))
            .map(|m| m.version.clone())
            .collect();

        Ok(MigrationStatus {
            total: all.len(),
            applied: applied.len(),
            pending: pending.len(),
            pending_versions: pending,
        })
    }

    /// Rollback the last migration (if supported)
    pub fn rollback_last(&self, conn: &Connection) -> Result<Option<String>> {
        let applied = self.get_applied_migrations(conn)?;

        if let Some(last_version) = applied.last() {
            conn.lock()
                .unwrap()
                .execute("DELETE FROM schema_migrations WHERE version = ?", [last_version])
                .map_err(|e| DatabaseError::MigrationFailed {
                    version: last_version.clone(),
                    error: format!("Rollback failed: {}", e),
                })?;

            tracing::warn!(version = %last_version, "Rolled back migration (metadata only)");
            return Ok(Some(last_version.clone()));
        }

        Ok(None)
    }
}

/// Migration status information
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    pub total: usize,
    pub applied: usize,
    pub pending: usize,
    pub pending_versions: Vec<String>,
}

impl MigrationStatus {
    pub fn is_up_to_date(&self) -> bool {
        self.pending == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_migration_runner() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let migrations_dir = dir.path().join("migrations");
        fs::create_dir_all(&migrations_dir).unwrap();

        // Create a test migration
        fs::write(
            migrations_dir.join("001_test.sql"),
            "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);",
        )
        .unwrap();

        let conn = crate::db::init_database(&db_path).unwrap();
        let runner = MigrationRunner::new(&migrations_dir);

        // Get migrations
        let migrations = runner.get_migrations().unwrap();
        assert_eq!(migrations.len(), 1);

        // Apply migrations
        let applied = runner.apply_pending(&conn).unwrap();
        assert_eq!(applied.len(), 1);

        // Check status
        let status = runner.status(&conn).unwrap();
        assert!(status.is_up_to_date());

        // Verify table was created
        let count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM sqlite_master WHERE name = 'test'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}

