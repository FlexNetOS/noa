//! Connector base traits and context helpers.

use crate::error::Result;
use crate::features::flags::FeatureFlagStore;

use super::ConnectorState;

/// Shared context passed to connectors (feature flags, future config/state)
#[derive(Debug, Clone)]
pub struct ConnectorContext {
    pub flags: FeatureFlagStore,
}

impl ConnectorContext {
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.is_enabled(flag)
    }
}

/// Helper to build a context from disk-backed feature flags
pub fn load_context() -> Result<ConnectorContext> {
    let flags = FeatureFlagStore::load(None)?;
    Ok(ConnectorContext { flags })
}

/// Base connector trait implemented by all external integrations (stubbed).
pub trait Connector {
    /// Unique connector name (used for feature flags and status)
    fn name(&self) -> &str;

    /// Basic ready state helper; connectors may override with richer health checks.
    fn state(&self) -> ConnectorState {
        ConnectorState::ready(self.name())
    }
}
