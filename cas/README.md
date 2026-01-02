# Content-Addressed Storage (CAS)

**Location:** `${NOA_ROOT}/cas/`
**Authority:** CANONICAL (per NOA policy 03-CONFIG_CAS.md)
**Status:** ✅ Implemented

---

## Overview

This is the **authoritative** Content-Addressed Storage root for NOA. All immutable artifacts, configurations, and versioned content are stored here using a Git-like Merkle DAG structure.

### Key Principles

1. **Immutability** - Content in `objects/` never changes
2. **Deduplication** - Same content = same hash = stored once
3. **Addressability** - All content referenced by cryptographic hash
4. **Versioning** - Mutable pointers in `refs/` and `tags/`
5. **Garbage Collection** - Unreachable objects can be pruned

---

## Directory Structure

```
cas/
├─ objects/              # Immutable content-addressed blobs
│  └─ <h0h1>/<h2h3>/<full_hash>
├─ refs/                 # Mutable pointers to objects
│  ├─ latest-kernel
│  ├─ active-config
│  └─ world-current
├─ tags/                 # Named stable references
│  ├─ v1.0.0
│  └─ stable
├─ registry/             # Catalog of CAS objects
│  ├─ models.json
│  ├─ prompts.json
│  └─ snapshots.json
├─ gc/                   # Garbage collection
│  ├─ sweep.log
│  └─ gc_rules.json
└─ merkle/               # Precomputed DAG structures
   └─ root.hash
```

---

## Components

### 1. objects/ - Immutable Storage

**Purpose:** Store all content-addressed blobs

**Structure:**
```
objects/<h0h1>/<h2h3>/<full_hash>
```

**Example:**
```
objects/ab/cd/abcdef1234567890...
```

**Hash Algorithm:** blake3 (configurable)

**Storage:**
- Binary blobs (models, binaries)
- JSON configuration snapshots
- Code artifacts
- Compressed archives

**Properties:**
- Write-once, read-many
- No modifications allowed
- Automatic deduplication
- Optional compression (zstd)

### 2. refs/ - Mutable Pointers

**Purpose:** Provide mutable references to immutable objects

**Format:**
```
refs/<name> → <hash>
```

**Common refs:**
- `latest-kernel` - Current kernel version
- `active-config` - Active configuration snapshot
- `agent-commander` - Main agent state
- `world-current` - Current world model

**Update Mechanism:**
- Atomic file replacement
- Previous value preserved in reflog (optional)
- No locks required (last-write-wins or CAS)

### 3. tags/ - Stable Named References

**Purpose:** Provide human-readable, stable version markers

**Format:**
```
tags/<name> → <hash>
```

**Examples:**
- `v1.0.0` - Release version
- `v1.1.0-beta` - Pre-release
- `stable` - Stable channel
- `dev` - Development channel

**Properties:**
- Should not change frequently
- Used for releases and milestones
- Can be signed for trust

### 4. registry/ - Object Catalog

**Purpose:** Index and classify CAS objects for discovery

**Files:**
- `models.json` - ML model registry
- `prompts.json` - Prompt templates
- `snapshots.json` - Configuration snapshots
- `binaries.json` - Binary artifacts
- `packages.json` - Package archives

**Schema:**
```json
{
  "objects": [
    {
      "hash": "abcdef...",
      "type": "model",
      "name": "llama-3.1-8b",
      "size": 5368709120,
      "created_at": "2026-01-02T00:00:00Z",
      "metadata": {
        "format": "gguf",
        "quantization": "Q4_K_M"
      }
    }
  ]
}
```

### 5. gc/ - Garbage Collection

**Purpose:** Manage cleanup of unreachable objects

**Files:**
- `gc_rules.json` - GC policies and configuration
- `sweep.log` - GC run history
- `orphans.json` - Detected orphaned objects

**GC Rules:**
```json
{
  "enabled": true,
  "interval_hours": 24,
  "min_age_days": 7,
  "keep_refs": true,
  "keep_tags": true,
  "keep_recent_days": 30
}
```

**GC Process:**
1. Mark: Start from all refs and tags
2. Sweep: Identify unreachable objects
3. Age check: Only remove if older than min_age
4. Purge: Delete orphaned objects
5. Log: Record what was removed

### 6. merkle/ - DAG Structures

**Purpose:** Precomputed Merkle DAG metadata for fast verification

**Files:**
- `root.hash` - Root of the DAG
- `tree.json` - Full tree structure
- `graph.dot` - GraphViz visualization

**Use Cases:**
- Fast integrity verification
- Efficient sync between devices
- Reachability analysis for GC
- Provenance tracking

---

## Usage Patterns

### Storing Content

```bash
# 1. Hash the content
HASH=$(cat file.bin | blake3sum)

# 2. Create directory structure
mkdir -p cas/objects/${HASH:0:2}/${HASH:2:2}

# 3. Store the object
cp file.bin cas/objects/${HASH:0:2}/${HASH:2:2}/$HASH

# 4. Update ref (if needed)
echo $HASH > cas/refs/latest-binary

# 5. Register in catalog
jq ".objects += [{hash: \"$HASH\", type: \"binary\"}]" \
  cas/registry/binaries.json > tmp.json && mv tmp.json cas/registry/binaries.json
```

### Retrieving Content

```bash
# 1. Read ref to get hash
HASH=$(cat cas/refs/latest-binary)

# 2. Retrieve object
H0H1=${HASH:0:2}
H2H3=${HASH:2:2}
cat cas/objects/$H0H1/$H2H3/$HASH
```

### Creating a Tag

```bash
# Tag current ref state
HASH=$(cat cas/refs/active-config)
echo $HASH > cas/tags/v1.0.0
```

### Running GC

```bash
# Find all reachable objects
find cas/refs -type f -exec cat {} \; > reachable.txt
find cas/tags -type f -exec cat {} \; >> reachable.txt

# Find all objects
find cas/objects -type f > all_objects.txt

# Identify orphans (simplified)
comm -23 <(sort all_objects.txt) <(sort reachable.txt) > orphans.txt

# Remove orphans older than 7 days
while read obj; do
  age=$(( ($(date +%s) - $(stat -f %m "$obj")) / 86400 ))
  if [ $age -gt 7 ]; then
    rm "$obj"
    echo "Removed: $obj" >> cas/gc/sweep.log
  fi
done < orphans.txt
```

---

## Integration with NOA

### Provider Artifacts

Providers store models and checkpoints in CAS:

```bash
# Store model
HASH=$(cat llama-3.1-8b.gguf | blake3sum)
# ... store in cas/objects/...

# Register
echo $HASH > cas/refs/model-llama-3.1-8b

# Add to registry
jq ".objects += [{
  hash: \"$HASH\",
  type: \"model\",
  name: \"llama-3.1-8b\",
  provider: \"llama_cpp\"
}]" cas/registry/models.json > tmp && mv tmp cas/registry/models.json
```

### Configuration Snapshots

Configs are versioned in CAS:

```bash
# Create snapshot of configs
tar czf configs-snapshot.tar.gz configs/

# Store in CAS
HASH=$(cat configs-snapshot.tar.gz | blake3sum)
# ... store ...

# Update ref
echo $HASH > cas/refs/active-config

# Tag for release
echo $HASH > cas/tags/v1.0.0
```

### Sandbox Artifacts

Sandbox outputs are promoted to CAS:

```bash
# After successful build
HASH=$(cat build-artifact.tar.gz | blake3sum)
# ... store ...

# Link to task
echo $HASH > cas/refs/task-$TASK_ID-output
```

### Rollback

```bash
# Rollback to previous version
PREV_HASH=$(cat cas/tags/v0.9.0)
echo $PREV_HASH > cas/refs/active-config

# Restart with previous config
# ... system reads cas/refs/active-config ...
```

---

## Configuration

### CAS Config

**Location:** `configs/base/cas/config.json`

```json
{
  "version": "1.0.0",
  "cas": {
    "root": "${NOA_ROOT}/cas",
    "hash_algorithm": "blake3",
    "compression": "zstd",
    "compression_level": 3
  },
  "objects": {
    "prefix_dirs": 2,
    "prefix_length": 2,
    "max_size_mb": 1024
  },
  "gc": {
    "enabled": true,
    "interval_hours": 24,
    "min_age_days": 7,
    "keep_refs": true,
    "keep_tags": true
  }
}
```

---

## Best Practices

### 1. Always Use CAS for Immutable Content

- Models, binaries, releases → CAS
- Temporary files, logs → cache/ or tmp/
- Mutable configs → configs/semantic/

### 2. Use Refs for Current State

- `latest-*` refs for rolling updates
- `active-*` refs for current selections
- Update refs atomically

### 3. Use Tags for Milestones

- Release versions (v1.0.0, v1.1.0)
- Stable channels (stable, beta, dev)
- Don't change tags frequently

### 4. Register Important Objects

- Add to appropriate registry file
- Include metadata (type, name, size)
- Enable discovery and tooling

### 5. Run GC Regularly

- Scheduled GC every 24 hours
- Manual GC before low-disk situations
- Review sweep.log for anomalies

### 6. Backup Refs and Tags

- Refs and tags are small but critical
- Back up to prevent data loss
- Can reconstruct from objects if needed

---

## Troubleshooting

### Object Not Found

**Symptom:** Ref points to non-existent object

**Solution:**
```bash
# Check ref
cat cas/refs/problematic-ref

# Search for object
find cas/objects -name <hash>

# Restore from backup or re-generate
```

### Disk Space Issues

**Symptom:** cas/objects/ consuming too much space

**Solution:**
```bash
# Run GC manually
# (See Running GC above)

# Check orphans
find cas/gc/orphans.json

# Review registry for large objects
jq '.objects | sort_by(.size) | reverse | .[0:10]' cas/registry/models.json
```

### Hash Collision (Extremely Rare)

**Symptom:** Two different files produce same hash

**Solution:**
- Blake3 collision is cryptographically infeasible
- If detected, this indicates corruption or attack
- Investigate immediately, restore from backup

---

## Security

### Integrity

- All objects verified by hash on read
- Corruption detected immediately
- No silent data corruption

### Authenticity

- Tags can be signed (future)
- Verify signatures before use
- Trust anchor in configs/base/

### Encryption

- Objects can be encrypted before storage
- Encryption keys in sys/core/secrets/
- Transparent decryption on read

---

## Migration from Legacy

### Old Location: data/cas/

If you have content in `data/cas/`, migrate:

```bash
# Move blobs to objects
mv data/cas/blobs/* cas/objects/

# Move refs
mv data/cas/refs/* cas/refs/

# Move index to registry
mv data/cas/index/* cas/registry/

# Update all config references
find configs providers gateway -name "*.json" -exec \
  sed -i 's|data/cas/|cas/|g' {} \;
```

---

## Status

- ✅ Directory structure created
- ✅ README documented
- ⏳ GC implementation (Phase 3)
- ⏳ Registry population (Phase 3)
- ⏳ Integration with providers (Phase 3)
- ⏳ Merkle DAG builder (Phase 3)

---

**End of CAS Documentation**
