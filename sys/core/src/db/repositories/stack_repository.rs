//! Stack repository (placeholder)
//!
//! This is a minimal stub to satisfy module exports while the stack subsystem
//! is being implemented.

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct StackRecord {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

pub struct StackRepository<'a> {
    #[allow(dead_code)]
    conn: &'a Connection,
}

impl<'a> StackRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self) -> Result<Vec<StackRecord>> {
        Ok(Vec::new())
    }
}
