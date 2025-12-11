//! In-memory provider registry

use crate::error::Result;
use crate::providers::{get_provider, list_providers, set_provider_enabled, ProviderInfo, ProviderStatus, set_provider_status};

/// Return all providers
pub fn providers() -> Result<Vec<ProviderInfo>> {
    Ok(list_providers())
}

/// Lookup provider by id
pub fn provider_by_id(id: &str) -> Result<Option<ProviderInfo>> {
    Ok(get_provider(id))
}

/// Enable a provider
pub fn enable_provider(id: &str) -> Result<bool> {
    Ok(set_provider_enabled(id, true))
}

/// Disable a provider
pub fn disable_provider(id: &str) -> Result<bool> {
    Ok(set_provider_enabled(id, false))
}

/// Update status for a provider
pub fn update_status(id: &str, status: ProviderStatus) -> Result<bool> {
    Ok(set_provider_status(id, status))
}
