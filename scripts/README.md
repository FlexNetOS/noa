# NOA Scripts Directory

This directory contains all NOA automation scripts organized by purpose.

## Directory Structure

```
scripts/
├── bootstrap/           # Unified bootstrap system
│   ├── bootstrap.ps1    # Main Windows entry point
│   ├── bootstrap.sh     # Main Unix entry point
│   ├── lib/             # Core libraries
│   ├── config/          # Configuration scripts
│   ├── installers/      # Tool installers
│   ├── verify/          # Verification scripts
│   ├── generators/      # Environment generators
│   └── report/          # Reporting scripts
├── setup/               # Setup and installation
│   ├── setup-noa.ps1    # Full setup (calls bootstrap)
│   ├── check-prereqs.ps1# Prerequisite checker
│   └── install-all-tools.ps1
├── powershell/          # PowerShell shims
│   └── check-prerequisites.ps1
├── bash/                # Bash shims
│   └── check-prerequisites.sh
└── deprecated/          # Deprecated scripts
```

## Quick Reference

### Bootstrap (Recommended)

```powershell
# Windows - Full setup
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools -InstallAiProviders -Verify

# Unix - Full setup
./scripts/bootstrap/bootstrap.sh --install-all-tools --install-ai-providers --verify
```

### Verification

```powershell
# Verify all installations
.\scripts\bootstrap\verify\verify-all.ps1

# Run smoke tests
.\scripts\bootstrap\verify\smoke-test.ps1

# Check cross-platform parity
.\scripts\bootstrap\verify\cross-platform-parity.ps1
```

### Prerequisites

```powershell
# Check prerequisites only
.\scripts\setup\check-prereqs.ps1

# Get feature paths (for spec-kit)
.\scripts\setup\check-prereqs.ps1 -PathsOnly -Json
```

## Script Pairs (Cross-Platform Parity) - CHK111-CHK114

**Requirement**: Every `.ps1` script has a `.sh` equivalent (or consolidated equivalent) that accepts the same arguments and returns the same exit codes.

### Core Bootstrap Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/bootstrap.ps1` | `bootstrap/bootstrap.sh` | Main bootstrap entry | `-InstallAllTools`, `-InstallAiProviders`, `-Verify` | 0=success, 1=error | ✅ |
| `bootstrap/lib/logging.ps1` | `bootstrap/lib/logging.sh` | Logging functions | N/A (library) | N/A | ✅ |
| `bootstrap/lib/platform.ps1` | `bootstrap/lib/platform.sh` | Platform detection | N/A (library) | N/A | ✅ |
| `bootstrap/lib/state.ps1` | `bootstrap/lib/state.sh` | State management | N/A (library) | N/A | ✅ |
| `bootstrap/lib/verification.ps1` | `bootstrap/lib/verification.sh` | Tool verification | N/A (library) | N/A | ✅ |
| `bootstrap/lib/download.ps1` | `bootstrap/lib/download.sh` | Download with caching | N/A (library) | N/A | ✅ |
| `bootstrap/lib/directories.ps1` | `bootstrap/lib/directories.sh` | Directory creation | N/A (library) | N/A | ✅ |

### Configuration Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/config/cache-setup.ps1` | `bootstrap/config/cache-setup.sh` | Cache directory setup | N/A | 0=success, 1=error | ✅ |
| `bootstrap/config/log-setup.ps1` | `bootstrap/config/log-setup.sh` | Log rotation setup | N/A | 0=success, 1=error | ✅ |
| `bootstrap/config/provider-cache.ps1` | `bootstrap/config/provider-cache.sh` | Provider cache setup | N/A | 0=success, 1=error | ✅ |
| `bootstrap/config/appdata-setup.ps1` | `bootstrap/config/appdata-setup.sh` | AppData directory setup | N/A | 0=success, 1=error | ✅ |

### Generator Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/generators/noa-env.ps1` | `bootstrap/generators/noa-env.sh` | Generate environment file | N/A | 0=success, 1=error | ✅ |
| `bootstrap/generators/shell-integration.ps1` | `bootstrap/generators/shell-integration.sh` | Shell profile integration | N/A | 0=success, 1=error | ✅ |

### Verification Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/verify/verify-all.ps1` | `bootstrap/verify/verify-all.sh` | Full verification | N/A | 0=success, 1=error | ✅ |
| `bootstrap/verify/smoke-test.ps1` | `bootstrap/verify/smoke-test.sh` | Smoke tests | N/A | 0=success, 1=error | ✅ |
| `bootstrap/verify-ai-providers.ps1` | `bootstrap/verify-ai-providers.sh` | AI provider verification | N/A | 0=success, 1=error | ✅ |
| `bootstrap/verify-shared-resources.ps1` | `bootstrap/verify-shared-resources.sh` | Shared resources verification | N/A | 0=success, 1=error | ✅ |

### Setup Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `setup/setup-noa.ps1` | `setup/setup-noa.sh` | Full NOA setup | `-InstallAllTools`, `-InstallAiProviders` | 0=success, 1=error | ✅ |
| `setup/check-prereqs.ps1` | `init/check-prereqs.sh` | Prerequisite check | `-Json`, `-PathsOnly`, `-RequireTasks`, `-IncludeTasks` | 0=success, 1=error | ✅ |
| `setup/install-all-tools.ps1` | `setup/install-all-tools.sh` | Install all tools | N/A | 0=success, 1=error | ✅ |
| `powershell/check-prerequisites.ps1` | `bash/check-prerequisites.sh` | Prerequisite check (shim) | `--json`, `--require-tasks`, `--include-tasks` | 0=success, 1=error | ✅ |

### Build Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `powershell/build.ps1` | `bash/build.sh` | Build all components | `-Release`, `-Test` | 0=success, 1=error | ✅ |

### Test Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `test/smoke-test-phase1.ps1` | `test/smoke-test-phase1.sh` | Phase 1 smoke tests | N/A | 0=success, 1=error | ✅ |
| `test/negative-tests-phase1.ps1` | `test/negative-tests-phase1.sh` | Phase 1 negative tests | N/A | 0=success, 1=error | ✅ |

### Installer Scripts (Consolidated)

**Note**: Many installer scripts are consolidated into single Unix scripts for efficiency (CHK111 allows "consolidated equivalent").

| PowerShell | Bash/Unix (Consolidated) | Purpose | Arguments | Exit Codes | Status |
|------------|--------------------------|---------|-----------|------------|--------|
| `bootstrap/installers/git.ps1` | `bootstrap/installers/git.sh` | Git installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/git-lfs.ps1` | `bootstrap/installers/git-lfs.sh` | Git LFS installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/gh.ps1` | `bootstrap/installers/gh.sh` | GitHub CLI installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/rust-portable.ps1` | `bootstrap/installers/rust-portable.sh` | Rust installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/go-portable.ps1` | `bootstrap/installers/go-portable.sh` | Go installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/node-portable.ps1` | `bootstrap/installers/node-portable.sh` | Node.js installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/python-portable.ps1` | `bootstrap/installers/python-portable.sh` | Python installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/protoc-portable.ps1` | `bootstrap/installers/protoc-portable.sh` | Protocol Buffers installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/rust-tools.ps1` | `bootstrap/installers/rust-tools.sh` | Rust tools installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/go-tools.ps1` | `bootstrap/installers/go-tools.sh` | Go tools installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/npm-tools.ps1` | `bootstrap/installers/npm-tools.sh` | npm tools installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/pip-tools.ps1` | `bootstrap/installers/pip-tools.sh` | pip tools installer | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/gitleaks.ps1` | `bootstrap/installers/security-tools.sh` | Security tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/trivy.ps1` | `bootstrap/installers/security-tools.sh` | Security tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/grype.ps1` | `bootstrap/installers/security-tools.sh` | Security tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/jq.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ripgrep.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/fd.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/bat.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/fzf.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/delta.ps1` | `bootstrap/installers/cli-tools.sh` | CLI tools (consolidated) | N/A | 0=success, 1=error | ✅ |

### AI Provider Installers

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/installers/ai-providers/claude-code.ps1` | `bootstrap/installers/ai-providers/claude-code.sh` | Claude Code CLI | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ai-providers/cursor-cli.ps1` | `bootstrap/installers/ai-providers/cursor-cli.sh` | Cursor CLI | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ai-providers/codex-cli.ps1` | `bootstrap/installers/ai-providers/codex-cli.sh` | Codex CLI | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ai-providers/vscode-copilot.ps1` | `bootstrap/installers/ai-providers/vscode-copilot.sh` | VS Code Copilot | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ai-providers/git-cli-provider.ps1` | `bootstrap/installers/ai-providers/git-cli-provider.sh` | Git CLI Provider | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/ai-providers/abacus-cli.ps1` | `bootstrap/installers/ai-providers/abacus-cli.sh` | Abacus CLI | N/A | 0=success, 1=error | ✅ |

### Shared Resources Scripts

| PowerShell | Bash/Unix | Purpose | Arguments | Exit Codes | Status |
|------------|-----------|---------|-----------|------------|--------|
| `bootstrap/installers/shared-resources/create-directories.ps1` | `bootstrap/installers/shared-resources/create-directories.sh` | Create shared dirs | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/shared-resources/execution-memory.ps1` | `bootstrap/installers/shared-resources/execution-memory.sh` | Init execution memory | N/A | 0=success, 1=error | ✅ |
| `bootstrap/installers/shared-resources/provider-sync.ps1` | `bootstrap/installers/shared-resources/provider-sync.sh` | Provider state sync | N/A | 0=success, 1=error | ✅ |

### Cross-Platform Parity Verification (CHK111-CHK113)

**CHK111**: ✅ Every `.ps1` script has a `.sh` equivalent (or consolidated equivalent)
**CHK112**: ✅ Mirrored scripts accept the same arguments
**CHK113**: ✅ Mirrored scripts return the same exit codes

**Verification**: Run `bootstrap/verify/cross-platform-parity.ps1` or `.sh` to verify all script pairs.

## Environment Variables

All scripts respect these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_ROOT` | NOA root directory | Auto-detect |
| `NOA_BIN` | Binary directory | `$NOA_ROOT/bin` |
| `NOA_OPT` | Optional packages | `$NOA_ROOT/opt` |
| `NOA_CACHE` | Cache directory | `$NOA_ROOT/cache` |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error / Missing prerequisites |
| 2 | Invalid arguments |
| 3 | Permission denied |
| 4 | Network error |

## Logging

All scripts log to `logs/bootstrap/` with timestamps.
Set `$env:NOA_DEBUG = "1"` for verbose output.
