# NOA Internal Crates

Documentation for NOA's internal Rust crates, organized by subsystem.

## Overview

NOA is built from four major subsystems, each containing multiple Rust crates:

| Subsystem | Purpose | Crate Count |
|-----------|---------|-------------|
| [sys-core](sys-core/index.md) | Microkernel, agents, ML, observability | 20+ modules |
| [p2p](p2p/index.md) | libp2p networking stack | 50+ crates |
| [ui-app](ui-app/index.md) | Dioxus UI, Tauri desktop | 9 crates |
| [rust-lovable](rust-lovable.md) | Component library, AI UI builder | - |

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         NOA Platform                             │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │    ui-app       │  │    sys-core     │  │      p2p        │  │
│  │                 │  │                 │  │                 │  │
│  │  noa-ui-shell   │  │  agents         │  │  gossipsub      │  │
│  │  noa-ui-core    │◄─┤  autonomy       │◄─┤  kad            │  │
│  │  noa-ui-desktop │  │  neural         │  │  relay          │  │
│  │  noa-ui-hived   │  │  api            │  │  quic           │  │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘  │
│           │                    │                    │           │
│  ┌────────▼────────┐           │                    │           │
│  │  rust-lovable   │           │                    │           │
│  │  (components)   │───────────┼────────────────────┘           │
│  └─────────────────┘           │                                │
│                                ▼                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                     SQLite + Vector DB                      ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow

1. **UI Layer** (`ui-app`) receives user input via Dioxus components
2. **Core Layer** (`sys-core`) orchestrates agents and ML inference
3. **Network Layer** (`p2p`) handles peer discovery and distributed compute
4. **Storage Layer** persists state to SQLite with vector embeddings

## Feature Flags

NOA uses Cargo feature flags to enable conditional compilation:

```toml
# sys/core features
[features]
default = []
full = ["api", "autonomy", "agents", "neural", "observability", ...]
compression = ["lz4_flex", "zstd"]
ml-devops = ["ort"]
```

## Build Configurations

| Configuration | Command | Purpose |
|---------------|---------|---------|
| Minimal | `cargo build` | Bootstrap, init only |
| Full | `cargo build --features full` | Complete platform |
| ML DevOps | `cargo build --features "full,ml-devops"` | + ONNX inference |

---

*See individual crate pages for detailed API documentation.*
