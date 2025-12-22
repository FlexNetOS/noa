//! NOA Core Error Types
//!
//! Defines error types and Result wrapper for NOA core operations.
//! FR-148: Error Recovery - graceful handling of failures
//! §3.1: Core error definition

use std::fmt;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

/// Core error type for NOA operations
#[derive(Debug)]
pub enum NoaError {
    // Database errors
    Database(DatabaseError),

    // Configuration errors
    Config(ConfigError),

    // Agent errors
    Agent(AgentError),

    // API errors
    Api(ApiError),

    // IO errors
    Io(std::io::Error),

    // Serialization errors
    Serialization(String),

    // Validation errors
    Validation(ValidationError),

    // Resource not found
    NotFound { resource: String, id: String },

    // Permission denied
    PermissionDenied { action: String, resource: String },

    // Timeout
    Timeout { operation: String, duration_ms: u64 },

    // Internal error with context
    Internal { message: String, source: Option<Box<dyn std::error::Error + Send + Sync>> },
}

/// Database-specific errors
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed { query: String, error: String },
    MigrationFailed { version: String, error: String },
    PoolExhausted,
    TransactionFailed(String),
    IntegrityViolation(String),
    Corruption { path: String, details: String },
}

/// Configuration errors
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError { path: String, error: String },
    ValidationError { field: String, message: String },
    MissingRequired(String),
    InvalidValue { field: String, value: String, expected: String },
    EnvironmentVariableNotSet(String),
}

/// Agent-related errors
#[derive(Debug)]
pub enum AgentError {
    NotFound(String),
    AlreadyExists(String),
    InvalidState { agent: String, current: String, expected: String },
    ExecutionFailed { agent: String, action: String, error: String },
    CapabilityMissing { agent: String, capability: String },
    Timeout { agent: String, operation: String },
}

/// API errors
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    RateLimited { retry_after_seconds: u64 },
    ServiceUnavailable(String),
    InternalError(String),
}

/// Validation errors
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: code.into(),
        }
    }
}

impl fmt::Display for NoaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoaError::Database(e) => write!(f, "Database error: {}", e),
            NoaError::Config(e) => write!(f, "Configuration error: {}", e),
            NoaError::Agent(e) => write!(f, "Agent error: {}", e),
            NoaError::Api(e) => write!(f, "API error: {}", e),
            NoaError::Io(e) => write!(f, "IO error: {}", e),
            NoaError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            NoaError::Validation(e) => write!(f, "Validation error: {} - {}", e.field, e.message),
            NoaError::NotFound { resource, id } => write!(f, "{} not found: {}", resource, id),
            NoaError::PermissionDenied { action, resource } => {
                write!(f, "Permission denied: {} on {}", action, resource)
            }
            NoaError::Timeout { operation, duration_ms } => {
                write!(f, "Timeout after {}ms: {}", duration_ms, operation)
            }
            NoaError::Internal { message, .. } => write!(f, "Internal error: {}", message),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DatabaseError::QueryFailed { query, error } => {
                write!(f, "Query failed [{}]: {}", query, error)
            }
            DatabaseError::MigrationFailed { version, error } => {
                write!(f, "Migration {} failed: {}", version, error)
            }
            DatabaseError::PoolExhausted => write!(f, "Connection pool exhausted"),
            DatabaseError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            DatabaseError::IntegrityViolation(msg) => write!(f, "Integrity violation: {}", msg),
            DatabaseError::Corruption { path, details } => {
                write!(f, "Database corruption at {}: {}", path, details)
            }
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Config file not found: {}", path),
            ConfigError::ParseError { path, error } => {
                write!(f, "Failed to parse {}: {}", path, error)
            }
            ConfigError::ValidationError { field, message } => {
                write!(f, "Invalid config {}: {}", field, message)
            }
            ConfigError::MissingRequired(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidValue { field, value, expected } => {
                write!(f, "Invalid value for {}: got '{}', expected {}", field, value, expected)
            }
            ConfigError::EnvironmentVariableNotSet(var) => {
                write!(f, "Environment variable not set: {}", var)
            }
        }
    }
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::NotFound(name) => write!(f, "Agent not found: {}", name),
            AgentError::AlreadyExists(name) => write!(f, "Agent already exists: {}", name),
            AgentError::InvalidState { agent, current, expected } => {
                write!(f, "Agent {} in invalid state: {} (expected {})", agent, current, expected)
            }
            AgentError::ExecutionFailed { agent, action, error } => {
                write!(f, "Agent {} failed on {}: {}", agent, action, error)
            }
            AgentError::CapabilityMissing { agent, capability } => {
                write!(f, "Agent {} missing capability: {}", agent, capability)
            }
            AgentError::Timeout { agent, operation } => {
                write!(f, "Agent {} timed out on: {}", agent, operation)
            }
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::RateLimited { retry_after_seconds } => {
                write!(f, "Rate limited, retry after {} seconds", retry_after_seconds)
            }
            ApiError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for NoaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NoaError::Io(e) => Some(e),
            NoaError::Internal { source: Some(e), .. } => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl std::error::Error for DatabaseError {}
impl std::error::Error for ConfigError {}
impl std::error::Error for AgentError {}
impl std::error::Error for ApiError {}

// Conversion implementations
impl From<std::io::Error> for NoaError {
    fn from(err: std::io::Error) -> Self {
        NoaError::Io(err)
    }
}

impl From<DatabaseError> for NoaError {
    fn from(err: DatabaseError) -> Self {
        NoaError::Database(err)
    }
}

impl From<ConfigError> for NoaError {
    fn from(err: ConfigError) -> Self {
        NoaError::Config(err)
    }
}

impl From<AgentError> for NoaError {
    fn from(err: AgentError) -> Self {
        NoaError::Agent(err)
    }
}

impl From<ApiError> for NoaError {
    fn from(err: ApiError) -> Self {
        NoaError::Api(err)
    }
}

impl From<ValidationError> for NoaError {
    fn from(err: ValidationError) -> Self {
        NoaError::Validation(err)
    }
}

impl From<uuid::Error> for NoaError {
    fn from(err: uuid::Error) -> Self {
        NoaError::Validation(ValidationError::new(
            "uuid",
            format!("Invalid UUID: {}", err),
            "INVALID_UUID",
        ))
    }
}

impl From<serde_json::Error> for NoaError {
    fn from(err: serde_json::Error) -> Self {
        NoaError::Serialization(err.to_string())
    }
}

impl From<prometheus::Error> for NoaError {
    fn from(err: prometheus::Error) -> Self {
        NoaError::Internal {
            message: format!("Prometheus error: {}", err),
            source: None,
        }
    }
}

impl From<std::string::FromUtf8Error> for NoaError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        NoaError::Serialization(err.to_string())
    }
}

impl From<anyhow::Error> for NoaError {
    fn from(err: anyhow::Error) -> Self {
        NoaError::Internal {
            message: err.to_string(),
            source: None,
        }
    }
}

impl From<rusqlite::Error> for NoaError {
    fn from(err: rusqlite::Error) -> Self {
        NoaError::Database(DatabaseError::QueryFailed {
            query: "unknown".to_string(),
            error: err.to_string(),
        })
    }
}

impl From<chrono::ParseError> for NoaError {
    fn from(err: chrono::ParseError) -> Self {
        NoaError::Validation(ValidationError::new(
            "datetime",
            err.to_string(),
            "INVALID_DATETIME",
        ))
    }
}

/// Standardized API error response format.
/// All error responses follow this structure for consistency.
#[derive(Debug, serde::Serialize)]
pub struct ApiErrorResponse {
    /// Human-readable error message
    pub error: String,
    /// Machine-readable error code
    pub code: String,
    /// HTTP status code
    pub status: u16,
    /// Optional field-specific validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Request ID for correlation (populated by middleware)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl NoaError {
    /// Get the error code for machine parsing.
    pub fn error_code(&self) -> &'static str {
        match self {
            NoaError::Database(DatabaseError::ConnectionFailed(_)) => "DB_CONNECTION_FAILED",
            NoaError::Database(DatabaseError::QueryFailed { .. }) => "DB_QUERY_FAILED",
            NoaError::Database(DatabaseError::MigrationFailed { .. }) => "DB_MIGRATION_FAILED",
            NoaError::Database(DatabaseError::PoolExhausted) => "DB_POOL_EXHAUSTED",
            NoaError::Database(DatabaseError::TransactionFailed(_)) => "DB_TRANSACTION_FAILED",
            NoaError::Database(DatabaseError::IntegrityViolation(_)) => "DB_INTEGRITY_VIOLATION",
            NoaError::Database(DatabaseError::Corruption { .. }) => "DB_CORRUPTION",

            NoaError::Config(ConfigError::FileNotFound(_)) => "CONFIG_FILE_NOT_FOUND",
            NoaError::Config(ConfigError::ParseError { .. }) => "CONFIG_PARSE_ERROR",
            NoaError::Config(ConfigError::ValidationError { .. }) => "CONFIG_VALIDATION_ERROR",
            NoaError::Config(ConfigError::MissingRequired(_)) => "CONFIG_MISSING_REQUIRED",
            NoaError::Config(ConfigError::InvalidValue { .. }) => "CONFIG_INVALID_VALUE",
            NoaError::Config(ConfigError::EnvironmentVariableNotSet(_)) => "CONFIG_ENV_NOT_SET",

            NoaError::Agent(AgentError::NotFound(_)) => "AGENT_NOT_FOUND",
            NoaError::Agent(AgentError::AlreadyExists(_)) => "AGENT_ALREADY_EXISTS",
            NoaError::Agent(AgentError::InvalidState { .. }) => "AGENT_INVALID_STATE",
            NoaError::Agent(AgentError::ExecutionFailed { .. }) => "AGENT_EXECUTION_FAILED",
            NoaError::Agent(AgentError::CapabilityMissing { .. }) => "AGENT_CAPABILITY_MISSING",
            NoaError::Agent(AgentError::Timeout { .. }) => "AGENT_TIMEOUT",

            NoaError::Api(ApiError::BadRequest(_)) => "BAD_REQUEST",
            NoaError::Api(ApiError::Unauthorized(_)) => "UNAUTHORIZED",
            NoaError::Api(ApiError::Forbidden(_)) => "FORBIDDEN",
            NoaError::Api(ApiError::NotFound(_)) => "NOT_FOUND",
            NoaError::Api(ApiError::Conflict(_)) => "CONFLICT",
            NoaError::Api(ApiError::RateLimited { .. }) => "RATE_LIMITED",
            NoaError::Api(ApiError::ServiceUnavailable(_)) => "SERVICE_UNAVAILABLE",
            NoaError::Api(ApiError::InternalError(_)) => "INTERNAL_ERROR",

            NoaError::Io(_) => "IO_ERROR",
            NoaError::Serialization(_) => "SERIALIZATION_ERROR",
            NoaError::Validation(_) => "VALIDATION_ERROR",
            NoaError::NotFound { .. } => "RESOURCE_NOT_FOUND",
            NoaError::PermissionDenied { .. } => "PERMISSION_DENIED",
            NoaError::Timeout { .. } => "TIMEOUT",
            NoaError::Internal { .. } => "INTERNAL_ERROR",
        }
    }

    /// Get optional details for the error (e.g., validation field errors).
    pub fn details(&self) -> Option<serde_json::Value> {
        match self {
            NoaError::Validation(v) => Some(serde_json::json!({
                "field": v.field,
                "code": v.code,
            })),
            NoaError::Api(ApiError::RateLimited { retry_after_seconds }) => Some(serde_json::json!({
                "retry_after_seconds": retry_after_seconds,
            })),
            NoaError::NotFound { resource, id } => Some(serde_json::json!({
                "resource": resource,
                "id": id,
            })),
            NoaError::PermissionDenied { action, resource } => Some(serde_json::json!({
                "action": action,
                "resource": resource,
            })),
            NoaError::Timeout { operation, duration_ms } => Some(serde_json::json!({
                "operation": operation,
                "duration_ms": duration_ms,
            })),
            _ => None,
        }
    }
}

impl IntoResponse for NoaError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            NoaError::Api(ApiError::BadRequest(_)) => StatusCode::BAD_REQUEST,
            NoaError::Api(ApiError::Unauthorized(_)) => StatusCode::UNAUTHORIZED,
            NoaError::Api(ApiError::Forbidden(_)) => StatusCode::FORBIDDEN,
            NoaError::Api(ApiError::NotFound(_)) => StatusCode::NOT_FOUND,
            NoaError::Api(ApiError::Conflict(_)) => StatusCode::CONFLICT,
            NoaError::Api(ApiError::RateLimited { .. }) => StatusCode::TOO_MANY_REQUESTS,
            NoaError::Api(ApiError::ServiceUnavailable(_)) => StatusCode::SERVICE_UNAVAILABLE,
            NoaError::Api(ApiError::InternalError(_)) => StatusCode::INTERNAL_SERVER_ERROR,

            NoaError::Validation(_) => StatusCode::BAD_REQUEST,
            NoaError::NotFound { .. } => StatusCode::NOT_FOUND,
            NoaError::PermissionDenied { .. } => StatusCode::FORBIDDEN,
            NoaError::Timeout { .. } => StatusCode::GATEWAY_TIMEOUT,

            // Database errors - 503 for connection issues, 500 for others
            NoaError::Database(DatabaseError::ConnectionFailed(_)) => StatusCode::SERVICE_UNAVAILABLE,
            NoaError::Database(DatabaseError::PoolExhausted) => StatusCode::SERVICE_UNAVAILABLE,
            NoaError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // Default: internal
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let response = ApiErrorResponse {
            error: self.to_string(),
            code: self.error_code().to_string(),
            status: status.as_u16(),
            details: self.details(),
            request_id: None, // Populated by middleware if available
        };

        (status, Json(response)).into_response()
    }
}
/// Result type alias for NOA operations
pub type Result<T> = std::result::Result<T, NoaError>;

/// Extension trait for adding context to errors
pub trait ResultExt<T> {
    fn context(self, message: impl Into<String>) -> Result<T>;
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for std::result::Result<T, E> {
    fn context(self, message: impl Into<String>) -> Result<T> {
        self.map_err(|e| NoaError::Internal {
            message: message.into(),
            source: Some(Box::new(e)),
        })
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        self.map_err(|e| NoaError::Internal {
            message: f(),
            source: Some(Box::new(e)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NoaError::NotFound {
            resource: "Agent".to_string(),
            id: "test-123".to_string(),
        };
        assert_eq!(format!("{}", err), "Agent not found: test-123");
    }

    #[test]
    fn test_database_error() {
        let err = DatabaseError::ConnectionFailed("localhost:5432".to_string());
        assert!(format!("{}", err).contains("Connection failed"));
    }

    #[test]
    fn test_validation_error() {
        let err = ValidationError::new("email", "Invalid format", "INVALID_EMAIL");
        let noa_err: NoaError = err.into();
        assert!(format!("{}", noa_err).contains("email"));
    }
}

