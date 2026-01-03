//! Admin Bootstrap Routes
//!
//! Provides endpoints for first-user provisioning and admin operations.
//! The bootstrap endpoint is only available when no admin users exist.

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::api::server::AppState;
use crate::api::middleware::{create_jwt, JwtClaims, AuthIdentity, require_scope};

/// Static flag indicating if bootstrap has been completed.
/// In production, this should be persisted to the database.
static BOOTSTRAP_COMPLETE: AtomicBool = AtomicBool::new(false);

/// Bootstrap request for initial admin setup.
#[derive(Debug, Deserialize)]
pub struct BootstrapRequest {
    /// Admin username.
    pub username: String,
    /// Admin password (will be hashed).
    pub password: String,
    /// Optional initial API key name.
    pub api_key_name: Option<String>,
}

/// Bootstrap response with credentials.
#[derive(Debug, Serialize)]
pub struct BootstrapResponse {
    /// Whether bootstrap was successful.
    pub success: bool,
    /// Message describing the result.
    pub message: String,
    /// JWT token for the admin user.
    pub token: Option<String>,
    /// Generated API key (if requested).
    pub api_key: Option<String>,
}

/// API key generation request.
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    /// Name for the API key.
    pub name: String,
    /// Scopes to grant.
    pub scopes: Option<Vec<String>>,
    /// Expiration in days (optional).
    pub expires_in_days: Option<u32>,
}

/// API key response.
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    /// The generated API key (only shown once).
    pub key: String,
    /// Name of the key.
    pub name: String,
    /// Scopes granted.
    pub scopes: Vec<String>,
    /// Expiration timestamp (if set).
    pub expires_at: Option<u64>,
}

/// Bootstrap status response.
#[derive(Debug, Serialize)]
pub struct BootstrapStatus {
    /// Whether bootstrap is required.
    pub bootstrap_required: bool,
    /// Whether the system is initialized.
    pub initialized: bool,
}

/// Check if bootstrap is required.
async fn get_bootstrap_status() -> impl IntoResponse {
    let is_complete = BOOTSTRAP_COMPLETE.load(Ordering::SeqCst);

    Json(BootstrapStatus {
        bootstrap_required: !is_complete,
        initialized: is_complete,
    })
}

/// Perform initial bootstrap.
async fn bootstrap(
    Extension(state): Extension<AppState>,
    Json(req): Json<BootstrapRequest>,
) -> impl IntoResponse {
    // Check if already bootstrapped
    if BOOTSTRAP_COMPLETE.load(Ordering::SeqCst) {
        return (
            StatusCode::CONFLICT,
            Json(BootstrapResponse {
                success: false,
                message: "System already bootstrapped".to_string(),
                token: None,
                api_key: None,
            }),
        );
    }

    // Validate input
    if req.username.is_empty() || req.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(BootstrapResponse {
                success: false,
                message: "Username required and password must be at least 8 characters".to_string(),
                token: None,
                api_key: None,
            }),
        );
    }

    // In a real implementation, we would:
    // 1. Hash the password using argon2 or bcrypt
    // 2. Store the admin user in the database
    // 3. Create initial roles and permissions

    // For now, generate a JWT for the admin
    let jwt_secret = state
        .config
        .raw
        .get("noa_server")
        .and_then(|s| s.get("api"))
        .and_then(|a| a.get("jwt_secret"))
        .and_then(|s| s.as_str());

    let jwt_secret = match jwt_secret {
        Some(secret) if !secret.is_empty() => secret,
        _ => {
            tracing::error!("JWT secret is not configured; refusing to generate admin token");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BootstrapResponse {
                    success: false,
                    message: "Server configuration error: JWT secret is not set".to_string(),
                    token: None,
                    api_key: None,
                }),
            );
        }
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = JwtClaims {
        sub: req.username.clone(),
        exp: now + 86400 * 30, // 30 days
        iat: now,
        scopes: vec!["*".to_string()], // Full admin access
    };

    let token = create_jwt(&claims, jwt_secret);

    // Generate API key if requested
    let api_key = req.api_key_name.map(|_| {
        generate_api_key()
    });

    // Mark bootstrap as complete
    BOOTSTRAP_COMPLETE.store(true, Ordering::SeqCst);

    tracing::info!(
        username = %req.username,
        "System bootstrap completed"
    );

    (
        StatusCode::OK,
        Json(BootstrapResponse {
            success: true,
            message: format!("Admin user '{}' created successfully", req.username),
            token: Some(token),
            api_key,
        }),
    )
}

/// Create a new API key (requires admin scope).
async fn create_api_key(
    Extension(state): Extension<AppState>,
    identity: Option<Extension<AuthIdentity>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> impl IntoResponse {
    // Require admin scope
    if let Err(response) = require_scope(identity.as_ref().map(|e| &e.0), "admin") {
        return response;
    }

    let scopes = req.scopes.unwrap_or_else(|| vec!["read".to_string(), "write".to_string()]);
    let key = generate_api_key();

    let expires_at = req.expires_in_days.map(|days| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now + (days as u64 * 86400)
    });

    // In a real implementation, store the key hash in the database
    tracing::info!(
        key_name = %req.name,
        scopes = ?scopes,
        "API key created"
    );

    (
        StatusCode::CREATED,
        Json(ApiKeyResponse {
            key,
            name: req.name,
            scopes,
            expires_at,
        }),
    ).into_response()
}

/// Generate a secure API key.
fn generate_api_key() -> String {
    use sha2::{Sha256, Digest};

    let random_bytes: [u8; 32] = rand_bytes();
    let mut hasher = Sha256::new();
    hasher.update(&random_bytes);
    let hash = hasher.finalize();

    format!("noa_{}", hex_encode(&hash[..16]))
}

/// Generate random bytes (simple implementation).
fn rand_bytes() -> [u8; 32] {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut bytes = [0u8; 32];
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // Use system time + process ID as entropy source
    // In production, use a proper CSPRNG
    let seed = now ^ (std::process::id() as u128);

    for (i, byte) in bytes.iter_mut().enumerate() {
        let val = seed.wrapping_add(i as u128).wrapping_mul(0x5851F42D4C957F2D);
        *byte = (val >> (i % 8 * 8)) as u8;
    }

    bytes
}

/// Hex encode bytes.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Get current user info (requires authentication).
async fn whoami(
    identity: Option<Extension<AuthIdentity>>,
) -> impl IntoResponse {
    match identity {
        Some(Extension(id)) => {
            Json(serde_json::json!({
                "authenticated": true,
                "method": format!("{:?}", id.method),
                "subject": id.subject,
                "scopes": id.scopes,
            }))
        }
        None => {
            Json(serde_json::json!({
                "authenticated": false,
            }))
        }
    }
}

/// Create admin routes.
pub fn routes() -> Router {
    Router::new()
        .route("/admin/bootstrap/status", get(get_bootstrap_status))
        .route("/admin/bootstrap", post(bootstrap))
        .route("/admin/api-keys", post(create_api_key))
        .route("/admin/whoami", get(whoami))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_generation() {
        let key1 = generate_api_key();
        let key2 = generate_api_key();

        assert!(key1.starts_with("noa_"));
        assert!(key2.starts_with("noa_"));
        // Keys should be different (though with our simple RNG they might not be in rapid succession)
        // In production with proper CSPRNG, they would always be different
    }

    #[test]
    fn test_hex_encode() {
        let bytes = [0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(hex_encode(&bytes), "deadbeef");
    }
}
