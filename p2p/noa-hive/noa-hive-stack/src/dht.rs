//! Kademlia DHT for distributed key-value storage.

use libp2p::kad::{self, RecordKey};
use std::time::Duration;

/// DHT interface for NOA-Hive.
pub struct Dht {
    /// Default record TTL.
    pub record_ttl: Duration,
}

impl Dht {
    /// Create a new DHT interface.
    pub fn new() -> Self {
        Self {
            record_ttl: Duration::from_secs(3600), // 1 hour default
        }
    }

    /// Create a record key for a given path.
    pub fn key(path: &str) -> RecordKey {
        RecordKey::new(&path.as_bytes().to_vec())
    }

    /// Create a key for device metadata.
    pub fn device_key(peer_id: &str) -> RecordKey {
        Self::key(&format!("/noa-hive/device/{}", peer_id))
    }

    /// Create a key for model location.
    pub fn model_key(model_hash: &str) -> RecordKey {
        Self::key(&format!("/noa-hive/model/{}", model_hash))
    }

    /// Create a key for state room membership.
    pub fn state_room_key(room_id: &str) -> RecordKey {
        Self::key(&format!("/noa-hive/state/{}", room_id))
    }
}

impl Default for Dht {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a DHT query.
#[derive(Debug, Clone)]
pub struct DhtRecord {
    /// The key that was queried.
    pub key: RecordKey,
    /// The value, if found.
    pub value: Option<Vec<u8>>,
    /// The peer that provided the record.
    pub publisher: Option<libp2p::PeerId>,
    /// When the record expires.
    pub expires: Option<std::time::Instant>,
}
