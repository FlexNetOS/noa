//! Connector framework (US10)
//!
//! Provides base traits and connector implementations for external services
//! such as GitHub, Gmail/Google, OpenAI, Claude, cloud storage, and email.

pub mod base;
pub mod cache;
pub mod claude;
pub mod cloud_storage;
pub mod email;
pub mod github;
pub mod google;
pub mod network;
pub mod openai;
pub mod status;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

use self::base::{Connector, ConnectorContext};
use self::claude::ClaudeConnector;
use self::cloud_storage::{CloudStorageConnector, CloudStorageProvider};
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
        return Ok(default_connector_ids().into_iter().map(ConnectorState::disabled).collect());
    }

    let mut states = Vec::new();

    // GitHub connector
    states.push(GithubConnector::new().state());

    // Google connector
    if let Ok(c) = GoogleConnector::new() {
        states.push(c.state());
    } else {
        states.push(ConnectorState::disabled("google"));
    }

    // OpenAI connector
    if let Ok(c) = OpenAIConnector::new() {
        states.push(c.state());
    } else {
        states.push(ConnectorState::disabled("openai"));
    }

    // Claude connector
    if let Ok(c) = ClaudeConnector::new() {
        states.push(c.state());
    } else {
        states.push(ConnectorState::disabled("claude"));
    }

    // Cloud storage connector
    if let Ok(c) = CloudStorageConnector::new(CloudStorageProvider::S3) {
        states.push(c.state());
    } else {
        states.push(ConnectorState::disabled("cloud_storage"));
    }

    // Email connector
    states.push(EmailConnector::new().state());

    Ok(states)
}

/// Known connector identifiers used for defaults
pub fn default_connector_ids() -> Vec<&'static str> {
    vec![
        "github",
        "google",
        "openai",
        "claude",
        "cloud_storage",
        "email",
    ]
}
