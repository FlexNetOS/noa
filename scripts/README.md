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

## Script Pairs (Cross-Platform Parity)

| PowerShell | Bash | Purpose |
|------------|------|---------|
| `bootstrap/bootstrap.ps1` | `bootstrap/bootstrap.sh` | Main entry |
| `bootstrap/config/cache-setup.ps1` | `bootstrap/config/cache-setup.sh` | Cache setup |
| `bootstrap/config/log-setup.ps1` | `bootstrap/config/log-setup.sh` | Log setup |
| `bootstrap/verify/verify-all.ps1` | `bootstrap/verify/verify-all.sh` | Full verify |
| `bootstrap/verify/smoke-test.ps1` | `bootstrap/verify/smoke-test.sh` | Smoke tests |
| `setup/check-prereqs.ps1` | `../init/check-prereqs.sh` | Prereq check |

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
