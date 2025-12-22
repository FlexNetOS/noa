use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRecord {
    pub app: String,
    pub token_type: String,
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    pub stored_at: u64,
    pub checksum: String,
}

#[derive(Clone)]
pub struct Vault {
    path: PathBuf,
}

impl Vault {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load_all(&self) -> Result<Vec<TokenRecord>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(&self.path)?;
        let records: Vec<TokenRecord> = serde_json::from_str(&data)?;
        Ok(records)
    }

    pub fn write_all(&self, records: &[TokenRecord]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = serde_json::to_string_pretty(records)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }

    pub fn store(&self, app: &str, token_type: &str, access_token: &str, refresh_token: Option<String>, expires_at: Option<u64>) -> Result<()> {
        let mut records = self.load_all()?;
        let checksum = hash_token(access_token);
        let record = TokenRecord {
            app: app.to_string(),
            token_type: token_type.to_string(),
            access_token: access_token.to_string(),
            refresh_token,
            expires_at,
            stored_at: now(),
            checksum,
        };

        records.retain(|r| !(r.app == app && r.token_type == token_type));
        records.push(record);
        self.write_all(&records)
    }

    pub fn get(&self, app: &str, token_type: &str) -> Result<Option<TokenRecord>> {
        let records = self.load_all()?;
        Ok(records.into_iter().find(|r| r.app == app && r.token_type == token_type))
    }
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
