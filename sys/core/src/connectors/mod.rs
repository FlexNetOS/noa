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

    // Use lightweight placeholder connectors; real integrations can extend Connector trait.
    states.push(GithubConnector::default().state());
    states.push(GoogleConnector::default().state());
    states.push(OpenAIConnector::default().state());
    states.push(ClaudeConnector::default().state());
    states.push(CloudStorageConnector::default().state());
    states.push(EmailConnector::default().state());

    Ok(states)
}

/// Known connector identifiers used for defaults
pub fn default_connector_ids() -> Vec<&'static str> {
    vec!["github", "google", "openai", "claude", "cloud_storage", "email"]
}
