# ADR-0011: Modern Development Tooling Integration

## Status
**Accepted** | 2026-01-03

## Context

Following ADR-0010 (Unified Package Manager), NOA now has Pixi and Nushell as foundational tools. However, modern "vibe coding" workflows require additional specialized tools:

1. **Tool/Runtime Version Management**: Need to manage multiple runtime versions (Node.js, Python, etc.)
2. **Directory-Based Environment Switching**: Automatic environment changes per project
3. **AI-Assisted Terminal Coding**: Direct AI integration for pair programming
4. **Dotfile Management**: Centralized, portable configuration across machines
5. **XDG Compliance Auditing**: Detect and remediate policy violations

### Constitutional Requirements
- §3.1: All code, configuration, and state MUST resolve under `noa_root`
- §3.2: Core workflows MUST work offline
- §3.5: All inputs, outputs, and state changes MUST be verifiable
- §3.6: All significant decisions MUST be logged

## Decision

Integrate the following modern development tools, all configured for §3.1 compliance:

### 1. mise (Tool/Environment/Task Manager)

**Source**: https://github.com/jdx/mise (22.7k stars, v2025.12.13)

| Capability | Description |
|------------|-------------|
| Version management | Replaces asdf, nvm, pyenv, rbenv |
| Task runner | Project-level task definitions |
| Env variables | Directory-scoped environment |
| Plugin system | Extensible for custom tools |

**Installation**:
- Binary: `N:\noa\opt\mise\bin\mise.exe`
- Config: `N:\noa\etc\mise\`
- Cache: `N:\noa\cache\mise\`
- State: `N:\noa\data\state\mise\`
- Wrapper: `N:\noa\bin\mise.cmd`

**Environment Variables**:
```bash
MISE_DATA_DIR=$NOA_ROOT/opt/mise
MISE_CONFIG_DIR=$NOA_ROOT/etc/mise
MISE_CACHE_DIR=$NOA_ROOT/cache/mise
MISE_STATE_DIR=$NOA_ROOT/data/state/mise
```

### 2. direnv (Directory-Based Environment)

**Source**: https://github.com/direnv/direnv (14.5k stars, v2.37.1)

| Capability | Description |
|------------|-------------|
| Auto-activation | Loads `.envrc` on directory change |
| Shell integration | Works with bash, zsh, fish, nushell |
| Security | Requires explicit `direnv allow` |

**Installation**:
- Via Pixi: `pixi add direnv`
- Binary: `N:\noa\.pixi\envs\default\bin\direnv.exe`
- Wrapper: `N:\noa\bin\direnv.cmd`

**Nushell Integration** (in `env.nu`):
```nu
$env.config.hooks.env_change.PWD ++= [{||
    let direnv_path = ($noa_root | path join ".pixi" "envs" "default" "bin" "direnv.exe")
    ^$direnv_path export json | from json | default {} | load-env
}]
```

### 3. aider (AI Pair Programming)

**Source**: https://github.com/paul-gauthier/aider (39.4k stars, v0.86.1)

| Capability | Description |
|------------|-------------|
| Terminal AI | Direct code editing from terminal |
| Git-aware | Automatic commits for changes |
| Multi-model | Supports Claude, GPT-4, local models |
| Context-aware | Understands project structure |

**Installation**:
- Via Pixi: `pixi add aider-chat`
- Config: `N:\noa\etc\aider\`
- Cache: `N:\noa\cache\aider\`
- Wrapper: `N:\noa\bin\aider.cmd`

**Environment Variables**:
```bash
AIDER_HOME=$NOA_ROOT/etc/aider
AIDER_CACHE=$NOA_ROOT/cache/aider
```

### 4. chezmoi (Dotfile Management)

**Source**: https://github.com/twpayne/chezmoi (14k stars, v2.68.1)

| Capability | Description |
|------------|-------------|
| Multi-machine | Sync dotfiles across devices |
| Templates | Go template support for machine-specific config |
| Encryption | Built-in secrets management |
| Git-backed | Version controlled dotfiles |

**Installation**:
- Via Pixi: `pixi add chezmoi`
- Source: `N:\noa\etc\dotfiles\` (CHEZMOI_SOURCE_DIR)
- Config: `N:\noa\etc\chezmoi\chezmoi.toml`
- Cache: `N:\noa\cache\chezmoi\`
- Wrapper: `N:\noa\bin\chezmoi.cmd`

**Configuration** (`chezmoi.toml`):
```toml
[sourceDir]
path = "N:/noa/etc/dotfiles"

[destDir]
path = "N:/noa"

[git]
autoCommit = true
autoPush = false
```

### 5. xdg-ninja (XDG Compliance Audit)

**Source**: https://github.com/b3nj5m1n/xdg-ninja (3.1k stars)

| Capability | Description |
|------------|-------------|
| Audit | Scans home for XDG violations |
| Remediation | Provides fix instructions |
| Database | 600+ program configurations |

**Installation**:
- Repository: `N:\noa\opt\xdg-ninja\` (git clone)
- PowerShell Adaptation: `N:\noa\scripts\xdg-ninja.ps1`
- Wrapper: `N:\noa\bin\xdg-ninja.cmd`

**Usage**:
```powershell
# Console output
N:\noa\bin\xdg-ninja.cmd

# JSON output for automation
N:\noa\bin\xdg-ninja.cmd -OutputFormat json

# Markdown report
N:\noa\bin\xdg-ninja.cmd -OutputFormat markdown > audit.md
```

## Tool Coordination Strategy

```
┌─────────────────────────────────────────────────────────────────┐
│                    NOA Development Environment                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐      │
│  │  Pixi   │───▶│  mise   │───▶│ direnv  │───▶│ Project │      │
│  │ (pkgs)  │    │ (tools) │    │  (env)  │    │  (code) │      │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘      │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐      │
│  │ chezmoi │    │ aider   │    │  shell  │    │  IDE    │      │
│  │(dotfile)│    │  (AI)   │    │(nushell)│    │(vscode) │      │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘      │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              xdg-ninja (compliance audit)                  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Alternatives Considered

### Tool Version Managers
| Tool | Stars | Reason Not Primary |
|------|-------|-------------------|
| asdf | 21k | Slower than mise, no native Windows |
| nvm | 80k | Node.js only |
| pyenv | 39k | Python only |
| **mise** | 22k | ✅ Unified, fast, all runtimes |

### Dotfile Managers
| Tool | Stars | Reason Not Primary |
|------|-------|-------------------|
| GNU Stow | 24k | Symlink-only, no templates |
| yadm | 5k | Less active development |
| home-manager | 9k | Requires Nix ecosystem |
| **chezmoi** | 14k | ✅ Templates, encryption, cross-platform |

### AI Coding Tools
| Tool | Stars | Reason Not Primary |
|------|-------|-------------------|
| Cursor | N/A | Full IDE replacement |
| Cline | 56k | VS Code extension (already available) |
| Claude Code | N/A | Cloud-only, no terminal |
| **aider** | 39k | ✅ Terminal-native, any editor |

## Consequences

### Positive
1. **Unified workflow**: Single set of tools across platforms
2. **Constitution compliance**: All paths under `noa_root`
3. **Reproducibility**: Lock files and version pinning
4. **AI-augmented**: aider provides terminal-native AI coding
5. **Portable**: chezmoi enables machine-independent configuration
6. **Auditable**: xdg-ninja enforces XDG compliance

### Negative
1. **Learning curve**: Multiple new tools to master
2. **Maintenance**: Tool updates require wrapper updates
3. **Duplication**: Some overlap between mise and pixi tasks

### Risks
1. **Tool abandonment**: Mitigated by choosing high-star, active projects
2. **Breaking changes**: Mitigated by version pinning in pixi.lock
3. **Windows compatibility**: Mitigated by thorough testing

## Implementation Checklist

- [x] Install mise (v2025.12.13)
- [x] Configure mise environment variables
- [x] Create mise.cmd wrapper
- [x] Install direnv (v2.37.1) via pixi
- [x] Configure Nushell direnv hook
- [x] Create direnv.cmd wrapper
- [x] Install aider-chat (v0.86.1) via pixi
- [x] Configure AIDER_HOME and AIDER_CACHE
- [x] Create aider.cmd wrapper
- [x] Install chezmoi (v2.68.1) via pixi
- [x] Create chezmoi.toml configuration
- [x] Create chezmoi.cmd wrapper
- [x] Clone xdg-ninja repository
- [x] Create PowerShell adaptation (xdg-ninja.ps1)
- [x] Run initial XDG audit (19 violations found)
- [x] Create xdg-ninja.cmd wrapper

## References

- [ADR-0010: Unified Package Manager](./ADR-0010-unified-package-manager.md)
- [mise Documentation](https://mise.jdx.dev/)
- [direnv Documentation](https://direnv.net/)
- [aider Documentation](https://aider.chat/)
- [chezmoi Documentation](https://www.chezmoi.io/)
- [xdg-ninja Repository](https://github.com/b3nj5m1n/xdg-ninja)

## Decision Record

| Date | Author | Change |
|------|--------|--------|
| 2026-01-03 | AI (noa+ai@flexnetos.com) | Initial decision |
