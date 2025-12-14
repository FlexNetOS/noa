//! Board agent reasoning trace logger
use crate::error::Result;

pub struct BoardTraceLogger;

impl BoardTraceLogger {
    pub fn log(&self, message: &str) -> Result<()> {
        println!("[board-trace] {}", message);
        Ok(())
    }
}
