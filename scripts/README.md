# NOA Scripts - Cross-Platform Reference

All scripts are available on Windows (PowerShell), Linux (Bash), and macOS (Bash/Zsh).

---

## Quick Start

### Windows
```powershell
# Full bootstrap
.\scripts\bootstrap\bootstrap.ps1

# Load environment
. .\noa-env.ps1
```

### Linux/macOS
```bash
# Full bootstrap
./scripts/bootstrap/bootstrap.sh

# Load environment
source ./noa-env.sh
```

---

## Script Mapping

| Purpose | Windows (PowerShell) | Linux/macOS (Bash) |
|---------|---------------------|-------------------|
| **Bootstrap** |||
| Full setup | `bootstrap\bootstrap.ps1` | `bootstrap/bootstrap.sh` |
| Check prereqs | `setup\check-prereqs.ps1` | `init/check-prereqs.sh` |
| Install prereqs | `setup\install-prereqs.ps1` | *(included in bootstrap)* |
| Download binaries | `download-static-binaries.ps1` | `download-static-binaries` |
| **Library Bundling** |||
| Bundle single | `bundle-libraries.ps1` | `bundle-libraries` |
| Bundle all | `bundle-all-libs.ps1` | `bundle-all-libs` |
| Patch libs | *(use bundle)* | `patch-binary-libs` |
| **Services** |||
| Docker | `docker-service.ps1` | `docker-service` |
| Ollama | `ollama-service.ps1` | `ollama-service` |
| SSH | `ssh-service.ps1` | `ssh-service` |
| Gitea | `gitea-service.ps1` | `gitea-service` |
| **Kernel/System** |||
| Kernel params | `noa-kernel-params.ps1` | `noa-kernel-params` |
| Kernel modules | `noa-kmod.ps1` | `noa-kmod` |
| Namespace | `noa-namespace.ps1` | `noa-namespace` |
| **Git Workflow** |||
| CI/CD | `git-ci.ps1` | `git-ci` |
| Conflict | `git-conflict.ps1` | `git-conflict` |
| PR | `git-pr.ps1` | `git-pr` |
| **Main CLI** |||
| NOA CLI | `noa.ps1` | `noa` |

---

## Platform Detection

All scripts detect the current platform:

| Platform | Detection |
|----------|-----------|
| Windows Native | `$env:OS -eq "Windows_NT"` |
| WSL1 | `/proc/version` contains "microsoft", no `/run/WSL` |
| WSL2 | `/proc/version` contains "microsoft", has `/run/WSL` |
| Linux | `/proc/version` exists, not microsoft |
| macOS | `$(uname) == "Darwin"` |

---

## Environment Variables

All platforms use the same environment variable names:

| Variable | Description |
|----------|-------------|
| `NOA_ROOT` | Root directory for all NOA files |
| `NOA_BIN` | Self-contained binaries |
| `NOA_OPT` | Portable toolchains |
| `NOA_LIB` | Bundled libraries |
| `NOA_CONFIG` | Configuration files |
| `NOA_LOGS` | Log files |
| `NOA_TMP` | Temporary files |

### Toolchain Variables

| Variable | Description |
|----------|-------------|
| `RUSTUP_HOME` | Rust toolchain location |
| `CARGO_HOME` | Cargo home directory |
| `GOROOT` | Go installation |
| `GOPATH` | Go workspace |
| `GOBIN` | Go binaries |
| `npm_config_prefix` | Node.js global prefix |

---

## Kernel Independence

NOA can run in different isolation modes:

| Mode | Windows | Linux | macOS |
|------|---------|-------|-------|
| Native | Windows kernel | Linux kernel | Darwin kernel |
| Container | Docker Windows | Docker/Podman | Docker |
| VM | Hyper-V | KVM/QEMU | Virtualization.framework |
| Sandbox | Windows Sandbox | Bubblewrap | Sandbox |

See `specs/002-unified-bootstrap/kernel-independence.md` for details.

---

## Adding New Scripts

When adding a new script:

1. Create the bash version in `scripts/` (no extension)
2. Create the PowerShell version in `scripts/` (`.ps1` extension)
3. Update this README with the mapping
4. Ensure both scripts have:
   - Same command-line interface
   - Same exit codes
   - Platform detection
   - NOA_ROOT auto-detection

### Template

**Bash Template** (`scripts/my-script`):
```bash
#!/bin/bash
# NOA My Script Description
# Windows equivalent: scripts/my-script.ps1

NOA_ROOT="${NOA_ROOT:-$(cd "$(dirname "$0")/.." && pwd)}"

case "$1" in
    action) echo "Doing action..." ;;
    *) echo "Usage: my-script {action}" ;;
esac
```

**PowerShell Template** (`scripts/my-script.ps1`):
```powershell
<#
.SYNOPSIS
    NOA My Script Description
.DESCRIPTION
    Bash equivalent: scripts/my-script
#>
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("action")]
    [string]$Action,
    [string]$NoaRoot
)

if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else { Split-Path -Parent $PSScriptRoot }
}

switch ($Action) {
    "action" { Write-Host "Doing action..." }
}
```

---

## Testing

Run tests for all scripts:

```powershell
# Windows
.\scripts\tests\Setup.Tests.ps1
```

```bash
# Linux/macOS
./scripts/tests/setup-tests.sh
```

