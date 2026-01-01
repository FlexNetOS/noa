# cli Module

Command-line interface for NOA.

**Location**: `sys/core/src/cli/`  
**Always Available**: Yes (no feature flag required)

## Overview

The CLI module provides the main entry point for NOA commands, including:

- `noa init` — Bootstrap a new NOA instance
- `noa run` — Start the NOA daemon
- `noa agent` — Manage agents
- `noa config` — Configuration utilities

## Key Types

### CliArgs

Main CLI argument parser using `clap`.

```rust
#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
    
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    
    #[arg(long, global = true)]
    pub verbose: bool,
}
```

### Command

Available subcommands.

```rust
pub enum Command {
    Init(InitCommand),
    Run(RunCommand),
    Agent(AgentCommand),
    Config(ConfigCommand),
}
```

## Usage

```bash
# Initialize NOA
noa init --data-dir ~/.noa

# Run daemon
noa run --config config.toml

# List agents
noa agent list

# Show config
noa config show
```

## See Also

- [init module](init.md) — Initialization logic
- [config module](config.md) — Configuration loading
