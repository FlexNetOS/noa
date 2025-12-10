use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct FinanceExecutiveAgent;

impl BaseAgent for FinanceExecutiveAgent {
    fn name(&self) -> &str {
        "executive-finance"
    }

    fn description(&self) -> &str {
        "Reviews financial impact and budgets"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["budget".into(), "forecast".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("Finance executive processed '{}'", task))
    }
}
