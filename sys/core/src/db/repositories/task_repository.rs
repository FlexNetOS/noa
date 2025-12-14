//! Task repository (Phase 9 - T260)
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub agent_id: Option<i64>,
    pub title: String,
    pub status: String,
    pub payload: Option<String>,
}

pub struct TaskRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TaskRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, title, status, payload FROM tasks")
            .map_err(to_db_err("prepare list tasks"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    agent_id: row.get(1)?,
                    title: row.get(2)?,
                    status: row.get(3)?,
                    payload: row.get(4)?,
                })
            })
            .map_err(to_db_err("query tasks"))?;
        let tasks = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(to_db_err("map tasks"))?;
        Ok(tasks)
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
// impl<'a> Repository<Task, i64> for TaskRepository<'a> {}
