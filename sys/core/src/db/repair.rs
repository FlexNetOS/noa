//! Database Repair Utilities
//!
//! Provides functions to repair corrupted database components, particularly FTS tables.

use super::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use std::io::Write;

/// Repair corrupted FTS5 virtual tables
/// This function attempts to drop and recreate FTS tables that are corrupted
pub fn repair_fts_tables(conn: &Connection) -> Result<()> {
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "db/repair.rs:repair_fts_tables",
        "message": "Starting FTS table repair",
        "data": {},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "repair",
        "hypothesisId": "G"
    });
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion

    // Check if memory table exists before trying to repair memory_fts
    let memory_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='memory'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if memory_exists {
        // #region agent log
        let log_entry = serde_json::json!({
            "location": "db/repair.rs:repair_fts_tables",
            "message": "Memory table exists, attempting to repair memory_fts",
            "data": {},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "repair",
            "hypothesisId": "G"
        });
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion

        // Drop triggers first
        let _ = conn.execute_batch(
            "DROP TRIGGER IF EXISTS memory_ai;
             DROP TRIGGER IF EXISTS memory_ad;
             DROP TRIGGER IF EXISTS memory_au;",
        );

        // Try to drop the corrupted FTS table
        // Note: If the table is severely corrupted, DROP may fail
        // In that case, manual database repair may be required
        let drop_result = conn.execute("DROP TABLE IF EXISTS memory_fts", []);

        // #region agent log
        let drop_succeeded = drop_result.is_ok();
        let log_entry = serde_json::json!({
            "location": "db/repair.rs:repair_fts_tables",
            "message": "Drop attempt result",
            "data": {"drop_succeeded": drop_succeeded, "error": drop_result.as_ref().err().map(|e| e.to_string())},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "repair",
            "hypothesisId": "G"
        });
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion

        // Recreate the FTS table
        // If the old table couldn't be dropped due to corruption, CREATE IF NOT EXISTS will fail
        // In that case, we return an error indicating manual repair is needed
        let create_result = conn.execute_batch(
            "CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(
                id UNINDEXED,
                content,
                tags,
                content='memory',
                content_rowid='rowid'
            );",
        );

        if let Err(e) = create_result {
            // #region agent log
            let log_entry = serde_json::json!({
                "location": "db/repair.rs:repair_fts_tables",
                "message": "FTS table creation failed - may need manual repair",
                "data": {"error": e.to_string()},
                "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                "sessionId": "debug-session",
                "runId": "repair",
                "hypothesisId": "G"
            });
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
                let _ = writeln!(file, "{}", log_entry);
            }
            // #endregion

            // If creation failed, the table may be too corrupted to repair automatically
            // Return error but don't fail completely - the database is still functional
            return Err(crate::error::NoaError::Database(DatabaseError::QueryFailed {
                query: "CREATE VIRTUAL TABLE memory_fts (repair)".to_string(),
                error: format!("FTS table repair failed: {}. Manual database repair may be required.", e),
            }));
        }

        // Recreate triggers
        conn.execute_batch(
            "CREATE TRIGGER IF NOT EXISTS memory_ai AFTER INSERT ON memory BEGIN
                INSERT INTO memory_fts(rowid, id, content, tags)
                VALUES (NEW.rowid, NEW.id, NEW.content, NEW.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS memory_ad AFTER DELETE ON memory BEGIN
                INSERT INTO memory_fts(memory_fts, rowid, id, content, tags)
                VALUES('delete', OLD.rowid, OLD.id, OLD.content, OLD.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS memory_au AFTER UPDATE ON memory BEGIN
                INSERT INTO memory_fts(memory_fts, rowid, id, content, tags)
                VALUES('delete', OLD.rowid, OLD.id, OLD.content, OLD.tags);
                INSERT INTO memory_fts(rowid, id, content, tags)
                VALUES (NEW.rowid, NEW.id, NEW.content, NEW.tags);
            END;",
        )
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "CREATE TRIGGER memory_fts".to_string(),
            error: e.to_string(),
        }))?;

        // #region agent log
        let log_entry = serde_json::json!({
            "location": "db/repair.rs:repair_fts_tables",
            "message": "memory_fts repair completed",
            "data": {},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "repair",
            "hypothesisId": "G"
        });
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion
    }

    Ok(())
}

