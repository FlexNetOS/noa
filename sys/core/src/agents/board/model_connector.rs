//! Connect Board Agents to ModelSelector (stub)
use crate::error::Result;

pub struct BoardModelConnector;

impl BoardModelConnector {
    pub fn route(&self, query: &str) -> Result<String> {
        Ok(format!("routed board query '{}'", query))
    }
}
