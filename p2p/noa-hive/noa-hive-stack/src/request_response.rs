//! Request-Response protocol for direct peer communication.

use libp2p::PeerId;
use std::time::Duration;

/// Request-Response interface for NOA-Hive.
pub struct RequestResponse {
    /// Request timeout.
    pub timeout: Duration,
}

impl RequestResponse {
    /// Create a new Request-Response interface.
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }

    /// Create with a custom timeout.
    pub fn with_timeout(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl Default for RequestResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// A pending request.
#[derive(Debug)]
pub struct PendingRequest {
    /// The request ID.
    pub id: u64,
    /// The target peer.
    pub peer: PeerId,
    /// The request method.
    pub method: String,
    /// When the request was sent.
    pub sent_at: std::time::Instant,
}

/// Protocol identifier for NOA-Hive request-response.
pub const PROTOCOL_ID: &str = "/noa-hive/req-resp/1.0.0";
