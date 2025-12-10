use crate::providers::list_providers;

/// Simple distributor that returns provider ids for visibility.
pub fn distribute(task: &str) -> Vec<String> {
    list_providers().into_iter().map(|p| format!("{}:{}", task, p.id)).collect()
}
