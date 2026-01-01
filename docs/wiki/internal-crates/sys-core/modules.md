# modules Module

Dynamic module loading.

**Location**: `sys/core/src/modules/`  
**Feature**: `full`

## Overview

Plugin system for dynamic functionality:

- Module discovery
- Lazy loading
- Version management
- Dependency resolution

## Key Types

### Module

Module definition.

```rust
pub struct Module {
    pub id: ModuleId,
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub entry_point: PathBuf,
}
```

### ModuleRegistry

Central module management.

```rust
pub struct ModuleRegistry {
    modules: HashMap<ModuleId, LoadedModule>,
    search_paths: Vec<PathBuf>,
}

impl ModuleRegistry {
    pub fn discover(&mut self) -> NoaResult<Vec<Module>>;
    pub async fn load(&mut self, id: &ModuleId) -> NoaResult<()>;
    pub fn unload(&mut self, id: &ModuleId) -> NoaResult<()>;
    pub fn get(&self, id: &ModuleId) -> Option<&LoadedModule>;
}
```

## Module Types

| Type | Purpose |
|------|---------|
| Agent | Custom agent implementations |
| Provider | External service integrations |
| Transform | Data transformers |
| UI | UI extensions |

## Module Structure

```
modules/
├── my-agent/
│   ├── module.toml
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
```

```toml
# module.toml
[module]
id = "my-agent"
name = "My Custom Agent"
version = "1.0.0"
type = "agent"

[dependencies]
noa-core = "^0.1"
```

## Usage

```rust
use noa_core::modules::ModuleRegistry;

async fn example() -> NoaResult<()> {
    let mut registry = ModuleRegistry::new();
    registry.add_search_path("~/.noa/modules");
    
    // Discover available modules
    let modules = registry.discover()?;
    
    // Load a specific module
    registry.load(&ModuleId::new("my-agent")).await?;
    
    Ok(())
}
```

## See Also

- [agents module](agents.md) — Agent modules
- [providers module](providers.md) — Provider modules
