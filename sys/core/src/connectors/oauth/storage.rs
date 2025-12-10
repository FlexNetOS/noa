use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Persisted OAuth token bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub provider: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Default)]
pub struct TokenStorage;

impl TokenStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&self, _token: &StoredToken) -> Result<()> {
        Ok(())
    }
}
