# events Module

Event bus and pub/sub messaging.

**Location**: `sys/core/src/events/`  
**Feature**: `full`

## Overview

Internal event system for loose coupling:

- Typed events
- Async handlers
- Event filtering
- Audit logging

## Key Types

### Event

Base event type.

```rust
pub struct Event {
    pub id: EventId,
    pub kind: EventKind,
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

pub enum EventKind {
    AgentSpawned,
    AgentStopped,
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    ConfigChanged,
    UserAction,
    SystemAlert,
}
```

### EventBus

Central event dispatcher.

```rust
pub struct EventBus {
    handlers: HashMap<EventKind, Vec<Box<dyn EventHandler>>>,
}

impl EventBus {
    pub fn subscribe<H: EventHandler>(&mut self, kind: EventKind, handler: H);
    pub async fn publish(&self, event: Event) -> NoaResult<()>;
}
```

### EventHandler

```rust
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &Event) -> NoaResult<()>;
}
```

## Usage

```rust
use noa_core::events::{EventBus, Event, EventKind};

struct LoggingHandler;

#[async_trait]
impl EventHandler for LoggingHandler {
    async fn handle(&self, event: &Event) -> NoaResult<()> {
        tracing::info!("Event: {:?}", event);
        Ok(())
    }
}

async fn example() -> NoaResult<()> {
    let mut bus = EventBus::new();
    bus.subscribe(EventKind::TaskCompleted, LoggingHandler);
    
    bus.publish(Event {
        kind: EventKind::TaskCompleted,
        payload: json!({"task_id": "123"}),
        ..Default::default()
    }).await?;
    
    Ok(())
}
```

## See Also

- [automation module](automation.md) — Event triggers
- [observability module](observability.md) — Event logging
