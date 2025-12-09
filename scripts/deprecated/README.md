# Deprecated Scripts

These scripts have been superseded by the unified bootstrap system.

## Redirect Map

| Old Script | New Script | Notes |
|------------|------------|-------|
| `scripts/setup.ps1` | `scripts/bootstrap/bootstrap.ps1` | Use `-InstallAllTools` flag |
| `scripts/install-tools.ps1` | `scripts/setup/install-all-tools.ps1` | Called by bootstrap |
| `scripts/check-prereqs.ps1` | `scripts/setup/check-prereqs.ps1` | Use via bootstrap |

## Migration

If you have automation using old scripts, update as follows:

### Before (Old)
```powershell
.\scripts\setup.ps1
```

### After (New)
```powershell
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools -InstallAiProviders
```

## Why Deprecated?

The old scripts:
1. Were not unified (different entry points for different tasks)
2. Lacked proper phase ordering
3. Didn't support all AI providers
4. Missing shared resource setup

The new bootstrap system:
1. Single entry point (`bootstrap.ps1` / `bootstrap.sh`)
2. Proper phase ordering with dependencies
3. Full AI provider support
4. Shared resource setup built-in
5. Cross-platform parity guaranteed

## Questions?

See `docs/setup/bootstrap-complete-guide.md` for full documentation.

