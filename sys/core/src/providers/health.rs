//! Provider health checking

use crate::error::Result;
use crate::providers::{ProviderInfo, ProviderStatus};
use std::time::Duration;

/// Simple health status derived from provider metadata
pub fn health_summary(provider: &ProviderInfo) -> ProviderStatus {
    if !provider.enabled {
        ProviderStatus::Offline
    } else {
        provider.status.clone()
    }
}

/// Run a lightweight health check (stubbed)
pub async fn check_provider(_provider: &ProviderInfo) -> Result<ProviderStatus> {
    // Placeholder: real implementation would call provider-specific ping
    tokio::time::sleep(Duration::from_millis(10)).await;
    Ok(ProviderStatus::Ready)
}
