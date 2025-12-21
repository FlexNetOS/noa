# Tools Lifecycle Policy

| Status | Version | FR |
|--------|---------|-----|
| **Active** | 1.0.0 | FR-163 |

## Core Principle

**Upgrade, never downgrade.**

Tool versions MUST only increase. Downgrades are prohibited without explicit approval and documented rationale.

---

## Version Pinning

All tools are version-pinned in `config/bootstrap-tools.json`:

```json
{
  "versionPinning": {
    "enabled": true,
    "archiveRetentionDays": 7,
    "archivePath": "${NOA_ROOT}/opt/archive",
    "rollbackEnabled": true,
    "autoUpgrade": false,
    "upgradeRequiresExplicitFlag": true,
    "versionCheckOnBootstrap": true,
    "warnIfOutdated": true
  }
}
```

### Pinning Fields

| Field | Description |
|-------|-------------|
| `min_version` | Minimum acceptable version |
| `latest_version` | Current recommended version |
| `pinned_version` | Exact version to install (if set) |

---

## Rules

### R1: Explicit Upgrade Flag

Upgrades require explicit intent:

```bash
# ✓ CORRECT - Explicit upgrade
./scripts/install-all-tools.ps1 -UpdateExisting -Tool rust

# ✗ FORBIDDEN - Silent upgrade
./scripts/install-all-tools.ps1  # Does NOT upgrade existing
```

### R2: Archive Before Upgrade

Before any upgrade:

1. Check current version
2. Archive current installation to `${NOA_ROOT}/opt/archive/`
3. Add README note with reason
4. Apply upgrade
5. Verify new version
6. Update `bootstrap-state.json`

```powershell
# Example upgrade flow
$tool = "rust"
$currentVersion = Get-ToolVersion $tool
$archivePath = Archive-Tool $tool -Reason "Upgrading to 1.84.0"
Install-Tool $tool -Version "1.84.0"
Update-BootstrapState $tool -Version "1.84.0" -PreviousArchive $archivePath
```

### R3: Version Check on Bootstrap

Bootstrap validates tool versions:

```javascript
// scripts/bootstrap.js
const state = getBootstrapState();
const config = getBootstrapTools();

for (const [tool, spec] of Object.entries(config.portable_toolchains)) {
  const installed = state.tools[tool]?.version;
  if (installed && semver.lt(installed, spec.min_version)) {
    console.warn(`⚠️ ${tool} ${installed} is below minimum ${spec.min_version}`);
  }
}
```

### R4: No Downgrades

Downgrade attempts fail with error:

```bash
# ✗ This will FAIL
./scripts/install-all-tools.ps1 -Tool rust -Version 1.82.0
# ERROR: Downgrade from 1.84.0 to 1.82.0 not permitted
# Use -ForceDowngrade with documented rationale
```

To force downgrade (requires documentation):

```bash
./scripts/install-all-tools.ps1 -Tool rust -Version 1.82.0 \
  -ForceDowngrade \
  -Reason "Critical regression in 1.84.0 async runtime, see issue #1234"
```

### R5: Warn If Outdated

Bootstrap warns when tools are outdated:

```
⚠️ Outdated tools detected:
   rust: 1.83.0 installed, 1.84.0 available
   node: 22.11.0 installed, 22.12.0 available

Run with -UpdateExisting to upgrade.
```

---

## Upgrade Commands

```bash
# Upgrade single tool
./scripts/install-all-tools.ps1 -UpdateExisting -Tool <name>

# Upgrade all tools
./scripts/install-all-tools.ps1 -UpdateExisting

# Check for updates (dry run)
./scripts/install-all-tools.ps1 -CheckUpdates

# Rollback to previous version
./scripts/install-all-tools.ps1 -Rollback -Tool <name>

# List archived versions
./scripts/install-all-tools.ps1 -ListArchived
```

---

## Archival Steps

Per `bootstrap-tools.json`:

1. **Check existence**: Verify tool is currently installed
2. **Move to archive**: `${NOA_ROOT}/opt/archive/{tool}-{version}-{timestamp}/`
3. **Download new**: Fetch new version from configured source
4. **Install**: Extract/install to standard location
5. **Update state**: Record new version in `bootstrap-state.json`
6. **Retention**: Keep archived version for `archiveRetentionDays`

---

## Rollback Steps

1. **Find archive**: Locate most recent archived version
2. **Remove current**: Delete current installation
3. **Restore**: Copy archived version to original location
4. **Update state**: Record rollback in `bootstrap-state.json`

---

## Bootstrap State

State tracked in `config/bootstrap-state.json`:

```json
{
  "version": "1.0.0",
  "lastBootstrap": "2025-12-19T00:00:00Z",
  "tools": {
    "rust": {
      "version": "1.84.0",
      "installedAt": "2025-12-19T00:00:00Z",
      "previousVersion": "1.83.0",
      "archivePath": "opt/archive/rust-1.83.0-20251219/"
    }
  }
}
```

---

## Verification

After any tool change:

```bash
# Verify all tools
./scripts/verify-tools.ps1

# Verify specific tool
./scripts/verify-tools.ps1 -Tool rust
```

Verification checks:
- Binary exists at expected path
- Version matches expected
- Dependencies satisfied
- PATH correctly configured

---

## Related Policies

- [04-GOVERNANCE_RETENTION.md](04-GOVERNANCE_RETENTION.md) - Archive retention
- [03-CONFIG_PACKAGE-MANAGER.md](03-CONFIG_PACKAGE-MANAGER.md) - pnpm upgrades
- [02-ENV_CANONICAL-VARS.md](02-ENV_CANONICAL-VARS.md) - Tool paths

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-19 | Initial policy; version pinning; upgrade-only rule |
