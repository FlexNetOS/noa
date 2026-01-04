//! Rate Limiting Middleware
//!
//! Provides per-IP and per-token rate limiting for API endpoints.
//! Protects expensive operations like search and inference.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use tokio::sync::RwLock;

/// Rate limit configsuration.
#[derive(Debug, Clone)]
pub struct RateLimitconfigs {
    /// Maximum requests per window.
    pub max_requests: u32,
    /// Time window duration.
    pub window: Duration,
    /// Whether to apply per-IP limiting.
    pub per_ip: bool,
    /// Whether to apply per-token limiting.
    pub per_token: bool,
}

impl Default for RateLimitconfigs {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: true,
        }
    }
}

impl RateLimitconfigs {
    /// Create a configs for expensive endpoints (search, inference).
    pub fn expensive() -> Self {
        Self {
            max_requests: 20,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: true,
        }
    }

    /// Create a relaxed configs for read-heavy endpoints.
    pub fn relaxed() -> Self {
        Self {
            max_requests: 500,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: false,
        }
    }
}

/// Rate limit entry tracking requests.
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

impl RateLimitEntry {
    fn new() -> Self {
        Self {
            count: 1,
            window_start: Instant::now(),
        }
    }

    fn increment(&mut self, window: Duration) -> u32 {
        if self.window_start.elapsed() > window {
            // Reset window
            self.count = 1;
            self.window_start = Instant::now();
        } else {
            self.count += 1;
        }
        self.count
    }

    fn remaining(&self, max: u32, window: Duration) -> u32 {
        if self.window_start.elapsed() > window {
            max
        } else {
            max.saturating_sub(self.count)
        }
    }

    fn reset_after(&self, window: Duration) -> u64 {
        let elapsed = self.window_start.elapsed();
        if elapsed >= window {
            0
        } else {
            (window - elapsed).as_secs()
        }
    }
}

/// Shared rate limiter state.
#[derive(Debug)]
pub struct RateLimiter {
    configs: RateLimitconfigs,
    ip_limits: RwLock<HashMap<IpAddr, RateLimitEntry>>,
    token_limits: RwLock<HashMap<String, RateLimitEntry>>,
}

impl RateLimiter {
    pub fn new(configs: RateLimitconfigs) -> Self {
        Self {
            configs,
            ip_limits: RwLock::new(HashMap::new()),
            token_limits: RwLock::new(HashMap::new()),
        }
    }

    /// Check and update rate limit for an IP.
    pub async fn check_ip(&self, ip: IpAddr) -> RateLimitResult {
        if !self.configs.per_ip {
            return RateLimitResult::Allowed {
                remaining: self.configs.max_requests,
                reset_after: 0,
            };
        }

        let mut limits = self.ip_limits.write().await;
        let entry = limits.entry(ip).or_insert_with(RateLimitEntry::new);
        let count = entry.increment(self.configs.window);

        if count > self.configs.max_requests {
            RateLimitResult::Limited {
                retry_after: entry.reset_after(self.configs.window),
            }
        } else {
            RateLimitResult::Allowed {
                remaining: entry.remaining(self.configs.max_requests, self.configs.window),
                reset_after: entry.reset_after(self.configs.window),
            }
        }
    }

    /// Check and update rate limit for a token.
    pub async fn check_token(&self, token: &str) -> RateLimitResult {
        if !self.configs.per_token {
            return RateLimitResult::Allowed {
                remaining: self.configs.max_requests,
                reset_after: 0,
            };
        }

        let mut limits = self.token_limits.write().await;
        let entry = limits.entry(token.to_string()).or_insert_with(RateLimitEntry::new);
        let count = entry.increment(self.configs.window);

        if count > self.configs.max_requests {
            RateLimitResult::Limited {
                retry_after: entry.reset_after(self.configs.window),
            }
        } else {
            RateLimitResult::Allowed {
                remaining: entry.remaining(self.configs.max_requests, self.configs.window),
                reset_after: entry.reset_after(self.configs.window),
            }
        }
    }

    /// Periodically clean up old entries.
    pub async fn cleanup(&self) {
        let window = self.configs.window;

        // Clean IP limits
        {
            let mut limits = self.ip_limits.write().await;
            limits.retain(|_, entry| entry.window_start.elapsed() <= window * 2);
        }

        // Clean token limits
        {
            let mut limits = self.token_limits.write().await;
            limits.retain(|_, entry| entry.window_start.elapsed() <= window * 2);
        }
    }
}

/// Result of a rate limit check.
#[derive(Debug)]
pub enum RateLimitResult {
    Allowed { remaining: u32, reset_after: u64 },
    Limited { retry_after: u64 },
}

/// Extract client IP from request, considering X-Forwarded-For.
fn extract_client_ip(headers: &HeaderMap, connect_info: Option<&std::net::SocketAddr>) -> Option<IpAddr> {
    // Check X-Forwarded-For first (for proxied requests)
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            // Take the first IP in the chain (original client)
            if let Some(first) = value.split(',').next() {
                if let Ok(ip) = first.trim().parse() {
                    return Some(ip);
                }
            }
        }
    }

    // Check X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            if let Ok(ip) = value.trim().parse() {
                return Some(ip);
            }
        }
    }

    // Fall back to connection info
    connect_info.map(|c| c.ip())
}

/// Extract auth token from request.
fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| {
            if v.starts_with("Bearer ") {
                v[7..].to_string()
            } else {
                v.to_string()
            }
        })
}

/// Rate limiting middleware layer.
pub async fn rate_limit_middleware(
    limiter: Arc<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let headers = req.headers().clone();

    // Extract client IP
    let client_ip = extract_client_ip(&headers, None);

    // Check IP rate limit
    if let Some(ip) = client_ip {
        match limiter.check_ip(ip).await {
            RateLimitResult::Limited { retry_after } => {
                return rate_limit_response(retry_after);
            }
            RateLimitResult::Allowed { .. } => {
                // Continue
            }
        }
    }

    // Check token rate limit
    if let Some(token) = extract_token(&headers) {
        match limiter.check_token(&token).await {
            RateLimitResult::Limited { retry_after } => {
                return rate_limit_response(retry_after);
            }
            RateLimitResult::Allowed { .. } => {
                // Continue
            }
        }
    }

    next.run(req).await
}

/// Create a rate limit exceeded response.
fn rate_limit_response(retry_after: u64) -> Response {
    let body = serde_json::json!({
        "error": "Rate limit exceeded",
        "code": "RATE_LIMITED",
        "status": 429,
        "details": {
            "retry_after_seconds": retry_after,
        }
    });

    let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(body)).into_response();
    response.headers_mut().insert(
        "retry-after",
        retry_after.to_string().parse().unwrap(),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new(RateLimitconfigs {
            max_requests: 5,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: false,
        });

        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        for _ in 0..5 {
            match limiter.check_ip(ip).await {
                RateLimitResult::Allowed { .. } => {}
                RateLimitResult::Limited { .. } => panic!("Should be allowed"),
            }
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new(RateLimitconfigs {
            max_requests: 2,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: false,
        });

        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        // First two should pass
        assert!(matches!(limiter.check_ip(ip).await, RateLimitResult::Allowed { .. }));
        assert!(matches!(limiter.check_ip(ip).await, RateLimitResult::Allowed { .. }));

        // Third should be blocked
        assert!(matches!(limiter.check_ip(ip).await, RateLimitResult::Limited { .. }));
    }
}
