# CAS Utility Scripts

Content-Addressed Storage (CAS) utility scripts for the NOA framework.

## Overview

These scripts provide low-level operations for the CAS layer:

- **store-object.sh** - Store files as content-addressed objects
- **retrieve-object.sh** - Retrieve objects by hash
- **update-ref.sh** - Update mutable references atomically
- **create-tag.sh** - Create named tags pointing to objects
- **gc-run.sh** - Garbage collect unreferenced objects

## Quick Start

```bash
# Store a file
HASH=$(./store-object.sh /path/to/file.txt generic)

# Retrieve by hash
./retrieve-object.sh $HASH /tmp/output.txt

# Create a reference
./update-ref.sh heads/main $HASH "Initial version"

# Create a tag
./create-tag.sh v1.0.0 $HASH "Release v1.0.0"

# Run garbage collection (dry run first)
./gc-run.sh --dry-run
./gc-run.sh
```

## Scripts

### store-object.sh

Store a file as a content-addressed object.

**Usage:**
```bash
./store-object.sh <file> [type] [metadata-json]
```

**Arguments:**
- `file` - Path to file to store
- `type` - Object type: model, prompt, snapshot, binary, package (optional)
- `metadata-json` - JSON metadata (optional)

**Returns:**
Object hash (blake3) on stdout

**Example:**
```bash
# Store a model file
HASH=$(./store-object.sh /models/llama-3.1-8b.gguf model '{"quantization":"Q4_K_M"}')
echo "Stored as: $HASH"
```

**Features:**
- Deduplication (existing objects not re-stored)
- Automatic compression (zstd level 3 for files > 1KB)
- Registry integration
- blake3 hashing (sha256 fallback)

### retrieve-object.sh

Retrieve an object from CAS by hash.

**Usage:**
```bash
./retrieve-object.sh <hash> [output-path]
```

**Arguments:**
- `hash` - Object hash (64 hex chars)
- `output-path` - Optional output file (default: stdout)

**Example:**
```bash
# Retrieve to stdout
./retrieve-object.sh abc123... | head

# Retrieve to file
./retrieve-object.sh abc123... /tmp/restored.gguf
```

**Features:**
- Automatic decompression
- Hash verification (set `CAS_VERIFY=false` to disable)
- Corruption detection

### update-ref.sh

Update a mutable reference atomically.

**Usage:**
```bash
./update-ref.sh <ref-name> <hash> [message]
```

**Arguments:**
- `ref-name` - Reference name (e.g., "heads/main", "models/current")
- `hash` - Target object hash
- `message` - Optional reflog message

**Example:**
```bash
# Update main branch
./update-ref.sh heads/main $NEW_HASH "Deploy v2.0"

# Update current model pointer
./update-ref.sh models/llama/current $MODEL_HASH "Switch to Q4_K_M"
```

**Features:**
- Atomic updates (temp file + rename)
- Reflog tracking (last 100 entries)
- Object existence validation

### create-tag.sh

Create a named tag pointing to an object.

**Usage:**
```bash
./create-tag.sh <tag-name> <hash> [message]
```

**Arguments:**
- `tag-name` - Tag name (e.g., "v1.0.0", "latest")
- `hash` - Target object hash
- `message` - Optional tag annotation

**Example:**
```bash
# Create release tag
./create-tag.sh v1.0.0 $HASH "First stable release"

# Create latest pointer
./create-tag.sh latest $HASH "Latest model snapshot"
```

**Features:**
- JSON tag metadata
- Timestamp tracking
- Tag annotations

### gc-run.sh

Run garbage collection to remove unreferenced objects.

**Usage:**
```bash
./gc-run.sh [--dry-run] [--force]
```

**Options:**
- `--dry-run` - Show what would be deleted
- `--force` - Skip age checks, delete immediately

**Example:**
```bash
# See what would be deleted
./gc-run.sh --dry-run

# Run actual GC
./gc-run.sh

# Force delete all unreferenced (dangerous!)
./gc-run.sh --force
```

**Features:**
- Reachability analysis from refs and tags
- Age-based retention (default 7 days)
- Safety checks
- Size reporting

**GC Policy:**
Objects are deleted if:
1. Not referenced by any ref or tag
2. Older than `MIN_AGE_DAYS` (default: 7)
3. Not a pinned object type (per gc_rules.json)

**Safety:**
- Always run `--dry-run` first
- Verify critical tags exist before GC
- GC does NOT delete:
  - Referenced objects
  - Recent objects (< MIN_AGE_DAYS)
  - Objects in active use

## Environment Variables

All scripts respect these environment variables:

- `NOA_ROOT` - NOA root directory (default: /n/noa)
- `CAS_ROOT` - CAS directory (default: ${NOA_ROOT}/cas)
- `CAS_VERIFY` - Verify hashes on read (default: true)

**Example:**
```bash
export NOA_ROOT=/opt/noa
export CAS_VERIFY=false  # Skip verification for speed
./retrieve-object.sh $HASH
```

## Integration Examples

### Store and Tag a Model

```bash
#!/bin/bash
# Deploy a new model version

MODEL_FILE="/models/llama-3.2-8b-q4.gguf"

# Store model in CAS
HASH=$(./store-object.sh "$MODEL_FILE" model '{
  "quantization": "Q4_K_M",
  "architecture": "llama",
  "version": "3.2"
}')

# Create version tag
./create-tag.sh v3.2-q4 "$HASH" "Llama 3.2 8B Q4_K_M quantization"

# Update current pointer
./update-ref.sh models/llama/current "$HASH" "Deploy Llama 3.2"

echo "Model deployed: $HASH"
echo "Access via: models/llama/current or tag v3.2-q4"
```

### Snapshot configsuration

```bash
#!/bin/bash
# Create configsuration snapshot

configs_DIR="/n/noa/configss"
SNAPSHOT_FILE=$(mktemp)

# Create tarball
tar czf "$SNAPSHOT_FILE" -C "$configs_DIR" .

# Store in CAS
HASH=$(./store-object.sh "$SNAPSHOT_FILE" snapshot '{
  "type": "full_configs",
  "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}')

# Tag with timestamp
TAG="snapshot-$(date +%Y%m%d-%H%M%S)"
./create-tag.sh "$TAG" "$HASH" "configsuration snapshot"

# Update latest
./update-ref.sh snapshots/latest "$HASH" "Latest configs snapshot"

rm "$SNAPSHOT_FILE"
echo "Snapshot created: $TAG ($HASH)"
```

### Rollback configsuration

```bash
#!/bin/bash
# Rollback to previous snapshot

TAG="${1:-snapshots/latest}"

# Resolve tag/ref to hash
if [[ -f "${CAS_ROOT}/tags/${TAG}" ]]; then
    HASH=$(grep -oP '"object":\s*"\K[a-f0-9]{64}' "${CAS_ROOT}/tags/${TAG}")
elif [[ -f "${CAS_ROOT}/refs/${TAG}" ]]; then
    HASH=$(cat "${CAS_ROOT}/refs/${TAG}")
else
    echo "Error: Tag/ref not found: $TAG"
    exit 1
fi

# Retrieve snapshot
TEMP_SNAPSHOT=$(mktemp)
./retrieve-object.sh "$HASH" "$TEMP_SNAPSHOT"

# Extract to configss
tar xzf "$TEMP_SNAPSHOT" -C /n/noa/configss/

rm "$TEMP_SNAPSHOT"
echo "Rolled back to: $TAG ($HASH)"
```

## Troubleshooting

### Object Not Found

```bash
# Check if hash exists
HASH="abc123..."
find ${CAS_ROOT}/objects -name "${HASH}*"

# Search in registry
grep "$HASH" ${CAS_ROOT}/registry/*.json
```

### Corrupted Object

```bash
# Verify object integrity
CAS_VERIFY=true ./retrieve-object.sh $HASH > /dev/null

# Re-compute hash
./retrieve-object.sh $HASH /tmp/object
b3sum /tmp/object
```

### GC Deleted Important Object

```bash
# Check reflog for recent changes
cat ${CAS_ROOT}/refs/logs/models/current

# Restore from reflog
OLD_HASH=$(head -n2 ${CAS_ROOT}/refs/logs/models/current | tail -n1 | awk '{print $2}')
./update-ref.sh models/current "$OLD_HASH" "Restore from reflog"
```

### Missing blake3

```bash
# Install blake3 (recommended)
cargo install b3sum

# Or use sha256 fallback (automatic)
# Scripts detect missing blake3 and use sha256sum
```

## Performance Tips

1. **Disable verification for bulk retrieval:**
   ```bash
   CAS_VERIFY=false ./retrieve-object.sh $HASH
   ```

2. **Use compression for large objects:**
   - Automatic for files > 1KB
   - zstd level 3 (fast, good compression)

3. **Batch operations:**
   ```bash
   # Store multiple files
   for file in /models/*.gguf; do
       ./store-object.sh "$file" model
   done
   ```

4. **GC during low-traffic periods:**
   - Schedule via cron at 02:00
   - Use `--dry-run` first

## See Also

- [CAS Framework Documentation](../../cas/README.md)
- [CAS configsuration](../../configss/base/cas/configs.json)
- [GC Rules](../../cas/gc/gc_rules.json)
- [NOA Architecture](../../README.md)
