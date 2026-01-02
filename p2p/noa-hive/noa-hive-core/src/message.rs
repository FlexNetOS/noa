//! Message types for NOA-Hive protocol.

use serde::{Deserialize, Serialize};
use crate::PeerId;

/// Envelope for all NOA-Hive messages.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    /// Protocol version.
    pub version: String,
    /// Sender peer ID.
    pub from: PeerId,
    /// Message timestamp (Unix millis).
    pub timestamp: u64,
    /// Message payload.
    pub payload: Payload,
}

impl Envelope {
    /// Create a new envelope with the current timestamp.
    pub fn new(from: PeerId, payload: Payload) -> Self {
        Self {
            version: crate::PROTOCOL_VERSION.to_string(),
            from,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            payload,
        }
    }
}

/// Message payload variants.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Payload {
    /// Presence announcement.
    Presence(PresenceMessage),
    /// State operation (CRDT).
    StateOp(StateOpMessage),
    /// Release manifest notification.
    Release(ReleaseMessage),
    /// Request message.
    Request(RequestMessage),
    /// Response message.
    Response(ResponseMessage),
}

/// Presence announcement for device discovery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PresenceMessage {
    /// Device name.
    pub name: Option<String>,
    /// Device capabilities.
    pub capabilities: Vec<String>,
    /// Whether the device is going offline.
    pub leaving: bool,
}

/// State operation for CRDT synchronization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateOpMessage {
    /// State room ID.
    pub room_id: String,
    /// loro document ID.
    pub doc_id: String,
    /// Encoded CRDT operation bytes.
    #[serde(with = "serde_bytes")]
    pub op_bytes: Vec<u8>,
    /// Operation sequence number.
    pub seq: u64,
}

/// Release manifest for binary distribution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseMessage {
    /// Release name/identifier.
    pub name: String,
    /// Release version.
    pub version: String,
    /// iroh blob hash.
    pub blob_hash: String,
    /// Total size in bytes.
    pub size: u64,
    /// Content type (e.g., "model/gguf", "binary/wasm").
    pub content_type: String,
}

/// Request message for request-response protocol.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestMessage {
    /// Request ID for correlation.
    pub request_id: String,
    /// Request method.
    pub method: String,
    /// Request payload as JSON.
    pub payload: serde_json::Value,
}

/// Response message for request-response protocol.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    /// Corresponding request ID.
    pub request_id: String,
    /// Whether the request succeeded.
    pub success: bool,
    /// Response payload or error message.
    pub payload: serde_json::Value,
}
