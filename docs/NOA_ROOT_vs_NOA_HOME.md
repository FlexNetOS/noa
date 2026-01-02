# NOA_ROOT vs NOA_HOME

**Version:** 1.0.0
**Date:** 2026-01-02
**Authority:** NOA Policy 03-CONFIG_CAS.md

---

## Overview

NOA uses **two distinct** environment variables to separate shared/persistent resources from instance-specific/runtime resources:

- **`NOA_ROOT`** - Ecosystem base install anchor (shared, persistent)
- **`NOA_HOME`** - Active instance directory (versioned/runtime-specific)

This separation enables:
- Multi-version installations
- Shared resource pools
- Instance isolation
- Clean upgrades/rollbacks

---

## NOA_ROOT (Shared, Persistent)

### Definition

**`NOA_ROOT`** is the **authoritative** installation root where all shared, persistent, and version-independent resources reside.

### Purpose

- Contains immutable baselines (Layer 1 configs)
- Stores content-addressed artifacts (CAS)
- Holds provider definitions and shared caches
- Maintains centralized data (databases, indexes)

### Typical Locations

| Platform | Default Path |
|----------|--------------|
| Windows | `C:\noa\` or `%LOCALAPPDATA%\noa\` |
| macOS/Linux | `/opt/noa/` or `~/.noa/` |
| Container | `/noa/` |

### Contents

```
${NOA_ROOT}/
├─ cas/                   # Content-addressed storage
├─ configs/base/          # Immutable baseline configs
├─ providers/             # Provider definitions
├─ data/                  # Shared databases, indexes
├─ gateway/               # MCP gateway
├─ sys/core/              # Core microkernel
├─ tools/                 # Shared tools
└─ lib/                   # Shared libraries
```

### Characteristics

- **Shared** - Multiple instances can reference same NOA_ROOT
- **Persistent** - Survives instance upgrades
- **Version-independent** - Not tied to specific runtime version
- **Read-mostly** - Rarely modified after install

---

## NOA_HOME (Instance-Specific, Runtime)

### Definition

**`NOA_HOME`** is the **active instance** directory where runtime-specific, mutable, and version-dependent resources reside.

### Purpose

- Contains runtime configurations (compiled settings)
- Stores instance-specific logs and caches
- Holds temporary files and workspaces
- Maintains process state and PIDs

### Typical Locations

| Scenario | Path |
|----------|------|
| Single-folder install | `${NOA_ROOT}` (same as ROOT) |
| Multi-version install | `${NOA_ROOT}/instances/v1.0.0/` |
| Development | `${NOA_ROOT}/instances/dev/` |
| User-specific | `~/.noa/home/` |

### Contents

```
${NOA_HOME}/
├─ configs/semantic/      # Mutable semantic configs (symlink or copy)
├─ configs/enforcement/   # Enforcement layer (symlink or copy)
├─ settings/resolved/     # Compiled runtime settings
├─ logs/                  # Instance logs
├─ cache/                 # Instance cache
├─ tmp/                   # Temporary files
├─ run/                   # PIDs, sockets
└─ state/                 # Runtime state
```

### Characteristics

- **Instance-specific** - Tied to one running instance
- **Ephemeral** - Can be deleted and recreated
- **Version-dependent** - May differ between versions
- **Read-write** - Frequently modified

---

## When to Use Which

### Use NOA_ROOT for:

✅ **Immutable Content**
- CAS objects (models, binaries, artifacts)
- Base layer configs (Layer 1)
- Provider definitions
- Shared tools and libraries

✅ **Shared Resources**
- Provider shared caches (KV cache, embedding cache)
- Centralized databases
- Gateway infrastructure

✅ **Version-Independent**
- Configuration schemas
- Policy documents
- Documentation

**Example Paths:**
```bash
${NOA_ROOT}/cas/objects/
${NOA_ROOT}/configs/base/schemas/
${NOA_ROOT}/providers/local/llama_cpp/
${NOA_ROOT}/data/db/postgres/
${NOA_ROOT}/gateway/mcp/agentgateway/
```

### Use NOA_HOME for:

✅ **Runtime State**
- Process PIDs
- Socket files
- Lock files

✅ **Instance Logs**
- Application logs
- Audit logs
- Metrics logs

✅ **Instance Cache**
- Temporary build artifacts
- Downloaded files (not in CAS yet)
- Session data

✅ **Compiled Settings**
- Resolved configuration (Layer 1 + Layer 2 + Layer 3)
- Profile-specific overrides

**Example Paths:**
```bash
${NOA_HOME}/logs/noa-api.log
${NOA_HOME}/cache/downloads/
${NOA_HOME}/tmp/sandbox-workspace-12345/
${NOA_HOME}/settings/resolved/runtime.json
${NOA_HOME}/run/noa.pid
```

---

## Configuration Examples

### Single-Folder Install (Default)

**Setup:**
```bash
export NOA_ROOT=/opt/noa
export NOA_HOME=${NOA_ROOT}  # Same directory
```

**Result:**
- All resources in one place
- Simpler for single-version setups
- Runtime and shared mixed (acceptable)

**Directory Structure:**
```
/opt/noa/
├─ cas/                   # From NOA_ROOT
├─ providers/             # From NOA_ROOT
├─ logs/                  # From NOA_HOME
├─ cache/                 # From NOA_HOME
└─ settings/resolved/     # From NOA_HOME
```

### Multi-Version Install

**Setup:**
```bash
export NOA_ROOT=/opt/noa
export NOA_HOME=/opt/noa/instances/v1.0.0
```

**Result:**
- Shared resources in NOA_ROOT
- Version-specific runtime in NOA_HOME
- Multiple versions can coexist

**Directory Structure:**
```
/opt/noa/                      # NOA_ROOT
├─ cas/                        # Shared
├─ providers/                  # Shared
├─ configs/base/               # Shared
└─ instances/
   ├─ v1.0.0/                  # NOA_HOME for v1.0.0
   │  ├─ logs/
   │  ├─ cache/
   │  └─ settings/resolved/
   └─ v1.1.0-beta/             # NOA_HOME for v1.1.0-beta
      ├─ logs/
      ├─ cache/
      └─ settings/resolved/
```

### Development Setup

**Setup:**
```bash
export NOA_ROOT=/opt/noa
export NOA_HOME=~/.noa/dev
```

**Result:**
- Shared resources remain stable
- Development instance isolated
- Safe to experiment

---

## Config File Usage

### Using NOA_ROOT

```json
{
  "cas": {
    "root": "${NOA_ROOT}/cas",
    "objects": "${NOA_ROOT}/cas/objects"
  },
  "providers": {
    "local": "${NOA_ROOT}/providers/local",
    "shared": "${NOA_ROOT}/providers/shared"
  },
  "gateway": {
    "binary": "${NOA_ROOT}/gateway/mcp/agentgateway/target/release/agentgateway-app"
  }
}
```

### Using NOA_HOME

```json
{
  "logging": {
    "log_path": "${NOA_HOME}/logs/application.log",
    "audit_log": "${NOA_HOME}/logs/audit.log"
  },
  "cache": {
    "downloads": "${NOA_HOME}/cache/downloads",
    "build": "${NOA_HOME}/cache/build"
  },
  "runtime": {
    "pid_file": "${NOA_HOME}/run/noa.pid",
    "settings": "${NOA_HOME}/settings/resolved/runtime.json"
  }
}
```

### Mixed Usage (Common)

```json
{
  "database": {
    "path": "${NOA_ROOT}/data/db/sqlite/noa.db",
    "wal": "${NOA_HOME}/cache/db-wal/"
  },
  "sandbox": {
    "definitions": "${NOA_ROOT}/configs/base/sandbox-definitions/",
    "workspaces": "${NOA_HOME}/tmp/sandboxes/"
  }
}
```

---

## Environment Setup

### Linux/macOS

**In `~/.bashrc` or `~/.zshrc`:**
```bash
# NOA Environment Variables
export NOA_ROOT=/opt/noa
export NOA_HOME=${NOA_ROOT}  # Single-folder install

# Or for multi-version:
# export NOA_HOME=${NOA_ROOT}/instances/$(cat ${NOA_ROOT}/active-version)

# Add to PATH
export PATH="${NOA_ROOT}/bin:${PATH}"
```

### Windows (PowerShell)

**In profile or environment:**
```powershell
# NOA Environment Variables
$env:NOA_ROOT = "C:\noa"
$env:NOA_HOME = $env:NOA_ROOT  # Single-folder install

# Or for multi-version:
# $env:NOA_HOME = "$env:NOA_ROOT\instances\1.0.0"

# Add to PATH
$env:PATH = "$env:NOA_ROOT\bin;$env:PATH"
```

### Docker/Container

```dockerfile
ENV NOA_ROOT=/noa
ENV NOA_HOME=/noa

# Or separate for multi-instance:
ENV NOA_HOME=/noa/instance
```

---

## Migration Guide

### From Old Structure (No Distinction)

**Before:**
- Everything in `${NOA_ROOT}`
- No separation of shared vs instance

**After:**
- Shared resources stay in `${NOA_ROOT}`
- Instance-specific moved to `${NOA_HOME}`

**Steps:**

1. **Set NOA_HOME (if not set):**
   ```bash
   export NOA_HOME=${NOA_ROOT}  # Start with same
   ```

2. **Identify instance-specific content:**
   - logs/
   - cache/ (non-CAS)
   - tmp/
   - settings/resolved/
   - run/

3. **For multi-version, create instance directory:**
   ```bash
   mkdir -p ${NOA_ROOT}/instances/v1.0.0
   export NOA_HOME=${NOA_ROOT}/instances/v1.0.0
   ```

4. **Move instance-specific content:**
   ```bash
   mv ${NOA_ROOT}/logs ${NOA_HOME}/
   mv ${NOA_ROOT}/cache ${NOA_HOME}/
   mv ${NOA_ROOT}/tmp ${NOA_HOME}/
   ```

5. **Update configs:**
   ```bash
   # Find and replace in configs
   find configs -name "*.json" -exec \
     sed -i 's|${NOA_ROOT}/logs|${NOA_HOME}/logs|g' {} \;
   find configs -name "*.json" -exec \
     sed -i 's|${NOA_ROOT}/cache|${NOA_HOME}/cache|g' {} \;
   ```

---

## Best Practices

### 1. Always Define Both Variables

Even for single-folder installs:
```bash
export NOA_ROOT=/opt/noa
export NOA_HOME=${NOA_ROOT}
```

### 2. Use NOA_ROOT for Shared Resources

- CAS storage
- Provider definitions
- Base configurations
- Shared libraries

### 3. Use NOA_HOME for Runtime State

- Logs
- Cache
- Temporary files
- Compiled settings

### 4. Keep NOA_ROOT Stable

- Minimize writes to NOA_ROOT
- Use NOA_HOME for frequent changes
- Allows safe multi-version setups

### 5. Backup Differently

- **NOA_ROOT**: Full backup, infrequent
- **NOA_HOME**: Incremental or skip (ephemeral)

### 6. Document in Configs

Always document which variable is appropriate:

```json
{
  "_comment": "Use NOA_ROOT for immutable shared resources",
  "cas_root": "${NOA_ROOT}/cas",

  "_comment": "Use NOA_HOME for instance-specific runtime",
  "log_dir": "${NOA_HOME}/logs"
}
```

---

## Troubleshooting

### Both Variables Not Set

**Symptom:** Configs fail to resolve paths

**Solution:**
```bash
# Check if set
echo $NOA_ROOT
echo $NOA_HOME

# Set if missing
export NOA_ROOT=/opt/noa
export NOA_HOME=${NOA_ROOT}
```

### Wrong Variable Used

**Symptom:** Logs appear in NOA_ROOT, or CAS in NOA_HOME

**Solution:**
- Review config files
- Update to use correct variable
- Move files to correct location

### Multi-Version Conflicts

**Symptom:** Different versions overwriting each other

**Solution:**
- Ensure each version has unique NOA_HOME
- Use `${NOA_ROOT}/instances/<version>/`
- Never run two versions with same NOA_HOME

---

## Summary

| Aspect | NOA_ROOT | NOA_HOME |
|--------|----------|----------|
| **Purpose** | Shared, persistent | Instance-specific |
| **Contains** | CAS, providers, base configs | Logs, cache, runtime |
| **Mutability** | Read-mostly | Read-write |
| **Versioning** | Version-independent | Version-dependent |
| **Sharing** | Shared across instances | Per-instance |
| **Backup** | Full, infrequent | Incremental or skip |
| **Example** | `/opt/noa/` | `/opt/noa/instances/v1.0.0/` |

**Default for Single-Folder:**
```bash
NOA_HOME=${NOA_ROOT}
```

**For Multi-Version:**
```bash
NOA_HOME=${NOA_ROOT}/instances/$(cat ${NOA_ROOT}/active-version)
```

---

**Status:** ✅ Documented | 📋 Ready for Implementation
