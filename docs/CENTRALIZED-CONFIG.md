# NOA Centralized configsuration Reference

## Constitution §3.1 Compliance
All configsuration, settings, and schemas resolve under `noa_root` (N:\noa).

## Directory Structure

```
N:\noa\
├── .build/                    # Build outputs
│   ├── debug/                 # Debug builds
│   ├── release/               # Release builds
│   ├── artifacts/             # Build artifacts
│   └── cache/                 # Build cache
│
├── etc/                       # configsuration (XDG_configs_HOME)
│   ├── aider/                 # Aider AI configs
│   ├── azure/                 # Azure CLI configs
│   ├── chezmoi/               # Chezmoi dotfile manager
│   │   └── chezmoi.toml       # Chezmoi configs
│   ├── claude/                # Claude configss
│   │   ├── configs.json        # MCP server configs
│   │   ├── cli-configs.json    # CLI configs (from ~/.claude.json)
│   │   └── desktop/           # Junction to AppData\Claude
│   ├── copilot/               # GitHub Copilot configs
│   ├── cursor/                # Cursor editor configs
│   ├── docker/                # Docker configs
│   ├── dotfiles/              # Chezmoi source directory
│   ├── git/                   # Git configsuration
│   │   ├── configs             # Global git configs
│   │   └── ignore             # Global gitignore
│   ├── gnupg/                 # GnuPG configs
│   ├── mise/                  # mise tool manager configs
│   ├── npm/                   # NPM configsuration
│   │   └── npmrc              # NPM configs file
│   ├── nushell/               # Nushell configs
│   │   └── env.nu             # Environment setup
│   ├── openai/                # OpenAI configs
│   ├── python/                # Python configs
│   │   └── pythonrc.py        # Python startup
│   ├── rattler/               # Rattler/Pixi configs
│   ├── ssh/                   # SSH configs
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
└── configs/                    # JSON configss & schemas
    ├── schemas/               # JSON validation schemas
    │   ├── capsule.schema.json
    │   ├── configs_schema.json
    │   ├── desktop-apps.json
    │   ├── mcp-servers.json
    │   └── providers.yaml
    └── templates/             # configs templates
        ├── agent.yaml
        ├── noa-instance.yaml
        └── provider.yaml
```

## Environment Variables (31 persisted)

| Variable | Value | Purpose |
|----------|-------|---------|
| NOA_ROOT | N:\noa | Root directory |
| XDG_configs_HOME | N:\noa\etc | configsuration |
| XDG_CACHE_HOME | N:\noa\cache | Cache |
| XDG_DATA_HOME | N:\noa\data | Data |
| XDG_STATE_HOME | N:\noa\data\state | State |
| CARGO_HOME | N:\noa\data\cargo | Rust cargo |
| RUSTUP_HOME | N:\noa\data\rustup | Rust toolchains |
| GIT_configs_GLOBAL | N:\noa\etc\git\configs | Git configs |
| NPM_configs_USERconfigs | N:\noa\etc\npm\npmrc | NPM configs |
| CHEZMOI_SOURCE_DIR | N:\noa\etc\dotfiles | Dotfiles source |
| OLLAMA_HOME | N:\noa\data\ollama | Ollama data |
| OLLAMA_MODELS | N:\noa\cache\ollama\models | Ollama models |
| CLAUDE_configs_DIR | N:\noa\etc\claude | Claude configs |
| VSCODE_EXTENSIONS | N:\noa\etc\vscode\extensions | VS Code extensions |

## Dev Tool configs Pointers

| Tool | C: Drive Location | NOA Location | Type |
|------|-------------------|--------------|------|
| VS Code | %APPDATA%\Code\User | etc\vscode\user-settings.json | Copy |
| Claude Desktop | %APPDATA%\Claude | etc\claude\desktop | Junction |
| Claude CLI | ~/.claude.json | etc\claude\cli-configs.json | Copy |
| Git | ~/.gitconfigs | etc\git\configs | Env var |
| NPM | ~/.npmrc | etc\npm\npmrc | Env var |

## Scripts

| Script | Purpose |
|--------|---------|
| scripts\noa-env-persist.ps1 | Persist all env vars to User scope |
| scripts\sync-dev-configss.ps1 | Sync dev tool configss to NOA |
| scripts\xdg-ninja.ps1 | Audit XDG compliance |
| scripts\xdg-env-fix.ps1 | Set XDG env vars for session |

## Usage

### Initial Setup
```powershell
# Persist environment variables
powershell -File N:\noa\scripts\noa-env-persist.ps1

# Sync dev tool configss
powershell -File N:\noa\scripts\sync-dev-configss.ps1

# Restart terminal to apply
```

### Verify configsuration
```powershell
# Check environment variables
[Environment]::GetEnvironmentVariable("NOA_ROOT", "User")

# Run XDG audit
N:\noa\bin\xdg-ninja.cmd
```
