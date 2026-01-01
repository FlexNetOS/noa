# noa-ui-paths Crate

Route definitions and path utilities.

**Location**: `ui/app/crates/noa-ui-paths/`

## Overview

Application routing:

- Route enum
- Path constants
- Navigation utilities

## Route Enum

```rust
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/agents")]
    Agents {},
    
    #[route("/agents/:id")]
    AgentDetail { id: String },
    
    #[route("/tasks")]
    Tasks {},
    
    #[route("/tasks/:id")]
    TaskDetail { id: String },
    
    #[route("/settings")]
    Settings {},
    
    #[route("/settings/:section")]
    SettingsSection { section: String },
}
```

## Path Constants

```rust
pub mod paths {
    pub const HOME: &str = "/";
    pub const AGENTS: &str = "/agents";
    pub const TASKS: &str = "/tasks";
    pub const SETTINGS: &str = "/settings";
}
```

## See Also

- [noa-ui-shell](noa-ui-shell.md) — Navigation components
