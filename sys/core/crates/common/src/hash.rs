//! Hashing utilities for NOA

use blake3::Hasher as Blake3Hasher;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hash of data
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Compute BLAKE3 hash of data (faster than SHA-256)
pub fn blake3(data: &[u8]) -> String {
    let hash = Blake3Hasher::new().update(data).finalize();
    hash.to_hex().to_string()
}

/// Compute checksum for entity data
pub fn checksum(data: &[u8]) -> String {
    blake3(data)
}

/// Verify checksum matches data
pub fn verify_checksum(data: &[u8], expected: &str) -> bool {
    checksum(data) == expected
}

// Note: hex crate is needed - add to Cargo.toml

