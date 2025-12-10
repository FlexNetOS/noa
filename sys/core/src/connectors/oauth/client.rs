use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::error::{NoaError, Result};

/// Basic OAuth2 client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientConfig {
    pub provider: String,
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Minimal OAuth2 client helper for connector implementations
#[derive(Clone)]
pub struct OAuthClient {
    config: OAuthClientConfig,
    http: reqwest::Client,
}

impl OAuthClient {
    pub fn new(config: OAuthClientConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    pub fn config(&self) -> &OAuthClientConfig {
        &self.config
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.http
    }

    /// Build the OAuth authorization URL with the supplied state
    pub fn authorization_url(&self, state: &str) -> Result<String> {
        let mut url =
            Url::parse(&self.config.auth_url).map_err(|e| NoaError::Internal {
                message: format!("Invalid auth URL {}: {}", self.config.auth_url, e),
                source: None,
            })?;

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("response_type", "code");
            query.append_pair("client_id", &self.config.client_id);
            query.append_pair("redirect_uri", &self.config.redirect_uri);
            query.append_pair("state", state);
            if !self.config.scopes.is_empty() {
                query.append_pair("scope", &self.config.scopes.join(" "));
            }
        }

        Ok(url.to_string())
    }
}
