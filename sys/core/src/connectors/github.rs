use async_trait::async_trait;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::connectors::base::{Connector, ConnectorContext};
use crate::connectors::cache::ConnectorCache;
use crate::connectors::network;
use crate::connectors::oauth::{
    client::{OAuthClient, OAuthClientConfig},
    refresh,
    storage::{FileTokenStorage, StoredToken},
    token_exchange,
};
use crate::connectors::ConnectorState;
use crate::error::Result;

/// GitHub connector implementation (OAuth)
pub struct GithubConnector {
    client: OAuthClient,
    storage: FileTokenStorage,
}

impl GithubConnector {
    pub fn new(config: OAuthClientConfig) -> Result<Self> {
        Ok(Self {
            client: OAuthClient::new(config),
            storage: FileTokenStorage::new(None)?,
        })
    }

    pub async fn store_code(&self, code: &str) -> Result<()> {
        let tokens = token_exchange::exchange_code(&self.client, code).await?;
        let stored = StoredToken::from_response(self.name(), tokens);
        self.storage.save(self.name(), stored).await
    }

    async fn refresh_if_needed(&self) -> Result<()> {
        if let Some(token) = self.storage.load(self.name()).await? {
            if token
                .expires_at
                .map(|ts| ts <= Utc::now())
                .unwrap_or(false)
            {
                if let Some(refresh_token) = token.refresh_token {
                    let refreshed = refresh::refresh_token(&self.client, &refresh_token).await?;
                    let stored = StoredToken::from_response(self.name(), refreshed);
                    self.storage.save(self.name(), stored).await?;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Connector for GithubConnector {
    fn name(&self) -> &str {
        "github"
    }

    fn feature_flag(&self) -> &str {
        "connectors.github"
    }

    fn scopes(&self) -> Vec<String> {
        vec!["repo".into(), "read:org".into(), "workflow".into()]
    }

    async fn authorize_url(&self, ctx: &ConnectorContext) -> Result<Option<String>> {
        if !ctx.is_enabled(self.feature_flag()) {
            return Ok(None);
        }
        let state = format!("github-{}", Uuid::new_v4());
        Ok(Some(self.client.authorization_url(&state)?))
    }

    async fn refresh_credentials(&self, ctx: &ConnectorContext) -> Result<()> {
        if ctx.is_enabled(self.feature_flag()) {
            self.refresh_if_needed().await?;
        }
        Ok(())
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

        // Attempt refresh if needed; ignore errors to avoid blocking status
        let _ = self.refresh_if_needed().await;

        let state = if self.storage.load(self.name()).await?.is_some() {
            ConnectorState::ready(self.name())
        } else {
            ConnectorState::degraded(self.name(), "Awaiting GitHub authorization")
        };
        cache.store(&state)?;
        Ok(state)
    }
}
