//! Task repository (Phase 9 - T260)
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub agent_id: Option<Uuid>,
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

    /// Create a new task
    pub fn create(&self, task: &Task) -> Result<Uuid> {
        self.conn
            .execute(
                "INSERT INTO tasks (id, agent_id, title, status, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    task.id.to_string(),
                    task.agent_id.map(|id| id.to_string()),
                    task.title,
                    task.status,
                    task.payload,
                ],
            )
            .map_err(to_db_err("create task"))?;
        Ok(task.id)
    }

    /// Find task by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, title, status, payload FROM tasks WHERE id = ?1")
            .map_err(to_db_err("prepare find task by id"))?;
        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_task(row))
            .map_err(to_db_err("query task by id"))?;
        match rows.next() {
            Some(Ok(task)) => Ok(Some(task)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "find task by id".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Update a task
    pub fn update(&self, task: &Task) -> Result<()> {
        let rows_affected = self
            .conn
            .execute(
                "UPDATE tasks SET agent_id = ?2, title = ?3, status = ?4, payload = ?5 WHERE id = ?1",
                params![
                    task.id.to_string(),
                    task.agent_id.map(|id| id.to_string()),
                    task.title,
                    task.status,
                    task.payload,
                ],
            )
            .map_err(to_db_err("update task"))?;
        if rows_affected == 0 {
            return Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "update task".to_string(),
                error: "Task not found".to_string(),
            }));
        }
        Ok(())
    }

    /// Delete a task
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id.to_string()])
            .map_err(to_db_err("delete task"))?;
        Ok(rows_affected > 0)
    }

    /// List tasks with pagination
    pub fn list(&self, offset: u64, limit: u64) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, title, status, payload FROM tasks LIMIT ?1 OFFSET ?2")
            .map_err(to_db_err("prepare list tasks"))?;
        let rows = stmt
            .query_map(params![limit as i64, offset as i64], |row| self.row_to_task(row))
            .map_err(to_db_err("query tasks"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// Find tasks by status
    pub fn find_by_status(&self, status: &str) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, agent_id, title, status, payload FROM tasks WHERE status = ?1")
            .map_err(to_db_err("prepare find tasks by status"))?;
        let rows = stmt
            .query_map(params![status], |row| self.row_to_task(row))
            .map_err(to_db_err("query tasks by status"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// Count all tasks
    pub fn count(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
            .map_err(to_db_err("count tasks"))?;
        Ok(count as u64)
    }

    fn row_to_task(&self, row: &rusqlite::Row) -> rusqlite::Result<Task> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;
        let agent_id_str: Option<String> = row.get(1)?;
        let agent_id = agent_id_str.and_then(|s| Uuid::parse_str(&s).ok());
        Ok(Task {
            id,
            agent_id,
            title: row.get(2)?,
            status: row.get(3)?,
            payload: row.get(4)?,
        })
    }
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| NoaError::Database(DatabaseError::QueryFailed {
        query: context.into(),
        error: err.to_string(),
    })
}
