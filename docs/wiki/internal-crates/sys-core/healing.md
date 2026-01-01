# healing Module

Self-healing and recovery mechanisms.

**Location**: `sys/core/src/healing/`  
**Feature**: `full`

## Overview

Automatic failure detection and recovery:

- Health checks
- Circuit breakers
- Automatic restarts
- Fallback strategies

## Key Types

### HealthCheck

Health check definition.

```rust
#[async_trait]
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self) -> HealthStatus;
    fn interval(&self) -> Duration;
}

pub enum HealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
}
```

### CircuitBreaker

Circuit breaker pattern.

```rust
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    threshold: u32,
    reset_timeout: Duration,
}

pub enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing, reject requests
    HalfOpen,  // Testing recovery
}
```

### RecoveryAction

```rust
pub enum RecoveryAction {
    Restart,
    Fallback { handler: Box<dyn FallbackHandler> },
    Alert { message: String },
    Escalate,
}
```

## Built-in Health Checks

| Check | Purpose | Default Interval |
|-------|---------|------------------|
| `DatabaseCheck` | SQLite connectivity | 30s |
| `ModelCheck` | Model loaded | 60s |
| `MemoryCheck` | RAM usage < 90% | 15s |
| `DiskCheck` | Disk usage < 95% | 60s |

## Usage

```rust
use noa_core::healing::{HealthChecker, DatabaseCheck, CircuitBreaker};

async fn example() -> NoaResult<()> {
    let mut checker = HealthChecker::new();
    checker.register(DatabaseCheck::new(db.clone()));
    
    // Run health checks
    let results = checker.check_all().await;
    
    for result in results {
        if let HealthStatus::Unhealthy { reason } = result.status {
            tracing::error!("{} is unhealthy: {}", result.name, reason);
        }
    }
    
    Ok(())
}
```

## Circuit Breaker Usage

```rust
let breaker = CircuitBreaker::new(5, Duration::from_secs(30));

match breaker.call(|| external_api_call()).await {
    Ok(result) => handle_result(result),
    Err(CircuitError::Open) => use_fallback(),
    Err(CircuitError::Failed(e)) => handle_error(e),
}
```

## See Also

- [observability module](observability.md) — Health metrics
- [services module](services.md) — Service health
