//! 6-digit PIN pairing flow (FR-103).

use crate::error::{NoaError, Result};
use chrono::{DateTime, Duration, Utc};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinPairing {
    pub code: String,
    pub device_id: String,
    pub expires_at: DateTime<Utc>,
}

static PINS: OnceLock<Mutex<HashMap<String, PinPairing>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<String, PinPairing>> {
    PINS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Issue a six-digit PIN that expires after five minutes.
pub fn issue_pin(device_id: &str) -> Result<PinPairing> {
    let rng = SystemRandom::new();
    let mut bytes = [0u8; 4];
    rng.fill(&mut bytes).map_err(|e| NoaError::Internal {
        message: "Failed to generate secure PIN".to_string(),
        source: Some(Box::new(e)),
    })?;
    let value = u32::from_le_bytes(bytes) % 1_000_000;
    let code = format!("{:06}", value);

    let record = PinPairing {
        code: code.clone(),
        device_id: device_id.to_string(),
        expires_at: Utc::now() + Duration::minutes(5),
    };

    registry()
        .lock()
        .expect("pin registry poisoned")
        .insert(code.clone(), record.clone());

    Ok(record)
}

/// Validate a PIN code and evict expired entries.
pub fn validate_pin(code: &str) -> bool {
    let mut guard = registry().lock().expect("pin registry poisoned");
    if let Some(record) = guard.get(code) {
        if record.expires_at > Utc::now() {
            return true;
        }
    }
    guard.remove(code);
    false
}
