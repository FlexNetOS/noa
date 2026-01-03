//! Agent log repository (Phase 9 - T259)
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AgentLog {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub message: String,
    pub level: String,
    /// Optional structured data fields (maps to PostgreSQL JSONB)
    pub fields: Option<String>,
}

pub struct AgentLogRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentLogRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new agent log entry
    pub fn create(&self, log: &AgentLog) -> Result<Uuid> {
        self.conn
            .execute(
                "INSERT INTO agent_logs (id, agent_id, message, level, fields) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![log.id.to_string(), log.agent_id.to_string(), log.message, log.level, log.fields],
            )
            .map_err(to_db_err("insert agent log"))?;
        Ok(log.id)
    }

    /// Append a log entry (convenience method)
    pub fn append(&self, agent_id: &Uuid, message: &str, level: &str) -> Result<()> {
        let id = Uuid::new_v4();
        self.conn
            .execute(
                "INSERT INTO agent_logs (id, agent_id, message, level) VALUES (?1, ?2, ?3, ?4)",
                params![id.to_string(), agent_id.to_string(), message, level],
            )
            .map_err(to_db_err("insert agent log"))?;
        Ok(())
    }

    /// Find logs by agent ID with limit
    pub fn find_by_agent(&self, agent_id: &Uuid, limit: u64) -> Result<Vec<AgentLog>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, message, level, fields FROM agent_logs WHERE agent_id = ?1 ORDER BY id DESC LIMIT ?2")
            .map_err(to_db_err("prepare find logs by agent"))?;
        let rows = stmt
            .query_map(params![agent_id.to_string(), limit as i64], |row| self.row_to_log(row))
            .map_err(to_db_err("query logs by agent"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// Find logs by level
    pub fn find_by_level(&self, level: &str, limit: u64) -> Result<Vec<AgentLog>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, message, level, fields FROM agent_logs WHERE level = ?1 ORDER BY id DESC LIMIT ?2")
            .map_err(to_db_err("prepare find logs by level"))?;
        let rows = stmt
            .query_map(params![level, limit as i64], |row| self.row_to_log(row))
            .map_err(to_db_err("query logs by level"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// List recent logs
    pub fn list_recent(&self, limit: u64) -> Result<Vec<AgentLog>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, message, level, fields FROM agent_logs ORDER BY id DESC LIMIT ?1")
            .map_err(to_db_err("prepare list recent logs"))?;
        let rows = stmt
            .query_map(params![limit as i64], |row| self.row_to_log(row))
            .map_err(to_db_err("query recent logs"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// Count logs by agent
    pub fn count_by_agent(&self, agent_id: &Uuid) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1",
                params![agent_id.to_string()],
                |row| row.get(0),
            )
            .map_err(to_db_err("count logs by agent"))?;
        Ok(count as u64)
    }

    /// List all logs for an agent (legacy)
    pub fn list(&self, agent_id: &Uuid) -> Result<Vec<AgentLog>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, message, level, fields FROM agent_logs WHERE agent_id = ?1")
            .map_err(to_db_err("prepare list agent logs"))?;
        let rows = stmt
            .query_map(params![agent_id.to_string()], |row| self.row_to_log(row))
            .map_err(to_db_err("query agent logs"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    fn row_to_log(&self, row: &rusqlite::Row) -> rusqlite::Result<AgentLog> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;
        let agent_id_str: String = row.get(1)?;
        let agent_id = Uuid::parse_str(&agent_id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;
        Ok(AgentLog {
            id,
            agent_id,
            message: row.get(2)?,
            level: row.get(3)?,
            fields: row.get(4)?,
        })
    }
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| NoaError::Database(DatabaseError::QueryFailed {
        query: context.into(),
        error: err.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS agent_logs (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                message TEXT NOT NULL,
                level TEXT NOT NULL,
                fields TEXT
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_agentlog_create() {
        let conn = setup_test_db();
        let repo = AgentLogRepository::new(&conn);

        let agent_id = Uuid::new_v4();
        let log = AgentLog {
            id: Uuid::new_v4(),
            agent_id,
            message: "Test log message".to_string(),
            level: "info".to_string(),
            fields: Some(r#"{"test": true}"#.to_string()),
        };

        let id = repo.create(&log).unwrap();
        assert_eq!(id, log.id);
    }

    #[test]
    fn test_agentlog_append() {
        let conn = setup_test_db();
        let repo = AgentLogRepository::new(&conn);

        let agent_id = Uuid::new_v4();
        repo.append(&agent_id, "Appended message", "warn").unwrap();

        let logs = repo.find_by_agent(&agent_id, 10).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Appended message");
        assert_eq!(logs[0].level, "warn");
    }

    #[test]
    fn test_agentlog_find_by_agent() {
        let conn = setup_test_db();
        let repo = AgentLogRepository::new(&conn);

        let agent_id = Uuid::new_v4();
        for i in 0..5 {
            repo.append(&agent_id, &format!("Message {}", i), "info").unwrap();
        }

        let logs = repo.find_by_agent(&agent_id, 10).unwrap();
        assert_eq!(logs.len(), 5);
    }

    #[test]
    fn test_agentlog_find_by_level() {
        let conn = setup_test_db();
        let repo = AgentLogRepository::new(&conn);

        let agent_id = Uuid::new_v4();
        repo.append(&agent_id, "Info message", "info").unwrap();
        repo.append(&agent_id, "Error message", "error").unwrap();
        repo.append(&agent_id, "Another error", "error").unwrap();

        let errors = repo.find_by_level("error", 10).unwrap();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_agentlog_count_by_agent() {
        let conn = setup_test_db();
        let repo = AgentLogRepository::new(&conn);

        let agent_id = Uuid::new_v4();
        for _ in 0..3 {
            repo.append(&agent_id, "Log entry", "debug").unwrap();
        }

        let count = repo.count_by_agent(&agent_id).unwrap();
        assert_eq!(count, 3);
    }
}
