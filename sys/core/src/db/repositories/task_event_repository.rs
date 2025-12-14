//! Task event repository (Phase 9 - T261)
use crate::db::repository::Repository;
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct TaskEvent {
    pub id: i64,
    pub task_id: i64,
    pub kind: String,
    pub message: Option<String>,
}

pub struct TaskEventRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TaskEventRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn append(&self, task_id: i64, kind: &str, message: Option<&str>) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO task_events (task_id, kind, message) VALUES (?1, ?2, ?3)",
                params![task_id, kind, message],
            )
            .map_err(to_db_err("insert task event"))?;
        Ok(())
    }

    pub fn list(&self, task_id: i64) -> Result<Vec<TaskEvent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, task_id, kind, message FROM task_events WHERE task_id = ?1")
            .map_err(to_db_err("prepare list task events"))?;
        let rows = stmt
            .query_map(params![task_id], |row| {
                Ok(TaskEvent {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    kind: row.get(2)?,
                    message: row.get(3)?,
                })
            })
            .map_err(to_db_err("query task events"))?;
        Ok(rows.filter_map(Result::ok).collect())
    }
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| NoaError::Database(DatabaseError::QueryFailed {
        query: context.into(),
        error: err.to_string(),
    })
}

impl<'a> Repository<TaskEvent, i64> for TaskEventRepository<'a> {}
