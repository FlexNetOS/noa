//! TaskService (Phase 9 - T292)
use crate::db::repositories::{TaskRepository, TaskEventRepository};
use crate::db::ConnectionPool;
use crate::error::Result;

pub struct TaskService {
    pool: ConnectionPool,
}

impl TaskService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    pub fn list(&self) -> Result<Vec<String>> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        let tasks = repo.list()?;
        Ok(tasks.into_iter().map(|t| t.title).collect())
    }

    pub fn append_event(&self, task_id: i64, kind: &str, message: Option<&str>) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = TaskEventRepository::new(&conn);
        repo.append(task_id, kind, message)
    }
}
