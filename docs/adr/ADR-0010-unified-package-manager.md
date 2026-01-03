# ADR-0010: Unified Cross-Platform Package Manager and Shell

## Status
**Proposed** | 2026-01-03

## Context

NOA's current build tooling has platform-specific gaps:
- **Windows**: Missing development libraries (e.g., libcurl headers/libs)
- **Shell fragmentation**: PowerShell (Windows), Bash (Linux/macOS)
- **Multiple package managers**: vcpkg, npm, pnpm, cargo, pip, apt/brew/winget

This creates issues like the recent llama.cpp build failure where CURL development
libraries were unavailable despite `curl.exe` being present.

### Requirements from CONSTITUTION.md
- §3.1: Self-Contained & Autonomous - all tools under `noa_root`
- §3.2: Local-First & Offline-Capable - cached packages work without network
- §3.5: Test Everything, Trust Nothing - reproducible builds via lockfiles

## Decision

Adopt **Pixi** as the unified package manager and **Nushell** as the unified shell.

### 1. Pixi (Package Manager)

**Source**: https://github.com/prefix-dev/pixi (v0.62.2+)

| Capability | Description |
|------------|-------------|
| Multi-language | Python, C/C++, Rust, Node.js, Java, R |
| Cross-platform | Windows, Linux, macOS (incl. ARM) |
| Lock files | `pixi.lock` ensures reproducible environments |
| Global tools | Replace apt/brew/winget with `pixi global` |
| Offline cache | Packages cached in `$NOA_ROOT/cache/pixi` |
| Task runner | Built-in task system (replaces Makefile complexity) |

**Key packages available via conda-forge:**
- `libcurl`, `zlib`, `openssl` - C/C++ libraries with headers
- `cmake`, `ninja`, `clang` - Build tools
- `rust`, `cargo-edit` - Rust toolchain
- `nodejs`, `pnpm` - Node.js ecosystem
- `python`, `pytorch`, `scikit-learn` - Python ML stack
- `llama.cpp` - Direct installation possible!

### 2. Nushell (Unified Shell)

**Source**: https://github.com/FlexNetOS/nushell (fork of nushell/nushell)

| Capability | Description |
|------------|-------------|
| Cross-platform | Identical behavior on Win/Linux/macOS |
| Structured data | Tables, records, lists - not text parsing |
| Rust-native | Aligns with NOA's Rust core |
| MCP support | Built-in MCP server (nushell#17200) |
| Pixi integration | Native completion support |
| Plugin system | Extensible for NOA-specific commands |

### 3. pnpm (Node.js Packages)

Keep pnpm for Node.js packages but install it via Pixi:
```bash
pixi global install pnpm
```

This ensures pnpm is available on all platforms without manual installation.

## Installation Layout

```
$NOA_ROOT/
├── opt/
│   ├── pixi/           # Pixi installation
│   │   └── bin/
│   │       └── pixi    # pixi executable
│   └── nushell/        # Nushell installation
│       └── bin/
│           └── nu      # nu executable
├── cache/
│   └── pixi/           # Pixi package cache
│       ├── envs/       # Environment prefixes
│       └── pkgs/       # Downloaded packages
├── bin/
│   ├── nu.cmd          # Nushell wrapper (Windows)
│   ├── nu              # Nushell wrapper (Unix)
│   ├── pixi.cmd        # Pixi wrapper (Windows)
│   └── pixi            # Pixi wrapper (Unix)
└── pixi.toml           # Root workspace manifest
```

## Root Workspace Configuration

```toml
# $NOA_ROOT/pixi.toml
[project]
name = "noa"
version = "0.1.0"
description = "NOA Platform Development Environment"
channels = ["conda-forge"]
platforms = ["win-64", "linux-64", "osx-64", "osx-arm64"]

[dependencies]
# Core build tools
cmake = ">=3.28"
ninja = ">=1.11"
rust = ">=1.82"

# C/C++ libraries for llama.cpp
libcurl = ">=8.0"
zlib = ">=1.3"
openssl = ">=3.0"

# Node.js ecosystem
nodejs = ">=20"
pnpm = ">=9.0"

# Python (for ML/AI)
python = ">=3.11"

[feature.llama]
dependencies = { llama-cpp = "*" }

[feature.ml]
dependencies = { pytorch = "*", scikit-learn = "*" }

[tasks]
build-core = "cd sys/core && cargo build --workspace"
test-core = "cd sys/core && cargo test --workspace"
build-llama = "cd llama.cpp && cmake -B build && cmake --build build -j"
```

## Migration Path

### Phase 1: Install Pixi + Nushell (Week 1)
1. Install Pixi to `$NOA_ROOT/opt/pixi`
2. Install Nushell via Pixi: `pixi global install nushell`
3. Create `pixi.toml` with base dependencies
4. Add wrappers in `$NOA_ROOT/bin/`

### Phase 2: Migrate Build Scripts (Week 2)
1. Convert `noa-init.ps1` to `noa-init.nu`
2. Add Pixi tasks for common operations
3. Update AGENTS.md with new commands

### Phase 3: Deprecate Platform-Specific Tools (Week 3)
1. Remove vcpkg dependency (replaced by Pixi)
2. Document migration from PowerShell to Nushell
3. Update CI/CD to use Pixi environments

## Consequences

### Positive
- **Single shell** across all platforms
- **Reproducible builds** via `pixi.lock`
- **Offline development** with cached packages
- **Simplified onboarding** - one install command
- **No more "missing library" errors** - Pixi provides headers + libs

### Negative
- Learning curve for Nushell syntax
- Initial migration effort from existing scripts
- Pixi is newer (less battle-tested than conda)

### Neutral
- vcpkg still available as fallback if needed
- PowerShell/Bash still work alongside Nushell

## References

- [Pixi Documentation](https://pixi.sh/)
- [Nushell Book](https://www.nushell.sh/book/)
- [conda-forge](https://conda-forge.org/)
- [FlexNetOS/nushell Fork](https://github.com/FlexNetOS/nushell)
