# error Module

Error types and result handling.

**Location**: `sys/core/src/error.rs`  
**Always Available**: Yes (no feature flag required)

## Overview

Centralized error handling using `thiserror` for derive macros and structured error types.

## Key Types

### NoaError

Main error enum covering all error categories.

```rust
#[derive(Debug, thiserror::Error)]
pub enum NoaError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Agent error: {0}")]
    Agent(String),
    
    #[error("Neural error: {0}")]
    Neural(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}
```

### NoaResult

Type alias for `Result<T, NoaError>`.

```rust
pub type NoaResult<T> = Result<T, NoaError>;
```

## Error Conversion

Automatic conversion from common error types:

```rust
impl From<sqlx::Error> for NoaError { ... }
impl From<std::io::Error> for NoaError { ... }
impl From<serde_json::Error> for NoaError { ... }
impl From<toml::de::Error> for NoaError { ... }
```

## Usage

```rust
use noa_core::error::{NoaError, NoaResult};

fn load_config() -> NoaResult<Config> {
    let content = std::fs::read_to_string("config.toml")?; // Io
    let config: Config = toml::from_str(&content)?; // Config
    
    if config.database_url.is_empty() {
        return Err(NoaError::Validation("database_url is required".into()));
    }
    
    Ok(config)
}
```

## HTTP Status Mapping

For API responses:

| Error Variant | HTTP Status |
|---------------|-------------|
| `NotFound` | 404 |
| `Unauthorized` | 401 |
| `Validation` | 400 |
| `Config` | 500 |
| `Database` | 500 |
| `*` | 500 |
