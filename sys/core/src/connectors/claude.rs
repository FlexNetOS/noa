// Claude API Connector
// Implements integration with Anthropic Claude API

use crate::connectors::{ConnectorHealth, ConnectorState};
use crate::error::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConnector {
    api_key: Option<String>,
    base_url: String,
}

impl ClaudeConnector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            api_key: None,
            base_url: "https://api.anthropic.com".to_string(),
        })
    }

    pub async fn connect(&mut self, api_key: String) -> Result<()> {
        self.api_key = Some(api_key);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.api_key = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.api_key.is_some()
    }

    pub fn state(&self) -> ConnectorState {
        if self.is_connected() {
            ConnectorState {
                name: "claude".to_string(),
                health: ConnectorHealth::Ready,
                last_checked: Utc::now(),
                message: None,
            }
        } else {
            ConnectorState {
                name: "claude".to_string(),
                health: ConnectorHealth::Offline,
                last_checked: Utc::now(),
                message: Some("Not connected".to_string()),
            }
        }
    }
}

impl Default for ClaudeConnector {
    fn default() -> Self {
        Self::new().expect("Failed to create ClaudeConnector")
    }
}
