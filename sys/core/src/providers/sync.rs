//! Synchronization across providers

use crate::providers::shared_memory::SharedProviderMemory;

/// Broadcast a shared context to all providers (stub)
pub fn broadcast_context(memory: &SharedProviderMemory, provider: &str, context_type: &str) {
    if let Some(ctx) = memory.get(provider, context_type) {
        // In a real implementation we would push to other providers.
        let _ = ctx;
    }
}
