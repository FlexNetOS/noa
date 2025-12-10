use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct ExecutiveCommanderChiefAgent;

impl BaseAgent for ExecutiveCommanderChiefAgent {
    fn name(&self) -> &str {
        "executive-commander"
    }

    fn description(&self) -> &str {
        "Top-level executive commander agent"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["strategy".into(), "oversight".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("Executive commander oversaw '{}'", task))
    }
}
