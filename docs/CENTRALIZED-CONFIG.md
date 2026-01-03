# NOA Centralized Configuration Reference

## Constitution §3.1 Compliance
All configuration, settings, and schemas resolve under `noa_root` (N:\noa).

## Directory Structure

```
N:\noa\
├── .build/                    # Build outputs
│   ├── debug/                 # Debug builds
│   ├── release/               # Release builds
│   ├── artifacts/             # Build artifacts
│   └── cache/                 # Build cache
│
├── etc/                       # Configuration (XDG_CONFIG_HOME)
│   ├── aider/                 # Aider AI config
│   ├── azure/                 # Azure CLI config
│   ├── chezmoi/               # Chezmoi dotfile manager
│   │   └── chezmoi.toml       # Chezmoi config
│   ├── claude/                # Claude configs
│   │   ├── config.json        # MCP server config
│   │   ├── cli-config.json    # CLI config (from ~/.claude.json)
│   │   └── desktop/           # Junction to AppData\Claude
│   ├── copilot/               # GitHub Copilot config
│   ├── cursor/                # Cursor editor config
│   ├── docker/                # Docker config
│   ├── dotfiles/              # Chezmoi source directory
│   ├── git/                   # Git configuration
│   │   ├── config             # Global git config
│   │   └── ignore             # Global gitignore
│   ├── gnupg/                 # GnuPG config
│   ├── mise/                  # mise tool manager config
│   ├── npm/                   # NPM configuration
│   │   └── npmrc              # NPM config file
│   ├── nushell/               # Nushell config
│   │   └── env.nu             # Environment setup
│   ├── openai/                # OpenAI config
│   ├── python/                # Python config
│   │   └── pythonrc.py        # Python startup
│   ├── rattler/               # Rattler/Pixi config
│   ├── ssh/                   # SSH config
│   └── vscode/                # VS Code settings
│       ├── settings.json      # NOA-specific settings
│       └── user-settings.json # Copy of user settings
│
├── cache/                     # Cache (XDG_CACHE_HOME)
│   ├── aider/                 # Aider cache
│   ├── chezmoi/               # Chezmoi cache
│   ├── conda/pkgs/            # Conda packages
│   ├── mise/                  # mise cache
│   ├── npm/                   # NPM cache
│   ├── nuget/packages/        # NuGet packages
│   ├── ollama/models/         # Ollama models
│   └── pixi/                  # Pixi package cache
│
├── data/                      # Data (XDG_DATA_HOME)
│   ├── android/               # Android SDK data
│   ├── cargo/                 # Cargo home
│   ├── conda/envs/            # Conda environments
│   ├── dotnet/                # .NET CLI home
│   ├── gnupg/                 # GnuPG data
│   ├── ollama/                # Ollama data
│   ├── rustup/                # Rustup home
│   └── state/                 # XDG_STATE_HOME
│       ├── less/history       # Less history
│       ├── mise/              # mise state
│       └── python/history     # Python history
│
└── config/                    # JSON configs & schemas
    ├── schemas/               # JSON validation schemas
    │   ├── capsule.schema.json
    │   ├── config_schema.json
    │   ├── desktop-apps.json
    │   ├── mcp-servers.json
    │   └── providers.yaml
    └── templates/             # Config templates
        ├── agent.yaml
        ├── noa-instance.yaml
        └── provider.yaml
```

## Environment Variables (31 persisted)

| Variable | Value | Purpose |
|----------|-------|---------|
| NOA_ROOT | N:\noa | Root directory |
| XDG_CONFIG_HOME | N:\noa\etc | Configuration |
| XDG_CACHE_HOME | N:\noa\cache | Cache |
| XDG_DATA_HOME | N:\noa\data | Data |
| XDG_STATE_HOME | N:\noa\data\state | State |
| CARGO_HOME | N:\noa\data\cargo | Rust cargo |
| RUSTUP_HOME | N:\noa\data\rustup | Rust toolchains |
| GIT_CONFIG_GLOBAL | N:\noa\etc\git\config | Git config |
| NPM_CONFIG_USERCONFIG | N:\noa\etc\npm\npmrc | NPM config |
| CHEZMOI_SOURCE_DIR | N:\noa\etc\dotfiles | Dotfiles source |
| OLLAMA_HOME | N:\noa\data\ollama | Ollama data |
| OLLAMA_MODELS | N:\noa\cache\ollama\models | Ollama models |
| CLAUDE_CONFIG_DIR | N:\noa\etc\claude | Claude config |
| VSCODE_EXTENSIONS | N:\noa\etc\vscode\extensions | VS Code extensions |

## Dev Tool Config Pointers

| Tool | C: Drive Location | NOA Location | Type |
|------|-------------------|--------------|------|
| VS Code | %APPDATA%\Code\User | etc\vscode\user-settings.json | Copy |
| Claude Desktop | %APPDATA%\Claude | etc\claude\desktop | Junction |
| Claude CLI | ~/.claude.json | etc\claude\cli-config.json | Copy |
| Git | ~/.gitconfig | etc\git\config | Env var |
| NPM | ~/.npmrc | etc\npm\npmrc | Env var |

## Scripts

| Script | Purpose |
|--------|---------|
| scripts\noa-env-persist.ps1 | Persist all env vars to User scope |
| scripts\sync-dev-configs.ps1 | Sync dev tool configs to NOA |
| scripts\xdg-ninja.ps1 | Audit XDG compliance |
| scripts\xdg-env-fix.ps1 | Set XDG env vars for session |

## Usage

### Initial Setup
```powershell
# Persist environment variables
powershell -File N:\noa\scripts\noa-env-persist.ps1

# Sync dev tool configs
powershell -File N:\noa\scripts\sync-dev-configs.ps1

# Restart terminal to apply
```

### Verify Configuration
```powershell
# Check environment variables
[Environment]::GetEnvironmentVariable("NOA_ROOT", "User")

# Run XDG audit
N:\noa\bin\xdg-ninja.cmd
```
