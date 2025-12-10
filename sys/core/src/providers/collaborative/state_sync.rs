//! Provider state synchronization

use crate::providers::{ProviderInfo, ProviderStatus};

pub fn sync_state(provider: &ProviderInfo) -> ProviderStatus {
    if provider.enabled {
        ProviderStatus::Ready
    } else {
        ProviderStatus::Offline
    }
}
