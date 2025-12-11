//! Database Initialization
//!
//! T080: Implement database initialization
//! §3.2: Local-First & Offline-Capable

use crate::error::{NoaError, Result};
use crate::init::paths::NoaPaths;
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

/// Database initializer
pub struct DatabaseInitializer;

impl DatabaseInitializer {
    /// Initialize the NOA database
    pub fn initialize(noa_root: &Path, force: bool) -> Result<()> {
        let db_path = NoaPaths::data(noa_root).join("noa.db");

        if db_path.exists() && !force {
            debug!(path = %db_path.display(), "Database already exists");
            return Ok(());
        }

        info!(path = %db_path.display(), "Initializing database");

        // Ensure data directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create database connection
        let conn = Connection::open(&db_path).map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::ConnectionFailed(e.to_string()))
        })?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", []).map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::QueryFailed {
                query: "PRAGMA foreign_keys".to_string(),
                error: e.to_string(),
            })
        })?;

        // Run migrations if they exist
        let migrations_dir = NoaPaths::init_migrations(noa_root);
        if migrations_dir.exists() {
            Self::run_migrations(&conn, &migrations_dir)?;
        } else {
            warn!("Migrations directory not found, skipping migrations");
        }

        info!(path = %db_path.display(), "Database initialized successfully");
        Ok(())
    }

    /// Run database migrations
    fn run_migrations(conn: &Connection, migrations_dir: &Path) -> Result<()> {
        info!("Running database migrations");

        // Create migrations table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::QueryFailed {
                query: "CREATE TABLE schema_migrations".to_string(),
                error: e.to_string(),
            })
        })?;

        // Get applied migrations
        let mut stmt = conn
            .prepare("SELECT version FROM schema_migrations ORDER BY version")
            .map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::QueryFailed {
                    query: "SELECT version".to_string(),
                    error: e.to_string(),
                })
            })?;

        let applied: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::QueryFailed {
                    query: "SELECT version".to_string(),
                    error: e.to_string(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::QueryFailed {
                    query: "SELECT version".to_string(),
                    error: e.to_string(),
                })
            })?;

        // Find migration files
        let migration_files = std::fs::read_dir(migrations_dir)
            .map_err(|e| NoaError::Io(e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "sql" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Apply pending migrations
        for migration_file in migration_files {
            let version = migration_file.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                NoaError::Internal {
                    message: "Invalid migration filename".to_string(),
                    source: None,
                }
            })?;

            if applied.contains(&version.to_string()) {
                debug!(version = %version, "Migration already applied");
                continue;
            }

            info!(version = %version, "Applying migration");

            let sql = std::fs::read_to_string(&migration_file).map_err(|e| NoaError::Io(e))?;

            // Execute migration in a transaction
            let tx = conn.unchecked_transaction().map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::TransactionFailed(
                    e.to_string(),
                ))
            })?;

            // Split SQL by semicolons, but handle BEGIN...END blocks (triggers) specially
            for statement in Self::split_sql_statements(&sql) {
                let statement = statement.trim();
                if statement.is_empty() {
                    continue;
                }

                tx.execute(statement, []).map_err(|e| {
                    NoaError::Database(crate::error::DatabaseError::QueryFailed {
                        query: statement.to_string(),
                        error: e.to_string(),
                    })
                })?;
            }

            // Record migration
            tx.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                params![version],
            )
            .map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::QueryFailed {
                    query: "INSERT INTO schema_migrations".to_string(),
                    error: e.to_string(),
                })
            })?;

            tx.commit().map_err(|e| {
                NoaError::Database(crate::error::DatabaseError::TransactionFailed(
                    e.to_string(),
                ))
            })?;

            info!(version = %version, "Migration applied successfully");
        }

        Ok(())
    }

    /// Split SQL statements handling BEGIN...END blocks (for triggers).
    /// This function handles multi-line statements and preserves trigger blocks.
    fn split_sql_statements(sql: &str) -> Vec<String> {
        let mut statements = Vec::new();
        let mut current = String::new();
        let mut in_begin_block = 0; // Track nested BEGIN blocks
        
        for line in sql.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();
            
            // Skip comment-only lines
            if trimmed.starts_with("--") {
                continue;
            }
            
            // Track BEGIN blocks (can be nested in complex triggers)
            if upper.contains("BEGIN") && !upper.starts_with("--") {
                in_begin_block += 1;
            }
            
            current.push_str(line);
            current.push('\n');
            
            // Track END statements
            if (upper.starts_with("END;") || upper.ends_with("END;") || upper == "END") 
                && in_begin_block > 0 
            {
                in_begin_block -= 1;
            }
            
            // If not in a BEGIN block and line ends with semicolon, it's a complete statement
            if in_begin_block == 0 && trimmed.ends_with(';') {
                let stmt = current.trim().to_string();
                // Remove trailing semicolon for SQLite execute()
                let stmt = stmt.strip_suffix(';').unwrap_or(&stmt).trim().to_string();
                if !stmt.is_empty() && !stmt.starts_with("--") {
                    statements.push(stmt);
                }
                current.clear();
            }
        }
        
        // Don't forget any remaining content
        let remaining = current.trim().to_string();
        let remaining = remaining.strip_suffix(';').unwrap_or(&remaining).trim().to_string();
        if !remaining.is_empty() && !remaining.starts_with("--") {
            statements.push(remaining);
        }
        
        statements
    }

    /// Verify database is operational
    pub fn verify(noa_root: &Path) -> Result<bool> {
        let db_path = NoaPaths::data(noa_root).join("noa.db");

        if !db_path.exists() {
            return Ok(false);
        }

        let conn = Connection::open(&db_path).map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::ConnectionFailed(e.to_string()))
        })?;

        // Test query
        let mut stmt = conn.prepare("SELECT 1").map_err(|e| {
            NoaError::Database(crate::error::DatabaseError::QueryFailed {
                query: "SELECT 1".to_string(),
                error: e.to_string(),
            })
        })?;

        let result: std::result::Result<i32, _> = stmt.query_row([], |row| row.get(0));
        Ok(result.is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_initialization() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(NoaPaths::data(root)).unwrap();

        DatabaseInitializer::initialize(root, false).unwrap();
        assert!(NoaPaths::data(root).join("noa.db").exists());
    }
}
