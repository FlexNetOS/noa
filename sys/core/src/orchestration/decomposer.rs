//! Goal decomposition (Phase 9 - T280)
use crate::error::Result;

pub fn decompose(goal: &str) -> Result<Vec<String>> {
    Ok(vec![format!("Subtask for {}", goal)])
}
