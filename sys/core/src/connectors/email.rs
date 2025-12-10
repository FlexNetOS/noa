use async_trait::async_trait;
use chrono::Duration;

use crate::connectors::base::{Connector, ConnectorContext};
use crate::connectors::cache::ConnectorCache;
use crate::connectors::network;
use crate::connectors::ConnectorState;
use crate::error::Result;

/// SMTP/IMAP email connector
pub struct EmailConnector;

impl EmailConnector {
    pub fn new() -> Self {
        Self
    }

    fn smtp_host(&self) -> Option<String> {
        std::env::var("NOA_EMAIL_SMTP")
            .or_else(|_| std::env::var("EMAIL_SMTP_HOST"))
            .ok()
    }

    fn imap_host(&self) -> Option<String> {
        std::env::var("NOA_EMAIL_IMAP")
            .or_else(|_| std::env::var("EMAIL_IMAP_HOST"))
            .ok()
    }
}

#[async_trait]
impl Connector for EmailConnector {
    fn name(&self) -> &str {
        "email"
    }

    fn feature_flag(&self) -> &str {
        "connectors.email"
    }

    async fn authorize_url(&self, _ctx: &ConnectorContext) -> Result<Option<String>> {
        // Email connectors rely on app passwords or OAuth handled externally
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

        let smtp = self.smtp_host();
        let imap = self.imap_host();

        let state = if smtp.is_some() && imap.is_some() {
            ConnectorState::ready(self.name())
        } else {
            ConnectorState::degraded(
                self.name(),
                "Missing SMTP/IMAP host configuration for email connector",
            )
        };

        cache.store(&state)?;
        Ok(state)
    }
}
