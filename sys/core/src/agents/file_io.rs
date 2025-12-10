use crate::agents::base::BaseAgent;
use crate::error::Result;

pub struct FileIOAgent;

impl BaseAgent for FileIOAgent {
    fn name(&self) -> &str {
        "file-io"
    }

    fn description(&self) -> &str {
        "Handles filesystem operations"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["read".into(), "write".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("FileIOAgent executed task '{}'", task))
    }
}
