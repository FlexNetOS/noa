//! Provider selection logic

use crate::providers::{ProviderInfo, ProviderStatus};

/// Pick the next ready provider by priority (lowest number wins).
pub fn select_best(providers: &[ProviderInfo]) -> Option<ProviderInfo> {
    let mut candidates: Vec<_> = providers
        .iter()
        .filter(|p| p.enabled && p.status == ProviderStatus::Ready)
        .cloned()
        .collect();
    candidates.sort_by_key(|p| p.priority);
    candidates.first().cloned()
}
