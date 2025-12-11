use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::{NoaError, Result};

/// Minimal token response used by placeholder OAuth flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub scope: Option<String>,
}

/// Stored OAuth token with integrity metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub provider: String,
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub scope: Option<String>,
    pub checksum: String,
}

impl StoredToken {
    pub fn from_response(provider: &str, response: TokenResponse) -> Self {
        let expires_at = response
            .expires_in
            .map(|seconds| Utc::now() + Duration::seconds(seconds as i64));

        let mut token = StoredToken {
            provider: provider.to_string(),
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
            scope: response.scope,
            checksum: String::new(),
        };
        token.checksum = checksum(&token);
        token
    }

    pub fn is_valid(&self) -> bool {
        self.checksum == checksum(self)
    }
}

fn checksum(token: &StoredToken) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.provider.as_bytes());
    hasher.update(token.access_token.as_bytes());
    if let Some(refresh) = &token.refresh_token {
        hasher.update(refresh.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Token storage abstraction
#[async_trait]
pub trait TokenStorage: Send + Sync {
    async fn save(&self, provider: &str, token: StoredToken) -> Result<()>;
    async fn load(&self, provider: &str) -> Result<Option<StoredToken>>;
}

/// File-backed token storage with integrity checks
pub struct FileTokenStorage {
    path: PathBuf,
    cache: std::sync::Mutex<HashMap<String, StoredToken>>,
}

impl FileTokenStorage {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(|| PathBuf::from("data/secrets/connector-tokens.json"));
        let cache = if path.exists() {
            let content = fs::read_to_string(&path).map_err(NoaError::from)?;
            serde_json::from_str::<HashMap<String, StoredToken>>(&content)
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self {
            path,
            cache: std::sync::Mutex::new(cache),
        })
    }

    fn persist(&self) -> Result<()> {
        let guard = self.cache.lock().map_err(|_| NoaError::Internal {
            message: "Token cache poisoned".to_string(),
            source: None,
        })?;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(NoaError::from)?;
        }

        let serialized = serde_json::to_string_pretty(&*guard)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&self.path, serialized).map_err(NoaError::from)
    }
}

#[async_trait]
impl TokenStorage for FileTokenStorage {
    async fn save(&self, provider: &str, token: StoredToken) -> Result<()> {
        let mut guard = self.cache.lock().map_err(|_| NoaError::Internal {
            message: "Token cache poisoned".to_string(),
            source: None,
        })?;

        guard.insert(provider.to_string(), token);
        drop(guard);
        self.persist()
    }

    async fn load(&self, provider: &str) -> Result<Option<StoredToken>> {
        let guard = self.cache.lock().map_err(|_| NoaError::Internal {
            message: "Token cache poisoned".to_string(),
            source: None,
        })?;

        if let Some(token) = guard.get(provider) {
            if token.is_valid() {
                return Ok(Some(token.clone()));
            }
        }

        Ok(None)
    }
}
