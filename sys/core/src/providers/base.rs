//! Provider trait definitions (Phase 2.6)

use crate::error::Result;
use crate::providers::{ProviderInfo, ProviderStatus};

/// Common interface for all providers
pub trait Provider {
    /// Provider identifier (matches registry id)
    fn id(&self) -> &str;

    /// Return provider metadata
    fn info(&self) -> ProviderInfo;

    /// Perform a health check
    fn health(&self) -> Result<ProviderStatus>;

    /// Execute a provider task with the given payload
    fn execute(&self, _payload: &str) -> Result<String> {
        Ok("noop".to_string())
    }
}
