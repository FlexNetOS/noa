# NOA Bootstrap System

The bootstrap system provides a single-command installation of the complete NOA environment.

## Quick Start

### Windows (PowerShell)

```powershell
# Minimal setup (directories only)
.\scripts\bootstrap\bootstrap.ps1

# Full setup with all toolchains
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools

# Full setup including AI providers
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools -InstallAiProviders
```

### Unix (Bash)

```bash
# Minimal setup (directories only)
./scripts/bootstrap/bootstrap.sh

# Full setup with all toolchains
./scripts/bootstrap/bootstrap.sh --install-all-tools

# Full setup including AI providers
./scripts/bootstrap/bootstrap.sh --install-all-tools --install-ai-providers
```

## Architecture

```
scripts/bootstrap/
├── bootstrap.ps1          # Main Windows orchestrator
├── bootstrap.sh           # Main Unix orchestrator
├── lib/                   # Core libraries
│   ├── directories.ps1    # Directory creation
│   ├── download.ps1       # Download utilities
│   ├── logging.ps1        # Logging utilities
│   ├── platform.ps1       # Platform detection
│   ├── state.ps1          # State management
│   └── verification.ps1   # Tool verification
├── configs/                # configsuration scripts
│   ├── cache-setup.ps1    # Cache directory setup
│   ├── provider-cache.ps1 # AI provider caches
│   └── log-setup.ps1      # Logging setup
├── installers/            # Tool installers
│   ├── rust-portable.ps1  # Rust toolchain
│   ├── go-portable.ps1    # Go toolchain
│   ├── node-portable.ps1  # Node.js
│   ├── python-portable.ps1# Python
│   ├── ai-providers/      # AI CLI installers
│   ├── dev-tools/         # IDE & dev tools
│   └── shared-resources/  # Shared AI resources
├── verify/                # Verification scripts
│   ├── verify-all.ps1     # Full verification
│   └── smoke-test.ps1     # Toolchain tests
├── generators/            # Environment generators
│   ├── noa-env.ps1        # Generate env script
│   └── shell-integration.ps1
└── report/                # Reporting
    └── generate-report.ps1
```

## Phases

Bootstrap runs in phases:

1. **Platform Detection** - Detect OS, architecture, shell
2. **Directory Creation** - Create noa_root structure
3. **State Initialization** - Initialize bootstrap state tracking
4. **Tool Installation** - Install toolchains (if requested)
5. **AI Providers** - Install AI CLIs (if requested)
6. **Shared Resources** - Setup shared AI resources
7. **Verification** - Verify all installations
8. **Report** - Generate installation report

## configsuration

### bootstrap-tools.json

Lists all tools with versions and download URLs:

```json
{
  "tools": {
    "rust": { "version": "1.83.0", ... },
    "go": { "version": "1.23.4", ... },
    ...
  }
}
```

### bootstrap-state.json

Tracks installation state:

```json
{
  "tools": {
    "rust": { "version": "1.83.0", "status": "installed" },
    ...
  }
}
```

## Verification

After bootstrap, verify with:

```powershell
# Full verification
.\scripts\bootstrap\verify\verify-all.ps1

# Smoke tests (compile & run)
.\scripts\bootstrap\verify\smoke-test.ps1

# Generate report
.\scripts\bootstrap\report\generate-report.ps1
```

## Troubleshooting

### Windows

- Ensure PowerShell execution policy allows scripts
- Run as Administrator if writing to protected paths
- Check `logs/bootstrap/` for detailed logs

### Unix

- Ensure scripts have execute permission: `chmod +x *.sh`
- Check for curl/wget availability
- Check `logs/bootstrap/` for detailed logs

## Related Files

- `configs/bootstrap-tools.json` - Tool versions and URLs
- `configs/bootstrap-state.json` - Installation state
- `noa-env.ps1` / `.noa-env` - Generated environment scripts

