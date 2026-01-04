//! Error types for NOA-Hive.

use thiserror::Error;

/// NOA-Hive error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Network-related errors.
    #[error("Network error: {0}")]
    Network(String),

    /// Serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// configsuration errors.
    #[error("configsuration error: {0}")]
    configs(String),

    /// Peer not found.
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Topic error.
    #[error("Topic error: {0}")]
    Topic(String),

    /// DHT error.
    #[error("DHT error: {0}")]
    Dht(String),

    /// CRDT state error.
    #[error("State sync error: {0}")]
    StateSync(String),

    /// gRPC error.
    #[error("gRPC error: {0}")]
    Grpc(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error.
    #[error("{0}")]
    Other(String),
}

/// Result type alias for NOA-Hive operations.
pub type Result<T> = std::result::Result<T, Error>;
