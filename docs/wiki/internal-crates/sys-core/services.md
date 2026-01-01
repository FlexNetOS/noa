# services Module

Background services and daemons.

**Location**: `sys/core/src/services/`  
**Feature**: `full`

## Overview

Long-running background services:

- Service lifecycle management
- Graceful shutdown
- Health monitoring
- Restart policies

## Key Types

### ServiceRunner

Service orchestrator.

```rust
pub struct ServiceRunner {
    services: Vec<Box<dyn Service>>,
    shutdown_rx: watch::Receiver<bool>,
}

impl ServiceRunner {
    pub fn register<S: Service + 'static>(&mut self, service: S);
    pub async fn run(&self) -> NoaResult<()>;
    pub async fn shutdown(&self);
}
```

### Service Trait

```rust
#[async_trait]
pub trait Service: Send + Sync {
    fn name(&self) -> &str;
    async fn start(&self) -> NoaResult<()>;
    async fn stop(&self) -> NoaResult<()>;
    async fn health(&self) -> HealthStatus;
}
```

## Built-in Services

| Service | Purpose |
|---------|---------|
| `ApiService` | HTTP server |
| `AgentService` | Agent runtime |
| `SchedulerService` | Task scheduling |
| `MetricsService` | Prometheus exporter |
| `P2PService` | Peer networking |

## Usage

```rust
use noa_core::services::{ServiceRunner, ApiService, AgentService};

async fn main() -> NoaResult<()> {
    let mut runner = ServiceRunner::new();
    
    runner.register(ApiService::new(config.api));
    runner.register(AgentService::new(config.agents));
    
    // Run until shutdown signal
    runner.run().await
}
```

## See Also

- [api module](api.md) — API service
- [agents module](agents.md) — Agent service
