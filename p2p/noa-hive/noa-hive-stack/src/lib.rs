//! NOA-Hive P2P Stack
//!
//! The network stack built on libp2p providing:
//! - GossipSub for pub/sub messaging
//! - Kademlia DHT for distributed key-value storage
//! - Request-Response for direct peer communication
//!
//! Derived from hyveos p2p-stack with mesh-hardware dependencies removed.

pub mod behaviour;
pub mod swarm;
pub mod pubsub;
pub mod dht;
pub mod request_response;

pub use swarm::{HiveSwarm, SwarmConfig, SwarmEvent};
pub use pubsub::PubSub;
pub use dht::Dht;
pub use request_response::RequestResponse;

use libp2p::identity::Keypair;
use noa_hive_core::PeerId;

/// Generate a new Ed25519 keypair and derive the peer ID.
pub fn generate_identity() -> (Keypair, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from_libp2p(libp2p::PeerId::from(keypair.public()));
    (keypair, peer_id)
}

/// Load or generate an identity from a file.
pub fn load_or_generate_identity(path: &std::path::Path) -> anyhow::Result<(Keypair, PeerId)> {
    if path.exists() {
        let bytes = std::fs::read(path)?;
        let keypair = Keypair::from_protobuf_encoding(&bytes)?;
        let peer_id = PeerId::from_libp2p(libp2p::PeerId::from(keypair.public()));
        Ok((keypair, peer_id))
    } else {
        let (keypair, peer_id) = generate_identity();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, keypair.to_protobuf_encoding()?)?;
        Ok((keypair, peer_id))
    }
}
