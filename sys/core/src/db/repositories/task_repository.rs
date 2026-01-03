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
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                agent_id TEXT,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                payload TEXT
            );
            "#,
        ).unwrap();
        // Leak tempdir to keep it alive
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_task_create_and_find() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        let task = Task {
            id: Uuid::new_v4(),
            agent_id: None,
            title: "Test Task".to_string(),
            status: "pending".to_string(),
            payload: Some(r#"{"priority": "high"}"#.to_string()),
        };

        let id = repo.create(&task).unwrap();
        assert_eq!(id, task.id);

        let found = repo.find_by_id(&task.id).unwrap().unwrap();
        assert_eq!(found.title, "Test Task");
        assert_eq!(found.status, "pending");
    }

    #[test]
    fn test_task_update() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        let task = Task {
            id: Uuid::new_v4(),
            agent_id: None,
            title: "Original Title".to_string(),
            status: "pending".to_string(),
            payload: None,
        };

        repo.create(&task).unwrap();

        let mut updated = task.clone();
        updated.title = "Updated Title".to_string();
        updated.status = "completed".to_string();
        repo.update(&updated).unwrap();

        let found = repo.find_by_id(&task.id).unwrap().unwrap();
        assert_eq!(found.title, "Updated Title");
        assert_eq!(found.status, "completed");
    }

    #[test]
    fn test_task_delete() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        let task = Task {
            id: Uuid::new_v4(),
            agent_id: None,
            title: "To Delete".to_string(),
            status: "pending".to_string(),
            payload: None,
        };

        repo.create(&task).unwrap();
        assert!(repo.find_by_id(&task.id).unwrap().is_some());

        let deleted = repo.delete(&task.id).unwrap();
        assert!(deleted);
        assert!(repo.find_by_id(&task.id).unwrap().is_none());
    }

    #[test]
    fn test_task_list_and_count() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        for i in 0..5 {
            let task = Task {
                id: Uuid::new_v4(),
                agent_id: None,
                title: format!("Task {}", i),
                status: if i % 2 == 0 { "pending" } else { "completed" }.to_string(),
                payload: None,
            };
            repo.create(&task).unwrap();
        }

        let count = repo.count().unwrap();
        assert_eq!(count, 5);

        let page = repo.list(0, 3).unwrap();
        assert_eq!(page.len(), 3);

        let pending = repo.find_by_status("pending").unwrap();
        assert_eq!(pending.len(), 3);
    }

    #[test]
    fn test_task_find_nonexistent() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        let result = repo.find_by_id(&Uuid::new_v4()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_task_delete_nonexistent() {
        let conn = setup_test_db();
        let repo = TaskRepository::new(&conn);

        let deleted = repo.delete(&Uuid::new_v4()).unwrap();
        assert!(!deleted);
    }
}
