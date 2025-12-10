use crate::error::Result;

pub struct ExecutiveAuditLogger;

impl ExecutiveAuditLogger {
    pub fn log(&self, message: &str) -> Result<()> {
        println!("[executive-audit] {}", message);
        Ok(())
    }
}
