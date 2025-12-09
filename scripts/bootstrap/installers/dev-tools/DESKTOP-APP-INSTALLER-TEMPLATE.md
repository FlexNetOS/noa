# Desktop App Installer Template - NOA Internal Global Install Pattern

## Overview

This document provides the template and guidelines for creating desktop app installers that follow NOA's **Install Containment Policy** (FR-001, FR-081).

### Key Principle: Internal Global Install

NOA implements a three-layer architecture that enables "internal global install" within `noa_root`:

1. **Directory Structure Layer**: Unix FHS-like structure (`bin/`, `opt/`, `etc/`, `lib/`, `sys/`)
2. **Environment Isolation Layer**: Environment variables and PATH precedence
3. **Kernel Abstraction Layer (NKAL)**: Unified interface across kernel modes

**Result**: Desktop apps in `opt/` function as if globally installed, while maintaining complete containment within `noa_root`.

---

## Policy Requirements

### FR-001: Self-Contained Operation
System MUST operate entirely inside `noa_root` directory with no hard dependencies on external paths.

### FR-031: Optional Packages in opt/
System MUST create and populate `noa_root/opt/` with optional packages (llama.cpp, llama-cpp-rs, **desktop apps**).

### FR-035: Executables and Wrappers in bin/
System MUST create and populate `noa_root/bin/` with executable binaries and wrappers.

### FR-081: Portable Toolchains Pattern
Desktop apps follow the same pattern as toolchains:
- Download/Install → `noa_root/opt/{app-name}/`
- Wrapper scripts → `noa_root/bin/{app-name}.cmd` (Windows) or `noa_root/bin/{app-name}` (Unix)
- Environment variables point to internal paths
- PATH precedence ensures internal tools override system-wide

### Spec Clarifications (spec.md:1110-1111)
- **Internal** = under `noa_root`, **External** = outside `noa_root`
- Globally-installed tools are **deprecated** - use internal versions
- Internal tools take precedence via `noa_root/bin` prepended to PATH
- Global tools NOT used unless `--allow-global` flag passed

---

## Installation Pattern

### Directory Structure

```
noa_root/
├── opt/                          # Optional packages (FR-031)
│   ├── {app-name}-desktop/       # Desktop app installation
│   │   ├── {App}.exe             # Main executable (Windows)
│   │   ├── resources/            # App resources
│   │   └── ...                   # Other files
│   └── {App}Setup-latest.exe     # Installer (temp, can be deleted after)
│
├── bin/                          # Wrappers (FR-035)
│   ├── {app-name}-desktop.cmd    # Windows wrapper
│   └── {app-name}-desktop        # Unix wrapper
│
└── ai/providers/.../config.json  # Provider config (if AI provider)
```

### Installation Flow

```
1. Download installer → opt/{App}Setup-latest.exe
2. Install/Extract  → opt/{app-name}-desktop/
3. Create wrapper   → bin/{app-name}-desktop.cmd
4. Update config    → ai/providers/.../config.json (if applicable)
5. Verify           → Test wrapper execution
```

---

## Template: PowerShell Installer

```powershell
<#
.SYNOPSIS
    Install {AppName} Desktop to NOA opt directory.

.DESCRIPTION
    Downloads and installs {AppName} Desktop to noa_root/opt/{app-name}-desktop/.
    Creates wrapper script in noa_root/bin/ for easy access.
    {Additional notes, e.g., authentication requirements, subscriptions}.

.PARAMETER NoaRoot
    NOA root directory (default: auto-detect)

.PARAMETER Force
    Force reinstall even if already installed

.EXAMPLE
    .\{app-name}-desktop.ps1
    .\{app-name}-desktop.ps1 -Force
#>
[CmdletBinding()]
param(
    [string]$NoaRoot,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# ============================================================================
# CONFIGURATION - Update these values for your app
# ============================================================================

$APP_NAME = "{app-name}"                           # e.g., "abacus", "claude", "chatgpt"
$APP_EXE_NAME = "{App}.exe"                        # e.g., "Abacus.exe", "Claude.exe"
$DOWNLOAD_URL = "{download-url}"                   # Direct download URL
$INSTALLER_NAME = "{App}Setup-latest.exe"          # Downloaded installer filename

# ============================================================================
# SETUP
# ============================================================================

# Auto-detect NOA_ROOT
if (-not $NoaRoot) {
    $NoaRoot = if ($env:NOA_ROOT) { $env:NOA_ROOT } else {
        Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)))
    }
}

$OPT_DIR = Join-Path $NoaRoot "opt"
$BIN_DIR = Join-Path $NoaRoot "bin"
$INSTALL_DIR = Join-Path $OPT_DIR "$APP_NAME-desktop"
$INSTALLER_PATH = Join-Path $OPT_DIR $INSTALLER_NAME
$WRAPPER_PATH = Join-Path $BIN_DIR "$APP_NAME-desktop.cmd"

Write-Host "NOA $APP_NAME Desktop Installer" -ForegroundColor Cyan
Write-Host "NOA_ROOT: $NoaRoot" -ForegroundColor Gray
Write-Host ""

# Create directories
New-Item -ItemType Directory -Path $OPT_DIR -Force -ErrorAction SilentlyContinue | Out-Null
New-Item -ItemType Directory -Path $BIN_DIR -Force -ErrorAction SilentlyContinue | Out-Null

# ============================================================================
# CHECK EXISTING INSTALLATION
# ============================================================================

$existingExe = Join-Path $INSTALL_DIR $APP_EXE_NAME
if ((Test-Path $existingExe) -and -not $Force) {
    try {
        $version = (Get-Item $existingExe).VersionInfo.ProductVersion
        Write-Host "  [OK] $APP_NAME Desktop already installed: v$version" -ForegroundColor Green
        Write-Host "  Location: $INSTALL_DIR" -ForegroundColor Gray
        Write-Host "  Use -Force to reinstall" -ForegroundColor Gray
        exit 0
    } catch {
        Write-Host "  [INFO] Existing installation found" -ForegroundColor Yellow
    }
}

# ============================================================================
# DOWNLOAD INSTALLER
# ============================================================================

if (-not (Test-Path $INSTALLER_PATH)) {
    Write-Host "  [INFO] Downloading $APP_NAME Desktop installer..." -ForegroundColor Yellow
    Write-Host "  URL: $DOWNLOAD_URL" -ForegroundColor Gray

    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFile($DOWNLOAD_URL, $INSTALLER_PATH)
        Write-Host "  [OK] Downloaded: $INSTALLER_PATH" -ForegroundColor Green
    } catch {
        Write-Host "  [ERROR] Download failed: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "  Manual installation:" -ForegroundColor Yellow
        Write-Host "    1. Download from: {manual-download-page-url}" -ForegroundColor Gray
        Write-Host "    2. Save to: $INSTALLER_PATH" -ForegroundColor Gray
        Write-Host "    3. Run this script again" -ForegroundColor Gray
        exit 1
    }
} else {
    Write-Host "  [OK] Installer already downloaded" -ForegroundColor Green
}

# ============================================================================
# INSTALL APPLICATION
# ============================================================================

Write-Host "  [INFO] Installing $APP_NAME Desktop to NOA opt directory..." -ForegroundColor Yellow
Write-Host "  Target: $INSTALL_DIR" -ForegroundColor Gray

try {
    # NOTE: Adjust installer flags based on actual installer type
    # Common installer types and their flags:
    #   - NSIS: /S (silent), /D={dir} (directory)
    #   - Inno Setup: /VERYSILENT, /DIR="{dir}"
    #   - MSI: /quiet, INSTALLDIR="{dir}"
    #   - Squirrel: --silent, --install-path="{dir}"

    $installArgs = @(
        "/S",                    # Silent install (adjust as needed)
        "/D=$INSTALL_DIR"        # Installation directory (adjust as needed)
    )

    $process = Start-Process -FilePath $INSTALLER_PATH -ArgumentList $installArgs -Wait -PassThru -NoNewWindow

    if ($process.ExitCode -eq 0) {
        Write-Host "  [OK] $APP_NAME Desktop installed successfully" -ForegroundColor Green
    } else {
        Write-Host "  [WARN] Installer exited with code: $($process.ExitCode)" -ForegroundColor Yellow
    }
} catch {
    Write-Host "  [ERROR] Installation failed: $_" -ForegroundColor Red
    exit 1
}

# ============================================================================
# VERIFY INSTALLATION
# ============================================================================

$appExe = Get-ChildItem -Path $INSTALL_DIR -Filter $APP_EXE_NAME -Recurse -ErrorAction SilentlyContinue |
    Select-Object -First 1

if (-not $appExe) {
    Write-Host "  [ERROR] Installation verification failed - $APP_EXE_NAME not found" -ForegroundColor Red
    Write-Host "  Expected location: $INSTALL_DIR" -ForegroundColor Gray
    exit 1
}

Write-Host "  [OK] Found: $($appExe.FullName)" -ForegroundColor Green

# ============================================================================
# CREATE WRAPPER SCRIPT
# ============================================================================

Write-Host "  [INFO] Creating wrapper script..." -ForegroundColor Yellow

$wrapperContent = @"
@echo off
REM $APP_NAME Desktop Wrapper - Generated by NOA bootstrap
REM Launches $APP_NAME Desktop from NOA opt directory

"$($appExe.FullName)" %*
"@

$wrapperContent | Set-Content -Path $WRAPPER_PATH -Encoding ASCII
Write-Host "  [OK] Created wrapper: $WRAPPER_PATH" -ForegroundColor Green

# ============================================================================
# UPDATE PROVIDER CONFIG (if AI provider)
# ============================================================================

# Uncomment and adjust if this is an AI provider
<#
$providerConfig = Join-Path $NoaRoot "ai\providers\{cloud|local|hybrid}\$APP_NAME\config.json"
if (Test-Path $providerConfig) {
    try {
        $config = Get-Content $providerConfig -Raw | ConvertFrom-Json

        if (-not $config.PSObject.Properties['desktop']) {
            $config | Add-Member -MemberType NoteProperty -Name 'desktop' -Value @{} -Force
        }

        $config.desktop = @{
            binaryPath = @{
                windows = "`${NOA_ROOT}/opt/$APP_NAME-desktop/$($appExe.Name)"
                unix = "`${NOA_ROOT}/opt/$APP_NAME-desktop/bin/$APP_NAME"
            }
            wrapper = @{
                windows = "`${NOA_ROOT}/bin/$APP_NAME-desktop.cmd"
                unix = "`${NOA_ROOT}/bin/$APP_NAME-desktop"
            }
        }

        $config | ConvertTo-Json -Depth 10 | Set-Content $providerConfig -Encoding UTF8
        Write-Host "  [OK] Updated provider config" -ForegroundColor Green
    } catch {
        Write-Host "  [WARN] Failed to update provider config: $_" -ForegroundColor Yellow
    }
}
#>

# ============================================================================
# COMPLETION
# ============================================================================

Write-Host ""
Write-Host "$APP_NAME Desktop installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Location: $INSTALL_DIR" -ForegroundColor Gray
Write-Host "Wrapper:  $WRAPPER_PATH" -ForegroundColor Gray
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  $APP_NAME-desktop        # Launch $APP_NAME Desktop" -ForegroundColor Gray
Write-Host ""
# Add any additional notes here
# Write-Host "Note: {Additional information}" -ForegroundColor Yellow
```

---

## Template: Bash Installer

```bash
#!/bin/bash
#
# {AppName} Desktop installer for NOA (Unix)
#
# Downloads and installs {AppName} Desktop to noa_root/opt/{app-name}-desktop/.
# Creates wrapper script in noa_root/bin/ for easy access.
# {Additional notes}

set -euo pipefail

# ============================================================================
# CONFIGURATION - Update these values for your app
# ============================================================================

APP_NAME="{app-name}"                              # e.g., "abacus", "claude"
APP_EXE_NAME="{App}"                               # e.g., "Abacus", "Claude"

# Platform-specific download URLs
MACOS_DOWNLOAD_URL="{macos-download-url}"          # macOS .dmg or .app.zip
LINUX_DOWNLOAD_URL="{linux-download-url}"          # Linux .AppImage or .tar.gz

# ============================================================================
# SETUP
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$SCRIPT_DIR")")")")}"

OPT_DIR="$NOA_ROOT/opt"
BIN_DIR="$NOA_ROOT/bin"
INSTALL_DIR="$OPT_DIR/$APP_NAME-desktop"
WRAPPER_PATH="$BIN_DIR/$APP_NAME-desktop"
FORCE="${1:-}"

# Platform detection
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    DOWNLOAD_URL="$MACOS_DOWNLOAD_URL"
    INSTALLER_EXT="dmg"  # or "app.zip"
else
    PLATFORM="linux"
    DOWNLOAD_URL="$LINUX_DOWNLOAD_URL"
    INSTALLER_EXT="AppImage"  # or "tar.gz"
fi

INSTALLER_PATH="$OPT_DIR/$APP_EXE_NAME-latest.$INSTALLER_EXT"

echo "NOA $APP_NAME Desktop Installer"
echo "Platform: $PLATFORM"
echo "NOA_ROOT: $NOA_ROOT"
echo ""

# Create directories
mkdir -p "$OPT_DIR" "$BIN_DIR"

# ============================================================================
# CHECK EXISTING INSTALLATION
# ============================================================================

if [[ -d "$INSTALL_DIR" && "$FORCE" != "--force" ]]; then
    if [[ -x "$INSTALL_DIR/bin/$APP_NAME" ]] || [[ -x "$INSTALL_DIR/$APP_EXE_NAME.app/Contents/MacOS/$APP_EXE_NAME" ]]; then
        echo "  [OK] $APP_NAME Desktop already installed"
        echo "  Location: $INSTALL_DIR"
        echo "  Use --force to reinstall"
        exit 0
    fi
fi

# ============================================================================
# DOWNLOAD INSTALLER
# ============================================================================

if [[ ! -f "$INSTALLER_PATH" ]]; then
    echo "  [INFO] Downloading $APP_NAME Desktop..."
    echo "  URL: $DOWNLOAD_URL"

    if command -v curl &>/dev/null; then
        curl -L -o "$INSTALLER_PATH" "$DOWNLOAD_URL" || {
            echo "  [ERROR] Download failed"
            echo ""
            echo "  Manual installation:"
            echo "    1. Download from: {manual-download-url}"
            echo "    2. Save to: $INSTALLER_PATH"
            echo "    3. Run this script again"
            exit 1
        }
    elif command -v wget &>/dev/null; then
        wget -O "$INSTALLER_PATH" "$DOWNLOAD_URL" || exit 1
    else
        echo "  [ERROR] Neither curl nor wget available"
        exit 1
    fi

    echo "  [OK] Downloaded: $INSTALLER_PATH"
else
    echo "  [OK] Installer already downloaded"
fi

# ============================================================================
# INSTALL APPLICATION
# ============================================================================

echo "  [INFO] Installing $APP_NAME Desktop to NOA opt directory..."
echo "  Target: $INSTALL_DIR"

mkdir -p "$INSTALL_DIR"

case "$PLATFORM" in
    macos)
        if [[ "$INSTALLER_EXT" == "dmg" ]]; then
            # Mount DMG and copy app bundle
            MOUNT_POINT=$(hdiutil attach "$INSTALLER_PATH" | grep Volumes | awk '{print $3}')
            cp -R "$MOUNT_POINT"/*.app "$INSTALL_DIR/" 2>/dev/null || true
            hdiutil detach "$MOUNT_POINT"
            APP_BIN="$INSTALL_DIR/$APP_EXE_NAME.app/Contents/MacOS/$APP_EXE_NAME"
        else
            # Unzip .app.zip
            unzip -q "$INSTALLER_PATH" -d "$INSTALL_DIR"
            APP_BIN="$INSTALL_DIR/$APP_EXE_NAME.app/Contents/MacOS/$APP_EXE_NAME"
        fi
        echo "  [OK] Installed $APP_EXE_NAME.app to $INSTALL_DIR"
        ;;

    linux)
        if [[ "$INSTALLER_EXT" == "AppImage" ]]; then
            # Make AppImage executable and place in install dir
            chmod +x "$INSTALLER_PATH"
            cp "$INSTALLER_PATH" "$INSTALL_DIR/$APP_EXE_NAME.AppImage"
            APP_BIN="$INSTALL_DIR/$APP_EXE_NAME.AppImage"
        else
            # Extract tar.gz
            tar -xzf "$INSTALLER_PATH" -C "$INSTALL_DIR"
            APP_BIN="$INSTALL_DIR/bin/$APP_NAME"
        fi
        echo "  [OK] Installed to $INSTALL_DIR"
        ;;
esac

# ============================================================================
# VERIFY INSTALLATION
# ============================================================================

if [[ ! -e "$APP_BIN" ]]; then
    echo "  [ERROR] Installation verification failed"
    echo "  Expected: $APP_BIN"
    exit 1
fi

echo "  [OK] Found: $APP_BIN"

# ============================================================================
# CREATE WRAPPER SCRIPT
# ============================================================================

echo "  [INFO] Creating wrapper script..."

cat > "$WRAPPER_PATH" <<EOF
#!/bin/bash
# $APP_NAME Desktop Wrapper - Generated by NOA bootstrap
# Launches $APP_NAME Desktop from NOA opt directory

"$APP_BIN" "\$@"
EOF

chmod +x "$WRAPPER_PATH"
echo "  [OK] Created wrapper: $WRAPPER_PATH"

# ============================================================================
# UPDATE PROVIDER CONFIG (if AI provider)
# ============================================================================

# Uncomment and adjust if this is an AI provider
: <<'COMMENT'
PROVIDER_CONFIG="$NOA_ROOT/ai/providers/{cloud|local|hybrid}/$APP_NAME/config.json"
if [[ -f "$PROVIDER_CONFIG" ]]; then
    echo "  [INFO] Updating provider configuration..."

    if command -v jq &>/dev/null; then
        jq --arg bin "$APP_BIN" --arg wrapper "$WRAPPER_PATH" \
            '.desktop = {
                binaryPath: {
                    windows: "${NOA_ROOT}/opt/'"$APP_NAME"'-desktop/'"$APP_EXE_NAME"'.exe",
                    unix: $bin
                },
                wrapper: {
                    windows: "${NOA_ROOT}/bin/'"$APP_NAME"'-desktop.cmd",
                    unix: $wrapper
                }
            }' "$PROVIDER_CONFIG" > "$PROVIDER_CONFIG.tmp"

        mv "$PROVIDER_CONFIG.tmp" "$PROVIDER_CONFIG"
        echo "  [OK] Updated provider config"
    else
        echo "  [WARN] jq not available, skipping config update"
    fi
fi
COMMENT

# ============================================================================
# COMPLETION
# ============================================================================

echo ""
echo "$APP_NAME Desktop installation complete!"
echo ""
echo "Location: $INSTALL_DIR"
echo "Wrapper:  $WRAPPER_PATH"
echo ""
echo "Usage:"
echo "  $APP_NAME-desktop        # Launch $APP_NAME Desktop"
echo ""
# Add any additional notes here
# echo "Note: {Additional information}"
```

---

## Examples

See the following reference implementations:
- **Abacus Desktop**: `scripts/bootstrap/installers/dev-tools/abacus-desktop.{ps1,sh}`
- **Claude Desktop**: `scripts/bootstrap/installers/dev-tools/claude-desktop.ps1`
- **ChatGPT Desktop**: `scripts/bootstrap/installers/dev-tools/chatgpt-desktop.ps1`

---

## Installer Types Reference

### Common Windows Installer Types

| Type | Silent Flag | Install Dir Flag | Example |
|------|-------------|------------------|---------|
| **NSIS** | `/S` | `/D={dir}` | Many Electron apps |
| **Inno Setup** | `/VERYSILENT` | `/DIR="{dir}"` | Notepad++, Paint.NET |
| **MSI** | `/quiet` or `/qn` | `INSTALLDIR="{dir}"` | Microsoft apps |
| **Squirrel** | `--silent` | `--install-path="{dir}"` | Slack, Discord |
| **Electron Builder** | `/S` | `/D={dir}` | VSCode, Atom |

### Common Unix Package Types

| Type | Platform | Extraction | Notes |
|------|----------|------------|-------|
| **DMG** | macOS | `hdiutil attach` + copy | Mount, copy .app, unmount |
| **App Bundle ZIP** | macOS | `unzip` | Extract and copy |
| **AppImage** | Linux | `chmod +x` | Single executable file |
| **tar.gz** | Linux | `tar -xzf` | Traditional archive |
| **deb** | Debian/Ubuntu | `dpkg -x` | Extract without installing |
| **Snap** | Linux | N/A | Requires snapd (avoid) |

---

## Checklist for New Desktop App Installer

- [ ] Replace all `{app-name}`, `{App}`, `{AppName}` placeholders
- [ ] Update download URLs for Windows, macOS, Linux
- [ ] Determine installer type and adjust flags
- [ ] Test silent install to custom directory
- [ ] Verify executable name and location after install
- [ ] Create both PowerShell (.ps1) and Bash (.sh) versions
- [ ] Test wrapper script execution
- [ ] Update AI provider config if applicable
- [ ] Add to bootstrap master script
- [ ] Document any special requirements (subscriptions, auth, etc.)
- [ ] Follow cross-platform parity (FR-088, FR-089)

---

## Why This Pattern Works

### Three-Layer Isolation Architecture

1. **Directory Layer** (`noa_root/` structure)
   - Mimics Unix FHS
   - Self-contained environment

2. **Environment Layer** (`noa-env.ps1` / `.noa-env`)
   - Environment variables point to internal paths
   - PATH precedence: `noa_root/bin` first

3. **Kernel Layer** (NKAL)
   - Unified abstraction
   - Supports VM/Container/Native modes

**Result**: Apps in `opt/` function as if globally installed, while maintaining complete containment.

---

## Troubleshooting

### Installer doesn't support custom directory
- Extract manually to `opt/{app-name}-desktop/`
- Update installer script to handle extraction instead

### Silent install flags unknown
- Run installer interactively and check for flags in help: `/? or --help`
- Use tools like 7-Zip to extract installer and examine setup scripts
- Search for "{InstallerName} silent install flags"

### App won't run from custom location
- Some apps hardcode paths - may need registry or config file edits
- Use symbolic links as workaround if necessary
- Consider requesting portable version from vendor

### Download URL changes frequently
- Use "latest" redirect URL if available
- Add fallback to manual download instructions
- Consider bundling installer in repository (if license allows)

---

## References

- **Spec**: `specs/001-noa-seed-foundation/spec.md`
  - FR-001: Self-contained operation
  - FR-031: opt/ directory
  - FR-035: bin/ directory  - FR-081: Portable toolchains pattern
  - Clarifications (1110-1111): Internal vs external dependencies

- **Environment**: `noa-env.ps1` / `.noa-env`
  - PATH precedence
  - Environment variable isolation

- **Bootstrap**: `scripts/bootstrap/bootstrap.{ps1,sh}`
  - Dependency order
  - Tool installation sequence
