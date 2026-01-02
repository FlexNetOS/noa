//! GossipSub pub/sub messaging for NOA-Hive.

use libp2p::gossipsub::{self, TopicHash};
use noa_hive_core::topic::Topic;
use std::collections::HashSet;

/// Pub/Sub interface for NOA-Hive.
pub struct PubSub {
    subscribed_topics: HashSet<TopicHash>,
}

impl PubSub {
    /// Create a new PubSub interface.
    pub fn new() -> Self {
        Self {
            subscribed_topics: HashSet::new(),
        }
    }

    /// Subscribe to a topic.
    pub fn subscribe(&mut self, topic: &Topic) -> bool {
        self.subscribed_topics.insert(topic.to_hash())
    }

    /// Unsubscribe from a topic.
    pub fn unsubscribe(&mut self, topic: &Topic) -> bool {
        self.subscribed_topics.remove(&topic.to_hash())
    }

    /// Check if subscribed to a topic.
    pub fn is_subscribed(&self, topic: &Topic) -> bool {
        self.subscribed_topics.contains(&topic.to_hash())
    }

    /// Get all subscribed topics.
    pub fn subscriptions(&self) -> impl Iterator<Item = &TopicHash> {
        self.subscribed_topics.iter()
    }
}

impl Default for PubSub {
    fn default() -> Self {
        Self::new()
    }
}

/// Message received from GossipSub.
#[derive(Debug, Clone)]
pub struct PubSubMessage {
    /// The topic this message was published to.
    pub topic: String,
    /// The message data.
    pub data: Vec<u8>,
    /// The peer that sent the message (if known).
    pub source: Option<libp2p::PeerId>,
    /// Message ID.
    pub message_id: gossipsub::MessageId,
}
