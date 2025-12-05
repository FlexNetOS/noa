# Legacy Setup Scripts

This directory contains archived setup scripts that are no longer recommended for use but are preserved for reference.

## Archived Scripts

### `setup-direct.ps1`
**Purpose**: Direct setup script with basic logging functionality.

**Status**: DEPRECATED - Replaced by `scripts/setup/setup-noa.ps1`

**Why Archived**:
- Limited functionality (basic directory creation only)
- Hardcoded paths to N:\noa
- No parameter support
- Basic error handling
- No prerequisite checking or profile integration

**Historical Note**: This script was used for early testing of the NOA directory structure with .NET file system APIs and basic logging to `setup-log.txt`.

### `adapt-script.ps1`
**Purpose**: Helper script to adapt the original `setup-agentic-os.ps1` script for NOA by performing path replacements.

**Status**: DEPRECATED - No longer needed as setup-noa.ps1 is purpose-built

**Why Archived**:
- Was a temporary adaptation tool
- Source script (setup-agentic-os.ps1) is external
- Direct implementation is more maintainable than adaptation
- Path replacement approach was fragile

**Historical Note**: This script performed regex-based path replacements (W:\WSL → n:\noa) to adapt an external script for NOA use. It served as a bridge during early development.

## Current Setup Solution

Use the modern, fully-featured setup script instead:

```powershell
# Full setup with all features
.\scripts\setup\setup-noa.ps1 -NoaRoot "N:\noa" -InstallPrereqs -IntegrateProfile

# CI/Testing setup
.\scripts\setup\setup-noa.ps1 -NoaRoot "$env:TEMP\noa" -InstallPrereqs:$false -IntegrateProfile:$false

# Custom location without integration
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\MyNOA"
```

See the main README.md for complete documentation.

## Migration Guide

If you were using these legacy scripts:

1. **From `setup-direct.ps1`**:
   - Replace with: `.\scripts\setup\setup-noa.ps1`
   - Gain: Parameter support, better logging, error handling, profile integration

2. **From `adapt-script.ps1`**:
   - No migration needed - this was a development tool
   - Use `setup-noa.ps1` directly instead

## Removal Timeline

These scripts are preserved for historical reference and may be removed in a future major version once migration is complete across all use cases.
