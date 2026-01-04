//! Swarm management for NOA-Hive.

use libp2p::{
    gossipsub, identify, kad, ping, request_response,
    swarm::NetworkBehaviour,
    Multiaddr, PeerId as LibP2PPeerId,
};
use std::time::Duration;

/// configsuration for the NOA-Hive swarm.
#[derive(Clone, Debug)]
pub struct Swarmconfigs {
    /// Addresses to listen on.
    pub listen_addrs: Vec<Multiaddr>,
    /// Bootstrap peers for initial connection.
    pub bootstrap_peers: Vec<(LibP2PPeerId, Multiaddr)>,
    /// Enable mDNS for local discovery.
    pub enable_mdns: bool,
    /// Enable relay for NAT traversal.
    pub enable_relay: bool,
    /// Idle connection timeout.
    pub idle_timeout: Duration,
}

impl Default for Swarmconfigs {
    fn default() -> Self {
        Self {
            listen_addrs: vec![
                "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
                "/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap(),
            ],
            bootstrap_peers: vec![],
            enable_mdns: true,
            enable_relay: true,
            idle_timeout: Duration::from_secs(60),
        }
    }
}

/// Combined network behaviour for NOA-Hive.
#[derive(NetworkBehaviour)]
pub struct HiveBehaviour {
    /// GossipSub for pub/sub messaging.
    pub gossipsub: gossipsub::Behaviour,
    /// Kademlia DHT for distributed storage.
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    /// Request-Response for direct communication.
    pub request_response: request_response::cbor::Behaviour<HiveRequest, HiveResponse>,
    /// Identify for peer info exchange.
    pub identify: identify::Behaviour,
    /// Ping for connection liveness.
    pub ping: ping::Behaviour,
}

/// Request type for request-response protocol.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HiveRequest {
    pub method: String,
    pub payload: Vec<u8>,
}

/// Response type for request-response protocol.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HiveResponse {
    pub success: bool,
    pub payload: Vec<u8>,
}

/// Wrapper around the libp2p Swarm with NOA-Hive behaviour.
pub struct HiveSwarm {
    // The actual swarm would be here, but we just define the interface
    configs: Swarmconfigs,
}

impl HiveSwarm {
    /// Create a new swarm with the given configsuration.
    pub fn new(configs: Swarmconfigs) -> Self {
        Self { configs }
    }

    /// Get the swarm configsuration.
    pub fn configs(&self) -> &Swarmconfigs {
        &self.configs
    }
}

/// Events emitted by the swarm.
#[derive(Debug)]
pub enum SwarmEvent {
    /// A new peer connected.
    PeerConnected(LibP2PPeerId),
    /// A peer disconnected.
    PeerDisconnected(LibP2PPeerId),
    /// Received a GossipSub message.
    GossipMessage {
        topic: String,
        data: Vec<u8>,
        source: LibP2PPeerId,
    },
    /// DHT query completed.
    DhtQueryComplete {
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    },
    /// Received a request.
    Request {
        peer: LibP2PPeerId,
        request: HiveRequest,
    },
}
