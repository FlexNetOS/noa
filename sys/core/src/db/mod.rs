//! NOA Database Module
//!
//! Provides database connectivity, connection pooling, and repository patterns.
//! §3.2: SQLite/PostgreSQL database management
//! FR-003: Local-first database with concurrent modifications

mod migrations;
mod pool;
pub mod repair;
pub mod repositories;
mod repository;
pub mod vector_search;

pub use migrations::{Migration, MigrationRunner};
pub use pool::{ConnectionPool, PoolConfig};
pub use repair::repair_fts_tables;
pub use repositories::{EmbeddingRepository, MemoryRepository};
pub use repository::{Repository, RepositoryError};
pub use vector_search::{VectorSearch, VectorSearchConfig, VectorSearchResult};

use crate::error::Result;
use std::path::Path;
use std::io::Write;

/// Database connection type alias
pub type Connection = rusqlite::Connection;

/// Initialize the database at the given path
pub fn init_database(path: &Path) -> Result<Connection> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path)
        .map_err(|e| crate::error::DatabaseError::ConnectionFailed(e.to_string()))?;

    // Configure SQLite for optimal NOA operation
    configure_connection(&conn)?;

    Ok(conn)
}

/// Configure a SQLite connection with optimal settings
fn configure_connection(conn: &Connection) -> Result<()> {
    // Enable WAL mode for concurrent reads
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
    )
    .map_err(|e| crate::error::DatabaseError::QueryFailed {
        query: "PRAGMA configuration".to_string(),
        error: e.to_string(),
    })?;

    Ok(())
}

/// Check database integrity
/// Returns Ok(true) if integrity is good, Ok(false) if there are issues
/// FTS table errors are treated as non-critical and return Ok(false) rather than Err
pub fn check_integrity(conn: &Connection) -> Result<bool> {
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "db/mod.rs:65",
        "message": "Before integrity check",
        "data": {},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "integrity-check",
        "hypothesisId": "F"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion

    let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0)).map_err(|e| {
        // #region agent log
        let error_msg = e.to_string();
        let log_entry = serde_json::json!({
            "location": "db/mod.rs:67",
            "message": "Integrity check query failed",
            "data": {"error": error_msg.clone()},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "integrity-check",
            "hypothesisId": "F"
        });
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion

        crate::error::DatabaseError::QueryFailed {
            query: "PRAGMA integrity_check".to_string(),
            error: error_msg,
        }
    })?;

    // #region agent log
    let log_entry = serde_json::json!({
        "location": "db/mod.rs:74",
        "message": "After integrity check",
        "data": {"result": result.clone()},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "integrity-check",
        "hypothesisId": "F"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion

    // Check if result is "ok" or if it only contains FTS-related errors (non-critical)
    if result == "ok" {
        Ok(true)
    } else if result.contains("memory_fts") || result.contains("vtable") || result.contains("fts") {
        // FTS table errors are non-critical - database is still functional
        // #region agent log
        let log_entry = serde_json::json!({
            "location": "db/mod.rs:85",
            "message": "FTS table error detected (non-critical)",
            "data": {"result": result.clone()},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "integrity-check",
            "hypothesisId": "F"
        });
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion
        Ok(false) // Database is functional, just FTS is broken
    } else {
        // Other integrity issues are critical
        Ok(false)
    }
}

/// Get database statistics
pub fn get_stats(conn: &Connection) -> Result<DatabaseStats> {
    let page_count: i64 = conn.query_row("PRAGMA page_count", [], |row| row.get(0)).unwrap_or(0);

    let page_size: i64 = conn.query_row("PRAGMA page_size", [], |row| row.get(0)).unwrap_or(4096);

    let freelist_count: i64 =
        conn.query_row("PRAGMA freelist_count", [], |row| row.get(0)).unwrap_or(0);

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
        let journal_mode: String =
            conn.query_row("PRAGMA journal_mode", [], |row| row.get(0)).unwrap();
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
