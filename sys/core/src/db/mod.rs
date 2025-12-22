//! NOA Database Module
//!
//! Provides database connectivity, connection pooling, and repository patterns.
//! §3.2: SQLite/PostgreSQL database management
//! FR-003: Local-first database with concurrent modifications

mod pool;
mod migrations;
mod repository;
mod backend;

#[cfg(feature = "full")]
mod postgres;

#[cfg(feature = "full")]
pub mod postgres_stores;

pub mod vector_search;
pub mod repositories;
pub mod sqlite_stores;

pub use pool::{ConnectionPool, PoolConfig, PooledConnection, PoolStatus};
pub use repository::{Repository, RepositoryError, PaginatedResult, Filter, Condition, Operator, Value, Direction};
pub use migrations::{MigrationRunner, Migration};
pub use vector_search::{VectorSearch, VectorSearchConfig, VectorSearchResult as LegacyVectorSearchResult};
pub use repositories::{EmbeddingRepository, MemoryRepository};

// Backend abstraction exports
pub use backend::{
    DatabaseKind,
    DatabaseBackend,
    DatabaseHandle,
    DatabaseHealth,
    HealthCheckResult,
    MemoryStore,
    ModelStore,
    TaskStore,
    AgentStore,
    AuditLogStore,
    VectorStore,
    VectorSearchResult,
};

#[cfg(feature = "full")]
pub use postgres::{check_postgres, connect_postgres, migrate_postgres};

use std::path::Path;
use crate::error::Result;

/// Database connection type alias
pub type Connection = rusqlite::Connection;

/// Initialize the database at the given path
pub fn init_database(path: &Path) -> Result<Connection> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = rusqlite::Connection::open(path).map_err(|e| {
        crate::error::DatabaseError::ConnectionFailed(e.to_string())
    })?;

    // Configure SQLite for optimal NOA operation
    configure_connection(&conn)?;

    Ok(conn)
}

/// Configure a SQLite connection with optimal settings
fn configure_connection(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -64000;  -- 64MB cache
        PRAGMA temp_store = MEMORY;
        PRAGMA mmap_size = 268435456;  -- 256MB mmap
        PRAGMA page_size = 4096;
        PRAGMA auto_vacuum = INCREMENTAL;
        PRAGMA foreign_keys = ON;
        "#,
    ).map_err(|e| {
        crate::error::DatabaseError::QueryFailed {
            query: "PRAGMA configuration".to_string(),
            error: e.to_string(),
        }
    })?;

    Ok(())
}

/// Check database integrity
pub fn check_integrity(conn: &Connection) -> Result<bool> {
    let result: String = conn
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .map_err(|e| {
            crate::error::DatabaseError::QueryFailed {
                query: "PRAGMA integrity_check".to_string(),
                error: e.to_string(),
            }
        })?;

    Ok(result == "ok")
}

/// Get database statistics
pub fn get_stats(conn: &Connection) -> Result<DatabaseStats> {
    let page_count: i64 = conn.query_row("PRAGMA page_count", [], |row| row.get(0)).unwrap_or(0);
    let page_size: i64 = conn.query_row("PRAGMA page_size", [], |row| row.get(0)).unwrap_or(4096);
    let freelist_count: i64 = conn.query_row("PRAGMA freelist_count", [], |row| row.get(0)).unwrap_or(0);

    Ok(DatabaseStats {
        total_pages: page_count as u64,
        page_size: page_size as u64,
        free_pages: freelist_count as u64,
        total_size_bytes: (page_count * page_size) as u64,
        used_size_bytes: ((page_count - freelist_count) * page_size) as u64,
    })
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_pages: u64,
    pub page_size: u64,
    pub free_pages: u64,
    pub total_size_bytes: u64,
    pub used_size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = init_database(&db_path).unwrap();
        assert!(db_path.exists());

        // Verify settings
        let journal_mode: String = conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(journal_mode, "wal");
    }

    #[test]
    fn test_check_integrity() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = init_database(&db_path).unwrap();

        assert!(check_integrity(&conn).unwrap());
    }
}

