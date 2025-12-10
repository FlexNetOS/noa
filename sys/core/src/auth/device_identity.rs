//! Device identity management (FR-100)
//!
//! Generates and persists per-device Ed25519 keypairs with a stable fingerprint.

use crate::error::{NoaError, Result};
use crate::init::paths::NoaPaths;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::{DateTime, Utc};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// JSON representation of the device identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub device_id: String,
    pub public_key: String,
    pub private_key: String,
    pub created_at: DateTime<Utc>,
    pub fingerprint: String,
}

impl DeviceIdentity {
    /// Location of the identity document.
    fn identity_path(noa_root: &Path) -> PathBuf {
        NoaPaths::data_state(noa_root).join("device-identity.json")
    }

    /// Load an existing identity from disk.
    pub fn load(noa_root: &Path) -> Result<Self> {
        let path = Self::identity_path(noa_root);
        let content = fs::read_to_string(&path)?;
        let identity: DeviceIdentity = serde_json::from_str(&content)?;
        Ok(identity)
    }

    /// Create a new identity and persist it.
    pub fn generate(noa_root: &Path) -> Result<Self> {
        fs::create_dir_all(NoaPaths::data_state(noa_root))?;

        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).map_err(|e| NoaError::Internal {
            message: "Failed to generate Ed25519 keypair".to_string(),
            source: Some(Box::new(e)),
        })?;

        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).map_err(|e| {
            NoaError::Internal {
                message: "Generated invalid Ed25519 keypair".to_string(),
                source: Some(Box::new(e)),
            }
        })?;

        let public_key = keypair.public_key().as_ref().to_vec();
        let private_key = pkcs8_bytes.as_ref().to_vec();

        let fingerprint = blake3::hash(&public_key).to_hex().to_string();

        let identity = DeviceIdentity {
            device_id: Uuid::new_v4().to_string(),
            public_key: B64.encode(&public_key),
            private_key: B64.encode(&private_key),
            created_at: Utc::now(),
            fingerprint,
        };

        identity.save(noa_root)?;
        Ok(identity)
    }

    /// Load an identity if it exists, otherwise generate and persist one.
    pub fn load_or_generate(noa_root: &Path) -> Result<Self> {
        if Self::identity_path(noa_root).exists() {
            Self::load(noa_root)
        } else {
            Self::generate(noa_root)
        }
    }

    /// Persist the identity document to disk.
    pub fn save(&self, noa_root: &Path) -> Result<()> {
        fs::create_dir_all(NoaPaths::data_state(noa_root))?;
        let path = Self::identity_path(noa_root);
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Decode the public key bytes from base64.
    pub fn public_key_bytes(&self) -> Result<Vec<u8>> {
        B64.decode(self.public_key.as_bytes()).map_err(|e| NoaError::Internal {
            message: "Failed to decode public key".to_string(),
            source: Some(Box::new(e)),
        })
    }
}
