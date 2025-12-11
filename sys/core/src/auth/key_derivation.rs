//! Argon2id key derivation for device key encryption (FR-101).

use crate::error::{NoaError, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use ring::rand::{SecureRandom, SystemRandom};

/// Derived key output in PHC string format plus the salt used.
#[derive(Debug, Clone)]
pub struct DerivedKey {
    pub hash: String,
    pub salt: String,
}

/// Derive an Argon2id hash for the provided secret.
pub fn derive_key(secret: &str, salt: Option<&str>) -> Result<DerivedKey> {
    let salt_string = match salt {
        Some(existing) => SaltString::new(existing).map_err(|e| NoaError::Internal {
            message: "Invalid salt provided for key derivation".to_string(),
            source: Some(Box::new(e)),
        })?,
        None => generate_salt()?,
    };

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(secret.as_bytes(), &salt_string)
        .map_err(|e| NoaError::Internal {
            message: "Failed to derive Argon2id hash".to_string(),
            source: Some(Box::new(e)),
        })?
        .to_string();

    Ok(DerivedKey {
        hash: hash.clone(),
        salt: salt_string.to_string(),
    })
}

/// Verify a secret against a stored Argon2id hash string.
pub fn verify_key(secret: &str, encoded_hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(encoded_hash).map_err(|e| NoaError::Internal {
        message: "Invalid stored password hash".to_string(),
        source: Some(Box::new(e)),
    })?;

    Ok(Argon2::default()
        .verify_password(secret.as_bytes(), &parsed)
        .is_ok())
}

fn generate_salt() -> Result<SaltString> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt).map_err(|e| NoaError::Internal {
        message: "Failed to generate random salt".to_string(),
        source: Some(Box::new(e)),
    })?;

    SaltString::encode_b64(&salt).map_err(|e| NoaError::Internal {
        message: "Failed to encode salt".to_string(),
        source: Some(Box::new(e)),
    })
}
