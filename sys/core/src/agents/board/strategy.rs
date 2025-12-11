use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct StrategyBoardAgent;

impl BaseAgent for StrategyBoardAgent {
    fn name(&self) -> &str {
        "board-strategy"
    }
    fn description(&self) -> &str {
        "Provides strategy and planning recommendations"
    }
    fn capabilities(&self) -> Vec<String> {
        vec!["strategy".into(), "planning".into()]
    }
    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("Strategy board advised on '{}'", task))
    }
}
