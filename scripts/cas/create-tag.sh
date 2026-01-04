#!/bin/bash
# create-tag.sh - Create or update a CAS tag
# Usage: create-tag.sh <tag-name> <hash> [message]
#
# Arguments:
#   tag-name - Tag name (e.g., "v1.0.0", "latest")
#   hash     - Target object hash
#   message  - Optional tag message/annotation
#
# Returns:
#   Exit 0 on success, 1 on error
#
# Environment:
#   NOA_ROOT      - Root directory (default: /n/noa)
#   CAS_ROOT      - CAS directory (default: ${NOA_ROOT}/cas)

set -euo pipefail

# configsuration
NOA_ROOT="${NOA_ROOT:-/n/noa}"
CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
TAGS_DIR="${CAS_ROOT}/tags"
OBJECTS_DIR="${CAS_ROOT}/objects"

# Input validation
if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <tag-name> <hash> [message]" >&2
    exit 1
fi

TAG_NAME="$1"
TARGET_HASH="$2"
MESSAGE="${3:-}"

# Validate hash format
if ! [[ "$TARGET_HASH" =~ ^[a-f0-9]{64}$ ]]; then
    echo "Error: Invalid hash format: $TARGET_HASH" >&2
    exit 1
fi

# Validate tag name
if ! [[ "$TAG_NAME" =~ ^[a-zA-Z0-9._-]+$ ]]; then
    echo "Error: Invalid tag name: $TAG_NAME" >&2
    echo "Allowed: alphanumeric, ., -, _" >&2
    exit 1
fi

# Check if object exists
PREFIX1="${TARGET_HASH:0:2}"
PREFIX2="${TARGET_HASH:2:2}"
OBJECT_PATH="${OBJECTS_DIR}/${PREFIX1}/${PREFIX2}/${TARGET_HASH}"

if [[ ! -f "$OBJECT_PATH" ]] && [[ ! -f "${OBJECT_PATH}.zst" ]]; then
    echo "Error: Object not found: $TARGET_HASH" >&2
    exit 1
fi

# Create tags directory
mkdir -p "$TAGS_DIR"

# Create tag file
TAG_PATH="${TAGS_DIR}/${TAG_NAME}"
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)

cat > "$TAG_PATH" <<EOF
{
  "object": "$TARGET_HASH",
  "tag": "$TAG_NAME",
  "created_at": "$TIMESTAMP",
  "message": "$MESSAGE"
}
EOF

echo "Created tag: $TAG_NAME -> $TARGET_HASH" >&2
