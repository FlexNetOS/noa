//! Request Validation Middleware
//!
//! Validates incoming requests for security and correctness.
//! §3.6: Request validation

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Maximum allowed request body size (10MB)
const MAX_BODY_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum allowed URL length
const MAX_URL_LENGTH: usize = 2048;

/// Validation error response
#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub error: String,
    pub code: String,
    pub details: Option<serde_json::Value>,
}

impl ValidationError {
    pub fn new(error: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            code: code.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// Request validation middleware
pub async fn validate_request(
    request: Request<Body>,
    next: Next,
) -> Result<Response, ValidationError> {
    // Validate URL length
    let uri = request.uri();
    if uri.to_string().len() > MAX_URL_LENGTH {
        return Err(ValidationError::new(
            "URL too long",
            "URL_TOO_LONG",
        ));
    }

    // Validate content-length if present
    if let Some(content_length) = request.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<u64>() {
                if length > MAX_BODY_SIZE {
                    return Err(ValidationError::new(
                        format!("Request body too large: {} bytes (max: {})", length, MAX_BODY_SIZE),
                        "BODY_TOO_LARGE",
                    ));
                }
            }
        }
    }

    // Validate content-type for POST/PUT/PATCH requests
    let method = request.method();
    if method == "POST" || method == "PUT" || method == "PATCH" {
        if let Some(content_type) = request.headers().get("content-type") {
            let content_type_str = content_type.to_str().unwrap_or("");

            // Allow JSON and form data
            let valid_content_types = [
                "application/json",
                "application/x-www-form-urlencoded",
                "multipart/form-data",
            ];

            let is_valid = valid_content_types.iter().any(|ct| content_type_str.starts_with(ct));

            if !is_valid && !content_type_str.is_empty() {
                tracing::warn!(
                    content_type = %content_type_str,
                    method = %method,
                    "Unusual content type"
                );
            }
        }
    }

    // Check for suspicious patterns in the path
    let path = uri.path();
    if contains_path_traversal(path) {
        return Err(ValidationError::new(
            "Invalid path",
            "PATH_TRAVERSAL_DETECTED",
        ));
    }

    // Continue to the next handler
    Ok(next.run(request).await)
}

/// Check for path traversal attempts
fn contains_path_traversal(path: &str) -> bool {
    let suspicious_patterns = [
        "..",
        "..%2f",
        "..%5c",
        "%2e%2e",
        "....//",
        "..\\",
    ];

    let lower_path = path.to_lowercase();
    suspicious_patterns.iter().any(|pattern| lower_path.contains(pattern))
}

/// Validate request body JSON
pub fn validate_json<T: serde::de::DeserializeOwned>(
    body: &str,
) -> Result<T, ValidationError> {
    serde_json::from_str(body).map_err(|e| {
        ValidationError::new(
            format!("Invalid JSON: {}", e),
            "INVALID_JSON",
        )
    })
}

/// Validate a required string field
pub fn validate_required_string<'a>(
    value: &'a Option<String>,
    field_name: &str,
) -> Result<&'a String, ValidationError> {
    match value {
        Some(s) if !s.trim().is_empty() => Ok(s),
        Some(_) => Err(ValidationError::new(
            format!("{} cannot be empty", field_name),
            "EMPTY_FIELD",
        )),
        None => Err(ValidationError::new(
            format!("{} is required", field_name),
            "MISSING_FIELD",
        )),
    }
}

/// Validate a string length
pub fn validate_string_length(
    value: &str,
    field_name: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len < min {
        return Err(ValidationError::new(
            format!("{} must be at least {} characters", field_name, min),
            "STRING_TOO_SHORT",
        ));
    }
    if len > max {
        return Err(ValidationError::new(
            format!("{} must be at most {} characters", field_name, max),
            "STRING_TOO_LONG",
        ));
    }
    Ok(())
}

/// Validate UUID format
pub fn validate_uuid(value: &str, field_name: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::parse_str(value).is_err() {
        return Err(ValidationError::new(
            format!("{} must be a valid UUID", field_name),
            "INVALID_UUID",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal_detection() {
        assert!(contains_path_traversal("../etc/passwd"));
        assert!(contains_path_traversal("/api/../../../etc"));
        assert!(contains_path_traversal("..%2f..%2f"));
        assert!(!contains_path_traversal("/api/v1/health"));
        assert!(!contains_path_traversal("/api/v1/agents/123"));
    }

    #[test]
    fn test_validate_required_string() {
        let some_val = Some("test".to_string());
        let empty_val = Some("".to_string());
        let none_val: Option<String> = None;

        assert!(validate_required_string(&some_val, "field").is_ok());
        assert!(validate_required_string(&empty_val, "field").is_err());
        assert!(validate_required_string(&none_val, "field").is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("hello", "field", 1, 10).is_ok());
        assert!(validate_string_length("", "field", 1, 10).is_err());
        assert!(validate_string_length("hello world!", "field", 1, 5).is_err());
    }

    #[test]
    fn test_validate_uuid() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000", "id").is_ok());
        assert!(validate_uuid("not-a-uuid", "id").is_err());
    }
}

