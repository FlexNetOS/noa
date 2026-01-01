# noa-ui-core Crate

Core utilities and types for NOA UI.

**Location**: `ui/app/crates/noa-ui-core/`

## Overview

Shared types and utilities across all UI crates:

- Application state management
- Custom Dioxus hooks
- Shared type definitions
- Error handling

## Key Modules

### state

Global application state with signals.

```rust
pub struct AppState {
    pub agents: Signal<Vec<Agent>>,
    pub tasks: Signal<Vec<Task>>,
    pub theme: Signal<Theme>,
    pub user: Signal<Option<User>>,
}
```

### hooks

Custom Dioxus hooks.

```rust
// useAgent - manage agent lifecycle
pub fn use_agent(id: &str) -> UseAgentHandle;

// useTask - task execution
pub fn use_task() -> UseTaskHandle;

// useTheme - theme management
pub fn use_theme() -> UseThemeHandle;
```

### types

Shared type definitions.

```rust
pub struct Agent { ... }
pub struct Task { ... }
pub struct Theme { ... }
```

## Dependencies

```toml
[dependencies]
dioxus = { workspace = true }
serde = { workspace = true }
tokio = { workspace = true }
```

## See Also

- [noa-ui-shell](noa-ui-shell.md) — Layout components
- [noa-ui-protocol](noa-ui-protocol.md) — Backend communication
