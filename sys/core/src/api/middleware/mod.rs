//! API Middleware Module
//!
//! Request processing middleware for validation, logging, and security.

pub mod validation;
pub mod logging;
pub mod telemetry;
pub mod rate_limit;
pub mod limits;
pub mod auth;

pub use rate_limit::{RateLimitConfig, RateLimiter, RateLimitResult, rate_limit_middleware};
pub use limits::{RequestLimitsConfig, RequestLimits, ConcurrencyLimiter, request_limits_middleware};
pub use auth::{
    AuthConfig, AuthState, AuthIdentity, AuthMethod, ApiKeyEntry, JwtClaims,
    auth_middleware, create_jwt, require_scope, forbidden_response,
};

