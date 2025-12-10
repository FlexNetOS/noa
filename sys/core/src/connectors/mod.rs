//! Connector framework (US10)
//!
//! Provides base traits and connector implementations for external services
//! such as GitHub, Gmail/Google, OpenAI, Claude, cloud storage, and email.

pub mod base;
pub mod oauth;
pub mod github;
pub mod google;
pub mod openai;
pub mod claude;
pub mod cloud_storage;
pub mod email;
pub mod cache;
pub mod network;
pub mod status;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

use self::base::ConnectorContext;
use self::claude::ClaudeConnector;
use self::cloud_storage::CloudStorageConnector;
use self::email::EmailConnector;
use self::github::GithubConnector;
use self::google::GoogleConnector;
use self::openai::OpenAIConnector;

/// High-level connector health categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectorHealth {
    Ready,
    Degraded,
    Offline,
    Unauthorized,
    Disabled,
}

/// Minimal connector state shared across implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorState {
    pub name: String,
    pub health: ConnectorHealth,
    pub last_checked: DateTime<Utc>,
    pub message: Option<String>,
}

impl ConnectorState {
    pub fn ready(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            health: ConnectorHealth::Ready,
            last_checked: Utc::now(),
            message: None,
        }
    }

    pub fn degraded(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            health: ConnectorHealth::Degraded,
            last_checked: Utc::now(),
            message: Some(message.into()),
        }
    }

    pub fn offline(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            health: ConnectorHealth::Offline,
            last_checked: Utc::now(),
            message: Some(message.into()),
        }
    }

    pub fn unauthorized(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            health: ConnectorHealth::Unauthorized,
            last_checked: Utc::now(),
            message: Some(message.into()),
        }
    }

    pub fn disabled(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            health: ConnectorHealth::Disabled,
            last_checked: Utc::now(),
            message: Some("Connector disabled via feature flag".to_string()),
        }
    }
}

/// Build default connector states based on feature flags and environment configuration
pub async fn collect_states(ctx: &ConnectorContext) -> Result<Vec<ConnectorState>> {
    // Master flag check: if disabled, mark all known connectors as disabled
    if !ctx.is_enabled("connectors.enabled") {
        return Ok(default_connector_ids()
            .into_iter()
            .map(ConnectorState::disabled)
            .collect());
    }

    let mut states: Vec<ConnectorState> = Vec::new();

    // GitHub (requires client id/secret)
    states.push(match github_from_env() {
        Some(conn) => conn.status(ctx).await?,
        None => ConnectorState::degraded(
            "github",
            "Missing GITHUB_CLIENT_ID/GITHUB_CLIENT_SECRET for OAuth",
        ),
    });

    // Google (requires client id/secret)
    states.push(match google_from_env() {
        Some(conn) => conn.status(ctx).await?,
        None => ConnectorState::degraded(
            "google",
            "Missing GOOGLE_CLIENT_ID/GOOGLE_CLIENT_SECRET for OAuth",
        ),
    });

    // OpenAI (API key)
    let openai = OpenAIConnector::new(None);
    states.push(openai.status(ctx).await?);

    // Claude (API key)
    let claude = ClaudeConnector::new(None);
    states.push(claude.status(ctx).await?);

    // Cloud storage (S3/GCS/MinIO)
    let cloud = CloudStorageConnector::new();
    states.push(cloud.status(ctx).await?);

    // Email (SMTP/IMAP)
    let email = EmailConnector::new();
    states.push(email.status(ctx).await?);

    Ok(states)
}

/// Known connector identifiers used for defaults
pub fn default_connector_ids() -> Vec<&'static str> {
    vec!["github", "google", "openai", "claude", "cloud_storage", "email"]
}

fn github_from_env() -> Option<GithubConnector> {
    let client_id = std::env::var("GITHUB_CLIENT_ID").ok()?;
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET").ok()?;
    let redirect_uri = std::env::var("GITHUB_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:3000/api/oauth/github/callback".to_string());

    let cfg = oauth::client::OAuthClientConfig {
        provider: "github".to_string(),
        auth_url: "https://github.com/login/oauth/authorize".to_string(),
        token_url: "https://github.com/login/oauth/access_token".to_string(),
        client_id,
        client_secret,
        redirect_uri,
        scopes: vec!["repo".into(), "read:org".into(), "workflow".into()],
    };

    GithubConnector::new(cfg).ok()
}

fn google_from_env() -> Option<GoogleConnector> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").ok()?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").ok()?;
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:3000/api/oauth/google/callback".to_string());

    let cfg = oauth::client::OAuthClientConfig {
        provider: "google".to_string(),
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        token_url: "https://oauth2.googleapis.com/token".to_string(),
        client_id,
        client_secret,
        redirect_uri,
        scopes: vec![
            "openid".into(),
            "email".into(),
            "https://www.googleapis.com/auth/gmail.readonly".into(),
        ],
    };

    GoogleConnector::new(cfg).ok()
}
