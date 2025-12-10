//! Encrypted file transfer pairing (FR-105).
//!
//! Issues short-lived transfer tickets with content checksums to prevent tampering.

use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransferTicket {
    pub transfer_id: String,
    pub device_id: String,
    pub file_name: String,
    pub checksum: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

static TICKETS: OnceLock<Mutex<HashMap<String, FileTransferTicket>>> = OnceLock::new();

fn registry() -> &'static Mutex<HashMap<String, FileTransferTicket>> {
    TICKETS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Issue a ticket for an encrypted file transfer with checksum validation.
pub fn issue_ticket(device_id: &str, file_name: &str, contents: &[u8]) -> Result<FileTransferTicket> {
    let mut hasher = Sha256::new();
    hasher.update(contents);
    let checksum = format!("{:x}", hasher.finalize());

    let ticket = FileTransferTicket {
        transfer_id: Uuid::new_v4().to_string(),
        device_id: device_id.to_string(),
        file_name: file_name.to_string(),
        checksum,
        issued_at: Utc::now(),
        expires_at: Utc::now() + Duration::minutes(10),
    };

    registry()
        .lock()
        .expect("transfer registry poisoned")
        .insert(ticket.transfer_id.clone(), ticket.clone());

    Ok(ticket)
}

/// Validate a transfer ticket and checksum.
pub fn validate_ticket(transfer_id: &str, checksum: &str) -> bool {
    let mut guard = registry().lock().expect("transfer registry poisoned");
    if let Some(ticket) = guard.get(transfer_id) {
        if ticket.expires_at > Utc::now() && ticket.checksum == checksum {
            return true;
        }
    }
    guard.remove(transfer_id);
    false
}
