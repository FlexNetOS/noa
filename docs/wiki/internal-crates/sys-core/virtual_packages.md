# virtual_packages Module

Package virtualization layer.

**Location**: `sys/core/src/virtual_packages/`  
**Feature**: `full`

## Overview

Virtualized package management:

- Sandbox package installation
- Content-addressable storage
- Version isolation
- Reproducible environments

## Key Types

### VirtualPackage

Virtualized package definition.

```rust
pub struct VirtualPackage {
    pub name: String,
    pub version: Version,
    pub hash: ContentHash,
    pub files: Vec<VirtualFile>,
}
```

### PackageStore

Content-addressable package store.

```rust
pub struct PackageStore {
    store_path: PathBuf,
}

impl PackageStore {
    pub async fn install(&self, spec: &PackageSpec) -> NoaResult<VirtualPackage>;
    pub fn get(&self, hash: &ContentHash) -> Option<&VirtualPackage>;
    pub fn gc(&self) -> NoaResult<usize>;
}
```

### VirtualEnv

Isolated environment.

```rust
pub struct VirtualEnv {
    packages: Vec<VirtualPackage>,
    env_vars: HashMap<String, String>,
}

impl VirtualEnv {
    pub fn activate(&self) -> EnvGuard;
}
```

## Usage

```rust
use noa_core::virtual_packages::{PackageStore, PackageSpec};

async fn example() -> NoaResult<()> {
    let store = PackageStore::new("~/.noa/packages");
    
    // Install a package
    let pkg = store.install(&PackageSpec::parse("python@3.12")?).await?;
    
    // Create environment
    let env = VirtualEnv::new()
        .add(pkg)
        .build()?;
    
    // Use environment
    let guard = env.activate();
    // ... run commands in isolated env
    
    Ok(())
}
```

## See Also

- [modules module](modules.md) — Module loading
- [init module](init.md) — Environment setup
