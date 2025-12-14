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
        // Since rusqlite operations are synchronous, we can execute directly
        self.conn.execute("SELECT 1", [])?;
        Ok(())
    }
}
