# ADR-0012: Unified Configuration Architecture

**Status**: Proposed  
**Date**: 2025-01-XX  
**Authors**: NOA Development Team  
**Supersedes**: None  
**References**: Constitution §3.1 (Self-Contained & Autonomous)

## Context

NOA currently has 14+ dot directories scattered in the root directory, creating:

1. **Inconsistency** - Some configs in `etc/`, some in root dot directories
2. **Portability issues** - Hard to relocate or backup configuration state
3. **Tool conflicts** - Preinstalled IDEs/tools on C: drive ignore NOA settings
4. **No layering** - No override mechanism (defaults → project → user → env)

### Current Dot Directory Audit

| Directory | Category | Can Relocate? | Notes |
|-----------|----------|---------------|-------|
| `.git/` | VCS | NO | Git requires at repo root |
| `.github/` | VCS | NO | GitHub Actions convention |
| `.gitmodules` | VCS | NO | Git submodules config |
| `.vscode/` | IDE Workspace | NO | VS Code workspace settings |
| `.cursor/` | IDE Workspace | NO | Cursor workspace settings |
| `.vs/` | IDE | Partial | Visual Studio cache |
| `.pixi/` | Tool Runtime | YES* | Pixi environments |
| `.cache/` | Cache | YES | Should use XDG_CACHE_HOME |
| `.npm/` | Cache | YES | Should use NPM_CONFIG_CACHE |
| `.backups/` | Data | YES | Should be under `data/` |
| `.build/` | Build | YES | Should be build/ |
| `.config/` | Config | YES | Should merge into `etc/` |
| `.claude/` | AI Tool | YES | Should use CLAUDE_CONFIG_DIR |
| `.specify/` | Tool | YES | Should use XDG pattern |
| `.cursor-server/` | IDE Runtime | YES | Cache/runtime data |

### Tool Config Override Problem

Tools like VS Code, Claude Desktop, npm, git installed on C: drive use:
- `%APPDATA%\Code\User\settings.json`
- `%APPDATA%\Claude\config.json`
- `%USERPROFILE%\.gitconfig`

These ignore NOA's centralized configs at `N:\noa\etc\`.

## Decision

Implement a **4-layer configuration architecture** with automatic override:

```
┌─────────────────────────────────────────────────────────────────┐
│ Layer 4: Environment Variables (highest priority)              │
│   - Runtime overrides, secrets                                  │
├─────────────────────────────────────────────────────────────────┤
│ Layer 3: User Config (etc/user/)                               │
│   - User-specific customizations                                │
├─────────────────────────────────────────────────────────────────┤
│ Layer 2: Project Config (etc/project/ or .noa/)                │
│   - Project-specific settings                                   │
├─────────────────────────────────────────────────────────────────┤
│ Layer 1: Defaults (etc/defaults/)                              │
│   - Safe defaults, base configuration                           │
└─────────────────────────────────────────────────────────────────┘
```

### Technology Stack

1. **config-rs** (Rust) - Layered configuration with 12-factor support
   - 3.1k stars, 36.5k dependents, battle-tested
   - Supports TOML, YAML, JSON, INI, environment
   - Clean priority-based source composition

2. **mlua** (Rust + Lua) - Portable scripting for config transforms
   - 2.5k stars, 3.8k dependents
   - Lua 5.4 / LuaJIT / Luau support
   - Sandboxing for safe config scripts
   - Cross-platform (Windows/macOS/Linux/WASM)

3. **Directory Consolidation**
   - Relocatable dirs → `etc/dotfiles/` with symlinks
   - Workspace-required dirs → remain in root
   - XDG compliance for all tools via env vars

### Implementation Components

```
noa/
├── etc/
│   ├── defaults/           # Layer 1: Base defaults (TOML/YAML)
│   ├── project/            # Layer 2: Project configs
│   ├── user/               # Layer 3: User customizations
│   ├── dotfiles/           # Consolidated relocatable dot dirs
│   │   ├── cache/          # Was .cache/
│   │   ├── claude/         # Was .claude/
│   │   ├── npm/            # Was .npm/
│   │   ├── pixi/           # Was .pixi/
│   │   └── ...
│   └── overrides/          # Config override scripts (Lua)
├── lib/
│   └── noa-config/         # Rust crate using config-rs + mlua
└── scripts/
    ├── sync-configs.ps1    # Scanner for preinstalled tool configs
    └── apply-overrides.ps1 # Apply NOA settings to system tools
```

### Config Override Scanner Design

```rust
// Pseudocode for noa-config crate
pub struct ConfigScanner {
    known_tools: Vec<ToolConfig>,
    noa_root: PathBuf,
}

impl ConfigScanner {
    /// Scan system for preinstalled tools and their config locations
    pub fn scan(&self) -> Vec<DiscoveredConfig> {
        // Check standard locations:
        // - %APPDATA% (Windows)
        // - ~/.config (Linux/macOS)
        // - Tool-specific paths
    }
    
    /// Apply NOA config overrides via symlinks, junctions, or merging
    pub fn apply_overrides(&self, strategy: OverrideStrategy) -> Result<()> {
        // Strategy: Symlink | Junction | Merge | EnvVar
    }
}
```

### Lua Config Transform Example

```lua
-- etc/overrides/vscode.lua
local config = require("noa.config")

-- Load NOA defaults
local noa_settings = config.load("etc/defaults/vscode.toml")

-- Load user overrides
local user_settings = config.load("etc/user/vscode.toml")

-- Merge with user taking precedence
return config.merge(noa_settings, user_settings, {
    -- Add runtime computed values
    ["python.defaultInterpreterPath"] = os.getenv("NOA_ROOT") .. "/opt/python/python.exe",
    ["terminal.integrated.env.windows"] = {
        ["NOA_ROOT"] = os.getenv("NOA_ROOT"),
        ["PATH"] = os.getenv("NOA_ROOT") .. "/bin;" .. os.getenv("PATH")
    }
})
```

## Consequences

### Positive

- **Single source of truth** - All config in `etc/` hierarchy
- **Portability** - Copy `etc/` to new machine, run sync script
- **Layered overrides** - Defaults → Project → User → Env
- **Tool agnostic** - config-rs handles any format
- **Scriptable** - Lua for complex transforms/computed values
- **Constitution compliant** - Everything under noa_root

### Negative

- **Complexity** - Multiple layers to debug
- **Learning curve** - Team must understand layering
- **Sync requirement** - Must run sync after tool installs

### Neutral

- Some dot directories MUST remain (`.git`, `.github`, `.vscode`)
- External tools need symlinks/junctions for discovery

## Alternatives Considered

1. **cfg-rs alone** - Lighter but less ecosystem support (49 stars vs 3.1k)
2. **Pure Lua (no Rust)** - Would require separate Lua runtime
3. **Nushell configs** - Already have `noa-env.nu`, but limited to shell
4. **chezmoi for everything** - Designed for home dir, not project configs

## Implementation Plan

### Phase 1: Directory Consolidation (Week 1)

1. Create `etc/dotfiles/` structure
2. Move relocatable dot dirs with symlinks
3. Update XDG env vars to point to new locations
4. Test tool functionality

### Phase 2: Config Crate (Week 2-3)

1. Create `lib/noa-config/` Rust crate
2. Integrate config-rs for layered loading
3. Add mlua for Lua transforms
4. Implement scanner for tool discovery

### Phase 3: Override System (Week 4)

1. Build sync script for Windows/Unix
2. Create override scripts for major tools (VS Code, git, npm, Claude)
3. Add to bootstrap/init process
4. Document in CENTRALIZED-CONFIG.md

## References

- [config-rs](https://github.com/rust-cli/config-rs) - Layered config for Rust
- [mlua](https://github.com/mlua-rs/mlua) - Lua bindings for Rust
- [12-Factor Config](https://12factor.net/config) - Environment-based config principles
- [XDG Base Directory Spec](https://specifications.freedesktop.org/basedir-spec/latest/)
- [NOA Constitution §3.1](../../CONSTITUTION.md) - Self-Contained & Autonomous
