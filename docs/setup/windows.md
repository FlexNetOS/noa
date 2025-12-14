# NOA Windows Setup Guide

Complete guide for setting up NOA on Windows 11.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Advanced Topics](#advanced-topics)
- [Migration from Legacy Scripts](#migration-from-legacy-scripts)

## Overview

NOA provides a fully automated setup process for Windows that:

- Creates a comprehensive directory structure for your NOA environment
- Configures environment variables and helper functions
- Optionally installs prerequisites (PowerShell 7.4+, Git, 7-Zip)
- Optionally integrates with your PowerShell profile for automatic loading
- Provides detailed logging for troubleshooting

### What Gets Created

```
N:\noa\                    # Default root (customizable)
├── repos\                 # Git repositories
├── containers\            # Docker containers and configs
├── workspace\             # Active project workspace
├── config\                # Configuration files
│   └── noa.json          # Main NOA configuration
├── scripts\               # Automation scripts
├── logs\                  # Log files
│   └── setup-*.log       # Setup logs with timestamps
├── tmp\                   # Temporary files
├── p2p\                   # Peer-to-peer networking
├── ai\                    # AI models and configs
├── git\                   # Git workflows and hooks
├── bin\                   # Executables
├── etc\                   # Additional configuration
├── lib\                   # Libraries
├── opt\                   # Optional packages
├── sys\                   # System files
├── init\                  # Initialization scripts
├── noa-profile.ps1       # Environment profile (auto-generated)
└── .noa                  # Marker file
```

## Prerequisites

### Required

- **Windows 11** (or Windows 10 with latest updates)
- **PowerShell 7.4+** - [Download from Microsoft](https://aka.ms/powershell)
  ```powershell
  # Check your version
  $PSVersionTable.PSVersion
  ```

### Optional (Auto-installed with `-InstallPrereqs`)

- **Git** - Version control system
- **winget** - Windows Package Manager (for automated installations)
- **7-Zip** - Archive utility (optional)

### Checking Prerequisites Manually

```powershell
# Check PowerShell version
$PSVersionTable.PSVersion

# Check for Git
git --version

# Check for winget
winget --version
```

## Installation

### Basic Installation (Default Location)

```powershell
# Clone the repository
git clone https://github.com/FlexNetOS/noa.git
cd noa

# Run setup with default settings (N:\noa)
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
.\scripts\setup\setup-noa.ps1
```

### Full Installation with Prerequisites

```powershell
# Install prerequisites and integrate with PowerShell profile
.\scripts\setup\setup-noa.ps1 -InstallPrereqs -IntegrateProfile
```

### Custom Installation Location

```powershell
# Install to C:\noa instead of N:\noa
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\noa"

# Install to a network share
.\scripts\setup\setup-noa.ps1 -NoaRoot "\\server\share\noa"

# Install to user profile
.\scripts\setup\setup-noa.ps1 -NoaRoot "$env:USERPROFILE\noa"
```

### CI/Testing Installation

```powershell
# Minimal installation for testing (no prereqs, no profile integration)
.\scripts\setup\setup-noa.ps1 -NoaRoot "$env:TEMP\noa" -InstallPrereqs:$false -IntegrateProfile:$false
```

## Configuration

### Setup Script Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `-NoaRoot` | String | `N:\noa` | Root directory for NOA installation |
| `-InstallPrereqs` | Switch | `$false` | Install prerequisites (Git, etc.) |
| `-IntegrateProfile` | Switch | `$false` | Add to PowerShell profile |
| `-Verbose` | Switch | `$false` | Enable verbose output |

### Profile Integration

When you use `-IntegrateProfile`, the setup adds this line to your PowerShell profile (`$PROFILE.CurrentUserAllHosts`):

```powershell
. "N:\noa\noa-profile.ps1"
```

This makes NOA load automatically in all PowerShell sessions.

**To manually integrate later:**

```powershell
# Add to your profile
$profilePath = "N:\noa\noa-profile.ps1"
$sourceLine = ". `"$profilePath`""

# Append to profile
$sourceLine | Add-Content -Path $PROFILE.CurrentUserAllHosts
```

**To manually load without integration:**

```powershell
. N:\noa\noa-profile.ps1
```

### Environment Variables

After loading the NOA profile, these variables are available:

```powershell
$env:NOA_ROOT        # N:\noa
$env:NOA_REPOS       # N:\noa\repos
$env:NOA_CONTAINERS  # N:\noa\containers
$env:NOA_WORKSPACE   # N:\noa\workspace
$env:NOA_CONFIG      # N:\noa\config
$env:NOA_SCRIPTS     # N:\noa\scripts
$env:NOA_LOGS        # N:\noa\logs
$env:NOA_TMP         # N:\noa\tmp
$env:NOA_P2P         # N:\noa\p2p
$env:NOA_AI          # N:\noa\ai
$env:NOA_GIT         # N:\noa\git
$env:NOA_BIN         # N:\noa\bin
```

### Navigation Functions

Quick navigation to NOA directories:

```powershell
cda      # Change to NOA root
cdr      # Change to repos
cdc      # Change to containers
cdw      # Change to workspace
cds      # Change to scripts
cdl      # Change to logs
cdp      # Change to p2p
cdai     # Change to ai
cdgit    # Change to git
```

## Troubleshooting

### Execution Policy Errors

**Error:**
```
.\setup-noa.ps1 : File cannot be loaded because running scripts is disabled on this system
```

**Solution:**
```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
```

This sets the policy for the current session only and doesn't require admin rights.

### Permission Denied

**Error:**
```
Access to the path is denied
```

**Solutions:**

1. **Run as Administrator** (if installing to system locations)
2. **Choose a different location** where you have write permissions:
   ```powershell
   .\scripts\setup\setup-noa.ps1 -NoaRoot "$env:USERPROFILE\noa"
   ```
3. **Check disk permissions** and ensure the target drive/folder is writable

### Drive Not Available

**Error:**
```
Cannot find drive. A drive with the name 'N' does not exist
```

**Solution:**

Use a different location:
```powershell
# Use C drive
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\noa"

# Or map N: drive first
net use N: \\server\share\noa
```

### winget Not Found

**Error:**
```
winget not found. Cannot auto-install prerequisites.
```

**Solution:**

1. **Install App Installer** from Microsoft Store
2. **Or install prerequisites manually:**
   ```powershell
   # Install Git manually
   # Download from: https://git-scm.com/download/win
   
   # Then run setup without -InstallPrereqs
   .\scripts\setup\setup-noa.ps1
   ```

### PowerShell Version Too Old

**Error:**
```
PowerShell 7.4+ required. Current: 5.1
```

**Solution:**

1. **Download PowerShell 7.4+**: https://aka.ms/powershell
2. **Or use winget:**
   ```powershell
   winget install Microsoft.PowerShell
   ```
3. **Close and reopen** your terminal
4. **Verify version:**
   ```powershell
   pwsh
   $PSVersionTable.PSVersion
   ```

### Setup Logs

All setup operations are logged to:
```
N:\noa\logs\setup-YYYYMMDD-HHMMSS.log
```

Check this file for detailed error information.

### Common Issues

**Profile not loading automatically:**
- Verify `-IntegrateProfile` was used during setup
- Check `$PROFILE.CurrentUserAllHosts` contains the source line
- Restart PowerShell

**Navigation functions not working:**
- Ensure profile is loaded: `. N:\noa\noa-profile.ps1`
- Check for typos in function names
- Verify NOA_ROOT is set: `echo $env:NOA_ROOT`

**Directories not created:**
- Check setup log file
- Verify write permissions
- Ensure disk has sufficient space

## Advanced Topics

### Idempotent Setup

The setup script is idempotent - you can run it multiple times safely:

```powershell
# First run
.\scripts\setup\setup-noa.ps1

# Later, to add profile integration
.\scripts\setup\setup-noa.ps1 -IntegrateProfile

# Run again (safe, won't duplicate)
.\scripts\setup\setup-noa.ps1 -IntegrateProfile
```

Existing files and directories are preserved. Generated files (`noa-profile.ps1`, `.noa`, `config/noa.json`) are regenerated.

### Multiple NOA Installations

You can have multiple NOA installations:

```powershell
# Development installation
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\noa-dev"

# Testing installation
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\noa-test"

# Production installation
.\scripts\setup\setup-noa.ps1 -NoaRoot "N:\noa"
```

Load the desired profile manually:
```powershell
. C:\noa-dev\noa-profile.ps1
```

### Unattended Installation

For scripting or automation:

```powershell
# Silent installation (no prompts)
$ErrorActionPreference = "Stop"
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\noa" -InstallPrereqs:$false -IntegrateProfile:$false

# Check exit code
if ($LASTEXITCODE -ne 0) {
    Write-Error "Setup failed with exit code: $LASTEXITCODE"
    exit 1
}
```

### Network/UNC Paths

NOA supports UNC paths:

```powershell
# Install to network share
.\scripts\setup\setup-noa.ps1 -NoaRoot "\\fileserver\shared\noa"

# Install to mapped drive
net use Z: \\fileserver\shared
.\scripts\setup\setup-noa.ps1 -NoaRoot "Z:\noa"
```

**Note:** Network paths may have different permissions. Ensure you have full control.

### Customizing noa.json

After setup, edit `config/noa.json` to customize your configuration:

```json
{
  "version": "2.0.0",
  "name": "NOA",
  "description": "Network Orchestration and Automation",
  "custom_settings": {
    "your_setting": "value"
  }
}
```

## Migration from Legacy Scripts

### From setup-noa-windows.ps1

The old script has been enhanced and moved:

**Old:**
```powershell
.\setup-noa-windows.ps1
```

**New:**
```powershell
.\scripts\setup\setup-noa.ps1
```

The new script adds:
- Parameter support for customization
- Prerequisite installation
- Profile integration
- Better error handling and logging
- Support for any path (not just N:\noa)

### From setup-noa-simple.ps1

The simple script remains as a fallback but points to the new script:

```powershell
# Old way (still works)
.\setup-noa-simple.ps1

# Recommended new way
.\scripts\setup\setup-noa.ps1
```

### From setup-direct.ps1 or adapt-script.ps1

These scripts have been archived to `scripts/archive/`:

**Old:**
```powershell
.\setup-direct.ps1
.\adapt-script.ps1
```

**New:**
```powershell
.\scripts\setup\setup-noa.ps1
```

See `scripts/archive/README.md` for historical context.

## Getting Help

### Check Setup Status

```powershell
# View setup log
Get-Content N:\noa\logs\setup-*.log | Select-Object -Last 50

# Check if NOA is loaded
echo $env:NOA_ROOT

# List all NOA environment variables
Get-ChildItem env:NOA_*
```

### Run Tests

```powershell
# Install Pester (if not installed)
Install-Module -Name Pester -MinimumVersion 5.0.0 -Force -Scope CurrentUser

# Run setup tests
Import-Module Pester
Invoke-Pester scripts/tests/Setup.Tests.ps1
```

### Common Commands

```powershell
# Re-run setup
.\scripts\setup\setup-noa.ps1

# Load profile manually
. N:\noa\noa-profile.ps1

# Check prerequisites
.\scripts\setup\install-prereqs.ps1

# View configuration
Get-Content N:\noa\config\noa.json | ConvertFrom-Json
```

## Additional Resources

- **Main README**: [README.md](../../README.md)
- **Setup Script**: [scripts/setup/setup-noa.ps1](../../scripts/setup/setup-noa.ps1)
- **Tests**: [scripts/tests/Setup.Tests.ps1](../../scripts/tests/Setup.Tests.ps1)
- **CI Workflow**: [.github/workflows/setup-ci.yml](../../.github/workflows/setup-ci.yml)

---

## History and Previous Fixes

This section consolidates information from previous setup documentation.

### Quote Parsing Issues (Fixed)

Earlier versions of setup scripts had PowerShell quote parsing issues that caused syntax errors. These were resolved by:

1. **Using array join instead of string concatenation**
   ```powershell
   # Bad (parser issues)
   $content = "Line 1`n" + "Line 2`n"
   
   # Good (quote-safe)
   $lines = @('Line 1', 'Line 2')
   $content = $lines -join "`r`n"
   ```

2. **Using `[char]34` for embedded quotes**
   ```powershell
   # Bad
   $sb.Append('"value"')
   
   # Good
   $sb.Append([char]34).Append('value').Append([char]34)
   ```

3. **Avoiding here-strings in complex contexts**
   ```powershell
   # Bad (parser issues in some contexts)
   $content = @'
   Some "content"
   '@
   
   # Good (explicit array)
   $content = @(
       'Some "content"'
   ) -join "`r`n"
   ```

These techniques ensure the script parses correctly in all PowerShell environments.

### Directory Structure Evolution

**Version 1.0** (setup-direct.ps1):
- Basic directories only
- Hardcoded N:\noa path
- Limited error handling

**Version 1.5** (setup-noa-windows.ps1):
- Full directory structure
- Quote-safe string building
- Better error messages

**Version 2.0** (scripts/setup/setup-noa.ps1):
- Parameterized paths
- Prerequisite installation
- Profile integration
- Comprehensive logging
- CI/CD testing

---

**Last Updated:** 2025-12-05  
**Script Version:** 2.0.0
