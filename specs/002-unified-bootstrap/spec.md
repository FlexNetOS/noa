# Specification: NOA Unified Bootstrap Installer

**Feature ID**: 002-unified-bootstrap
**Status**: Draft
**Created**: 2025-12-08
**Last Updated**: 2025-12-08

---

## Problem Statement

NOA currently has **17+ scattered setup scripts** across multiple directories:
- `scripts/setup.ps1`, `scripts/setup/setup-noa.ps1`, `scripts/setup/check-prereqs.ps1`
- `scripts/download-static-binaries.ps1`, `scripts/verify-environment.ps1`
- `init/check-prereqs.sh`, `init/noa-init`
- Various shims and wrappers

This fragmentation causes:
1. **Confusion**: Users don't know which script to run
2. **Incomplete installs**: Missing dependencies when scripts run out of order
3. **Duplication**: Same logic implemented multiple times
4. **Maintenance burden**: Updates require changes in multiple places
5. **Platform inconsistency**: Windows and Unix scripts diverge

---

## Solution Overview

Create a **100% self-contained bootstrap system** that:
1. Installs ALL tools to `noa_root` - **ZERO system-wide installations**
2. Downloads portable toolchains (Rust, Go, Node, Python) directly
3. Configures environment variables to point to `noa_root` directories
4. Works identically on Windows (PowerShell) and Unix (Bash)
5. Provides clear progress feedback and error handling
6. Supports idempotent re-runs (safe to run multiple times)
7. Works offline with pre-cached archives

### Key Innovation: Portable Toolchains

Instead of relying on system-wide package managers (winget, brew, apt), this solution:

| Traditional Approach | Self-Contained Approach |
|---------------------|------------------------|
| `winget install Rustlang.Rustup` (system-wide) | Download rustup-init → install to `noa_root/opt/rust/` |
| `winget install GoLang.Go` (system-wide) | Download go.zip → extract to `noa_root/opt/go/` |
| `winget install OpenJS.NodeJS.LTS` (system-wide) | Download node.zip → extract to `noa_root/opt/node/` |
| `winget install Python.Python.3.12` (system-wide) | Download embed zip → extract to `noa_root/opt/python/` |

### Environment Variable Strategy

All toolchain homes are redirected to `noa_root`:

```powershell
$env:RUSTUP_HOME = "$NOA_ROOT/opt/rust/rustup"
$env:CARGO_HOME  = "$NOA_ROOT/opt/rust/cargo"
$env:GOROOT      = "$NOA_ROOT/opt/go"
$env:GOPATH      = "$NOA_ROOT/opt/go/workspace"
$env:NODE_PATH   = "$NOA_ROOT/opt/node/node_modules"
$env:PYTHONHOME  = "$NOA_ROOT/opt/python"
$env:VIRTUAL_ENV = "$NOA_ROOT/opt/venv"
```

---

## User Stories

### US1: First-Time Setup (P1)

**As a** new NOA developer
**I want** to run a single command to set up my entire development environment
**So that** I can start contributing without manual setup steps

**Acceptance Criteria**:
- Single command: `.\bootstrap.ps1` (Windows) or `./bootstrap.sh` (Unix)
- All build toolchains installed (Rust, Go, Node, Python, protoc)
- All quality tools installed (rustfmt, clippy, golangci-lint, eslint, ruff)
- All portable utilities in `noa_root/bin/`
- Directory structure created
- Environment configured
- Verification passes

### US2: Incremental Update (P1)

**As a** returning NOA developer
**I want** to update my environment when requirements change
**So that** I stay current without full reinstall

**Acceptance Criteria**:
- Re-running bootstrap skips already-installed tools
- Version upgrades detected and offered
- New tools added automatically
- Existing configuration preserved

### US3: Offline Installation (P2)

**As a** developer in a restricted network environment
**I want** to bootstrap from cached/vendored tools
**So that** I can work without internet access

**Acceptance Criteria**:
- Bootstrap can use pre-downloaded tool packages
- Cache location configurable
- Clear error when required tool not cached

### US4: CI/CD Environment (P2)

**As a** CI pipeline
**I want** to verify tool availability without installing
**So that** builds fail fast with clear error messages

**Acceptance Criteria**:
- `--check-only` mode validates without installing
- JSON output for machine parsing
- Exit codes indicate specific failure types
- Compatible with GitHub Actions, GitLab CI

---

## Functional Requirements

### FR-001: Single Entry Point
The system MUST provide a single entry point script for each platform:
- Windows: `scripts/bootstrap/bootstrap.ps1`
- Unix: `scripts/bootstrap/bootstrap.sh`

### FR-002: Dependency-Ordered Installation
The system MUST install tools in correct dependency order:
1. Core prerequisites (Git, package managers)
2. Build toolchains (Rust, Go, Node, Python, protoc)
3. Quality tools (require build toolchains)
4. Security tools
5. Self-contained utilities

### FR-003: Platform Detection
The system MUST detect the current platform:
- Windows (native, WSL1, WSL2)
- macOS (Intel, Apple Silicon)
- Linux (Debian-based, RHEL-based, Arch-based)

### FR-004: Version Validation
The system MUST validate installed tool versions against minimum requirements:
- Rust >= 1.83.0
- Go >= 1.23.0
- Node >= 20.0.0
- Python >= 3.12.0
- protoc >= 28.0.0

### FR-005: Self-Contained Installation (§3.1)
The system MUST install portable tools to `noa_root/bin/`:
- jq, ripgrep (rg), fd, bat, fzf
- gitleaks, trivy, grype

### FR-006: System Toolchain Exception (§3.1)
The system MUST install build toolchains system-wide (per language requirements):
- Rust (rustup manages RUSTUP_HOME)
- Go (GOROOT system-wide, GOPATH in noa_root)
- Node.js (npm cache in noa_root)
- Python (venv in noa_root)
- protoc

### FR-007: Directory Structure Creation
The system MUST create the complete NOA directory structure per spec.

### FR-008: Environment Configuration
The system MUST generate environment files:
- `noa-env.ps1` / `.noa-env` (environment variables)
- `noa-profile.ps1` / `.noa-profile` (shell configuration)
- `config/noa.json` (JSON configuration)

### FR-009: Profile Integration
The system MUST optionally integrate with user shell profiles:
- PowerShell `$PROFILE.CurrentUserAllHosts`
- Bash `~/.bashrc`, `~/.bash_profile`
- Zsh `~/.zshrc`

### FR-010: Verification
The system MUST provide comprehensive verification:
- All directories exist
- All tools installed and accessible
- Correct versions installed
- Environment variables set
- PATH configured correctly

### FR-011: Logging
The system MUST log all actions to `logs/bootstrap-{timestamp}.log`.

### FR-012: Idempotent Execution
The system MUST be safe to run multiple times:
- Skip already-installed tools (correct version)
- Preserve existing configuration
- Update only what's needed

### FR-013: Error Recovery
The system MUST handle errors gracefully:
- Clear error messages
- Partial progress preserved
- Retry guidance provided

### FR-014: Kernel Setup (Linux/WSL)
The system MUST configure kernel-level features for P2P:
- Load required kernel modules
- Configure sysctl parameters
- Setup namespace isolation

---

## Non-Functional Requirements

### NFR-001: Performance
- Full bootstrap < 10 minutes on standard hardware
- Incremental update < 30 seconds

### NFR-002: Reliability
- 99% success rate on supported platforms
- Clear diagnostics for failures

### NFR-003: Maintainability
- Tool definitions in single JSON config
- DRY principles (no duplicate logic)
- Comprehensive test coverage

---

## Constitutional Compliance

### Data Locality & Offline Behavior

- **Offline Support**: ☑ Degraded (requires network for initial tool downloads, can cache)
- **Data Residency**: All data stored under `noa_root` directory? ☑ Yes (except system toolchains)
- **External Dependencies**: Build toolchains (system-wide), GitHub releases (for portable tools)

### Agent Orchestration

- **Responsible Agents**: Bootstrap Agent (future)
- **Multi-SLM Compatibility**: ☑ N/A (infrastructure layer)
- **Orchestration Pattern**: Sequential phases with parallel tasks within phases

### Memory & P2P Considerations

- **Memory Persistence**: Installation state tracked in `.noa` marker file
- **P2P Resource Sharing**: ☑ N/A (local setup only)
- **Cross-Device Sync**: Tool versions recorded for consistency

### Constitutional Flow

| Level | Document | Link |
|-------|----------|------|
| Goal | Self-Contained Autonomous OS | [constitution.md](../../project-mgmt/spec-kit/memory/constitution.md) |
| Policy | §3.1 Self-Contained | [constitution.md#3.1](../../project-mgmt/spec-kit/memory/constitution.md) |
| Rule | All paths under noa_root | [spec.md](../001-noa-seed-foundation/spec.md) |

---

## Success Criteria

- **SC-001**: Single command completes full setup on Windows and Unix
- **SC-002**: All 15+ tools installed with correct versions
- **SC-003**: Verification passes with 0 errors
- **SC-004**: Re-run completes in < 30 seconds (nothing to do)
- **SC-005**: Works in GitHub Actions CI environment

