# Content-Addressable Storage (CAS) — Modules Store Policy

**Document ID**: POL-CAS-MODULES-001  
**Version**: 1.0.0  
**Last Updated**: 2025-12-17  
**Status**: Active

---

## Overview

This document defines the configuration policy for the NOA **module artifact CAS**.

This is a **subsystem-specific CAS** (module artifacts). It is compatible with the broader, Git-like CAS spine described in `docs/05-policy/config-cas.md`.

---

## Storage Architecture

### Directory Structure

```text
${NOA_ROOT}/
  data/
    modules/
      cas/
        {h0}{h1}/                 # First 2 hex chars of hash
          {h2}{h3}/               # Next 2 hex chars of hash
            {full_hash}           # Complete SHA-256 hash as filename
        README.md                 # Module documentation
```

### Hash Format

- **Algorithm**: SHA-256 (256-bit, 64 hex characters)
- **Encoding**: Lowercase hexadecimal
- **Example**: `a1b2c3d4e5f6...` stored at `a1/b2/a1b2c3d4e5f6...`

---

## Configuration Settings

### Core Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.enabled` | boolean | `true` | Enable CAS storage |
| `cas.path` | string | `${NOA_ROOT}/data/modules/cas` | Module CAS root directory |
| `cas.hash_algorithm` | string | `sha256` | Hashing algorithm |
| `cas.dedup_enabled` | boolean | `true` | Enable deduplication |

### Storage Limits

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.max_blob_size` | integer | `104857600` | Max blob size (100MB) |
| `cas.max_total_size` | integer | `10737418240` | Max total storage (10GB) |
| `cas.gc_threshold` | float | `0.9` | GC trigger threshold (90%) |

### Retention Policy

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.retention_days` | integer | `30` | Unreferenced blob retention |
| `cas.gc_schedule` | string | `0 2 * * 0` | GC cron schedule (weekly) |
| `cas.gc_dry_run` | boolean | `false` | GC dry-run mode |

---

## Reference Counting

The module CAS uses reference counting to track blob usage:

1. **Increment**: When a module version references a blob
2. **Decrement**: When a module version is deleted
3. **Garbage Collection**: Blobs with zero references after retention period

### Database Schema

Reference counts are stored in `module_versions` table:

```sql
CREATE TABLE module_versions (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL,
    version TEXT NOT NULL,
    cas_hash TEXT NOT NULL,
    ref_count INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    FOREIGN KEY (module_id) REFERENCES modules(id)
);
```

---

## Operations

### Store Blob

```rust
/// Store content in CAS, returns hash
pub async fn store(content: &[u8]) -> Result<String, CasError> {
    let hash = sha256_hex(content);
    let path = cas_path(&hash);
    if !path.exists() {
        fs::write(&path, content)?;
    }
    Ok(hash)
}
```

### Retrieve Blob

```rust
/// Retrieve content by hash
pub async fn retrieve(hash: &str) -> Result<Vec<u8>, CasError> {
    let path = cas_path(hash);
    fs::read(&path).map_err(CasError::NotFound)
}
```

### Garbage Collection

```rust
/// Remove unreferenced blobs past retention
pub async fn gc(dry_run: bool) -> Result<GcReport, CasError> {
    let unreferenced = find_unreferenced_blobs()?;
    let expired = filter_expired(unreferenced, retention_days)?;
    if !dry_run {
        for blob in &expired {
            fs::remove_file(cas_path(blob))?;
        }
    }
    Ok(GcReport { removed: expired.len() })
}
```

---

## Security Considerations

### Integrity Verification

- Content hash is verified on read
- Corrupted blobs trigger integrity alerts
- Automatic re-fetch from source if available

### Access Control

- CAS directory permissions: `0750`
- Blob file permissions: `0640`
- Service user: `noa` or current user

### Audit Logging

All module CAS operations are logged:
- `cas.store`: Hash, size, source module
- `cas.retrieve`: Hash, requestor
- `cas.gc`: Removed count, freed space

---

## Integration Points

### Module Registry

The module registry (`data/modules/registry/`) uses the module CAS for:
- Module binary storage
- Configuration snapshots
- Artifact caching

### Execution Memory

Module CAS integrates with execution memory for:
- Caching large context blobs
- Storing reasoning artifacts
- Provider state snapshots

---

## Monitoring

### Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `cas_blobs_total` | gauge | Total blob count |
| `cas_bytes_total` | gauge | Total storage used |
| `cas_store_ops` | counter | Store operations |
| `cas_retrieve_ops` | counter | Retrieve operations |
| `cas_gc_removed` | counter | Blobs removed by GC |

### Health Checks

- `cas.available`: CAS directory writable
- `cas.integrity`: Random blob verification
- `cas.capacity`: Storage below threshold

---

## Related Documents

- `docs/05-policy/config-cas.md` - CAS spine and hybrid configuration model
- `data/modules/cas/README.md` - CAS directory documentation
- `config/database.yaml` - Database configuration
- `docs/05-policy/data-retention.md` - Data retention policy

---

**Approved By**: NOA Development Team  
**Review Date**: 2026-06-17
