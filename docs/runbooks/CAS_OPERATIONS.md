# CAS Operations Runbook

**System**: Content-Addressed Storage (CAS)
**Version**: 1.0.0
**Last Updated**: 2026-01-02
**Owner**: NOA Platform Team

---

## Overview

This runbook covers operational procedures for the NOA Content-Addressed Storage (CAS) system, including object storage, retrieval, garbage collection, and troubleshooting.

## Prerequisites

- NOA root: `/n/noa` (or `$NOA_ROOT`)
- Bash shell access
- Required tools: `sha256sum` (or `b3sum`), `zstd` (optional)
- Permissions: Read/write access to `${NOA_ROOT}/cas/`

---

## Quick Reference

| Operation | Command |
|-----------|---------|
| Store object | `scripts/cas/store-object.sh <file> [type]` |
| Retrieve object | `scripts/cas/retrieve-object.sh <hash> [output]` |
| Create tag | `scripts/cas/create-tag.sh <tag> <hash> [msg]` |
| Update ref | `scripts/cas/update-ref.sh <ref> <hash> [msg]` |
| Run GC | `scripts/cas/gc-run.sh [--dry-run]` |
| Monitor cache | `scripts/cache/monitor-cache.sh` |
| Cleanup cache | `scripts/cache/cleanup-cache.sh [--dry-run]` |

---

## Common Operations

### 1. Store a File in CAS

**Use Case**: Store a new object (model, config, binary) in CAS

**Procedure**:
```bash
cd /n/noa

# Store object
HASH=$(bash scripts/cas/store-object.sh <file-path> <type> '<metadata-json>')

# Verify storage
echo "Object stored: $HASH"
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/verify.tmp
diff <file-path> /tmp/verify.tmp
```

**Types**: `model`, `prompt`, `snapshot`, `binary`, `package`

**Example**:
```bash
# Store a model
HASH=$(bash scripts/cas/store-object.sh \
  /models/llama-3.1-8b.gguf \
  model \
  '{"quantization":"Q4_K_M","size_gb":5}')
```

**Expected Output**:
```
Warning: blake3 not found, using sha256 fallback
Stored object: abc123...def (5.0G)
abc123...def
```

### 2. Retrieve an Object

**Use Case**: Get an object from CAS by hash, tag, or ref

**Procedure**:
```bash
cd /n/noa

# By hash (direct)
bash scripts/cas/retrieve-object.sh <hash> /output/path

# By tag
HASH=$(grep -oP '"object":\s*"\K[a-f0-9]{64}' cas/tags/<tag-name>)
bash scripts/cas/retrieve-object.sh "$HASH" /output/path

# By ref
HASH=$(cat cas/refs/<ref-path>)
bash scripts/cas/retrieve-object.sh "$HASH" /output/path
```

**Example**:
```bash
# Retrieve model by tag
HASH=$(grep -oP '"object":\s*"\K[a-f0-9]{64}' cas/tags/llama-v3.1)
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/llama-3.1.gguf
```

### 3. Create a Version Tag

**Use Case**: Create a named, stable reference to an object

**Procedure**:
```bash
cd /n/noa

# Create tag
bash scripts/cas/create-tag.sh <tag-name> <hash> "<description>"

# Verify
cat cas/tags/<tag-name>
```

**Example**:
```bash
# Tag a release
bash scripts/cas/create-tag.sh v1.0.0 "$HASH" "Production release v1.0.0"
bash scripts/cas/create-tag.sh latest "$HASH" "Latest stable version"
```

### 4. Update a Mutable Reference

**Use Case**: Update a pointer to track "current" or "latest" object

**Procedure**:
```bash
cd /n/noa

# Update ref
bash scripts/cas/update-ref.sh <ref-path> <hash> "<message>"

# Verify
cat cas/refs/<ref-path>

# Check reflog
cat cas/refs/logs/<ref-path>
```

**Example**:
```bash
# Update current model
bash scripts/cas/update-ref.sh models/llama/current "$NEW_HASH" "Deploy Llama 3.2"

# View history
tail -5 cas/refs/logs/models/llama/current
```

### 5. Run Garbage Collection

**Use Case**: Remove unreferenced objects to free disk space

**Procedure**:
```bash
cd /n/noa

# ALWAYS dry run first!
bash scripts/cas/gc-run.sh --dry-run

# Review what will be deleted
# If acceptable, run actual GC
bash scripts/cas/gc-run.sh

# Check results
echo "GC complete. Check logs for details."
```

**Safety Checklist Before GC**:
- [ ] Verify all important tags exist
- [ ] Check refs point to correct objects
- [ ] Run dry run first
- [ ] Review objects to be deleted
- [ ] Ensure backups exist (if critical data)

**Example Output**:
```
=== GC Summary ===
Total objects:        1234
Referenced:           987
Unreferenced:         247
Deleted:              247 objects (2.3G)
```

### 6. Monitor Cache Usage

**Use Case**: Check cache disk usage and identify cleanup needs

**Procedure**:
```bash
cd /n/noa

# Monitor all caches
bash scripts/cache/monitor-cache.sh

# Export metrics to JSON
bash scripts/cache/monitor-cache.sh --export-metrics

# View metrics
cat metrics/cache-stats.json
```

**Status Indicators**:
- **OK** (green): < 80% usage
- **WARNING** (yellow): 80-90% usage
- **CRITICAL** (red): > 90% usage

### 7. Cleanup Caches

**Use Case**: Free disk space by cleaning old cache entries

**Procedure**:
```bash
cd /n/noa

# Dry run for all caches
bash scripts/cache/cleanup-cache.sh --dry-run

# Cleanup specific cache
bash scripts/cache/cleanup-cache.sh pnpm --dry-run
bash scripts/cache/cleanup-cache.sh pnpm

# Emergency cleanup (aggressive)
bash scripts/cache/cleanup-cache.sh --force
```

**Cache Types**:
- `pnpm` - Package manager cache (5GB limit)
- `playwright` - Browser test artifacts (2GB limit)
- `downloads` - Download cache (10GB limit)
- `build_cache` - Build artifacts (3GB limit)
- `model_inference` - AI inference cache (2GB limit)
- `embeddings` - Vector embeddings cache (1GB limit)

---

## Advanced Operations

### Configuration Snapshot and Rollback

**Create Snapshot**:
```bash
cd /n/noa

# Create tarball
tar czf /tmp/config-snapshot.tar.gz -C configs .

# Store in CAS
HASH=$(bash scripts/cas/store-object.sh /tmp/config-snapshot.tar.gz snapshot)

# Tag with timestamp
TAG="snapshot-$(date +%Y%m%d-%H%M%S)"
bash scripts/cas/create-tag.sh "$TAG" "$HASH" "Config snapshot"

# Update latest pointer
bash scripts/cas/update-ref.sh snapshots/latest "$HASH" "Latest config"

echo "Snapshot: $TAG ($HASH)"
```

**Rollback to Snapshot**:
```bash
cd /n/noa

# Find snapshot tag
ls -lt cas/tags/snapshot-*

# Get hash from tag
TAG="snapshot-20260102-120000"
HASH=$(grep -oP '"object":\s*"\K[a-f0-9]{64}' cas/tags/$TAG)

# Retrieve and extract
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/restore.tar.gz
tar xzf /tmp/restore.tar.gz -C configs/

echo "Restored from: $TAG"
```

### Model Version Management

**Deploy New Model**:
```bash
cd /n/noa

# Store model
MODEL_FILE="/models/llama-3.2-8b-q4.gguf"
HASH=$(bash scripts/cas/store-object.sh "$MODEL_FILE" model \
  '{"name":"llama-3.2-8b","quantization":"Q4_K_M","version":"3.2"}')

# Create version tag
bash scripts/cas/create-tag.sh llama-v3.2 "$HASH" "Llama 3.2 8B Q4_K_M"

# Update production pointer
bash scripts/cas/update-ref.sh models/llama/production "$HASH" "Deploy v3.2"
```

**Rollback Model**:
```bash
cd /n/noa

# Check reflog for previous version
cat cas/refs/logs/models/llama/production

# Get previous hash (2nd to last line)
PREV_HASH=$(tail -n2 cas/refs/logs/models/llama/production | head -n1 | awk '{print $2}')

# Rollback
bash scripts/cas/update-ref.sh models/llama/production "$PREV_HASH" "Rollback to previous"

echo "Rolled back to: $PREV_HASH"
```

### Verify Object Integrity

```bash
cd /n/noa

# Retrieve with verification enabled (default)
CAS_VERIFY=true bash scripts/cas/retrieve-object.sh "$HASH" /tmp/verify.tmp

# Manual verification
bash scripts/cas/retrieve-object.sh "$HASH" /tmp/object.tmp
sha256sum /tmp/object.tmp
# Compare with $HASH
```

### Search for Objects

**By Hash**:
```bash
# Find object file
find /n/noa/cas/objects -name "<hash>*"

# Search in registries
grep "<hash>" /n/noa/cas/registry/*.json
```

**By Type**:
```bash
# List all model tags
ls -lh /n/noa/cas/tags/ | grep model

# Find model refs
find /n/noa/cas/refs/models -type f
```

**By Name** (requires registry metadata):
```bash
# Search registry for model name
grep -i "llama" /n/noa/cas/registry/models.json
```

---

## Monitoring and Alerts

### Daily Health Check

```bash
#!/bin/bash
# daily-cas-health.sh

cd /n/noa

echo "=== CAS Health Check ==="
echo "Date: $(date)"
echo ""

# Check CAS size
echo "CAS Storage:"
du -sh cas/
echo ""

# Check cache usage
echo "Cache Status:"
bash scripts/cache/monitor-cache.sh
echo ""

# Check recent GC
if [ -f logs/cas-gc.log ]; then
  echo "Last GC Run:"
  tail -5 logs/cas-gc.log
fi
```

### Alerting Thresholds

Set up monitoring for:
- **WARNING**: Cache > 80% of limit
- **CRITICAL**: Cache > 90% of limit
- **ERROR**: CAS total size > 50GB
- **ERROR**: Unreferenced objects > 10GB

---

## Troubleshooting

### Problem: "Object not found"

**Symptoms**:
```
Error: Object not found: abc123...
Checked: /n/noa/cas/objects/ab/c1/abc123...[.zst]
```

**Diagnosis**:
```bash
# Check if hash exists anywhere
find /n/noa/cas -name "*abc123*"

# Check registry
grep "abc123" /n/noa/cas/registry/*.json

# Check if ref/tag exists
find /n/noa/cas/{refs,tags} -type f -exec grep "abc123" {} \;
```

**Resolution**:
- If object was GC'd: Restore from backup or re-create
- If hash typo: Verify correct hash from tag/ref
- If wrong location: Check `$NOA_ROOT` environment variable

### Problem: "Hash mismatch! Object corrupted"

**Symptoms**:
```
Error: Hash mismatch! Object corrupted.
Expected: abc123...
Computed: def456...
```

**Diagnosis**:
```bash
# Check object file
ls -lh /n/noa/cas/objects/ab/c1/abc123*

# Try manual hash
sha256sum /n/noa/cas/objects/ab/c1/abc123...
```

**Resolution**:
1. Object is corrupted - delete it:
   ```bash
   rm /n/noa/cas/objects/ab/c1/abc123*
   ```

2. Re-store from source if available:
   ```bash
   bash scripts/cas/store-object.sh /original/file.bin model
   ```

3. If no source available: Check backups

### Problem: Cache at 100% / Disk full

**Symptoms**:
```
CRITICAL: Cache usage at 95% - cleanup recommended
```

**Immediate Action**:
```bash
cd /n/noa

# Emergency cleanup
bash scripts/cache/cleanup-cache.sh --force

# Check results
bash scripts/cache/monitor-cache.sh
```

**Long-term Fix**:
1. Adjust cache limits in `configs/base/cache/cache-policies.json`
2. Schedule more frequent cleanup (cron every 6 hours)
3. Add more disk space

### Problem: GC deleted important object

**Symptoms**:
```
Error: Object not found: abc123... (was working yesterday)
```

**Recovery**:
```bash
cd /n/noa

# Check if object had a ref
REF_NAME="models/llama/current"

# View reflog
cat cas/refs/logs/$REF_NAME

# Get old hash
OLD_HASH=$(tail -n3 cas/refs/logs/$REF_NAME | head -n1 | awk '{print $2}')

# Check if old hash still exists
bash scripts/cas/retrieve-object.sh "$OLD_HASH" /tmp/test

# If found, restore ref
bash scripts/cas/update-ref.sh "$REF_NAME" "$OLD_HASH" "Restore from reflog"
```

**Prevention**:
- Always create tags for important objects
- Maintain refs for "current" objects
- Review GC dry run before running
- Keep backups of critical objects

### Problem: Scripts fail with "command not found"

**Symptoms**:
```
scripts/cas/store-object.sh: line 42: zstd: command not found
```

**Resolution**:

For `zstd`:
```bash
# Install zstd
# Ubuntu/Debian:
apt-get install zstd

# macOS:
brew install zstd

# Windows (Git Bash):
# Download from https://github.com/facebook/zstd/releases
```

For `b3sum` (blake3):
```bash
# Install via cargo
cargo install b3sum

# Or use sha256 fallback (automatic)
```

---

## Scheduled Maintenance

### Daily (Automated)

```bash
# crontab -e
0 2 * * * cd /n/noa && bash scripts/cas/gc-run.sh >> logs/cas-gc.log 2>&1
0 */6 * * * cd /n/noa && bash scripts/cache/cleanup-cache.sh >> logs/cache-cleanup.log 2>&1
```

### Weekly (Manual)

- Review GC logs for anomalies
- Check cache usage trends
- Verify important tags/refs still exist
- Test object retrieval for critical assets

### Monthly (Manual)

- Audit registry metadata
- Review and update retention policies
- Check disk space projections
- Update documentation

---

## Emergency Procedures

### Emergency Disk Space Recovery

```bash
#!/bin/bash
# emergency-space-recovery.sh

cd /n/noa

echo "=== Emergency Disk Space Recovery ==="

# 1. Aggressive cache cleanup
bash scripts/cache/cleanup-cache.sh --force

# 2. Force GC (skip age checks)
bash scripts/cas/gc-run.sh --force

# 3. Clear temp files
rm -rf /tmp/*.tmp
rm -rf cache/downloads/*.partial

# 4. Report results
bash scripts/cache/monitor-cache.sh
du -sh cas/
```

### CAS Restore from Backup

```bash
# Assuming backup at /backups/cas-backup.tar.gz

cd /n/noa

# Stop all services using CAS
# systemctl stop noa-services (if applicable)

# Restore CAS
rm -rf cas/
tar xzf /backups/cas-backup.tar.gz -C .

# Verify restore
bash scripts/tests/test-cas-phase3.sh

# Restart services
# systemctl start noa-services
```

---

## Performance Tuning

### Optimize for Large Objects

Edit `configs/base/cas/config.json`:
```json
{
  "compression": {
    "enabled": true,
    "algorithm": "zstd",
    "level": 1  // Faster compression for large files
  },
  "performance": {
    "read_cache_mb": 512,  // Increase read cache
    "hash_workers": 8      // More parallel workers
  }
}
```

### Optimize for Many Small Objects

```json
{
  "compression": {
    "threshold_bytes": 4096,  // Don't compress small files
    "level": 5                // Higher compression for better space
  },
  "objects": {
    "prefix_dirs": 3,  // More subdirectories
    "prefix_length": 3
  }
}
```

---

## Security Considerations

### Access Control

CAS directories should have restricted permissions:
```bash
chmod 755 /n/noa/cas
chmod 644 /n/noa/cas/objects/*/*/*
chmod 755 /n/noa/cas/{refs,tags}
chmod 600 /n/noa/cas/registry/*.json
```

### Audit Trail

All ref updates are logged in reflog:
```bash
# View audit trail for a ref
cat /n/noa/cas/refs/logs/models/production
```

### Immutability

Objects in `cas/objects/` are immutable. Never modify in place:
```bash
# WRONG - don't do this
echo "modified" >> cas/objects/ab/cd/abc123...

# RIGHT - store new version
HASH=$(bash scripts/cas/store-object.sh /new/version.bin model)
bash scripts/cas/update-ref.sh models/current "$HASH"
```

---

## References

- [CAS Framework Documentation](../cas/README.md)
- [CAS Utility Scripts](../scripts/cas/README.md)
- [Phase 3 Implementation Summary](../PHASE_3_COMPLETE.md)
- [Cache Policies](../configs/base/cache/cache-policies.json)
- [GC Rules](../cas/gc/gc_rules.json)

---

**Runbook Version**: 1.0.0
**Last Updated**: 2026-01-02
**Next Review**: 2026-02-02
