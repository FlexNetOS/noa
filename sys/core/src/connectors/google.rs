// Google API Connector
// Implements integration with Google services (Gmail, Drive, etc.)

use crate::error::Result;
use crate::connectors::{ConnectorState, ConnectorHealth};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConnector {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl GoogleConnector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            access_token: None,
            refresh_token: None,
        })
    }

    pub async fn connect(&mut self, access_token: String, refresh_token: String) -> Result<()> {
        self.access_token = Some(access_token);
        self.refresh_token = Some(refresh_token);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.access_token = None;
        self.refresh_token = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn state(&self) -> ConnectorState {
        if self.is_connected() {
            ConnectorState {
                name: "google".to_string(),
                health: ConnectorHealth::Ready,
                last_checked: Utc::now(),
                message: None,
            }
        } else {
            ConnectorState {
                name: "google".to_string(),
                health: ConnectorHealth::Offline,
                last_checked: Utc::now(),
                message: Some("Not connected".to_string()),
            }
        }
    }
}

impl Default for GoogleConnector {
    fn default() -> Self {
        Self::new().expect("Failed to create GoogleConnector")
    }
}

