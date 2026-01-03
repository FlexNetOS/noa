//! Agent repository (Phase 9 - T258)
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
}

pub struct AgentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new agent
    pub fn create(&self, agent: &Agent) -> Result<Uuid> {
        self.conn
            .execute(
                "INSERT INTO agents (id, name, description, status) VALUES (?1, ?2, ?3, ?4)",
                params![agent.id.to_string(), agent.name, agent.description, agent.status],
            )
            .map_err(to_db_err("create agent"))?;
        Ok(agent.id)
    }

    /// Find agent by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Agent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, status FROM agents WHERE id = ?1")
            .map_err(to_db_err("prepare find agent by id"))?;
        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_agent(row))
            .map_err(to_db_err("query agent by id"))?;
        match rows.next() {
            Some(Ok(agent)) => Ok(Some(agent)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "find agent by id".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find agent by name
    pub fn find_by_name(&self, name: &str) -> Result<Option<Agent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, status FROM agents WHERE name = ?1")
            .map_err(to_db_err("prepare find agent by name"))?;
        let mut rows = stmt
            .query_map(params![name], |row| self.row_to_agent(row))
            .map_err(to_db_err("query agent by name"))?;
        match rows.next() {
            Some(Ok(agent)) => Ok(Some(agent)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "find agent by name".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// List all agents
    pub fn list(&self) -> Result<Vec<Agent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, status FROM agents")
            .map_err(to_db_err("prepare list agents"))?;
        let rows = stmt
            .query_map([], |row| self.row_to_agent(row))
            .map_err(to_db_err("query agents"))?;
        Ok(rows.filter_map(std::result::Result::ok).collect())
    }

    /// Update an agent
    pub fn update(&self, agent: &Agent) -> Result<()> {
        let rows_affected = self
            .conn
            .execute(
                "UPDATE agents SET name = ?2, description = ?3, status = ?4 WHERE id = ?1",
                params![agent.id.to_string(), agent.name, agent.description, agent.status],
            )
            .map_err(to_db_err("update agent"))?;
        if rows_affected == 0 {
            return Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "update agent".to_string(),
                error: "Agent not found".to_string(),
            }));
        }
        Ok(())
    }

    /// Delete an agent
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM agents WHERE id = ?1", params![id.to_string()])
            .map_err(to_db_err("delete agent"))?;
        Ok(rows_affected > 0)
    }

    fn row_to_agent(&self, row: &rusqlite::Row) -> rusqlite::Result<Agent> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;
        Ok(Agent {
            id,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
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
            CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'active'
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_agent_create_and_find() {
        let conn = setup_test_db();
        let repo = AgentRepository::new(&conn);

        let agent = Agent {
            id: Uuid::new_v4(),
            name: "CECCA Agent".to_string(),
            description: Some("Chief Executive Commander".to_string()),
            status: "active".to_string(),
        };

        let id = repo.create(&agent).unwrap();
        assert_eq!(id, agent.id);

        let found = repo.find_by_id(&agent.id).unwrap().unwrap();
        assert_eq!(found.name, "CECCA Agent");
        assert_eq!(found.status, "active");
    }

    #[test]
    fn test_agent_update() {
        let conn = setup_test_db();
        let repo = AgentRepository::new(&conn);

        let agent = Agent {
            id: Uuid::new_v4(),
            name: "Test Agent".to_string(),
            description: None,
            status: "active".to_string(),
        };

        repo.create(&agent).unwrap();

        let mut updated = agent.clone();
        updated.name = "Updated Agent".to_string();
        updated.status = "paused".to_string();
        repo.update(&updated).unwrap();

        let found = repo.find_by_id(&agent.id).unwrap().unwrap();
        assert_eq!(found.name, "Updated Agent");
        assert_eq!(found.status, "paused");
    }

    #[test]
    fn test_agent_delete() {
        let conn = setup_test_db();
        let repo = AgentRepository::new(&conn);

        let agent = Agent {
            id: Uuid::new_v4(),
            name: "To Delete".to_string(),
            description: None,
            status: "active".to_string(),
        };

        repo.create(&agent).unwrap();
        assert!(repo.find_by_id(&agent.id).unwrap().is_some());

        let deleted = repo.delete(&agent.id).unwrap();
        assert!(deleted);
        assert!(repo.find_by_id(&agent.id).unwrap().is_none());
    }

    #[test]
    fn test_agent_list() {
        let conn = setup_test_db();
        let repo = AgentRepository::new(&conn);

        for i in 0..3 {
            let agent = Agent {
                id: Uuid::new_v4(),
                name: format!("Agent {}", i),
                description: Some(format!("Description {}", i)),
                status: "active".to_string(),
            };
            repo.create(&agent).unwrap();
        }

        let agents = repo.list().unwrap();
        assert_eq!(agents.len(), 3);
    }

    #[test]
    fn test_agent_update_nonexistent_fails() {
        let conn = setup_test_db();
        let repo = AgentRepository::new(&conn);

        let agent = Agent {
            id: Uuid::new_v4(),
            name: "Nonexistent".to_string(),
            description: None,
            status: "active".to_string(),
        };

        let result = repo.update(&agent);
        assert!(result.is_err());
    }
}
