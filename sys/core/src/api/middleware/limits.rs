//! Request Limits Middleware
//!
//! Provides request body size limits and concurrency control.

use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// configsuration for request limits.
#[derive(Debug, Clone)]
pub struct RequestLimitsconfigs {
    /// Maximum request body size in bytes.
    pub max_body_size: usize,
    /// Maximum concurrent requests.
    pub max_concurrent: usize,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

impl Default for RequestLimitsconfigs {
    fn default() -> Self {
        Self {
            max_body_size: 10 * 1024 * 1024, // 10MB
            max_concurrent: 100,
            timeout_secs: 30,
        }
    }
}

impl RequestLimitsconfigs {
    /// configsuration for large uploads.
    pub fn large_upload() -> Self {
        Self {
            max_body_size: 100 * 1024 * 1024, // 100MB
            max_concurrent: 20,
            timeout_secs: 300, // 5 minutes
        }
    }

    /// configsuration for inference endpoints.
    pub fn inference() -> Self {
        Self {
            max_body_size: 1 * 1024 * 1024, // 1MB
            max_concurrent: 10,
            timeout_secs: 120, // 2 minutes
        }
    }
}

/// Request limits state.
pub struct RequestLimits {
    configs: RequestLimitsconfigs,
    semaphore: Semaphore,
}

impl RequestLimits {
    pub fn new(configs: RequestLimitsconfigs) -> Self {
        Self {
            semaphore: Semaphore::new(configs.max_concurrent),
            configs,
        }
    }

    pub fn configs(&self) -> &RequestLimitsconfigs {
        &self.configs
    }
}

/// Check content-length header against limit.
fn check_content_length(headers: &HeaderMap, max_size: usize) -> Result<(), Response> {
    if let Some(content_length) = headers.get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                if length > max_size {
                    return Err(payload_too_large_response(length, max_size));
                }
            }
        }
    }
    Ok(())
}

/// Create a payload too large response.
fn payload_too_large_response(actual: usize, max: usize) -> Response {
    let body = serde_json::json!({
        "error": "Request body too large",
        "code": "PAYLOAD_TOO_LARGE",
        "status": 413,
        "details": {
            "actual_bytes": actual,
            "max_bytes": max,
        }
    });

    (StatusCode::PAYLOAD_TOO_LARGE, Json(body)).into_response()
}

/// Create a service unavailable response (concurrency limit).
fn service_busy_response() -> Response {
    let body = serde_json::json!({
        "error": "Server is busy, please retry later",
        "code": "SERVICE_BUSY",
        "status": 503,
        "details": {
            "reason": "concurrency_limit",
        }
    });

    (StatusCode::SERVICE_UNAVAILABLE, Json(body)).into_response()
}

/// Create a timeout response.
fn timeout_response(timeout_secs: u64) -> Response {
    let body = serde_json::json!({
        "error": "Request timed out",
        "code": "TIMEOUT",
        "status": 504,
        "details": {
            "timeout_seconds": timeout_secs,
        }
    });

    (StatusCode::GATEWAY_TIMEOUT, Json(body)).into_response()
}

/// Request limits middleware layer.
pub async fn request_limits_middleware(
    limits: Arc<RequestLimits>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let headers = req.headers().clone();

    // Check content length
    if let Err(response) = check_content_length(&headers, limits.configs.max_body_size) {
        return response;
    }

    // Try to acquire concurrency permit
    let permit = match limits.semaphore.try_acquire() {
        Ok(permit) => permit,
        Err(_) => return service_busy_response(),
    };

    // Apply timeout
    let timeout_duration = Duration::from_secs(limits.configs.timeout_secs);
    let result = timeout(timeout_duration, next.run(req)).await;

    // Release permit implicitly when it goes out of scope
    drop(permit);

    match result {
        Ok(response) => response,
        Err(_) => timeout_response(limits.configs.timeout_secs),
    }
}

/// Concurrency limiter for expensive operations.
pub struct ConcurrencyLimiter {
    semaphore: Arc<Semaphore>,
    name: String,
}

impl ConcurrencyLimiter {
    pub fn new(name: impl Into<String>, max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            name: name.into(),
        }
    }

    /// Try to acquire a permit for an operation.
    pub async fn acquire(&self) -> Option<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore.try_acquire().ok()
    }

    /// Wait to acquire a permit (with timeout).
    pub async fn acquire_timeout(&self, timeout_secs: u64) -> Option<tokio::sync::OwnedSemaphorePermit> {
        match timeout(
            Duration::from_secs(timeout_secs),
            self.semaphore.clone().acquire_owned(),
        )
        .await
        {
            Ok(Ok(permit)) => Some(permit),
            _ => None,
        }
    }

    /// Get current available permits.
    pub fn available(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Get the limiter name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_configs() {
        let configs = RequestLimitsconfigs::default();
        assert_eq!(configs.max_body_size, 10 * 1024 * 1024);
        assert_eq!(configs.max_concurrent, 100);
    }

    #[test]
    fn test_content_length_check() {
        let mut headers = HeaderMap::new();
        headers.insert("content-length", "1000".parse().unwrap());

        // Should pass for large limit
        assert!(check_content_length(&headers, 10000).is_ok());

        // Should fail for small limit
        assert!(check_content_length(&headers, 500).is_err());
    }

    #[tokio::test]
    async fn test_concurrency_limiter() {
        let limiter = ConcurrencyLimiter::new("test", 2);

        let p1 = limiter.acquire().await;
        assert!(p1.is_some());

        let p2 = limiter.acquire().await;
        assert!(p2.is_some());

        // Third should fail
        let p3 = limiter.acquire().await;
        assert!(p3.is_none());

        // After dropping one, should succeed
        drop(p1);
        let p4 = limiter.acquire().await;
        assert!(p4.is_some());
    }
}
