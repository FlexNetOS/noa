use crate::error::Result;
use crate::providers::ProviderInfo;

/// Route tasks to providers based on capability (stub)
pub fn route_task(task_type: &str, providers: &[ProviderInfo]) -> Result<Option<ProviderInfo>> {
    let selected = providers
        .iter()
        .find(|p| p.capabilities.iter().any(|c| c.contains(task_type)))
        .cloned();
    Ok(selected)
}
