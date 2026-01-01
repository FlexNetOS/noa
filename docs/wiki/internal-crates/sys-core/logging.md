# logging Module

Tracing and logging infrastructure.

**Location**: `sys/core/src/logging.rs`  
**Always Available**: Yes (no feature flag required)

## Overview

Configures the `tracing` ecosystem for structured logging:

- Multiple output formats (JSON, pretty)
- Log level filtering
- File and console output
- Span tracking for request tracing

## Key Functions

### setup_logging

Initialize the logging subsystem.

```rust
pub fn setup_logging(config: &LogConfig) -> NoaResult<()>;
```

### LogConfig

```rust
pub struct LogConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub file: Option<PathBuf>,
    pub console: bool,
}

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}
```

## Usage

### Application Setup

```rust
use noa_core::logging::{setup_logging, LogConfig, LogLevel, LogFormat};

fn main() -> NoaResult<()> {
    let config = LogConfig {
        level: LogLevel::Info,
        format: LogFormat::Pretty,
        file: Some("noa.log".into()),
        console: true,
    };
    
    setup_logging(&config)?;
    
    tracing::info!("NOA starting up");
    
    Ok(())
}
```

### Structured Logging

```rust
use tracing::{info, warn, error, instrument, span, Level};

#[instrument(skip(db))]
async fn process_task(task_id: &str, db: &Database) -> NoaResult<()> {
    info!(task_id, "Processing task");
    
    let span = span!(Level::DEBUG, "validation");
    let _guard = span.enter();
    
    // ... processing
    
    Ok(())
}
```

## Output Examples

### Pretty Format
```
2026-01-01T00:00:00.000Z  INFO noa_core::api Processing request method=GET path=/api/agents
```

### JSON Format
```json
{"timestamp":"2026-01-01T00:00:00.000Z","level":"INFO","target":"noa_core::api","message":"Processing request","method":"GET","path":"/api/agents"}
```

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `RUST_LOG` | Log level filter | `info` |
| `NOA_LOG_FORMAT` | Output format | `pretty` |

## See Also

- [observability module](observability.md) — Metrics and tracing
- [config module](config.md) — Log configuration
