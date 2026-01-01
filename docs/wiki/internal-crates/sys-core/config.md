# config Module

Configuration loading and management.

**Location**: `sys/core/src/config/`  
**Always Available**: Yes (no feature flag required)

## Overview

Handles loading, parsing, and validating NOA configuration from multiple sources:

1. Default values
2. Configuration file (TOML/YAML)
3. Environment variables
4. Command-line arguments

## Key Types

### AppConfig

Main configuration container.

```rust
pub struct AppConfig {
    pub database_url: String,
    pub data_dir: PathBuf,
    pub log_level: LogLevel,
    pub api: ApiConfig,
    pub agents: AgentsConfig,
    pub neural: NeuralConfig,
}
```

### ConfigBuilder

Fluent builder for configuration.

```rust
let config = ConfigBuilder::new()
    .file("config.toml")
    .env_prefix("NOA")
    .build()?;
```

## Configuration Files

### config.toml

```toml
[database]
url = "sqlite://~/.noa/noa.db"

[api]
host = "127.0.0.1"
port = 8080

[agents]
max_concurrent = 8
timeout_seconds = 300

[neural]
model_path = "~/.noa/models"
default_model = "qwen2.5-coder-7b"
```

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `NOA_DATABASE_URL` | Database connection string | `sqlite://noa.db` |
| `NOA_DATA_DIR` | Data directory path | `~/.noa` |
| `NOA_LOG_LEVEL` | Logging verbosity | `info` |
| `NOA_API_PORT` | API server port | `8080` |

## See Also

- [cli module](cli.md) — Command-line interface
- [db module](db.md) — Database connection
