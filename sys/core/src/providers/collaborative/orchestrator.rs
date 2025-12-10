use crate::error::Result;
use crate::providers::list_providers;
use crate::providers::selector::select_best;

/// Coordinate collaborative reasoning across providers.
pub fn orchestrate(task: &str) -> Result<String> {
    let providers = list_providers();
    let chosen = select_best(&providers);
    Ok(format!(
        "task '{}' sent to {:?}",
        task,
        chosen.map(|c| c.id)
    ))
}
