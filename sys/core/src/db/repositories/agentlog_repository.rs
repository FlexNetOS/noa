//! Agent log repository (Phase 9 - T259)
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct AgentLog {
    pub id: i64,
    pub agent_id: i64,
    pub message: String,
    pub level: String,
}

pub struct AgentLogRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentLogRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn append(&self, agent_id: i64, message: &str, level: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO agent_logs (agent_id, message, level) VALUES (?1, ?2, ?3)",
                params![agent_id, message, level],
            )
            .map_err(to_db_err("insert agent log"))?;
        Ok(())
    }

    pub fn list(&self, agent_id: i64) -> Result<Vec<AgentLog>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, message, level FROM agent_logs WHERE agent_id = ?1")
            .map_err(to_db_err("prepare list agent logs"))?;
        let rows = stmt
            .query_map(params![agent_id], |row| {
                Ok(AgentLog {
                    id: row.get(0)?,
                    agent_id: row.get(1)?,
                    message: row.get(2)?,
                    level: row.get(3)?,
                })
            })
            .map_err(to_db_err("query agent logs"))?;
        let logs = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(to_db_err("map agent logs"))?;
        Ok(logs)
    }
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| {
        NoaError::Database(DatabaseError::QueryFailed {
            query: context.into(),
            error: err.to_string(),
        })
    }
}

// TODO: Implement Repository trait when needed
// impl<'a> Repository<AgentLog, i64> for AgentLogRepository<'a> {}
