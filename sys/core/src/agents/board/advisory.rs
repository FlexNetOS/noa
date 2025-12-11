//! Advisory interface for Board Agents
use crate::error::Result;

pub struct AdvisoryInterface;

impl AdvisoryInterface {
    pub fn recommend(&self, topic: &str) -> Result<String> {
        Ok(format!("advisory recommendation for {}", topic))
    }
}
