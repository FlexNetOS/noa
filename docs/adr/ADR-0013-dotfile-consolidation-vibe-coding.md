# ADR-0013: Dot Directory Consolidation for Vibe Coding

**Status**: Proposed  
**Date**: 2026-01-03  
**Context**: NOA Platform workspace configuration management  
**Decision**: Implement comprehensive dot directory consolidation

## Problem Statement

The NOA root directory contains **~24GB of dot directory bloat** with significant overlap:

| Directory | Size | Category | Issue |
|-----------|------|----------|-------|
| `.git` | 18.9 GB | VCS | objects=16GB, lfs=2.7GB - needs gc/prune |
| `.pixi` | 3.5 GB | Env | Full conda envs - can use shared cache |
| `.vs` | 902 MB | IDE | Visual Studio cache - can be relocated |
| `.cursor-server` | 112 MB | IDE | Cursor server cache - duplicates vscode |
| `.npm` | 13 MB | Pkg | Can use shared cache |
| `.cache` | 5 MB | Cache | Various tool caches |
| `.vscode` | 0.04 MB | IDE | Config - overlaps with .cursor |
| `.cursor` | 0.07 MB | IDE | Config - overlaps with .vscode |
| `.github` | 0.3 MB | CI/CD | Workflows, templates - REQUIRED |

### Root Cause Analysis

1. **IDE Proliferation**: VSCode, Cursor, and VS each create independent directories
2. **No Shared Cache**: pixi, npm, pip each cache packages independently  
3. **Git History Bloat**: Accumulation of unreferenced objects and LFS files
4. **XDG Non-Compliance**: Many tools ignore XDG Base Directory spec
5. **Vibe Coding Pattern**: AI tools (Aider, Roo Code, Copilot) add their own configs

## Research Summary

### Vibe Coding (Andrej Karpathy, Feb 2025)
> "A new kind of coding where you fully give in to the vibes... The code grows beyond my usual comprehension"

Modern AI-assisted development creates config sprawl as each tool maintains state.

### Relevant Tools & Specs

| Solution | Stars | Purpose | Applicability |
|----------|-------|---------|---------------|
| **xdg-ninja** | 3.1k | Audit $HOME for XDG violations | HIGH - adapt for NOA |
| **DevPod** | 14.5k | Unified dev containers | MEDIUM - for isolation |
| **devcontainer.json** | 5k (spec) | IDE-agnostic workspace | HIGH - unify configs |
| **XDG Base Dir** | Standard | Config/cache/data separation | HIGH - implement |
| **chezmoi** | 15k+ | Dotfile management | MEDIUM - already installed |

### XDG Base Directory Mapping

```
XDG_CONFIG_HOME  → N:\noa\etc\config\   (was ~/.config)
XDG_CACHE_HOME   → N:\noa\cache\        (was ~/.cache)
XDG_DATA_HOME    → N:\noa\data\local\   (was ~/.local/share)
XDG_STATE_HOME   → N:\noa\data\state\   (was ~/.local/state)
XDG_RUNTIME_DIR  → N:\noa\tmp\runtime\  (was /run/user/$UID)
```

## Decision

### Phase 1: Required Workspace Directories (Cannot Move)

These directories are semantically tied to repository identity:

| Directory | Reason |
|-----------|--------|
| `.git` | Git identity - can only optimize, not move |
| `.github` | GitHub Actions, templates - repo-specific |
| `.gitignore` | Git ignore rules - must be at root |
| `.gitattributes` | Git attributes - must be at root |
| `.gitmodules` | Submodule config - must be at root |
| `.editorconfig` | Editor config - must be at root |

### Phase 2: Relocatable IDE Directories → `etc/ide/`

Create unified IDE layer:

```
N:\noa\etc\ide\
├── devcontainer.json          # Unified container spec (VSCode, Cursor, DevPod)
├── settings.json              # Shared IDE settings
├── extensions.json            # Shared extension list
├── tasks.json                 # Shared tasks
├── launch.json                # Shared debug configs
├── keybindings.json           # Shared keybindings
└── snippets/                  # Shared snippets
```

Replace with junctions:
- `.vscode/` → `etc/ide/vscode/` (workspace-specific overrides)
- `.cursor/` → `etc/ide/cursor/` (workspace-specific overrides)

### Phase 3: Cache Consolidation → `cache/`

```
N:\noa\cache\
├── git/                       # Git shared cache (GIT_OBJECT_DIRECTORY)
├── pixi/                      # Pixi package cache (PIXI_CACHE_DIR)
├── npm/                       # NPM cache (npm_config_cache)
├── pip/                       # Pip cache (PIP_CACHE_DIR)
├── cargo/                     # Cargo registry (CARGO_HOME)
├── go/                        # Go modules (GOMODCACHE)
├── huggingface/               # HF models (HF_HOME)
├── ollama/                    # Ollama models (OLLAMA_MODELS)
└── ide/                       # IDE caches
    ├── vscode/                # VS Code cache
    ├── cursor/                # Cursor cache
    └── vs/                    # Visual Studio cache
```

### Phase 4: Git Maintenance

Implement automated maintenance to reduce 18.9GB → target <5GB:

```powershell
# Scheduled git maintenance
git gc --aggressive --prune=now
git reflog expire --expire=now --all
git repack -a -d --depth=250 --window=250
git lfs prune

# Object deduplication
git maintenance run --task=gc --task=commit-graph --task=prefetch
```

### Phase 5: Devcontainer Unification

Single `etc/devcontainer/devcontainer.json`:

```json
{
  "name": "NOA Development",
  "image": "mcr.microsoft.com/devcontainers/rust:1",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {},
    "ghcr.io/devcontainers/features/python:1": {},
    "ghcr.io/devcontainers/features/rust:1": {}
  },
  "customizations": {
    "vscode": {
      "settings": { "$ref": "../ide/settings.json" },
      "extensions": [ "$ref": "../ide/extensions.json" ]
    },
    "cursor": {
      "settings": { "$ref": "../ide/settings.json" }
    }
  },
  "mounts": [
    "source=${localEnv:NOA_ROOT}/cache,target=/cache,type=bind"
  ],
  "remoteEnv": {
    "XDG_CACHE_HOME": "/cache",
    "PIXI_CACHE_DIR": "/cache/pixi",
    "CARGO_HOME": "/cache/cargo"
  }
}
```

## Implementation

### New Scripts

1. **`scripts/noa-xdg-audit.ps1`** - Audit dot directories for XDG compliance
2. **`scripts/noa-git-maintenance.ps1`** - Automated git gc/prune/lfs cleanup
3. **`scripts/noa-cache-consolidate.ps1`** - Relocate caches with junctions
4. **`scripts/noa-ide-unify.ps1`** - Create unified IDE config layer

### Environment Variables (add to noa-env.ps1)

```powershell
# XDG Base Directory
$env:XDG_CONFIG_HOME = "$env:NOA_ROOT\etc\config"
$env:XDG_CACHE_HOME = "$env:NOA_ROOT\cache"
$env:XDG_DATA_HOME = "$env:NOA_ROOT\data\local"
$env:XDG_STATE_HOME = "$env:NOA_ROOT\data\state"

# Tool-specific cache overrides
$env:PIXI_CACHE_DIR = "$env:XDG_CACHE_HOME\pixi"
$env:PIP_CACHE_DIR = "$env:XDG_CACHE_HOME\pip"
$env:npm_config_cache = "$env:XDG_CACHE_HOME\npm"
$env:CARGO_HOME = "$env:XDG_DATA_HOME\cargo"
$env:RUSTUP_HOME = "$env:XDG_DATA_HOME\rustup"
$env:GOMODCACHE = "$env:XDG_CACHE_HOME\go\mod"
$env:HF_HOME = "$env:XDG_CACHE_HOME\huggingface"
$env:OLLAMA_MODELS = "$env:XDG_CACHE_HOME\ollama\models"

# IDE cache relocation
$env:VSCODE_EXTENSIONS = "$env:XDG_DATA_HOME\vscode\extensions"
$env:CURSOR_EXTENSIONS = "$env:XDG_DATA_HOME\cursor\extensions"
```

## Expected Results

| Metric | Before | After | Reduction |
|--------|--------|-------|-----------|
| Total dot dir size | 24 GB | ~5 GB | 80% |
| Number of dot dirs | 14+ | 6 | 57% |
| IDE config files | Duplicated | Unified | Single source |
| Cache sharing | None | Full | Cross-project |

## Alternatives Considered

1. **Full Containerization (DevPod)**: Overkill for local development
2. **Complete XDG Migration**: Would break some tools expecting `.` directories
3. **Symlink Farm**: Less robust than junctions on Windows
4. **Accept Bloat**: Violates Constitution §3.1 (self-contained)

## References

- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/)
- [ArchWiki: XDG Base Directory](https://wiki.archlinux.org/title/XDG_Base_Directory)
- [xdg-ninja](https://github.com/b3nj5m1n/xdg-ninja)
- [DevPod](https://github.com/loft-sh/devpod)
- [devcontainer.json spec](https://containers.dev/)
- [Vibe Coding (Karpathy)](https://x.com/karpathy/status/1886192184808149383)
- [Aider](https://github.com/Aider-AI/aider) - Vibe coding tool
- [Roo Code](https://github.com/RooCodeInc/Roo-Code) - VS Code AI agent
