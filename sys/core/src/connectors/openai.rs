use async_trait::async_trait;
use chrono::Duration;

use crate::connectors::base::{Connector, ConnectorContext};
use crate::connectors::cache::ConnectorCache;
use crate::connectors::network;
use crate::connectors::ConnectorState;
use crate::error::Result;

/// OpenAI connector using API key authentication
pub struct OpenAIConnector {
    api_key_env: String,
}

impl OpenAIConnector {
    pub fn new(api_key_env: Option<String>) -> Self {
        Self {
            api_key_env: api_key_env.unwrap_or_else(|| "OPENAI_API_KEY".to_string()),
        }
    }
}

#[async_trait]
impl Connector for OpenAIConnector {
    fn name(&self) -> &str {
        "openai"
    }

    fn feature_flag(&self) -> &str {
        "connectors.openai"
    }

    async fn authorize_url(&self, _ctx: &ConnectorContext) -> Result<Option<String>> {
        // API key based connector does not need OAuth
        Ok(None)
    }

    async fn status(&self, ctx: &ConnectorContext) -> Result<ConnectorState> {
        let cache = ConnectorCache::new(None)?;
        if !ctx.is_enabled(self.feature_flag()) {
            let state = ConnectorState::disabled(self.name());
            cache.store(&state)?;
            return Ok(state);
        }

        let net = network::check_connectivity();
        if !net.available {
            if let Some(cached) = cache.get(self.name(), Duration::minutes(10)) {
                return Ok(cached);
            }
            let state = ConnectorState::offline(self.name(), "Network unavailable");
            cache.store(&state)?;
            return Ok(state);
        }

        let has_key = std::env::var(&self.api_key_env)
            .map(|v| !v.is_empty())
            .unwrap_or(false);

        let state = if has_key {
            ConnectorState::ready(self.name())
        } else {
            ConnectorState::degraded(
                self.name(),
                format!("Missing {} environment variable", self.api_key_env),
            )
        };

        cache.store(&state)?;
        Ok(state)
    }
}
