//! Database module for NOA API

use rusqlite::Connection;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub async fn new(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;

        // Enable WAL mode for better concurrency
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;

        let db = Self { conn };
        db.initialize_schema().await?;

        Ok(db)
    }

    async fn initialize_schema(&self) -> Result<(), rusqlite::Error> {
        let schema = include_str!("../../../../../../init/migrations/001_initial.sql");

        // Since rusqlite operations are synchronous, we can execute directly
        self.conn.execute_batch(schema)?;

        tracing::info!("Database schema initialized successfully");
        Ok(())
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub async fn health_check(&self) -> Result<(), rusqlite::Error> {
        // `Connection::execute` is for statements that do NOT return rows.
        // A SELECT will trigger rusqlite::Error::ExecuteReturnedResults.
        let _: i32 = self.conn.query_row("SELECT 1", [], |row| row.get(0))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn health_check_does_not_return_results_error() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();

        let db_path = std::env::temp_dir().join(format!("noa-api-test-{nonce}.db"));
        let _ = std::fs::remove_file(&db_path);

        let db = Database::new(&db_path).await.expect("db init");
        db.health_check().await.expect("health check");

        let _ = std::fs::remove_file(&db_path);
    }
}
