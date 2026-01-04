# AppData Containment - NOA FR-001 Compliance

## Overview

NOA implements complete AppData containment to ensure **FR-001 compliance**: "System MUST operate entirely inside `noa_root` directory with no hard dependencies on external paths."

## Problem Statement

Desktop applications typically store data in system-wide locations **OUTSIDE** `noa_root`:

### Windows Default Behavior (❌ VIOLATES FR-001)
```
C:\Users\{username}\AppData\Roaming\{App}\   # Application configss, sync data
C:\Users\{username}\AppData\Local\{App}\     # Cache, logs, local data
C:\Users\{username}\AppData\LocalLow\{App}\  # Low integrity data
C:\Users\{username}\AppData\Local\Temp\      # Temporary files
```

###Unix Default Behavior (❌ VIOLATES FR-001)
```
~/.configs/{app}/       # XDG_configs_HOME
~/.local/share/{app}/  # XDG_DATA_HOME
~/.cache/{app}/        # XDG_CACHE_HOME
~/.local/state/{app}/  # XDG_STATE_HOME
/tmp/                  # TMPDIR
```

## NOA Solution: Environment Variable Redirection

NOA redirects **ALL** application data paths to directories within `noa_root`.

### Windows AppData Redirection

**File**: `noa-env.ps1` (lines 59-64)

```powershell
# AppData redirection (FR-001: Self-contained operation)
$env:APPDATA = Join-Path $env:NOA_ROOT "data/appdata/roaming"
$env:LOCALAPPDATA = Join-Path $env:NOA_ROOT "data/appdata/local"
$env:TEMP = Join-Path $env:NOA_ROOT "tmp"
$env:TMP = Join-Path $env:NOA_ROOT "tmp"
```

**Result**:
```
Before: C:\Users\{user}\AppData\Roaming\Claude\
After:  N:\noa\data\appdata\roaming\Claude\      ✅ CONTAINED
```

### Unix XDG Redirection

**File**: `.noa-env` (lines 38-49)

```bash
# XDG Base Directory specification (FR-001: Self-contained operation)
export XDG_DATA_HOME=$NOA_ROOT/data
export XDG_configs_HOME=$NOA_ROOT/etc
export XDG_CACHE_HOME=$NOA_ROOT/data/cache
export XDG_STATE_HOME=$NOA_ROOT/data/state
export XDG_RUNTIME_DIR=$NOA_ROOT/tmp/runtime
export TMPDIR=$NOA_ROOT/tmp
export TEMP=$NOA_ROOT/tmp
export TMP=$NOA_ROOT/tmp
```

**Result**:
```
Before: ~/.configs/claude/
After:  /path/to/noa/etc/claude/                ✅ CONTAINED
```

## Directory Structure

```
noa_root/
├── data/                       # Persistent application data
│   ├── appdata/                # Windows AppData hierarchy
│   │   ├── roaming/            # Sync-able data ($APPDATA)
│   │   │   ├── Claude/         # Claude Desktop configs
│   │   │   ├── Abacus/         # Abacus Desktop configs
│   │   │   └── OpenAI/         # ChatGPT Desktop configs
│   │   └── local/              # Machine-local data ($LOCALAPPDATA)
│   │       ├── Claude/         # Claude cache/logs
│   │       ├── Abacus/         # Abacus cache/logs
│   │       └── OpenAI/         # ChatGPT cache/logs
│   ├── cache/                  # XDG_CACHE_HOME (Unix)
│   ├── state/                  # XDG_STATE_HOME (Unix)
│   ├── memory/                 # NOA memory store
│   ├── knowledge/              # Knowledge graphs
│   └── artifacts/              # CAS artifacts
│
├── etc/                        # XDG_configs_HOME (Unix)
│   ├── claude/                 # Claude configs (Unix)
│   ├── abacus/                 # Abacus configs (Unix)
│   └── chatgpt/                # ChatGPT configs (Unix)
│
└── tmp/                        # TEMP, TMP, TMPDIR
    ├── runtime/                # XDG_RUNTIME_DIR
    └── ...                     # Temporary files
```

## How It Works

### 1. Environment Initialization

When NOA starts:
```powershell
# Source environment file
. .\noa-env.ps1

# Environment variables are now redirected:
$env:APPDATA
# → N:\noa\data\appdata\roaming (NOT C:\Users\...)
```

### 2. Application Launch

Desktop apps launched through NOA wrappers inherit redirected environment:

```powershell
# bin/claude-desktop.cmd
"N:\noa\opt\claude-desktop\Claude.exe" %*
# ↑ Inherits $env:APPDATA = N:\noa\data\appdata\roaming
# ↓ Writes configs to N:\noa\data\appdata\roaming\Claude\
```

### 3. Automatic Containment

Applications automatically write to NOA directories:

```
App tries to access: %APPDATA%\Claude\configs.json
Windows resolves:    N:\noa\data\appdata\roaming\Claude\configs.json
✅ Contained within noa_root
```

## Installer Pattern

Desktop app installers MUST use redirected paths:

### ❌ WRONG (System AppData)
```powershell
$configsPath = "$env:APPDATA\Claude\configs.json"
# If run BEFORE sourcing noa-env.ps1, writes to C:\Users\...
```

### ✅ CORRECT (NOA AppData)
```powershell
# Option 1: Source noa-env.ps1 first (recommended)
. "$NoaRoot\noa-env.ps1"
$configsPath = "$env:APPDATA\Claude\configs.json"
# → N:\noa\data\appdata\roaming\Claude\configs.json

# Option 2: Explicit NOA path construction
$noaAppData = Join-Path $NoaRoot "data\appdata\roaming"
$configsPath = Join-Path $noaAppData "Claude\configs.json"
# → N:\noa\data\appdata\roaming\Claude\configs.json
```

## Example: Claude Desktop MCP configs

**Updated installer** (`claude-desktop.ps1:135-143`):

```powershell
# Use NOA AppData (FR-001: Self-contained within noa_root)
$noaAppData = Join-Path $NoaRoot "data\appdata\roaming"
$mcpconfigsPath = Join-Path $noaAppData "Claude\claude_desktop_configs.json"

# Create directory within NOA
$mcpconfigsDir = Split-Path -Parent $mcpconfigsPath
if (-not (Test-Path $mcpconfigsDir)) {
    New-Item -ItemType Directory -Path $mcpconfigsDir -Force | Out-Null
}
```

**Result**:
```
configs location: N:\noa\data\appdata\roaming\Claude\claude_desktop_configs.json
✅ CONTAINED within noa_root
```

## Benefits

### 1. Complete Containment (FR-001)
- ✅ All data within `noa_root`
- ✅ No dependencies on C: drive
- ✅ No user-specific paths

### 2. Portability
- ✅ Move `noa_root` to any location
- ✅ Move to different machine
- ✅ Works on different drives (C:, D:, N:, etc.)

### 3. Isolation
- ✅ Multiple NOA instances possible
- ✅ No conflict with system-wide apps
- ✅ Clean uninstall (delete `noa_root`)

### 4. Security
- ✅ Defined boundary (`noa_root`)
- ✅ Auditable data locations
- ✅ No data leakage outside boundary

### 5. P2P Sync
- ✅ All app data included in P2P sync
- ✅ Consistent state across devices
- ✅ Simple backup (backup `noa_root/data`)

## Cross-Platform Compatibility

### Windows
- `$env:APPDATA` → `noa_root/data/appdata/roaming`
- `$env:LOCALAPPDATA` → `noa_root/data/appdata/local`
- `$env:TEMP` → `noa_root/tmp`

### Unix (Linux/macOS)
- `XDG_configs_HOME` → `noa_root/etc`
- `XDG_DATA_HOME` → `noa_root/data`
- `XDG_CACHE_HOME` → `noa_root/data/cache`
- `XDG_STATE_HOME` → `noa_root/data/state`
- `TMPDIR` → `noa_root/tmp`

Both systems achieve **same result**: All app data contained in `noa_root`.

## Verification

### Check AppData Redirection

```powershell
# Windows
. .\noa-env.ps1
Write-Host $env:APPDATA
# Expected: N:\noa\data\appdata\roaming (or your noa_root path)

# Unix
source .noa-env
echo $XDG_configs_HOME
# Expected: /path/to/noa/etc
```

### Verify App Data Location

```powershell
# After installing Claude Desktop
Get-ChildItem -Recurse N:\noa\data\appdata\roaming\Claude
# Should show: claude_desktop_configs.json
```

### Test Containment

```powershell
# Search for any files outside noa_root
# (Should return nothing)
Get-ChildItem C:\Users\$env:USERNAME\AppData -Recurse -Filter "*claude*"
Get-ChildItem C:\Users\$env:USERNAME\AppData -Recurse -Filter "*abacus*"
```

## Troubleshooting

### App still writes to C:\Users\...

**Cause**: App launched without NOA environment

**Fix**:
```powershell
# Always source noa-env.ps1 first
. N:\noa\noa-env.ps1

# Then launch app
claude-desktop
```

### configs not found after moving noa_root

**Cause**: Absolute paths in configs files

**Fix**: Use `${NOA_ROOT}` variable in configss:
```json
{
  "path": "${NOA_ROOT}/ai/mcp/server.js"
}
```

### App ignores environment variables

**Cause**: App hardcodes paths

**Solutions**:
1. Check app settings for configs location override
2. Use symbolic links: `mklink /D "C:\Users\...\AppData\Roaming\App" "N:\noa\data\appdata\roaming\App"`
3. Request portable version from vendor

## Policy Compliance

✅ **FR-001**: System MUST operate entirely inside `noa_root` directory with no hard dependencies on external paths
✅ **Spec.md:765**: "Data Residency: All data stored under `noa_root` directory? ☑ Yes"
✅ **Spec.md:1110**: "Internal = under `noa_root`, External = outside `noa_root`"
✅ **FR-035**: Executable binaries and wrappers in `bin/`
✅ **FR-031**: Optional packages in `opt/`

## References

- **Environment files**: `noa-env.ps1`, `.noa-env`
- **Spec**: `specs/001-noa-seed-foundation/spec.md`
  - FR-001: Self-contained operation
  - FR-031: opt/ directory
  - FR-035: bin/ directory
  - Clarifications (1110): Internal vs external
- **XDG Base Directory**: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
- **Windows AppData**: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid
