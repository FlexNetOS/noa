//! Authentication Middleware
//!
//! Provides API key and JWT authentication for API endpoints.
//! Supports both header-based (`Authorization: Bearer <token>`) and
//! API key (`X-API-Key: <key>`) authentication.

use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

/// Authentication configsuration.
#[derive(Debug, Clone)]
pub struct Authconfigs {
    /// Whether authentication is enabled.
    pub enabled: bool,
    /// Whether to allow unauthenticated requests to health endpoints.
    pub allow_health_unauthenticated: bool,
    /// JWT secret for token verification (if using JWT).
    pub jwt_secret: Option<String>,
    /// Valid API keys (for simple API key auth).
    pub api_keys: Vec<ApiKeyEntry>,
}

impl Default for Authconfigs {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for development
            allow_health_unauthenticated: true,
            jwt_secret: None,
            api_keys: Vec::new(),
        }
    }
}

impl Authconfigs {
    /// Create configs for production with required JWT secret.
    pub fn production(jwt_secret: String) -> Self {
        Self {
            enabled: true,
            allow_health_unauthenticated: true,
            jwt_secret: Some(jwt_secret),
            api_keys: Vec::new(),
        }
    }

    /// Add an API key.
    pub fn with_api_key(mut self, key: ApiKeyEntry) -> Self {
        self.api_keys.push(key);
        self
    }
}

/// An API key entry with metadata.
#[derive(Debug, Clone)]
pub struct ApiKeyEntry {
    /// The API key value (should be securely generated).
    pub key: String,
    /// Human-readable name for the key.
    pub name: String,
    /// Scopes/permissions associated with this key.
    pub scopes: Vec<String>,
    /// Whether the key is active.
    pub active: bool,
}

impl ApiKeyEntry {
    pub fn new(key: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            name: name.into(),
            scopes: vec!["*".to_string()], // Full access by default
            active: true,
        }
    }

    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }
}

/// Authenticated identity extracted from request.
#[derive(Debug, Clone)]
pub struct AuthIdentity {
    /// Authentication method used.
    pub method: AuthMethod,
    /// Subject identifier (user ID, key name, etc.).
    pub subject: String,
    /// Scopes/permissions.
    pub scopes: Vec<String>,
}

impl AuthIdentity {
    /// Check if identity has a specific scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == "*" || s == scope)
    }
}

/// Authentication method.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// API key authentication.
    ApiKey,
    /// JWT bearer token.
    Jwt,
    /// No authentication (for unauthenticated endpoints).
    None,
}

/// Authentication state for the application.
pub struct AuthState {
    configs: Authconfigs,
}

impl AuthState {
    pub fn new(configs: Authconfigs) -> Self {
        Self { configs }
    }

    pub fn configs(&self) -> &Authconfigs {
        &self.configs
    }

    /// Validate an API key and return identity if valid.
    pub fn validate_api_key(&self, key: &str) -> Option<AuthIdentity> {
        self.configs
            .api_keys
            .iter()
            .find(|k| k.active && k.key == key)
            .map(|k| AuthIdentity {
                method: AuthMethod::ApiKey,
                subject: k.name.clone(),
                scopes: k.scopes.clone(),
            })
    }

    /// Validate a JWT token and return identity if valid.
    pub fn validate_jwt(&self, token: &str) -> Option<AuthIdentity> {
        let secret = self.configs.jwt_secret.as_ref()?;

        // Simple JWT validation (in production, use a proper JWT library)
        // For now, we'll do basic HMAC-SHA256 validation
        match decode_jwt(token, secret) {
            Ok(claims) => Some(AuthIdentity {
                method: AuthMethod::Jwt,
                subject: claims.sub,
                scopes: claims.scopes,
            }),
            Err(_) => None,
        }
    }
}

/// JWT claims structure.
#[derive(Debug, Clone)]
pub struct JwtClaims {
    /// Subject (user ID).
    pub sub: String,
    /// Expiration time (Unix timestamp).
    pub exp: u64,
    /// Issued at (Unix timestamp).
    pub iat: u64,
    /// Scopes/permissions.
    pub scopes: Vec<String>,
}

/// Decode and validate a JWT token.
fn decode_jwt(token: &str, secret: &str) -> Result<JwtClaims, AuthError> {
    // Split into header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AuthError::InvalidToken);
    }

    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    let signature_b64 = parts[2];

    // Verify signature using HMAC-SHA256
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let message = format!("{}.{}", header_b64, payload_b64);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| AuthError::InvalidToken)?;
    mac.update(message.as_bytes());

    let expected_sig = URL_SAFE_NO_PAD
        .decode(signature_b64)
        .map_err(|_| AuthError::InvalidToken)?;

    mac.verify_slice(&expected_sig)
        .map_err(|_| AuthError::InvalidSignature)?;

    // Decode payload
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| AuthError::InvalidToken)?;

    let payload: serde_json::Value =
        serde_json::from_slice(&payload_bytes).map_err(|_| AuthError::InvalidToken)?;

    // Extract claims
    let sub = payload
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or(AuthError::InvalidToken)?
        .to_string();

    let exp = payload
        .get("exp")
        .and_then(|v| v.as_u64())
        .ok_or(AuthError::InvalidToken)?;

    let iat = payload
        .get("iat")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let scopes = payload
        .get("scopes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(|| vec!["*".to_string()]);

    // Check expiration
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if now > exp {
        return Err(AuthError::TokenExpired);
    }

    Ok(JwtClaims {
        sub,
        exp,
        iat,
        scopes,
    })
}

/// Create a JWT token.
pub fn create_jwt(claims: &JwtClaims, secret: &str) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // Header
    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let header_b64 = URL_SAFE_NO_PAD.encode(header.as_bytes());

    // Payload
    let payload = serde_json::json!({
        "sub": claims.sub,
        "exp": claims.exp,
        "iat": claims.iat,
        "scopes": claims.scopes,
    });
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.to_string().as_bytes());

    // Signature
    let message = format!("{}.{}", header_b64, payload_b64);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);

    format!("{}.{}.{}", header_b64, payload_b64, signature_b64)
}

/// Authentication errors.
#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    InvalidSignature,
    TokenExpired,
    MissingCredentials,
    InsufficientScope,
}

/// Extract bearer token from Authorization header.
fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            if v.starts_with("Bearer ") {
                Some(v[7..].to_string())
            } else {
                None
            }
        })
}

/// Extract API key from X-API-Key header.
fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Check if path is a health endpoint.
fn is_health_path(path: &str) -> bool {
    path.starts_with("/health") || path == "/" || path.starts_with("/api/v1/health")
}

/// Authentication middleware layer.
pub async fn auth_middleware(
    auth: Arc<AuthState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Skip auth if disabled
    if !auth.configs().enabled {
        return next.run(req).await;
    }

    let path = req.uri().path();
    let headers = req.headers().clone();

    // Allow unauthenticated access to health endpoints if configsured
    if auth.configs().allow_health_unauthenticated && is_health_path(path) {
        return next.run(req).await;
    }

    // Try API key first
    if let Some(api_key) = extract_api_key(&headers) {
        if let Some(identity) = auth.validate_api_key(&api_key) {
            // Store identity in request extensions for later use
            let mut req = req;
            req.extensions_mut().insert(identity);
            return next.run(req).await;
        }
        return unauthorized_response("Invalid API key");
    }

    // Try JWT bearer token
    if let Some(token) = extract_bearer_token(&headers) {
        if let Some(identity) = auth.validate_jwt(&token) {
            let mut req = req;
            req.extensions_mut().insert(identity);
            return next.run(req).await;
        }
        return unauthorized_response("Invalid or expired token");
    }

    // No credentials provided
    unauthorized_response("Authentication required")
}

/// Create an unauthorized response.
fn unauthorized_response(message: &str) -> Response {
    let body = serde_json::json!({
        "error": message,
        "code": "UNAUTHORIZED",
        "status": 401,
    });

    (StatusCode::UNAUTHORIZED, Json(body)).into_response()
}

/// Create a forbidden response.
pub fn forbidden_response(message: &str) -> Response {
    let body = serde_json::json!({
        "error": message,
        "code": "FORBIDDEN",
        "status": 403,
    });

    (StatusCode::FORBIDDEN, Json(body)).into_response()
}

/// Require a specific scope for an endpoint.
pub fn require_scope(identity: Option<&AuthIdentity>, scope: &str) -> Result<(), Response> {
    match identity {
        Some(id) if id.has_scope(scope) => Ok(()),
        Some(_) => Err(forbidden_response(&format!(
            "Insufficient permissions. Required scope: {}",
            scope
        ))),
        None => Err(unauthorized_response("Authentication required")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_validation() {
        let configs = Authconfigs::default().with_api_key(
            ApiKeyEntry::new("test-key-123", "test-app")
                .with_scopes(vec!["read".to_string(), "write".to_string()]),
        );
        let auth = AuthState::new(configs);

        // Valid key
        let identity = auth.validate_api_key("test-key-123");
        assert!(identity.is_some());
        let id = identity.unwrap();
        assert_eq!(id.subject, "test-app");
        assert!(id.has_scope("read"));
        assert!(id.has_scope("write"));
        assert!(!id.has_scope("admin"));

        // Invalid key
        assert!(auth.validate_api_key("wrong-key").is_none());
    }

    #[test]
    fn test_jwt_creation_and_validation() {
        let secret_part1 = "test-secret";
        let secret_part2 = "-key-for-jwt";
        let secret = format!("{}{}", secret_part1, secret_part2);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = JwtClaims {
            sub: "user-123".to_string(),
            exp: now + 3600, // 1 hour from now
            iat: now,
            scopes: vec!["read".to_string(), "write".to_string()],
        };

        let token = create_jwt(&claims, secret);
        let decoded = decode_jwt(&token, secret);
        assert!(decoded.is_ok());

        let decoded_claims = decoded.unwrap();
        assert_eq!(decoded_claims.sub, "user-123");
        assert!(decoded_claims.scopes.contains(&"read".to_string()));
    }

    #[test]
    fn test_expired_jwt_rejected() {
        let secret = "test-secret-key-for-jwt";
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = JwtClaims {
            sub: "user-123".to_string(),
            exp: now - 3600, // 1 hour ago (expired)
            iat: now - 7200,
            scopes: vec!["read".to_string()],
        };

        let token = create_jwt(&claims, secret);
        let decoded = decode_jwt(&token, secret);
        assert!(matches!(decoded, Err(AuthError::TokenExpired)));
    }

    #[test]
    fn test_jwt_invalid_signature_rejected() {
        let secret = "test-secret-key-for-jwt";
        let wrong_secret = "wrong-secret-key";
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = JwtClaims {
            sub: "user-123".to_string(),
            exp: now + 3600,
            iat: now,
            scopes: vec!["read".to_string()],
        };

        let token = create_jwt(&claims, secret);
        let decoded = decode_jwt(&token, wrong_secret);
        assert!(matches!(decoded, Err(AuthError::InvalidSignature)));
    }

    #[test]
    fn test_health_path_detection() {
        assert!(is_health_path("/health"));
        assert!(is_health_path("/health/live"));
        assert!(is_health_path("/health/ready"));
        assert!(is_health_path("/api/v1/health"));
        assert!(is_health_path("/"));
        assert!(!is_health_path("/api/v1/memories"));
        assert!(!is_health_path("/api/v1/models"));
    }
}
