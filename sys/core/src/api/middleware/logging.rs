//! Request Logging Middleware
//!
//! Logs incoming requests and responses for observability.
//! §3.5: Audit logging
//! FR-155: Observability - tracing, metrics, logging

use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::{info, warn, Span};

/// Request logging middleware
pub async fn log_request(
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();

    // Extract request details
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());

    // Get request ID from header or generate one
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Get client info
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let content_length = request
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    // Create tracing span
    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        path = %path,
    );

    // Log request start
    let _enter = span.enter();
    info!(
        target: "http",
        request_id = %request_id,
        method = %method,
        path = %path,
        query = ?query,
        user_agent = ?user_agent,
        content_length = ?content_length,
        "Request started"
    );

    // Process the request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed();
    let duration_ms = duration.as_millis() as u64;

    // Get response status
    let status = response.status();
    let status_code = status.as_u16();

    // Log based on status
    if status.is_success() {
        info!(
            target: "http",
            request_id = %request_id,
            method = %method,
            path = %path,
            status = status_code,
            duration_ms = duration_ms,
            "Request completed"
        );
    } else if status.is_client_error() {
        warn!(
            target: "http",
            request_id = %request_id,
            method = %method,
            path = %path,
            status = status_code,
            duration_ms = duration_ms,
            "Client error"
        );
    } else if status.is_server_error() {
        tracing::error!(
            target: "http",
            request_id = %request_id,
            method = %method,
            path = %path,
            status = status_code,
            duration_ms = duration_ms,
            "Server error"
        );
    }

    // Record metrics
    record_request_metrics(&method.to_string(), &path, status_code, duration_ms);

    response
}

/// Record request metrics for monitoring
fn record_request_metrics(method: &str, path: &str, status: u16, duration_ms: u64) {
    // This would integrate with a metrics system like Prometheus
    // For now, just log at trace level
    tracing::trace!(
        target: "metrics",
        method = %method,
        path = %path,
        status = status,
        duration_ms = duration_ms,
        "request_metrics"
    );

    // Emit slow request warning
    if duration_ms > 1000 {
        warn!(
            target: "performance",
            method = %method,
            path = %path,
            duration_ms = duration_ms,
            "Slow request detected"
        );
    }
}

/// Log structure for audit trail
#[derive(Debug, serde::Serialize)]
pub struct RequestLog {
    pub request_id: String,
    pub timestamp: String,
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub status: u16,
    pub duration_ms: u64,
    pub user_agent: Option<String>,
    pub client_ip: Option<String>,
}

impl RequestLog {
    pub fn new(request_id: String, method: String, path: String) -> Self {
        Self {
            request_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            method,
            path,
            query: None,
            status: 0,
            duration_ms: 0,
            user_agent: None,
            client_ip: None,
        }
    }

    pub fn with_query(mut self, query: Option<String>) -> Self {
        self.query = query;
        self
    }

    pub fn with_response(mut self, status: u16, duration_ms: u64) -> Self {
        self.status = status;
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_client(mut self, user_agent: Option<String>, client_ip: Option<String>) -> Self {
        self.user_agent = user_agent;
        self.client_ip = client_ip;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_log_builder() {
        let log = RequestLog::new(
            "test-123".to_string(),
            "GET".to_string(),
            "/api/v1/health".to_string(),
        )
        .with_query(Some("key=value".to_string()))
        .with_response(200, 50)
        .with_client(Some("test-agent".to_string()), Some("127.0.0.1".to_string()));

        assert_eq!(log.method, "GET");
        assert_eq!(log.status, 200);
        assert_eq!(log.duration_ms, 50);
    }
}

