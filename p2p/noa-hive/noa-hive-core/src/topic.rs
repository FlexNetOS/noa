//! Topic definitions for GossipSub.

use crate::PROTOCOL_PREFIX;

/// A GossipSub topic for NOA-Hive.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    name: String,
}

impl Topic {
    /// Create a new topic with the given name.
    /// The name will be prefixed with the protocol prefix.
    pub fn new(name: &str) -> Self {
        Self {
            name: format!("{}/{}", PROTOCOL_PREFIX, name),
        }
    }

    /// Create a topic from a raw name (no prefix added).
    pub fn raw(name: String) -> Self {
        Self { name }
    }

    /// Get the full topic name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Convert to libp2p topic hash.
    pub fn to_hash(&self) -> libp2p::gossipsub::TopicHash {
        libp2p::gossipsub::IdentTopic::new(&self.name).hash()
    }

    /// Convert to libp2p IdentTopic.
    pub fn to_ident(&self) -> libp2p::gossipsub::IdentTopic {
        libp2p::gossipsub::IdentTopic::new(&self.name)
    }
}

/// Well-known topics for NOA-Hive protocol.
pub mod topics {
    use super::Topic;

    /// Device presence announcements.
    pub fn presence() -> Topic {
        Topic::new("presence")
    }

    /// State CRDT operations.
    pub fn state_op() -> Topic {
        Topic::new("state/op")
    }

    /// Release manifest notifications.
    pub fn release_manifest() -> Topic {
        Topic::new("release/manifest")
    }

    /// Chat messages for a specific room.
    pub fn chat_room(room_id: &str) -> Topic {
        Topic::new(&format!("chat/{}", room_id))
    }

    /// Inference requests.
    pub fn inference_request() -> Topic {
        Topic::new("inference/request")
    }

    /// Inference responses.
    pub fn inference_response() -> Topic {
        Topic::new("inference/response")
    }
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
