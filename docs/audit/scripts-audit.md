# Script Audit Report

This document contains the analysis of all scripts in the NOA workspace, organized by phase.

**Audit Date**: 2025-12-31  
**Total Scripts**: 174  
**Audit Status**: In Progress

---

## Phase 1: Core Entry Points

### 1.1 bootstrap.ps1

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/bootstrap.ps1` |
| **Lines** | 475 |
| **Purpose** | Main Windows bootstrap - single entry point for complete NOA environment setup |
| **Cross-Platform Pair** | `bootstrap.sh` ✅ |

**Description**: Creates directory structure, checks/installs prerequisites (portable toolchains), downloads self-contained utilities, configures kernel parameters, generates environment configuration. Constitution §3.1 compliant.

**Parameters**:
- `-NoaRoot` - Root directory (default: N:\noa or auto-detect)
- `-SkipKernel` - Skip kernel/networking configuration
- `-SkipServices` - Skip service setup
- `-Force` - Force reinstall all tools
- `-InstallAllTools` - Install all toolchains
- `-InstallAiProviders` - Install AI provider CLIs
- `-Verify` - Run verification
- `-Offline` - Offline mode

**References**:
- `specs/001-noa-seed-foundation/tasks.md` (B001, B078)
- `specs/001-noa-seed-foundation/checklists/phase0-quality-report.md`
- `sys/kernel/windows/sandbox/noa.wsb` (WSB sandbox)
- `Makefile` (via install-tools target)

**Recommendation**: ✅ **KEEP** - Core entry point, well-documented, actively referenced

---

### 1.2 bootstrap.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/bootstrap.sh` |
| **Lines** | 354 |
| **Purpose** | Main Unix/WSL bootstrap - single entry point for complete NOA environment setup |
| **Cross-Platform Pair** | `bootstrap.ps1` ✅ |

**Description**: Unix equivalent of bootstrap.ps1. Creates directory structure, checks prerequisites, downloads static binaries. Detects platform (linux, wsl1, wsl2, macos).

**Parameters**:
- `--skip-kernel` - Skip kernel module/param setup
- `--skip-services` - Skip service setup
- `--force` - Force reinstall all tools
- `--help` - Show help

**References**:
- `specs/001-noa-seed-foundation/tasks.md` (B002, B079)
- `specs/001-noa-seed-foundation/checklists/phase0-quality-report.md`

**Parity Check**:
| Feature | PowerShell | Bash |
|---------|------------|------|
| Parameters | 8 | 4 |
| Directory creation | ✅ | ✅ |
| Prereq check | ✅ | ✅ |
| Platform detection | ✅ | ✅ |
| AI provider install | ✅ | ❌ Missing flag |
| Offline mode | ✅ | ❌ Missing flag |

**Recommendation**: ✅ **KEEP** - Core entry point. Consider adding `--install-ai-providers` and `--offline` flags for parity.

---

### 1.3 noa.ps1

| Field | Value |
|-------|-------|
| **Path** | `scripts/noa.ps1` |
| **Lines** | 280 |
| **Purpose** | NOA CLI Tool - P2P Server & Multi-Provider AI Management (Windows) |
| **Cross-Platform Pair** | `scripts/noa` ✅ |

**Description**: Main CLI interface for NOA. Provides commands for P2P server management, AI provider switching, device orchestration.

**Commands**:
- `start/stop/status` - P2P server management
- `nodes` - List connected nodes
- `storage` - Storage information
- `compute` - Compute resources
- `ai providers/devices/shared/switch` - AI management
- `device register/list/capabilities` - Device management
- `env` - Environment information
- `validate` - Environment validation

**Implementation Status**: Mostly stub (`TODO: Implement` markers)

**References**:
- Used as main CLI entry point
- Should be symlinked/copied to `bin/noa` for PATH access

**Recommendation**: ✅ **KEEP** - Core CLI interface. Note: implementation is partial (stubs)

---

### 1.4 noa (bash)

| Field | Value |
|-------|-------|
| **Path** | `scripts/noa` |
| **Lines** | 77 |
| **Purpose** | NOA CLI Tool - P2P Server & Multi-Provider AI Management (Unix) |
| **Cross-Platform Pair** | `scripts/noa.ps1` ✅ |

**Description**: Bash equivalent of noa.ps1. Simpler implementation with same command structure.

**Issue Found**: Line 4 hardcodes `NOA_ROOT="${HOME}"` instead of detecting from script location.

**Parity Check**:
| Feature | PowerShell | Bash |
|---------|------------|------|
| Lines | 280 | 77 |
| Commands | 9 | 9 |
| Subcommands | Full | Full |
| Implementation | Stubs | Stubs |
| NOA_ROOT detection | Auto-detect | ❌ Hardcoded HOME |
| Error handling | Detailed | Basic |

**Recommendation**: ✅ **KEEP** - Core CLI. Fix NOA_ROOT to auto-detect from script location like .ps1 version.

---

### 1.5 setup-noa.ps1

| Field | Value |
|-------|-------|
| **Path** | `scripts/setup/setup-noa.ps1` |
| **Lines** | 534 |
| **Purpose** | Higher-level NOA environment setup with profile integration |
| **Cross-Platform Pair** | `setup-noa.sh` ✅ |

**Description**: Creates directory structure, generates configuration files, optionally installs prerequisites and integrates with PowerShell profile. More user-friendly than bootstrap.ps1.

**Parameters**:
- `-NoaRoot` - Root directory (default: N:\noa)
- `-InstallPrereqs` - Run prerequisite checker
- `-IntegrateProfile` - Add to PowerShell profile
- `-InstallAllTools` - Install all toolchains
- `-InstallAiProviders` - Install AI provider CLIs

**References**:
- `specs/001-noa-seed-foundation/tasks.md` (lines 97, 101)

**Relationship to bootstrap.ps1**: This is a higher-level wrapper that provides profile integration. For full setup, calls bootstrap.ps1 internally.

**Recommendation**: ✅ **KEEP** - User-friendly setup wrapper with profile integration

---

### 1.6 setup-noa.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/setup/setup-noa.sh` |
| **Lines** | 355 |
| **Purpose** | Higher-level NOA environment setup with shell profile integration (Unix) |
| **Cross-Platform Pair** | `setup-noa.ps1` ✅ |

**Description**: Unix equivalent of setup-noa.ps1. Creates directory structure, generates configs, integrates with .bashrc/.zshrc.

**Parameters**:
- `--noa-root PATH` - Set NOA root directory
- `--install-prereqs` - Install prerequisites
- `--install-all-tools` - Install all toolchains
- `--install-ai-providers` - Install AI provider CLIs
- `--integrate-profile` - Add to shell profile

**Parity Check**: ✅ Good parity with PowerShell version

**Recommendation**: ✅ **KEEP** - User-friendly setup wrapper

---

## Phase 1 Summary

| Script | Status | Action |
|--------|--------|--------|
| bootstrap.ps1 | ✅ Active | Keep |
| bootstrap.sh | ✅ Active | Keep, add missing flags |
| noa.ps1 | ✅ Active (stubs) | Keep |
| noa (bash) | ⚠️ Issue | Keep, fix NOA_ROOT detection |
| setup-noa.ps1 | ✅ Active | Keep |
| setup-noa.sh | ✅ Active | Keep |

### Recommended Fixes

1. **bootstrap.sh**: Add `--install-ai-providers` and `--offline` flags for parity
2. **scripts/noa**: Fix line 4 to auto-detect NOA_ROOT from script location:
   ```bash
   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
   NOA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
   ```

---

## Phase 2: Library Scripts

The `scripts/bootstrap/lib/` directory contains 16 shared utility scripts (8 PowerShell, 7 Bash, 1 README).

### 2.1 directories.ps1 / directories.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/directories.{ps1,sh}` |
| **Lines** | PS1: 195, SH: 143 |
| **Purpose** | Create complete NOA directory structure and manage .gitignore |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `New-NoaDirectoryStructure` / `create_noa_directory_structure` - Creates ~40+ directories
- `New-GitIgnoreEntries` / `get_gitignore_entries` - Returns NOA-specific .gitignore entries
- `Update-GitIgnore` / `update_gitignore` - Appends entries to .gitignore

**Used By**: bootstrap.ps1, bootstrap.sh, setup-noa.ps1, setup-noa.sh

**Recommendation**: ✅ **KEEP** - Core library, excellent cross-platform parity

---

### 2.2 download.ps1 / download.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/download.{ps1,sh}` |
| **Lines** | PS1: 266, SH: 230 |
| **Purpose** | Portable download and extraction utilities |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `Get-NoaDownload` / `noa_download` - Download with optional caching and checksum
- `Expand-NoaArchive` / `noa_extract` - Extract archives (.zip, .tar.gz, .tar.xz)
- `Get-GitHubRelease` / `get_github_release` - Get latest release info from GitHub
- `Install-GitHubReleaseBinary` / `install_github_release_binary` - Download and install binary

**Constitution Compliance**: §3.1 - Downloads to noa_root only

**Recommendation**: ✅ **KEEP** - Essential for self-contained installation

---

### 2.3 exit-codes.sh (Bash only)

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/exit-codes.sh` |
| **Lines** | 45 |
| **Purpose** | Standardized exit codes for consistent error handling |
| **Cross-Platform Pair** | ❌ No PowerShell equivalent |

**Exit Codes**:
- `EXIT_SUCCESS=0`, `EXIT_ERROR=1`, `EXIT_WARNING=2`
- `EXIT_INVALID_ARGS=3`, `EXIT_MISSING_DEP=4`, `EXIT_PERMISSION=5`
- `EXIT_NETWORK=6`, `EXIT_DISK_FULL=7`, `EXIT_TIMEOUT=8`, `EXIT_NOT_FOUND=9`

**Recommendation**: ✅ **KEEP** - Consider adding PowerShell equivalent for parity

---

### 2.4 logging.ps1 / logging.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/logging.{ps1,sh}` |
| **Lines** | PS1: 118, SH: 112 |
| **Purpose** | Centralized logging with file output and colored console |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `Initialize-Logging` / `initialize_logging` - Initialize log file
- `Write-Log*` / `log_*` - Debug, Info, Success, Warning, Error levels
- `Write-LogSection` / `log_section` - Section headers
- `Write-LogStep` / `log_step` - Numbered steps

**Constitution Compliance**: §3.1 - Logs to noa_root/logs/

**Recommendation**: ✅ **KEEP** - Essential for debugging

---

### 2.5 platform.ps1 / platform.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/platform.{ps1,sh}` |
| **Lines** | PS1: 150, SH: 230 |
| **Purpose** | Platform detection (OS, architecture, WSL, capabilities) |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `Get-PlatformInfo` / `get_platform_info` - Comprehensive platform detection (JSON output)
- `Get-OSDownloadSuffix` / `get_download_suffix` - Tool-specific download suffixes
- `Get-ExecutableExtension` / `get_executable_extension` - .exe on Windows
- `Test-CommandExists` / `command_exists` - Check if command is available

**Detected Properties**: OS (windows/linux/macos/wsl1/wsl2), architecture (amd64/arm64), shell, isWSL, isAdmin, hasDocker, hasGPU

**Recommendation**: ✅ **KEEP** - Critical for cross-platform support

---

### 2.6 schema.ps1 / schema.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/schema.{ps1,sh}` |
| **Lines** | PS1: 107, SH: 75 |
| **Purpose** | JSON schema validation for configuration files |
| **Cross-Platform Pair** | ✅ Good parity |

**Functions**:
- `Test-JsonSchema` / `test_json_schema` - Validate JSON against schema
- `Get-BootstrapStateSchema` / `get_bootstrap_state_schema` - Schema for bootstrap-state.json
- `Test-BootstrapStateSchema` / `test_bootstrap_state_schema` - Validate state file

**Note**: Basic validation only (required fields, type checking). Full JSON Schema validation would require external library.

**Recommendation**: ✅ **KEEP** - Useful for configuration validation

---

### 2.7 state.ps1 / state.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/state.{ps1,sh}` |
| **Lines** | PS1: 215, SH: 195 |
| **Purpose** | Bootstrap state management (tracks installed tools, versions) |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `Initialize-BootstrapState` / `initialize_bootstrap_state` - Load/create state file
- `Get/Set-ToolState` / `get/set_tool_state` - Individual tool tracking
- `Get/Set-ToolchainState` / `get/set_toolchain_state` - Rust/Go/Node/Python
- `Get/Set-ProviderState` / `get/set_provider_state` - AI provider tracking
- `Test-ToolInstalled` / `test_tool_installed` - Check with optional version

**State File**: `config/bootstrap-state.json` (PS1) or `.config/runtime/bootstrap-state.json` (SH)

**Issue Found**: State file path differs between platforms:
- PowerShell: `config/bootstrap-state.json`
- Bash: `.config/runtime/bootstrap-state.json`

**Recommendation**: ✅ **KEEP** - Align state file paths between platforms

---

### 2.8 verification.ps1 / verification.sh

| Field | Value |
|-------|-------|
| **Path** | `scripts/bootstrap/lib/verification.{ps1,sh}` |
| **Lines** | PS1: 258, SH: 203 |
| **Purpose** | Verify installed tools, determine action (SKIP/UPDATE/INSTALL/RELOCATE) |
| **Cross-Platform Pair** | ✅ Full parity |

**Functions**:
- `Test-ToolVerification` / `verify_tool` - Verify single tool
- `Test-ToolchainVerification` / `verify_toolchain` - Verify Rust/Go/Node/Python
- `Get-VerificationSummary` / `get_verification_summary` - Aggregate results

**Actions**:
- `SKIP` - Already installed and up-to-date
- `UPDATE` - Needs version update
- `INSTALL` - Not installed
- `RELOCATE` - In wrong location (needs move to noa_root)

**Recommendation**: ✅ **KEEP** - Essential for idempotent bootstrap

---

## Phase 2 Summary

| Script Pair | Lines (PS1/SH) | Parity | Status |
|-------------|----------------|--------|--------|
| directories | 195/143 | ✅ | Keep |
| download | 266/230 | ✅ | Keep |
| exit-codes | -/45 | ❌ SH only | Keep, add PS1 |
| logging | 118/112 | ✅ | Keep |
| platform | 150/230 | ✅ | Keep |
| schema | 107/75 | ✅ | Keep |
| state | 215/195 | ⚠️ Path diff | Keep, fix path |
| verification | 258/203 | ✅ | Keep |

### Recommended Fixes

1. **exit-codes**: Create `exit-codes.ps1` for PowerShell parity
2. **state**: Align state file paths:
   - Change bash to use `config/bootstrap-state.json` (match PowerShell)
   - Or update both to `.config/runtime/bootstrap-state.json`

---

## Phase 3: Installer Scripts

The `scripts/bootstrap/installers/` directory contains tool installation scripts organized by category.

### 3.1 Toolchain Installers (Portable)

These install full development toolchains to `noa_root/opt/`:

| Script Pair | Target | Location | Parity |
|-------------|--------|----------|--------|
| rust-portable | Rust + cargo + rustup | opt/rust/ | ✅ |
| go-portable | Go + workspace | opt/go/ | ✅ |
| node-portable | Node.js + npm | opt/node/ | ✅ |
| python-portable | Python + venv | opt/python/, opt/venv/ | ✅ |
| protoc-portable | Protocol Buffers | bin/, opt/protobuf/ | ✅ |
| cmake-portable | CMake | opt/cmake/ | ✅ |
| ninja-portable | Ninja build | bin/ | ✅ |
| llvm-portable | LLVM/Clang | opt/llvm/ | ✅ |
| mingw-portable | MinGW-w64 | opt/mingw/ | ✅ |
| cuda-portable | CUDA toolkit | opt/cuda/ | ✅ |

**Status**: ✅ All have cross-platform pairs, Constitution §3.1 compliant

---

### 3.2 Tool Installers (Rust/Go/npm/pip)

These install additional tools via their respective package managers:

| Script Pair | Tools Installed | Parity |
|-------------|-----------------|--------|
| rust-tools | rustfmt, clippy, rust-analyzer | ✅ |
| go-tools | golangci-lint, staticcheck | ✅ |
| npm-tools | eslint, prettier, typescript | ✅ |
| pip-tools | ruff, semgrep, pylint | ✅ |

**Status**: ✅ All maintain cross-platform parity

---

### 3.3 CLI Tool Installers (bin/)

Single-binary tools installed to `noa_root/bin/`:

| PS1 Script | Bash Equivalent | Tool | Parity |
|------------|-----------------|------|--------|
| bat.ps1 | cli-tools.sh | bat (cat replacement) | ⚠️ |
| delta.ps1 | cli-tools.sh | delta (git diff) | ⚠️ |
| fd.ps1 | cli-tools.sh | fd (find replacement) | ⚠️ |
| fzf.ps1 | cli-tools.sh | fzf (fuzzy finder) | ⚠️ |
| jq.ps1 | cli-tools.sh | jq (JSON processor) | ⚠️ |
| ripgrep.ps1 | cli-tools.sh | rg (grep replacement) | ⚠️ |
| gh.ps1 | gh.sh | GitHub CLI | ✅ |
| git.ps1 | git.sh | Git | ✅ |
| git-lfs.ps1 | git-lfs.sh | Git LFS | ✅ |
| git-portable.ps1 | - | Portable Git | ❌ PS1 only |

**Issue**: PowerShell has individual scripts, Bash consolidates in `cli-tools.sh`.
- This is acceptable (Bash script installs all 6 tools at once)
- PowerShell approach allows selective installation

**Status**: ⚠️ Functional but different architecture

---

### 3.4 Security Tool Installers

| PS1 Script | Bash Equivalent | Tool | Parity |
|------------|-----------------|------|--------|
| gitleaks.ps1 | security-tools.sh | gitleaks | ⚠️ |
| grype.ps1 | security-tools.sh | grype | ⚠️ |
| trivy.ps1 | security-tools.sh | trivy | ⚠️ |

**Note**: Same pattern as CLI tools - PowerShell individual, Bash consolidated

---

### 3.5 AI Provider Installers (ai-providers/)

| Script Pair | Provider | Parity |
|-------------|----------|--------|
| claude-code | Claude Code CLI | ✅ |
| cursor-cli | Cursor CLI | ✅ |
| codex-cli | OpenAI Codex CLI | ✅ |
| abacus-cli | AbacusAI CLI | ✅ |
| vscode-copilot | GitHub Copilot | ✅ |
| git-cli-provider | Git-based provider | ✅ |

**Count**: 12 scripts (6 pairs)

**Status**: ✅ All cross-platform pairs complete

---

### 3.6 Dev Tools Installers (dev-tools/)

| PS1 Script | Bash Equivalent | Tool | Parity |
|------------|-----------------|------|--------|
| vscode.ps1 | vscode.sh | VS Code | ✅ |
| cursor.ps1 | cursor.sh | Cursor IDE | ✅ |
| docker.ps1 | docker.sh | Docker Desktop | ✅ |
| abacus-desktop.ps1 | abacus-desktop.sh | Abacus Desktop | ✅ |
| claude-desktop.ps1 | - | Claude Desktop | ❌ PS1 only |
| chatgpt-desktop.ps1 | - | ChatGPT Desktop | ❌ PS1 only |
| dbeaver.ps1 | - | DBeaver | ❌ PS1 only |
| postman.ps1 | - | Postman | ❌ PS1 only |
| - | ai-apps.sh | Meta-installer | ❌ SH only |

**Count**: 14 scripts

**Status**: ⚠️ Some Windows-only apps (expected)

---

### 3.7 Desktop Apps Installers (desktop-apps/)

| Script | Tool | Parity |
|--------|------|--------|
| chatgpt.ps1 | ChatGPT | ❌ PS1 only |
| claude.ps1 | Claude | ❌ PS1 only |
| github-desktop.ps1 | GitHub Desktop | ❌ PS1 only |

**Note**: These appear to duplicate dev-tools/. May be cleanup candidates.

---

### 3.8 Shared Resources Installers (shared-resources/)

| Script Pair | Purpose | Parity |
|-------------|---------|--------|
| create-directories | Create AI shared resource dirs | ✅ |
| execution-memory | Set up execution memory system | ✅ |
| provider-sync | Sync AI provider configs | ✅ |

**Count**: 6 scripts (3 pairs)

**Status**: ✅ Good parity

---

### 3.9 Standalone Scripts

| Script | Purpose | Parity | Note |
|--------|---------|--------|------|
| make-portable.ps1 | GNU Make | ❌ PS1 only | Windows needs this |
| powershell-portable.ps1 | PowerShell Core | ❌ PS1 only | Expected |
| ollama-portable.ps1 | Ollama | ❌ PS1 only | Needs SH pair |
| llama-cpp-build.ps1/.sh | Build llama.cpp | ✅ | Build from source |

---

## Phase 3 Summary

**Total Installer Scripts**: 75+ files

| Category | PS1 | SH | Pairs | Status |
|----------|-----|-----|-------|--------|
| Toolchains | 10 | 10 | 10 | ✅ |
| Tool installers | 4 | 4 | 4 | ✅ |
| CLI tools | 6 | 1* | - | ⚠️ Different arch |
| Security | 3 | 1* | - | ⚠️ Different arch |
| AI Providers | 6 | 6 | 6 | ✅ |
| Dev Tools | 9 | 4 | 4 | ⚠️ Some PS1-only |
| Desktop Apps | 3 | 0 | 0 | ❌ PS1 only |
| Shared Resources | 3 | 3 | 3 | ✅ |

*Consolidated scripts

### Recommended Actions

1. **Remove desktop-apps/**: Duplicates content in dev-tools/
2. **Add ollama-portable.sh**: Missing Bash pair for Ollama
3. **Document architecture difference**: CLI/security tools consolidation is intentional

### Potential Cleanup

```
desktop-apps/chatgpt.ps1      → Duplicate of dev-tools/chatgpt-desktop.ps1
desktop-apps/claude.ps1       → Duplicate of dev-tools/claude-desktop.ps1
desktop-apps/github-desktop.ps1 → Could move to dev-tools/
```

---

## Phase 4: Service Scripts

Service management scripts for running services within NOA root (Constitution §3.1 compliant).

Located in: `scripts/`

### 4.1 docker-service

| Field | Value |
|-------|-------|
| **Path** | `scripts/docker-service{.ps1,}` |
| **Lines** | PS1: 100, SH: (pending) |
| **Purpose** | Manage Docker daemon contained within NOA |
| **Cross-Platform Pair** | ✅ |

**Features**:
- Starts dockerd with `--data-root` pointing to `containers/docker-data`
- Uses named pipe `\\.\\pipe\\noa-docker` (Windows) or socket (Unix)
- PID file at `init/run/dockerd.pid`
- Falls back to Docker Desktop/system Docker if not found

**Actions**: start, stop, status

---

### 4.2 ollama-service

| Field | Value |
|-------|-------|
| **Path** | `scripts/ollama-service{.ps1,}` |
| **Lines** | PS1: 105, SH: (pending) |
| **Purpose** | Manage Ollama AI backend within NOA |
| **Cross-Platform Pair** | ✅ |

**Features**:
- Models stored in `ai/shared/models/ollama`
- Serves on `127.0.0.1:11434`
- PID file at `init/run/ollama.pid`
- Status shows available models

**Actions**: start, stop, status

---

### 4.3 gitea-service

| Field | Value |
|-------|-------|
| **Path** | `scripts/gitea-service{.ps1,}` |
| **Lines** | PS1: 105, SH: (pending) |
| **Purpose** | Manage local Gitea Git server |
| **Cross-Platform Pair** | ✅ |

**Features**:
- Data stored in `git/gitea/{config,data,repos}`
- Serves on `localhost:3000`
- PID file at `init/run/gitea.pid`

**Actions**: start, stop, status

---

### 4.4 ssh-service

| Field | Value |
|-------|-------|
| **Path** | `scripts/ssh-service{.ps1,}` |
| **Lines** | PS1: 122, SH: (pending) |
| **Purpose** | Manage SSH server within NOA |
| **Cross-Platform Pair** | ✅ |

**Features**:
- Config at `etc/ssh/sshd_config`
- Host keys at `etc/ssh/ssh_host_rsa_key`
- Default port: 2222 (non-privileged)
- Auto-generates host key if missing
- PID file at `init/run/sshd.pid`

**Actions**: start, stop, status

---

## Phase 4 Summary

| Service | PS1 | Bash | Parity | Data Location |
|---------|-----|------|--------|---------------|
| docker | ✅ | ✅ | ✅ | containers/docker-data |
| ollama | ✅ | ✅ | ✅ | ai/shared/models/ollama |
| gitea | ✅ | ✅ | ✅ | git/gitea/ |
| ssh | ✅ | ✅ | ✅ | etc/ssh/ |

**All services**:
- Use `init/run/*.pid` for PID files
- Auto-detect NOA_ROOT from environment or script location
- Constitution §3.1 compliant (all data in noa_root)

### Recommendations

1. ✅ **KEEP ALL** - Essential for contained service operation
2. Consider adding systemd/launchd service file generators for production use
3. Add unified `noa services {start|stop|status|list}` wrapper command

---

## Phase 5: Git & Config Utilities

### 5.1 Git Workflow Scripts

Located in `scripts/`:

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| git-pr.ps1 | 107 | PR create/list/review/merge | ❌ Missing |
| git-ci.ps1 | 116 | Local CI/CD pipelines | ❌ Missing |
| git-conflict.ps1 | 89 | AI-assisted conflict resolution | ❌ Missing |
| git-push.ps1 | 92 | Auto-commit and PR to main | ❌ Missing |

**Features**:
- `git-pr.ps1`: Uses `gh` CLI for PR operations
- `git-ci.ps1`: Local CI/CD runner, stores logs/artifacts in `git/ci-cd/`
- `git-conflict.ps1`: AI-assisted conflict analysis and resolution
- `git-push.ps1`: Single-user workflow automation (commit → develop → PR to main)

**Config Files Used**:
- `config/git-pr-workflow.json`
- `config/git-local-cicd.json`
- `config/git-conflict-ai.json`

**Status**: ⚠️ PowerShell only - need Bash equivalents for cross-platform

---

### 5.2 Config Management Scripts

Located in `scripts/config/`:

| Script | Purpose | Cross-Platform |
|--------|---------|----------------|
| generate-config-readme.ps1 | Generate README from config files | ❌ PS1 only |
| validate-config-audit-table.ps1 | Validate config audit CSV | ❌ PS1 only |
| normalize-config-audit-csv.ps1 | Normalize CSV format | ❌ PS1 only |
| normalize-config-audit-csv.py | Same in Python | ✅ Cross-platform |
| add-missing-registry-rows.ps1 | Add missing config entries | ❌ PS1 only |
| count-config-audit-rows.ps1 | Count audit entries | ❌ PS1 only |
| find-provider-authority-conflicts.ps1 | Find provider conflicts | ❌ PS1 only |
| fix-registry-known-violations.ps1 | Auto-fix violations | ❌ PS1 only |
| rebaseline-registry.ps1 | Rebaseline config registry | ❌ PS1 only |
| remove-bom.ps1 | Remove BOM from files | ❌ PS1 only |

**Purpose**: Config file maintenance and validation tools

**Status**: ⚠️ Mostly PowerShell only. Python version of one script exists.

---

### 5.3 Bootstrap Config Scripts

Located in `scripts/bootstrap/config/`:

| Script Pair | Purpose | Parity |
|-------------|---------|--------|
| appdata-setup | Configure AppData paths | ✅ |
| cache-setup | Configure cache directories | ✅ |
| log-setup | Configure log directories | ✅ |
| provider-cache | Configure provider caching | ✅ |

**Count**: 8 scripts (4 pairs)

**Status**: ✅ All have cross-platform pairs

---

## Phase 5 Summary

| Category | PS1 | SH | Pairs | Status |
|----------|-----|-----|-------|--------|
| Git workflows | 4 | 0 | 0 | ❌ Need Bash |
| Config mgmt | 9+1.py | 0 | 0 | ⚠️ PS1 only |
| Bootstrap config | 4 | 4 | 4 | ✅ |

### Recommendations

1. **Create Bash pairs** for git workflow scripts (git-pr, git-ci, git-conflict, git-push)
2. **Config management**: Consider converting to Python for true cross-platform
3. Bootstrap config scripts are fine - good parity

---

## Phase 6: Self-Containment Scripts

Scripts that ensure NOA operates as a self-contained installation per Constitution §3.1.

### 6.1 Library Bundling

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| bundle-libraries.ps1 | 108 | Copy DLL dependencies to lib/ | ❌ Missing |
| bundle-all-libs.ps1 | 66 | Bundle all bin/*.exe dependencies | ❌ Missing |

**Purpose**: Collect DLL dependencies for Windows executables into `lib/` folder

**How it works**:
1. Uses `dumpbin.exe` (Visual Studio) or Dependencies tool
2. Copies non-system DLLs to `noa_root/lib/`
3. Scans `bin/` and `opt/*/bin/` directories

**Status**: ⚠️ PowerShell only. Bash equivalent less critical (Linux uses rpath/static linking)

---

### 6.2 Static Binary Download

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| download-static-binaries.ps1 | 115 | Download portable tools to bin/ | ❌ Missing |

**Tools Downloaded**:
- jq (jqlang/jq)
- ripgrep (BurntSushi/ripgrep)
- fd (sharkdp/fd)
- bat (sharkdp/bat)

**Status**: ⚠️ PowerShell only. Bash version would use curl + tar.

---

### 6.3 Kernel Module Management

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| noa-kmod.ps1 | 211 | Manage kernel-level features | ✅ noa-kmod |
| noa-kmod | N/A | Bash version for Linux | ✅ |

**Windows Modules Managed**:
- `tap` - TAP-Windows adapter (VPN/P2P)
- `hyperv_switch` - Hyper-V virtual switch
- `windivert` - Packet capture/modification
- `nat` - Windows NAT
- `bridge` - Network bridge

**Linux Modules Managed**:
- `tun/tap` - TUN/TAP devices
- `bridge` - Network bridge
- `veth` - Virtual ethernet
- `netfilter` - iptables/nftables

**Actions**: load, unload, list, required, check

**Status**: ✅ Cross-platform pair exists

---

### 6.4 Namespace Isolation

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| noa-namespace.ps1 | 179 | Process isolation on Windows | ✅ noa-namespace |
| noa-namespace | N/A | Bash version for Linux | ✅ |

**Windows Isolation Methods**:
- Job Objects (always available)
- Windows Sandbox (if enabled)
- Hyper-V isolation (if enabled)
- Windows Containers (Docker)

**Linux Isolation Methods**:
- Linux namespaces (pid, net, mnt, etc.)
- cgroups for resource limits
- seccomp for syscall filtering

**Actions**: run, sandbox, container, list

**Status**: ✅ Cross-platform pair exists

---

## Phase 6 Summary

| Script | PS1 | Bash | Parity | Priority |
|--------|-----|------|--------|----------|
| bundle-libraries | ✅ | ❌ | Windows-specific | Low |
| bundle-all-libs | ✅ | ❌ | Windows-specific | Low |
| download-static-binaries | ✅ | ❌ | Needs pair | Medium |
| noa-kmod | ✅ | ✅ | ✅ | High |
| noa-namespace | ✅ | ✅ | ✅ | High |

### Recommendations

1. ✅ **Keep noa-kmod and noa-namespace** - Critical for P2P and isolation
2. **Create download-static-binaries.sh** - Useful for automated setup
3. **Bundle scripts are Windows-specific** - Linux uses different mechanisms

---

## Phase 7: Test Scripts

### 7.1 Test Scripts (`scripts/test/`)

| Script | Lines | Purpose | Bash Pair |
|--------|-------|---------|-----------|
| smoke-test-phase1.ps1 | 132 | Verify Phase 1 artifacts | ✅ smoke-test-phase1.sh |
| smoke-test-phase2.ps1 | N/A | Verify Phase 2 artifacts | ✅ smoke-test-phase2.sh |
| negative-tests-phase1.ps1 | N/A | Test error handling | ✅ negative-tests-phase1.sh |
| comprehensive-test.ps1 | N/A | Full system test | ❌ Missing |
| status-check.ps1 | N/A | Quick status verification | ❌ Missing |

**Purpose**: Verify NOA installation and configuration

**Test Categories**:
- Directory structure verification (FR-029 to FR-036)
- Configuration file presence
- Tool availability
- Service status
- Environment variable validation

**Count**: 8 scripts total

---

### 7.2 Pester Tests (`scripts/tests/`)

| Script | Lines | Purpose |
|--------|-------|---------|
| Setup.Tests.ps1 | 266 | Pester tests for setup-noa.ps1 |

**Tests**:
- Script syntax validation
- Directory creation
- Profile content generation
- Config file creation
- Parameter handling

**Framework**: PowerShell Pester

---

## Phase 7 Summary

| Category | PS1 | Bash | Pairs | Status |
|----------|-----|------|-------|--------|
| Smoke tests | 2 | 2 | 2 | ✅ |
| Negative tests | 1 | 1 | 1 | ✅ |
| Comprehensive | 1 | 0 | 0 | ⚠️ |
| Status check | 1 | 0 | 0 | ⚠️ |
| Pester tests | 1 | N/A | N/A | ✅ |

### Recommendations

1. ✅ **Keep all test scripts** - Essential for verification
2. **Create comprehensive-test.sh and status-check.sh** for cross-platform testing
3. Consider adding more Pester tests for other scripts

---

## Phase 8: Cleanup Candidates & Miscellaneous

### 8.1 Deprecated/Archive Directories

| Directory | Contents | Status |
|-----------|----------|--------|
| scripts/deprecated/ | README.md only | ✅ Clean |
| scripts/archive/ | README.md only | ✅ Clean |

**Status**: Directories exist for organization but are empty. Good practice.

---

### 8.2 Orphan Scripts in `scripts/`

Scripts without matching cross-platform pairs:

| Script | Lines | Purpose | Pair Status |
|--------|-------|---------|-------------|
| verify-environment.ps1 | 236 | Verify NOA setup, fix issues | ❌ No Bash |
| sync.ps1 | 24 | Quick git sync (wrapper for git-push) | ❌ No Bash |
| release-gate.ps1 | 395 | Pre-release validation | ✅ release-gate.sh |
| install-profile.ps1 | N/A | Install NOA to PS profile | ❌ No Bash |
| remediate-config-orphans.ps1 | 7 | Stub/placeholder | ⚠️ Consider removal |
| build.ps1 | N/A | Build script | ⚠️ Check if used |
| setup.ps1 | N/A | Alternate setup entry | ⚠️ May duplicate setup/ |
| uninstall.ps1 | N/A | Uninstall NOA | ✅ uninstall.sh |
| noa-kernel-params.ps1 | N/A | Kernel parameters | ✅ noa-kernel-params |
| noa-kernel-mode.ps1 | N/A | Kernel mode toggle | ✅ noa-kernel-mode.sh |
| patch-binary-libs | N/A | Patch library paths | ❌ Bash only |

---

### 8.3 Archive Scripts (`data/archive/`)

Scripts in archived project management tools:

| Location | Scripts | Status |
|----------|---------|--------|
| data/archive/project-mgmt/spec-kit/scripts/ | 4 PS1 files | 📦 Archived |
| data/archive/project-mgmt/project-management-app/.specify/scripts/ | 4 PS1 files | 📦 Archived |

**Contents**: check-prerequisites.ps1, update-agent-context.ps1, setup-plan.ps1, common.ps1

**Status**: Part of archived projects, not active NOA scripts

---

### 8.4 Subdirectories Not Yet Analyzed

| Directory | Purpose | Status |
|-----------|---------|--------|
| scripts/apps/ | App-specific scripts | Needs review |
| scripts/bash/ | Bash utilities | Needs review |
| scripts/powershell/ | PowerShell utilities | Needs review |
| scripts/conda/ | Conda environment | Needs review |
| scripts/shell/ | Shell setup moved here | ✅ Analyzed earlier |

---

## Phase 8 Summary

### Cleanup Recommendations

| Script | Action | Reason |
|--------|--------|--------|
| remediate-config-orphans.ps1 | ⚠️ Remove or implement | Stub only |
| setup.ps1 | ⚠️ Check for duplication | May duplicate scripts/setup/ |
| scripts/apps/ | 📋 Review | Not yet analyzed |
| scripts/bash/ | 📋 Review | Not yet analyzed |
| scripts/powershell/ | 📋 Review | Not yet analyzed |
| scripts/conda/ | 📋 Review | Not yet analyzed |

### Missing Bash Pairs (High Priority)

These PowerShell scripts need Bash equivalents:
1. `verify-environment.ps1` → `verify-environment.sh`
2. `sync.ps1` → `sync.sh`
3. `install-profile.ps1` → `install-profile.sh`

---

# Audit Summary

## Overall Statistics

| Category | Total Scripts | PS1 | Bash | Cross-Platform Pairs |
|----------|---------------|-----|------|----------------------|
| Core Entry Points | 6 | 3 | 3 | 3 ✅ |
| Library Scripts | 16 | 8 | 8 | 7 ✅ (exit-codes missing PS1) |
| Installers | 75+ | ~50 | ~25 | Most have pairs |
| Service Scripts | 8 | 4 | 4 | 4 ✅ |
| Git/Config Utils | 21 | 17 | 4 | Partial |
| Self-Containment | 7 | 5 | 2 | 2 ✅ |
| Test Scripts | 9 | 6 | 3 | Partial |
| **Total Estimated** | **~174** | **~105** | **~55** | **~38 pairs** |

## Key Findings

### ✅ Well-Designed

1. **Bootstrap system** - Excellent cross-platform parity
2. **Library scripts** - Good modular design, well-documented
3. **Service scripts** - Clean, Constitution §3.1 compliant
4. **Installer architecture** - Comprehensive toolchain support

### ⚠️ Needs Attention

1. **Git workflow scripts** - PS1 only, need Bash pairs
2. **Config management scripts** - PS1 only, consider Python
3. **Some test scripts** - Missing Bash equivalents
4. **State file path inconsistency** - PS1 vs Bash use different paths

### 🔧 Recommended Actions

1. **Create Bash pairs** for: git-pr, git-ci, git-conflict, git-push, verify-environment, sync
2. **Create exit-codes.ps1** for PowerShell parity
3. **Align state file paths** between PowerShell and Bash
4. **Fix scripts/noa NOA_ROOT** to auto-detect from script location
5. **Add missing installer flags** to bootstrap.sh (--install-ai-providers, --offline)
6. **Review scripts/apps/, bash/, powershell/, conda/** for completeness
7. **Remove or implement** remediate-config-orphans.ps1

---

*Audit completed: 2025-12-31*
