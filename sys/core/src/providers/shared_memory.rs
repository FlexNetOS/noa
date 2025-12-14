//! Shared provider execution memory with SQLite persistence
//!
//! Implements §4.10: Context sharing across AI providers

use std::path::Path;
use std::sync::Arc;

use parking_lot::Mutex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{DatabaseError, NoaError, Result};

/// Shared context entry for provider execution memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedContext {
    pub provider: String,
    pub context_type: String,
    pub content: String,
    pub metadata: Option<String>,
    pub session_id: Option<String>,
}

/// Shared provider memory backed by SQLite
pub struct SharedProviderMemory {
    conn: Arc<Mutex<Connection>>,
}

// Safe to send across threads (Connection is protected by Mutex)
unsafe impl Send for SharedProviderMemory {}
unsafe impl Sync for SharedProviderMemory {}

impl Clone for SharedProviderMemory {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
        }
    }
}

impl SharedProviderMemory {
    /// Create a new shared provider memory with database connection
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path).map_err(|e| {
            NoaError::Database(DatabaseError::ConnectionFailed(
                format!("{}: {}", db_path.display(), e)
            ))
        })?;

        // Initialize schema
        Self::init_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Create in-memory instance (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().map_err(|e| {
            NoaError::Database(DatabaseError::ConnectionFailed(
                format!(":memory: connection failed: {}", e)
            ))
        })?;

        Self::init_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS execution_context (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                provider TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                context_type TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                UNIQUE(session_id, provider, context_type)
            );

            CREATE TABLE IF NOT EXISTS provider_state (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                provider TEXT NOT NULL UNIQUE,
                status TEXT DEFAULT 'unknown',
                last_heartbeat DATETIME,
                capabilities TEXT,
                current_load REAL DEFAULT 0.0
            );

            CREATE INDEX IF NOT EXISTS idx_context_session 
                ON execution_context(session_id);
            CREATE INDEX IF NOT EXISTS idx_context_provider 
                ON execution_context(provider);
            "#,
        )
        .map_err(|e| {
            NoaError::Database(DatabaseError::QueryFailed {
                query: "init_schema".to_string(),
                error: e.to_string(),
            })
        })?;

        Ok(())
    }

    /// Insert or update a context entry
    pub fn upsert(&self, context: SharedContext) -> Result<()> {
        let session_id = context
            .session_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let conn = self.conn.lock();
        conn.execute(
            r#"INSERT INTO execution_context (session_id, provider, context_type, content, metadata)
               VALUES (?1, ?2, ?3, ?4, ?5)
               ON CONFLICT(session_id, provider, context_type) 
               DO UPDATE SET content = ?4, metadata = ?5, timestamp = CURRENT_TIMESTAMP"#,
            rusqlite::params![
                session_id,
                context.provider,
                context.context_type,
                context.content,
                context.metadata,
            ],
        )
        .map_err(|e| {
            NoaError::Database(DatabaseError::QueryFailed {
                query: "upsert execution_context".to_string(),
                error: e.to_string(),
            })
        })?;

        Ok(())
    }

    /// Get context by provider and type
    pub fn get(&self, provider: &str, context_type: &str) -> Option<SharedContext> {
        let conn = self.conn.lock();
        conn.query_row(
            r#"SELECT provider, context_type, content, metadata, session_id 
               FROM execution_context 
               WHERE provider = ?1 AND context_type = ?2
               ORDER BY timestamp DESC LIMIT 1"#,
            rusqlite::params![provider, context_type],
            |row| {
                Ok(SharedContext {
                    provider: row.get(0)?,
                    context_type: row.get(1)?,
                    content: row.get(2)?,
                    metadata: row.get(3)?,
                    session_id: row.get(4)?,
                })
            },
        )
        .ok()
    }

    /// Get all contexts
    pub fn all(&self) -> Vec<SharedContext> {
        let conn = self.conn.lock();
        let mut stmt = match conn.prepare(
            r#"SELECT provider, context_type, content, metadata, session_id 
               FROM execution_context ORDER BY timestamp DESC"#,
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let rows = stmt.query_map([], |row| {
            Ok(SharedContext {
                provider: row.get(0)?,
                context_type: row.get(1)?,
                content: row.get(2)?,
                metadata: row.get(3)?,
                session_id: row.get(4)?,
            })
        });

        match rows {
            Ok(r) => r.filter_map(|x| x.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Get contexts for a specific session
    pub fn get_session(&self, session_id: &str) -> Vec<SharedContext> {
        let conn = self.conn.lock();
        let mut stmt = match conn.prepare(
            r#"SELECT provider, context_type, content, metadata, session_id 
               FROM execution_context 
               WHERE session_id = ?1 
               ORDER BY timestamp DESC"#,
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let rows = stmt.query_map([session_id], |row| {
            Ok(SharedContext {
                provider: row.get(0)?,
                context_type: row.get(1)?,
                content: row.get(2)?,
                metadata: row.get(3)?,
                session_id: row.get(4)?,
            })
        });

        match rows {
            Ok(r) => r.filter_map(|x| x.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Update provider state
    pub fn update_provider_state(
        &self,
        provider: &str,
        status: &str,
        capabilities: Option<&str>,
        load: f64,
    ) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            r#"INSERT INTO provider_state (provider, status, last_heartbeat, capabilities, current_load)
               VALUES (?1, ?2, CURRENT_TIMESTAMP, ?3, ?4)
               ON CONFLICT(provider) 
               DO UPDATE SET status = ?2, last_heartbeat = CURRENT_TIMESTAMP, 
                            capabilities = COALESCE(?3, capabilities), current_load = ?4"#,
            rusqlite::params![provider, status, capabilities, load],
        )
        .map_err(|e| {
            NoaError::Database(DatabaseError::QueryFailed {
                query: "update provider_state".to_string(),
                error: e.to_string(),
            })
        })?;

        Ok(())
    }

    /// Get provider state
    pub fn get_provider_state(&self, provider: &str) -> Option<serde_json::Value> {
        let conn = self.conn.lock();
        conn.query_row(
            r#"SELECT provider, status, last_heartbeat, capabilities, current_load 
               FROM provider_state WHERE provider = ?1"#,
            [provider],
            |row| {
                let provider: String = row.get(0)?;
                let status: String = row.get(1)?;
                let last_heartbeat: Option<String> = row.get(2)?;
                let capabilities: Option<String> = row.get(3)?;
                let current_load: f64 = row.get(4)?;

                Ok(serde_json::json!({
                    "provider": provider,
                    "status": status,
                    "last_heartbeat": last_heartbeat,
                    "capabilities": capabilities,
                    "current_load": current_load
                }))
            },
        )
        .ok()
    }

    /// Count total contexts
    pub fn count(&self) -> usize {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM execution_context", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap_or(0) as usize
    }
}
