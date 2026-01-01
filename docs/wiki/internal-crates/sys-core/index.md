# sys-core Crate

The core crate containing NOA's microkernel, agents, ML inference, and platform services.

**Location**: `sys/core/`  
**Edition**: Rust 2021  
**MSRV**: 1.83.0

## Module Overview

### Always-Available (Bootstrap)

These modules are compiled without feature flags and are required for `noa init`:

| Module | Purpose | Key Types |
|--------|---------|-----------|
| [cli](cli.md) | Command-line interface | `CliArgs`, subcommands |
| [config](config.md) | Configuration loading | `AppConfig`, `Settings` |
| [db](db.md) | SQLite persistence | `Database`, stores |
| [error](error.md) | Error handling | `NoaError`, `NoaResult` |
| [init](init.md) | Bootstrap initialization | `InitCommand` |
| [logging](logging.md) | Tracing setup | `setup_logging()` |
| [timestamp](timestamp.md) | Time utilities | `TimestampMeta`, HTTP dates |

### Full Feature Modules

These modules require `--features full`:

#### Agent System
| Module | Purpose | Key Types |
|--------|---------|-----------|
| [agents](agents.md) | Agent orchestration | `Agent`, `AgentCommand` |
| [autonomy](autonomy.md) | Self-governance | `Governor`, `Policy` |
| [automation](automation.md) | Task automation | `Trigger`, `Schedule` |

#### AI & ML
| Module | Purpose | Key Types |
|--------|---------|-----------|
| [neural](neural.md) | Neural networks | `NeuralModule`, inference |
| [learning](learning.md) | Adaptive learning | `LearningLoop` |
| [vector](vector.md) | Vector embeddings | `VectorStore`, similarity |
| [memory](memory.md) | Semantic memory | `MemoryBank`, retrieval |

#### API & Services
| Module | Purpose | Key Types |
|--------|---------|-----------|
| [api](api.md) | REST/gRPC endpoints | `Router`, handlers |
| [services](services.md) | Background services | `ServiceRunner` |
| [providers](providers.md) | External integrations | `Provider` trait |

#### Observability
| Module | Purpose | Key Types |
|--------|---------|-----------|
| [observability](observability.md) | Metrics & tracing | `Metrics`, spans |
| [events](events.md) | Event bus | `Event`, `EventHandler` |
| [healing](healing.md) | Self-healing | `HealthCheck`, recovery |

#### Orchestration
| Module | Purpose | Key Types |
|--------|---------|-----------|
| [modules](modules.md) | Module loading | `Module`, `ModuleRegistry` |
| [virtual_packages](virtual_packages.md) | Package virtualization | `VirtualPackage` |

## Dependency Graph

```
┌─────────────────────────────────────────────────────────┐
│                       api                               │
│                        │                                │
│        ┌───────────────┼───────────────┐                │
│        ▼               ▼               ▼                │
│    agents          services        observability        │
│        │               │               │                │
│        └───────┬───────┴───────┬───────┘                │
│                ▼               ▼                        │
│            autonomy         neural ◄─── vector          │
│                │               │            │           │
│                └───────┬───────┴────────────┘           │
│                        ▼                                │
│                       db                                │
│                        │                                │
│                        ▼                                │
│    config ◄─── init ◄─── cli ◄─── error                 │
└─────────────────────────────────────────────────────────┘
```

## External Dependencies

Key workspace dependencies:

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime |
| `sqlx` | 0.8.x | Database |
| `axum` | 0.8.x | HTTP server |
| `serde` | 1.x | Serialization |
| `tracing` | 0.1.x | Observability |
| `filetime` | 0.2.x | File timestamps |
| `httpdate` | 1.0.x | HTTP date formatting |

## Usage

```rust
use noa_core::{config, db, agents, error::NoaResult};

async fn example() -> NoaResult<()> {
    let config = config::load()?;
    let db = db::connect(&config.database_url).await?;
    let agent = agents::spawn("file-io", &db).await?;
    Ok(())
}
```

---

*Auto-generated from cargo doc + manual curation*
