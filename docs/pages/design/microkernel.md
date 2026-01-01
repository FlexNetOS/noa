# Microkernel Design

NOA follows a microkernel architecture pattern.

## Overview

The microkernel contains only essential services:
- Process management (agents)
- Memory management (database)
- IPC (events)

All other functionality is implemented as user-space modules.

## Core vs. Extensions

### Core (Trusted)

| Module | Purpose | Always Loaded |
|--------|---------|---------------|
| `cli` | Command parsing | Yes |
| `config` | Configuration | Yes |
| `db` | Persistence | Yes |
| `error` | Error handling | Yes |
| `init` | Bootstrap | Yes |
| `logging` | Tracing | Yes |
| `timestamp` | Time utilities | Yes |

### Extensions (Feature-Gated)

| Module | Feature Flag | Purpose |
|--------|--------------|---------|
| `agents` | `full` | Agent system |
| `api` | `full` | HTTP endpoints |
| `neural` | `full` | ML inference |
| `p2p` | `full` | Networking |

## Benefits

1. **Minimal Attack Surface**: Core is small and auditable
2. **Flexibility**: Load only needed features
3. **Isolation**: Failures don't crash core
4. **Testability**: Core can be tested independently

## Message Passing

Components communicate via events:

```rust
// Publish event
event_bus.publish(Event::TaskCompleted { task_id });

// Subscribe to events
event_bus.subscribe(EventKind::TaskCompleted, |event| {
    handle_completion(event);
});
```

## See Also

- [Architecture Overview](architecture.md)
- [Agent System](agent-system.md)
