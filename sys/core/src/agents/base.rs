//! BaseAgent trait (Phase 9 - T262)
use crate::error::Result;

pub trait BaseAgent {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn execute(&self, task: &str) -> Result<String>;
}
