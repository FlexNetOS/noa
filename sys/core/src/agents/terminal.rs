use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct TerminalAgent;

impl BaseAgent for TerminalAgent {
    fn name(&self) -> &str {
        "terminal"
    }

    fn description(&self) -> &str {
        "Executes shell commands in a controlled environment"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["shell".into(), "diagnostics".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("TerminalAgent would run '{}'", task))
    }
}
