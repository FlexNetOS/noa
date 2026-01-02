//! Peer identity types.

use libp2p::identity::Keypair;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A peer identifier in the NOA-Hive network.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(libp2p::PeerId);

impl PeerId {
    /// Create a new random peer ID.
    pub fn random() -> Self {
        let keypair = Keypair::generate_ed25519();
        Self(libp2p::PeerId::from(keypair.public()))
    }

    /// Create from a libp2p PeerId.
    pub fn from_libp2p(peer_id: libp2p::PeerId) -> Self {
        Self(peer_id)
    }

    /// Get the underlying libp2p PeerId.
    pub fn to_libp2p(&self) -> libp2p::PeerId {
        self.0
    }

    /// Convert to base58 string representation.
    pub fn to_base58(&self) -> String {
        self.0.to_base58()
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PeerId({})", self.0)
    }
}

impl FromStr for PeerId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        libp2p::PeerId::from_str(s)
            .map(PeerId)
            .map_err(|e| crate::Error::Other(format!("Invalid peer ID: {}", e)))
    }
}

impl From<libp2p::PeerId> for PeerId {
    fn from(peer_id: libp2p::PeerId) -> Self {
        Self(peer_id)
    }
}

impl From<PeerId> for libp2p::PeerId {
    fn from(peer_id: PeerId) -> Self {
        peer_id.0
    }
}

/// Device information associated with a peer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// The peer's unique identifier.
    pub peer_id: PeerId,
    /// Human-readable device name.
    pub name: Option<String>,
    /// Device platform (e.g., "windows", "linux", "macos").
    pub platform: String,
    /// NOA version running on the device.
    pub noa_version: String,
    /// Capabilities offered by this device.
    pub capabilities: Vec<String>,
    /// When this device was last seen (Unix timestamp).
    pub last_seen: u64,
}

impl DeviceInfo {
    /// Create new device info for the local device.
    pub fn local(peer_id: PeerId, name: Option<String>) -> Self {
        Self {
            peer_id,
            name,
            platform: std::env::consts::OS.to_string(),
            noa_version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                "chat".to_string(),
                "inference".to_string(),
                "state-sync".to_string(),
            ],
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}
