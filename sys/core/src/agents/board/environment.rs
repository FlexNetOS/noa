use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct EnvironmentBoardAgent;

impl BaseAgent for EnvironmentBoardAgent {
    fn name(&self) -> &str { "board-environment" }
    fn description(&self) -> &str { "Assesses environment and context" }
    fn capabilities(&self) -> Vec<String> { vec!["context".into(), "perception".into()] }
    fn execute(&self, task: &str) -> Result<String> { Ok(format!("Environment board advised on '{}'", task)) }
}
