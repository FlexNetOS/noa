//! Bluetooth/NFC proximity pairing (FR-104).
//!
//! Simple proximity proof using signal strength thresholds and short-lived nonces.

use crate::error::{NoaError, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD as B64, Engine};
use chrono::{DateTime, Duration, Utc};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityProof {
    pub device_id: String,
    pub nonce: String,
    pub signal_strength: i32,
    pub recorded_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

static PROOFS: OnceLock<Mutex<HashMap<String, ProximityProof>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<String, ProximityProof>> {
    PROOFS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Record a proximity proof with a random nonce and signal strength reading.
pub fn record_proximity(device_id: &str, signal_strength: i32) -> Result<ProximityProof> {
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 16];
    rng.fill(&mut nonce_bytes).map_err(|e| NoaError::Internal {
        message: "Failed to generate proximity nonce".to_string(),
        source: Some(Box::new(e)),
    })?;
    let nonce = B64.encode(nonce_bytes);

    let proof = ProximityProof {
        device_id: device_id.to_string(),
        nonce: nonce.clone(),
        signal_strength,
        recorded_at: Utc::now(),
        expires_at: Utc::now() + Duration::minutes(2),
    };

    registry()
        .lock()
        .expect("proximity registry poisoned")
        .insert(nonce.clone(), proof.clone());

    Ok(proof)
}

/// Validate that a proximity proof meets the minimum signal strength and is fresh.
pub fn validate_proximity(nonce: &str, minimum_signal: i32) -> bool {
    let mut guard = registry().lock().expect("proximity registry poisoned");
    if let Some(proof) = guard.get(nonce) {
        if proof.signal_strength >= minimum_signal && proof.expires_at > Utc::now() {
            return true;
        }
    }
    guard.remove(nonce);
    false
}
