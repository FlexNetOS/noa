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
