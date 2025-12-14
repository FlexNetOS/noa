use crate::error::Result;
use crate::providers::selector::select_best;
use crate::providers::list_providers;

/// Coordinate Cursor with other providers for parallel execution.
pub fn orchestrate(task: &str) -> Result<String> {
    let providers = list_providers();
    let chosen = select_best(&providers);
    Ok(format!(
        "orchestrated task '{}' with {:?}",
        task, chosen.map(|c| c.id)
    ))
}
