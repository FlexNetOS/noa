use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct ArchitectureExecutiveAgent;

impl BaseAgent for ArchitectureExecutiveAgent {
    fn name(&self) -> &str {
        "executive-architecture"
    }

    fn description(&self) -> &str {
        "Reviews system architecture and design choices"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["architecture".into(), "design".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("Architecture executive reviewed '{}'", task))
    }
}
