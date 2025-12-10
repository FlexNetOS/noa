//! Connector base traits and context helpers.

use async_trait::async_trait;

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
        self.flags.is_enabled("connectors.enabled") && self.flags.is_enabled(flag)
    }
}

/// Helper to build a context from disk-backed feature flags
pub fn load_context() -> Result<ConnectorContext> {
    let flags = FeatureFlagStore::load(None)?;
    Ok(ConnectorContext { flags })
}

/// Base connector trait implemented by all external integrations.
#[async_trait]
pub trait Connector {
    /// Unique connector name (used for feature flags and status)
    fn name(&self) -> &str;

    /// Feature flag name for this connector (e.g., `connectors.github`)
    fn feature_flag(&self) -> &str {
        "connectors.enabled"
    }

    /// Basic ready state helper; connectors may override with richer health checks.
    fn state(&self) -> ConnectorState {
        ConnectorState::ready(self.name())
    }

    /// OAuth scopes or API permissions requested by the connector
    fn scopes(&self) -> Vec<String> {
        Vec::new()
    }

    /// Construct an authorization URL if OAuth is required
    async fn authorize_url(&self, _ctx: &ConnectorContext) -> Result<Option<String>> {
        Ok(None)
    }

    /// Refresh tokens or renew credentials if applicable
    async fn refresh_credentials(&self, _ctx: &ConnectorContext) -> Result<()> {
        Ok(())
    }

    /// Report current connector status
    async fn status(&self, _ctx: &ConnectorContext) -> Result<ConnectorState> {
        Ok(self.state())
    }
}
