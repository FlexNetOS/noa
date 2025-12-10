//! Agent repository (Phase 9 - T258)
use crate::db::repository::Repository;
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: i64,
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

    pub fn list(&self) -> Result<Vec<Agent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, status FROM agents")
            .map_err(to_db_err("prepare list agents"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Agent {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .map_err(to_db_err("query agents"))?;
        let agents = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(to_db_err("map agents"))?;
        Ok(agents)
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

// TODO: Implement Repository<Agent, i64> trait when needed
// impl<'a> Repository<Agent, i64> for AgentRepository<'a> {}
