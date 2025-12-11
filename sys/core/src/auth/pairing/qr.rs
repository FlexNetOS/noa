//! QR code pairing flow (FR-102) with 5-minute expiry tokens.

use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrPairingToken {
    pub token: String,
    pub device_id: String,
    pub expires_at: DateTime<Utc>,
}

static TOKENS: OnceLock<Mutex<HashMap<String, QrPairingToken>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<String, QrPairingToken>> {
    TOKENS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Issue a short-lived QR pairing token for a device.
pub fn issue_token(device_id: &str) -> Result<QrPairingToken> {
    let token = QrPairingToken {
        token: Uuid::new_v4().to_string(),
        device_id: device_id.to_string(),
        expires_at: Utc::now() + Duration::minutes(5),
    };

    registry()
        .lock()
        .expect("pairing registry poisoned")
        .insert(token.token.clone(), token.clone());

    Ok(token)
}

/// Validate a QR token; expired tokens are evicted.
pub fn validate_token(token: &str) -> bool {
    let mut guard = registry().lock().expect("pairing registry poisoned");
    if let Some(record) = guard.get(token) {
        if record.expires_at > Utc::now() {
            return true;
        }
    }
    guard.remove(token);
    false
}
