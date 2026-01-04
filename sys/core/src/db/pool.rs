//! SQLite Connection Pool
//!
//! Provides connection pooling for SQLite with rusqlite.
//! §3.2: Database connection management

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::error::{DatabaseError, Result};
use super::Connection;

/// Connection pool configsuration
#[derive(Debug, Clone)]
pub struct Poolconfigs {
    /// Maximum number of connections in the pool
    pub max_connections: u32,

    /// Minimum number of idle connections to maintain
    pub min_idle: u32,

    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,

    /// Maximum lifetime of a connection in seconds
    pub max_lifetime_secs: u64,

    /// Idle timeout before connection is closed
    pub idle_timeout_secs: u64,
}

impl Default for Poolconfigs {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_idle: 2,
            connection_timeout_ms: 30000,
            max_lifetime_secs: 3600,
            idle_timeout_secs: 600,
        }
    }
}

/// A pooled database connection
pub struct PooledConnection {
    conn: Option<Connection>,
    pool: Arc<ConnectionPoolInner>,
    created_at: Instant,
}

impl PooledConnection {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        self.conn.as_ref().expect("Connection should exist")
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        self.conn.as_mut().expect("Connection should exist")
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            self.pool.return_connection(conn, self.created_at);
        }
    }
}

impl std::ops::Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection()
    }
}

impl std::ops::DerefMut for PooledConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.connection_mut()
    }
}

struct PooledConnectionEntry {
    conn: Connection,
    created_at: Instant,
    last_used: Instant,
}

struct ConnectionPoolInner {
    db_path: PathBuf,
    configs: Poolconfigs,
    connections: Mutex<VecDeque<PooledConnectionEntry>>,
    active_count: Mutex<u32>,
}

impl ConnectionPoolInner {
    fn return_connection(&self, conn: Connection, created_at: Instant) {
        let mut connections = self.connections.lock().unwrap();
        let mut active = self.active_count.lock().unwrap();

        *active = active.saturating_sub(1);

        // Check if connection is still valid (not too old)
        let max_lifetime = Duration::from_secs(self.configs.max_lifetime_secs);
        if created_at.elapsed() < max_lifetime {
            connections.push_back(PooledConnectionEntry {
                conn,
                created_at,
                last_used: Instant::now(),
            });
        }
        // Otherwise, let the connection drop
    }
}

/// SQLite connection pool
pub struct ConnectionPool {
    inner: Arc<ConnectionPoolInner>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(db_path: &Path, configs: Poolconfigs) -> Result<Self> {
        let inner = Arc::new(ConnectionPoolInner {
            db_path: db_path.to_path_buf(),
            configs,
            connections: Mutex::new(VecDeque::new()),
            active_count: Mutex::new(0),
        });

        let pool = Self { inner };

        // Pre-populate with minimum idle connections
        pool.populate_idle()?;

        Ok(pool)
    }

    /// Create a pool with default configsuration
    pub fn with_defaults(db_path: &Path) -> Result<Self> {
        Self::new(db_path, Poolconfigs::default())
    }

    /// Get a connection from the pool
    pub fn get(&self) -> Result<PooledConnection> {
        let timeout = Duration::from_millis(self.inner.configs.connection_timeout_ms);
        let start = Instant::now();

        loop {
            // Try to get an existing connection
            {
                let mut connections = self.inner.connections.lock().unwrap();

                // Clean up old connections
                let idle_timeout = Duration::from_secs(self.inner.configs.idle_timeout_secs);
                connections.retain(|entry| entry.last_used.elapsed() < idle_timeout);

                // Try to get a connection
                if let Some(entry) = connections.pop_front() {
                    let mut active = self.inner.active_count.lock().unwrap();
                    *active += 1;

                    return Ok(PooledConnection {
                        conn: Some(entry.conn),
                        pool: self.inner.clone(),
                        created_at: entry.created_at,
                    });
                }
            }

            // Check if we can create a new connection
            {
                let mut active = self.inner.active_count.lock().unwrap();
                if *active < self.inner.configs.max_connections {
                    *active += 1;
                    drop(active);

                    let conn = self.create_connection()?;
                    return Ok(PooledConnection {
                        conn: Some(conn),
                        pool: self.inner.clone(),
                        created_at: Instant::now(),
                    });
                }
            }

            // Check timeout
            if start.elapsed() >= timeout {
                return Err(DatabaseError::PoolExhausted.into());
            }

            // Wait a bit before retrying
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Get the current pool status
    pub fn status(&self) -> PoolStatus {
        let connections = self.inner.connections.lock().unwrap();
        let active = self.inner.active_count.lock().unwrap();

        PoolStatus {
            idle_connections: connections.len() as u32,
            active_connections: *active,
            max_connections: self.inner.configs.max_connections,
        }
    }

    fn create_connection(&self) -> Result<Connection> {
        super::init_database(&self.inner.db_path)
    }

    fn populate_idle(&self) -> Result<()> {
        let min_idle = self.inner.configs.min_idle;
        let mut connections = self.inner.connections.lock().unwrap();

        for _ in 0..min_idle {
            let conn = self.create_connection()?;
            connections.push_back(PooledConnectionEntry {
                conn,
                created_at: Instant::now(),
                last_used: Instant::now(),
            });
        }

        Ok(())
    }
}

/// Pool status information
#[derive(Debug, Clone)]
pub struct PoolStatus {
    pub idle_connections: u32,
    pub active_connections: u32,
    pub max_connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_pool_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let pool = ConnectionPool::with_defaults(&db_path).unwrap();
        let status = pool.status();

        assert_eq!(status.idle_connections, 2);  // min_idle default
        assert_eq!(status.active_connections, 0);
    }

    #[test]
    fn test_get_connection() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let pool = ConnectionPool::with_defaults(&db_path).unwrap();

        let conn = pool.get().unwrap();
        assert_eq!(pool.status().active_connections, 1);

        // Connection should be usable
        conn.execute_batch("SELECT 1").unwrap();

        drop(conn);
        assert_eq!(pool.status().active_connections, 0);
    }
}

