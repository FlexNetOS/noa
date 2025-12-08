# Implementation Plan: NOA Unified Bootstrap Installer

**Feature**: 002-unified-bootstrap
**Spec**: [spec.md](./spec.md)
**Branch**: `002-unified-bootstrap`
**Created**: 2025-12-08
**Updated**: 2025-12-08

---

## Technical Context

**Project Type**: Cross-platform self-contained setup automation
**Languages**: PowerShell 7.4+, Bash 5.0+
**Primary Dependencies**: NONE - Direct HTTP downloads only (no winget, brew, apt)
**Target Platforms**: Windows 10/11, macOS 12+, Ubuntu 22.04+, WSL2

### Self-Containment Strategy

**All installations target `noa_root`** with ZERO system-wide modifications:

| Component | Install Location | Source |
|-----------|-----------------|--------|
| Rust toolchain | `noa_root/opt/rust/` | Direct download from rust-lang.org |
| Go toolchain | `noa_root/opt/go/` | Direct download from go.dev |
| Node.js | `noa_root/opt/node/` | Direct download from nodejs.org |
| Python | `noa_root/opt/python/` | Embeddable Python / python-build-standalone |
| protoc | `noa_root/bin/` | GitHub releases |
| All utilities | `noa_root/bin/` | GitHub releases |
| npm packages | `noa_root/opt/node/node_modules/` | Local npm install |
| pip packages | `noa_root/opt/venv/` | Local venv install |
| Go modules | `noa_root/opt/go/workspace/` | go install to GOBIN |

---

## Phase 0: Prerequisites

This feature implements the prerequisites for all other NOA features.

### Minimum Requirements

| Component | Minimum | Install Location |
|-----------|---------|------------------|
| PowerShell | 7.4+ | System |
| Bash | 5.0+ | System |
| Git | 2.40+ | System |

---

## Constitution Check

### Core Principles Alignment

| Principle | Compliant | Notes |
|-----------|-----------|-------|
| 3.1 Self-Contained & Autonomous | ☑ Yes | All paths under `noa_root` except required system toolchains |
| 3.2 Local-First & Offline-Capable | ☑ Yes | Supports cached/vendored tools for offline |
| 3.3 Agentic Orchestration | ☑ N/A | Infrastructure layer |
| 3.4 Adaptive & Self-Improving | ☑ Yes | Version upgrade detection |
| 3.5 Transparent & Auditable | ☑ Yes | All actions logged |
| 3.6 Security & Privacy | ☑ Yes | HTTPS downloads, checksum verification |
| 3.7 Total Memory Sovereignty | ☑ N/A | Infrastructure layer |
| 3.8 P2P Hive-Mind | ☑ N/A | Infrastructure layer |
| 3.12 Test Everything | ☑ Yes | Comprehensive verification |

---

## Project Structure

```
scripts/bootstrap/
├── bootstrap.ps1           # Main entry point (Windows)
├── bootstrap.sh            # Main entry point (Unix)
├── README.md               # Usage documentation
├── ARCHITECTURE.md         # Design documentation
├── config/
│   └── tools.json          # Tool definitions and versions
├── lib/
│   ├── logging.ps1         # Logging functions
│   ├── logging.sh
│   ├── platform.ps1        # Platform detection
│   ├── platform.sh
│   ├── version.ps1         # Version comparison
│   ├── version.sh
│   ├── paths.ps1           # Path resolution
│   ├── paths.sh
│   ├── installer.ps1       # Base installer
│   ├── installer.sh
│   ├── directories.ps1     # Directory creation
│   ├── directories.sh
│   ├── winget.ps1          # winget helpers
│   ├── choco.ps1           # chocolatey helpers
│   ├── brew.sh             # Homebrew helpers
│   └── apt.sh              # apt helpers
├── installers/
│   ├── git.ps1             # Git installer
│   ├── git.sh
│   ├── git-lfs.ps1
│   ├── git-lfs.sh
│   ├── gh.ps1              # GitHub CLI
│   ├── gh.sh
│   ├── rust.ps1            # Rust toolchain
│   ├── rust.sh
│   ├── go.ps1              # Go toolchain
│   ├── go.sh
│   ├── node.ps1            # Node.js
│   ├── node.sh
│   ├── python.ps1          # Python
│   ├── python.sh
│   ├── protobuf.ps1        # protoc
│   ├── protobuf.sh
│   ├── rustfmt.ps1         # Rust formatter
│   ├── clippy.ps1          # Rust linter
│   ├── golangci-lint.ps1   # Go linter
│   ├── golangci-lint.sh
│   ├── eslint.ps1          # JS/TS linter
│   ├── eslint.sh
│   ├── ruff.ps1            # Python linter
│   ├── ruff.sh
│   ├── semgrep.ps1         # Security scanner
│   ├── semgrep.sh
│   ├── gitleaks.ps1        # Secret scanner
│   ├── gitleaks.sh
│   ├── trivy.ps1           # Container scanner
│   ├── trivy.sh
│   ├── grype.ps1           # Vulnerability scanner
│   ├── grype.sh
│   ├── jq.ps1              # JSON processor
│   ├── jq.sh
│   ├── ripgrep.ps1         # Search tool
│   ├── ripgrep.sh
│   ├── fd.ps1              # Find replacement
│   ├── fd.sh
│   ├── bat.ps1             # Cat replacement
│   ├── bat.sh
│   ├── fzf.ps1             # Fuzzy finder
│   ├── fzf.sh
│   ├── kernel-modules.sh   # Linux kernel modules
│   ├── kernel-params.sh    # sysctl parameters
│   ├── kernel.ps1          # Windows kernel/WSL
│   └── namespace.sh        # Linux namespaces
├── generators/
│   ├── noa-env.ps1         # Environment file generator
│   ├── noa-env.sh
│   ├── noa-profile.ps1     # Profile generator
│   ├── noa-profile.sh
│   ├── config-json.ps1     # config/noa.json generator
│   ├── config-json.sh
│   ├── env-file.ps1        # .env generator
│   └── marker.ps1          # .noa marker generator
├── integrators/
│   ├── profile-powershell.ps1
│   ├── profile-bash.sh
│   └── profile-zsh.sh
├── verify/
│   ├── verify-all.ps1      # Comprehensive verification
│   ├── verify-all.sh
│   ├── smoke-test.ps1      # Functional tests
│   └── smoke-test.sh
└── phases/
    ├── phase-runner.ps1    # Phase orchestration
    └── phase-runner.sh
```

---

## Tool Configuration (tools.json)

```json
{
  "version": "1.0.0",
  "tools": {
    "system_wide": {
      "git": { "min": "2.40.0", "install": { "windows": "winget install Git.Git", "macos": "brew install git", "linux": "apt install git" } },
      "git-lfs": { "min": "3.0.0", "install": { "windows": "winget install GitHub.GitLFS", "macos": "brew install git-lfs", "linux": "apt install git-lfs" } },
      "gh": { "min": "2.40.0", "install": { "windows": "winget install GitHub.cli", "macos": "brew install gh", "linux": "apt install gh" } },
      "rust": { "min": "1.83.0", "install": { "windows": "winget install Rustlang.Rustup && rustup default stable", "unix": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && rustup default stable" } },
      "go": { "min": "1.23.0", "install": { "windows": "winget install GoLang.Go", "macos": "brew install go", "linux": "apt install golang-go" } },
      "node": { "min": "20.0.0", "install": { "windows": "winget install OpenJS.NodeJS.LTS", "macos": "brew install node", "linux": "apt install nodejs" } },
      "python": { "min": "3.12.0", "install": { "windows": "winget install Python.Python.3.12", "macos": "brew install python@3.12", "linux": "apt install python3.12" } },
      "protoc": { "min": "28.0.0", "install": { "windows": "winget install Google.Protobuf", "macos": "brew install protobuf", "linux": "apt install protobuf-compiler" } }
    },
    "quality": {
      "rustfmt": { "min": "1.0.0", "depends": "rust", "install": "rustup component add rustfmt" },
      "clippy": { "min": "0.1.0", "depends": "rust", "install": "rustup component add clippy" },
      "golangci-lint": { "min": "1.62.0", "depends": "go", "install": "go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest" },
      "eslint": { "min": "9.0.0", "depends": "node", "install_local": true },
      "ruff": { "min": "0.8.0", "depends": "python", "install_venv": true },
      "semgrep": { "min": "1.97.0", "depends": "python", "install_venv": true }
    },
    "self_contained": {
      "jq": { "min": "1.7.0", "github": "jqlang/jq", "asset_pattern": { "windows": "jq-win64.exe", "macos": "jq-macos-*", "linux": "jq-linux-*" } },
      "rg": { "min": "14.0.0", "github": "BurntSushi/ripgrep", "asset_pattern": { "windows": "*-x86_64-pc-windows-msvc.zip", "macos": "*-x86_64-apple-darwin.tar.gz", "linux": "*-x86_64-unknown-linux-musl.tar.gz" } },
      "fd": { "min": "10.0.0", "github": "sharkdp/fd", "asset_pattern": { "windows": "*-x86_64-pc-windows-msvc.zip", "macos": "*-x86_64-apple-darwin.tar.gz", "linux": "*-x86_64-unknown-linux-musl.tar.gz" } },
      "bat": { "min": "0.24.0", "github": "sharkdp/bat", "asset_pattern": { "windows": "*-x86_64-pc-windows-msvc.zip", "macos": "*-x86_64-apple-darwin.tar.gz", "linux": "*-x86_64-unknown-linux-musl.tar.gz" } },
      "fzf": { "min": "0.55.0", "github": "junegunn/fzf", "asset_pattern": { "windows": "*-windows_amd64.zip", "macos": "*-darwin_amd64.zip", "linux": "*-linux_amd64.tar.gz" } },
      "gitleaks": { "min": "8.21.0", "github": "gitleaks/gitleaks", "asset_pattern": { "windows": "*_windows_x64.zip", "macos": "*_darwin_x64.tar.gz", "linux": "*_linux_x64.tar.gz" } },
      "trivy": { "min": "0.57.0", "github": "aquasecurity/trivy", "asset_pattern": { "windows": "*_Windows-64bit.zip", "macos": "*_macOS-64bit.tar.gz", "linux": "*_Linux-64bit.tar.gz" } },
      "grype": { "min": "0.84.0", "github": "anchore/grype", "asset_pattern": { "windows": "*_windows_amd64.zip", "macos": "*_darwin_amd64.tar.gz", "linux": "*_linux_amd64.tar.gz" } }
    }
  },
  "phases": [
    { "name": "core", "tools": ["git", "git-lfs", "gh"] },
    { "name": "toolchains", "tools": ["rust", "go", "node", "python", "protoc"] },
    { "name": "quality", "tools": ["rustfmt", "clippy", "golangci-lint", "eslint", "ruff", "semgrep"] },
    { "name": "security", "tools": ["gitleaks", "trivy", "grype"] },
    { "name": "utilities", "tools": ["jq", "rg", "fd", "bat", "fzf"] }
  ]
}
```

---

## Implementation Phases

### Phase 1: Foundation (T001-T014)
- Create directory structure
- Implement logging
- Implement platform detection
- Implement version comparison
- Implement path resolution

### Phase 2: Core Prerequisites (T015-T024)
- Git installers
- Git LFS installers
- GitHub CLI installers
- Package manager helpers

### Phase 3: Build Toolchains (T025-T034)
- Rust toolchain (rustup)
- Go toolchain
- Node.js toolchain
- Python toolchain
- Protocol Buffers

### Phase 4: Quality Tools (T035-T050)
- Rust tools (rustfmt, clippy)
- Go tools (golangci-lint)
- Node tools (eslint)
- Python tools (ruff, semgrep)
- Security tools (gitleaks, trivy, grype)

### Phase 5: Self-Contained Utilities (T051-T060)
- jq, ripgrep, fd, bat, fzf
- All downloaded to noa_root/bin/

### Phase 6: Infrastructure (T061-T077)
- Directory structure creation
- Kernel/namespace setup (Linux)
- Environment configuration
- Profile integration

### Phase 7: Verification & Integration (T078-T103)
- Comprehensive verification
- Smoke tests
- Integration with existing scripts
- Documentation
- Constitutional compliance

---

## Usage

### Basic Usage

```powershell
# Windows
.\scripts\bootstrap\bootstrap.ps1

# Unix
./scripts/bootstrap/bootstrap.sh
```

### Options

```powershell
# Skip profile integration
.\scripts\bootstrap\bootstrap.ps1 -SkipProfile

# Custom NOA root
.\scripts\bootstrap\bootstrap.ps1 -NoaRoot "D:\my-noa"

# Check only (no install)
.\scripts\bootstrap\bootstrap.ps1 -CheckOnly

# Force reinstall
.\scripts\bootstrap\bootstrap.ps1 -Force

# Verbose output
.\scripts\bootstrap\bootstrap.ps1 -Verbose

# Skip specific phases
.\scripts\bootstrap\bootstrap.ps1 -SkipPhases "kernel,profile"
```

---

## Migration from Existing Scripts

1. **scripts/setup.ps1** → Delegates to `bootstrap.ps1`
2. **scripts/setup/setup-noa.ps1** → Deprecate, redirect to `bootstrap.ps1`
3. **scripts/setup/install-prereqs.ps1** → Delete (was already a shim)
4. **scripts/setup/check-prereqs.ps1** → Keep for backward compatibility, call bootstrap verify
5. **scripts/download-static-binaries.ps1** → Replaced by self-contained installers
6. **scripts/verify-environment.ps1** → Delegates to `bootstrap.ps1 -Verify`

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking existing workflows | Keep shims that redirect to new scripts |
| Platform-specific failures | Comprehensive platform testing matrix |
| Network failures | Support cached/vendored tools |
| Version conflicts | Clear version requirements, upgrade path |

---

## Next Steps

1. Run `.\scripts\setup\check-prereqs.ps1` to see current state
2. Create `scripts/bootstrap/` directory structure
3. Implement Phase 1 foundation tasks
4. Test on Windows, then port to Unix

---

**Plan Updated**: 2025-12-08
**Total Tasks**: 103
**Estimated Duration**: 3-4 weeks (1-2 developers)

