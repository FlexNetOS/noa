#!/bin/bash
# store-object.sh - Store object in Content-Addressed Storage (CAS)
# Usage: store-object.sh <file> [type] [metadata-json]
#
# Arguments:
#   file          - Path to file to store in CAS
#   type          - Object type (model, prompt, snapshot, binary, package) [optional]
#   metadata-json - JSON metadata to attach [optional]
#
# Returns:
#   Object hash (blake3) on stdout
#
# Environment:
#   NOA_ROOT      - Root directory (default: /n/noa)
#   CAS_ROOT      - CAS directory (default: ${NOA_ROOT}/cas)

set -euo pipefail

# Configuration
NOA_ROOT="${NOA_ROOT:-/n/noa}"
CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
OBJECTS_DIR="${CAS_ROOT}/objects"
REGISTRY_DIR="${CAS_ROOT}/registry"
COMPRESSION_THRESHOLD=1024  # Compress files > 1KB

# Input validation
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <file> [type] [metadata-json]" >&2
    exit 1
fi

INPUT_FILE="$1"
OBJECT_TYPE="${2:-generic}"
METADATA="${3:-{}}"

if [[ ! -f "$INPUT_FILE" ]]; then
    echo "Error: File not found: $INPUT_FILE" >&2
    exit 1
fi

# Ensure directories exist
mkdir -p "$OBJECTS_DIR" "$REGISTRY_DIR"

# Compute blake3 hash
if command -v b3sum >/dev/null 2>&1; then
    HASH=$(b3sum "$INPUT_FILE" | awk '{print $1}')
elif command -v blake3 >/dev/null 2>&1; then
    HASH=$(blake3 "$INPUT_FILE" | awk '{print $1}')
else
    # Fallback to sha256 if blake3 not available
    # sha256sum may prefix hash with \ on Windows for paths with special chars
    echo "Warning: blake3 not found, using sha256 fallback" >&2
    HASH=$(sha256sum "$INPUT_FILE" | awk '{print $1}' | sed 's/^\\//g')
fi

# Create object path: objects/<h0h1>/<h2h3>/<full_hash>
PREFIX1="${HASH:0:2}"
PREFIX2="${HASH:2:2}"
OBJECT_DIR="${OBJECTS_DIR}/${PREFIX1}/${PREFIX2}"
OBJECT_PATH="${OBJECT_DIR}/${HASH}"

# Check if object already exists (deduplication)
if [[ -f "$OBJECT_PATH" ]] || [[ -f "${OBJECT_PATH}.zst" ]]; then
    echo "Object already exists: $HASH" >&2
    echo "$HASH"
    exit 0
fi

# Create object directory
mkdir -p "$OBJECT_DIR"

# Get file size
FILE_SIZE=$(stat -c%s "$INPUT_FILE" 2>/dev/null || stat -f%z "$INPUT_FILE" 2>/dev/null || echo 0)

# Compress if over threshold and zstd available
if [[ $FILE_SIZE -gt $COMPRESSION_THRESHOLD ]] && command -v zstd >/dev/null 2>&1; then
    zstd -3 -q -o "${OBJECT_PATH}.zst" "$INPUT_FILE"
    STORED_PATH="${OBJECT_PATH}.zst"
    COMPRESSED=true
else
    cp "$INPUT_FILE" "$OBJECT_PATH"
    STORED_PATH="$OBJECT_PATH"
    COMPRESSED=false
fi

# Update registry based on type
REGISTRY_FILE="${REGISTRY_DIR}/${OBJECT_TYPE}s.json"
if [[ ! -f "$REGISTRY_FILE" ]]; then
    # Create new registry if it doesn't exist
    cat > "$REGISTRY_FILE" <<EOF
{
  "version": "1.0.0",
  "description": "Registry of ${OBJECT_TYPE} objects stored in CAS",
  "metadata": {
    "version": "1.0.0",
    "registry_type": "${OBJECT_TYPE}s",
    "updated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  },
  "objects": [],
  "statistics": {
    "total_${OBJECT_TYPE}s": 0,
    "total_size_bytes": 0
  }
}
EOF
fi

# Add entry to registry (simplified - would use jq in production)
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
ENTRY=$(cat <<EOF
{
  "hash": "$HASH",
  "type": "$OBJECT_TYPE",
  "size": $FILE_SIZE,
  "compressed": $COMPRESSED,
  "created_at": "$TIMESTAMP",
  "metadata": $METADATA
}
EOF
)

echo "Stored object: $HASH ($(numfmt --to=iec $FILE_SIZE 2>/dev/null || echo ${FILE_SIZE}B))" >&2
echo "$HASH"
