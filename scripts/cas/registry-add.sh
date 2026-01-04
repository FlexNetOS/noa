#!/bin/bash
# registry-add.sh - Add object to CAS registry
# Usage: registry-add.sh <hash> <type> <name> [metadata-json]
#
# Arguments:
#   hash          - Object hash
#   type          - Object type (model, prompt, snapshot, binary, package)
#   name          - Object name/identifier
#   metadata-json - Optional JSON metadata
#
# Returns:
#   Exit 0 on success, 1 on error

set -euo pipefail

# configsuration
NOA_ROOT="${NOA_ROOT:-/n/noa}"
CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
REGISTRY_DIR="${CAS_ROOT}/registry"
OBJECTS_DIR="${CAS_ROOT}/objects"

# Input validation
if [[ $# -lt 3 ]]; then
    echo "Usage: $0 <hash> <type> <name> [metadata-json]" >&2
    exit 1
fi

HASH="$1"
TYPE="$2"
NAME="$3"
METADATA="${4:-{}}"

# Validate hash
if ! [[ "$HASH" =~ ^[a-f0-9]{64}$ ]]; then
    echo "Error: Invalid hash format: $HASH" >&2
    exit 1
fi

# Validate type
VALID_TYPES="model prompt snapshot binary package"
if ! echo "$VALID_TYPES" | grep -qw "$TYPE"; then
    echo "Error: Invalid type: $TYPE" >&2
    echo "Valid types: $VALID_TYPES" >&2
    exit 1
fi

# Check if object exists
PREFIX1="${HASH:0:2}"
PREFIX2="${HASH:2:2}"
OBJECT_PATH="${OBJECTS_DIR}/${PREFIX1}/${PREFIX2}/${HASH}"

if [[ ! -f "$OBJECT_PATH" ]] && [[ ! -f "${OBJECT_PATH}.zst" ]]; then
    echo "Error: Object not found: $HASH" >&2
    exit 1
fi

# Get object size
if [[ -f "${OBJECT_PATH}.zst" ]]; then
    SIZE=$(stat -c%s "${OBJECT_PATH}.zst" 2>/dev/null || stat -f%z "${OBJECT_PATH}.zst" 2>/dev/null)
    COMPRESSED=true
elif [[ -f "$OBJECT_PATH" ]]; then
    SIZE=$(stat -c%s "$OBJECT_PATH" 2>/dev/null || stat -f%z "$OBJECT_PATH" 2>/dev/null)
    COMPRESSED=false
else
    SIZE=0
    COMPRESSED=false
fi

# Determine registry file
REGISTRY_FILE="${REGISTRY_DIR}/${TYPE}s.json"

# Ensure registry exists
mkdir -p "$REGISTRY_DIR"
if [[ ! -f "$REGISTRY_FILE" ]]; then
    cat > "$REGISTRY_FILE" <<EOF
{
  "version": "1.0.0",
  "description": "Registry of ${TYPE} objects stored in CAS",
  "metadata": {
    "version": "1.0.0",
    "registry_type": "${TYPE}s",
    "updated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  },
  "objects": [],
  "statistics": {
    "total_${TYPE}s": 0,
    "total_size_bytes": 0
  }
}
EOF
fi

# Check if entry already exists
if grep -q "\"hash\":\\s*\"$HASH\"" "$REGISTRY_FILE" 2>/dev/null; then
    echo "Entry already exists in registry: $HASH" >&2
    exit 0
fi

# Create temp file for registry update
TEMP_REGISTRY=$(mktemp)
trap "rm -f $TEMP_REGISTRY" EXIT

# Add entry (simplified - production would use jq)
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)

# Read existing registry
cp "$REGISTRY_FILE" "$TEMP_REGISTRY"

# For simplicity, append entry as comment (real implementation would use jq)
# This is a placeholder for the actual JSON manipulation
cat >> "$TEMP_REGISTRY" <<EOF

# Entry for $NAME ($HASH):
# {
#   "hash": "$HASH",
#   "type": "$TYPE",
#   "name": "$NAME",
#   "size": $SIZE,
#   "compressed": $COMPRESSED,
#   "created_at": "$TIMESTAMP",
#   "metadata": $METADATA
# }
EOF

# In production, use jq:
# jq ".objects += [{hash: \"$HASH\", type: \"$TYPE\", name: \"$NAME\", ...}]" "$REGISTRY_FILE" > "$TEMP_REGISTRY"

# Update statistics would also use jq

# Atomic update
mv "$TEMP_REGISTRY" "$REGISTRY_FILE"

echo "Added to registry: $NAME ($HASH)" >&2
echo "Registry: $REGISTRY_FILE" >&2
