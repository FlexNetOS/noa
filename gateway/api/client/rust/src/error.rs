//! Error types for the API client.

use thiserror::Error;

/// API client errors.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API error response.
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;
