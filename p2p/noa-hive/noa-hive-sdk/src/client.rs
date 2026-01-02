//! Client for connecting to the NOA-Hive daemon.

use noa_hive_core::PeerId;
use crate::{pubsub::PubSubClient, dht::DhtClient, state::StateClient};

/// Client for the NOA-Hive daemon.
pub struct Client {
    endpoint: String,
    // In a real implementation, this would hold the gRPC channel
}

impl Client {
    /// Connect to the daemon at the given endpoint.
    pub async fn connect(endpoint: &str) -> anyhow::Result<Self> {
        tracing::info!(endpoint = %endpoint, "Connecting to NOA-Hive daemon");
        
        // In a real implementation, we would establish the gRPC connection here
        Ok(Self {
            endpoint: endpoint.to_string(),
        })
    }

    /// Connect to the daemon at the default endpoint.
    pub async fn connect_default() -> anyhow::Result<Self> {
        Self::connect(crate::DEFAULT_ENDPOINT).await
    }

    /// Get the local peer ID.
    pub async fn whoami(&self) -> anyhow::Result<PeerId> {
        // In a real implementation, this would call the gRPC service
        Ok(PeerId::random())
    }

    /// Get the pub/sub client.
    pub fn pubsub(&self) -> PubSubClient {
        PubSubClient::new(&self.endpoint)
    }

    /// Get the DHT client.
    pub fn dht(&self) -> DhtClient {
        DhtClient::new(&self.endpoint)
    }

    /// Get the state synchronization client.
    pub fn state(&self) -> StateClient {
        StateClient::new(&self.endpoint)
    }
}
