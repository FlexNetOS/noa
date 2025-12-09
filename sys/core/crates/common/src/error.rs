//! Error types for NOA

use thiserror::Error;

/// Result type alias for NOA operations
pub type Result<T> = std::result::Result<T, NoaError>;

/// Main error type for NOA
#[derive(Error, Debug)]
pub enum NoaError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("P2P error: {0}")]
    P2P(String),

    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("Serialization error: {source}")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl NoaError {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a new database error
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create a new agent error
    pub fn agent(msg: impl Into<String>) -> Self {
        Self::Agent(msg.into())
    }

    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}

