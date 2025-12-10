//! AgentService (Phase 9 - T291)
use crate::db::repositories::{AgentRepository, AgentLogRepository};
use crate::db::ConnectionPool;
use crate::error::Result;

pub struct AgentService {
    pool: ConnectionPool,
}

impl AgentService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    pub fn list(&self) -> Result<Vec<String>> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        let agents = repo.list()?;
        Ok(agents.into_iter().map(|a| a.name).collect())
    }

    pub fn log(&self, agent_id: i64, message: &str, level: &str) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        repo.append(agent_id, message, level)
    }
}
