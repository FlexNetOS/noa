//! MicroAgentStack repository (Phase 9 - T275)
use crate::db::repository::Repository;
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{Connection, params};

#[derive(Debug, Clone)]
pub struct StackRecord {
    pub id: i64,
    pub name: String,
    pub stage: String,
}

pub struct StackRepository<'a> {
    conn: &'a Connection,
}

impl<'a> StackRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self) -> Result<Vec<StackRecord>> {
        let mut stmt = self.conn.prepare("SELECT id, name, stage FROM microagent_stacks")
            .map_err(to_db_err("prepare list stacks"))?;
        let rows = stmt.query_map([], |row| {
            Ok(StackRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                stage: row.get(2)?,
            })
        }).map_err(to_db_err("query stacks"))?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn update_stage(&self, id: i64, stage: &str) -> Result<()> {
        self.conn.execute("UPDATE microagent_stacks SET stage = ?1 WHERE id = ?2", params![stage, id])
            .map_err(to_db_err("update stack stage"))?;
        Ok(())
    }
}

fn to_db_err(context: &'static str) -> impl Fn(rusqlite::Error) -> NoaError {
    move |err| NoaError::Database(DatabaseError::QueryFailed {
        query: context.into(),
        error: err.to_string(),
    })
}

// TODO: Implement Repository trait when needed
// impl<'a> Repository<Stack, i64> for StackRepository<'a> {}
