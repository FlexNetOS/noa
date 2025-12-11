use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct RAGAgent;

impl BaseAgent for RAGAgent {
    fn name(&self) -> &str {
        "rag"
    }

    fn description(&self) -> &str {
        "Retrieval-augmented generation agent"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["retrieve".into(), "generate".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("RAGAgent processed '{}'", task))
    }
}
