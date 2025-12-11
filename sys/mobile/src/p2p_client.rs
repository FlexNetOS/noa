use std::time::{Duration, Instant};

use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{info, warn};
use uuid::Uuid;

/// Configuration for the mobile P2P stub.
#[derive(Debug, Clone)]
pub struct MobileP2PClientConfig {
    pub device_name: String,
    pub relay_endpoint: String,
    pub heartbeat_interval: Duration,
}

impl Default for MobileP2PClientConfig {
    fn default() -> Self {
        Self {
            device_name: "mobile-companion".to_string(),
            relay_endpoint: "wss://relay.noa.invalid".to_string(),
            heartbeat_interval: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Error)]
pub enum MobileError {
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("not connected")]
    NotConnected,
}

#[derive(Debug)]
struct ClientState {
    connected: bool,
    last_heartbeat: Option<Instant>,
}

/// Minimal P2P client stub that can announce presence to the hive-mind.
pub struct MobileP2PClient {
    config: MobileP2PClientConfig,
    device_id: Uuid,
    state: Mutex<ClientState>,
}

impl MobileP2PClient {
    pub fn new(config: MobileP2PClientConfig) -> Self {
        Self {
            config,
            device_id: Uuid::new_v4(),
            state: Mutex::new(ClientState {
                connected: false,
                last_heartbeat: None,
            }),
        }
    }

    /// Connect to the relay and mark as online (stubbed handshake).
    pub async fn connect(&self) -> Result<(), MobileError> {
        let mut state = self.state.lock().await;
        // In a future iteration, perform real network handshake here.
        state.connected = true;
        state.last_heartbeat = Some(Instant::now());
        info!(
            device_id = %self.device_id,
            relay = %self.config.relay_endpoint,
            "mobile companion connected (stub)"
        );
        Ok(())
    }

    /// Disconnect from relay.
    pub async fn disconnect(&self) {
        let mut state = self.state.lock().await;
        state.connected = false;
        info!(device_id = %self.device_id, "mobile companion disconnected");
    }

    /// Send a lightweight heartbeat to keep the session alive.
    pub async fn heartbeat(&self) -> Result<(), MobileError> {
        let mut state = self.state.lock().await;
        if !state.connected {
            return Err(MobileError::NotConnected);
        }
        state.last_heartbeat = Some(Instant::now());
        info!(
            device_id = %self.device_id,
            relay = %self.config.relay_endpoint,
            "mobile heartbeat sent"
        );
        Ok(())
    }

    /// Check whether the client is considered online.
    pub async fn is_connected(&self) -> bool {
        self.state.lock().await.connected
    }

    /// Validate heartbeat recency; triggers a warning if overdue.
    pub async fn check_liveness(&self) -> Result<bool, MobileError> {
        let mut state = self.state.lock().await;
        if !state.connected {
            return Err(MobileError::NotConnected);
        }

        let overdue = match state.last_heartbeat {
            Some(last) => last.elapsed() > self.config.heartbeat_interval * 2,
            None => true,
        };

        if overdue {
            warn!(
                device_id = %self.device_id,
                "mobile heartbeat overdue; marking as disconnected"
            );
            state.connected = false;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn device_id(&self) -> Uuid {
        self.device_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connects_and_heartbeats() {
        let client = MobileP2PClient::new(MobileP2PClientConfig::default());
        client.connect().await.unwrap();
        assert!(client.is_connected().await);
        client.heartbeat().await.unwrap();
    }

    #[tokio::test]
    async fn detects_missing_connection() {
        let client = MobileP2PClient::new(MobileP2PClientConfig::default());
        let result = client.heartbeat().await;
        assert!(matches!(result, Err(MobileError::NotConnected)));
    }
}
