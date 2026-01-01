# init Module

Bootstrap initialization for NOA instances.

**Location**: `sys/core/src/init/`  
**Always Available**: Yes (no feature flag required)

## Overview

Handles first-time setup and instance bootstrapping:

- Data directory creation
- Database initialization
- Default configuration
- First admin user provisioning

## Key Types

### InitCommand

CLI command for initialization.

```rust
pub struct InitCommand {
    /// Data directory path
    pub data_dir: PathBuf,
    
    /// Skip interactive prompts
    pub non_interactive: bool,
    
    /// Force re-initialization
    pub force: bool,
}
```

### InitContext

State during initialization.

```rust
pub struct InitContext {
    pub data_dir: PathBuf,
    pub config_path: PathBuf,
    pub database_url: String,
    pub admin_created: bool,
}
```

## Initialization Steps

1. **Create directories**
   ```
   ~/.noa/
   ├── config/
   ├── data/
   ├── models/
   ├── cache/
   └── logs/
   ```

2. **Initialize database**
   - Create SQLite database file
   - Run migrations
   - Seed default data

3. **Generate config**
   - Create default `config.toml`
   - Generate secure secrets

4. **Provision admin**
   - Create first admin user (if interactive)
   - Store credentials securely

## Usage

```bash
# Interactive initialization
noa init

# Non-interactive with defaults
noa init --non-interactive --data-dir /opt/noa

# Force re-initialization
noa init --force
```

## Programmatic

```rust
use noa_core::init::{InitCommand, run_init};

async fn example() -> NoaResult<()> {
    let cmd = InitCommand {
        data_dir: PathBuf::from("~/.noa"),
        non_interactive: true,
        force: false,
    };
    
    let ctx = run_init(cmd).await?;
    println!("Initialized at: {:?}", ctx.data_dir);
    
    Ok(())
}
```

## See Also

- [cli module](cli.md) — CLI interface
- [config module](config.md) — Configuration
- [db module](db.md) — Database setup
