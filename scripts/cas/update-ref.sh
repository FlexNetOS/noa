#!/bin/bash
# update-ref.sh - Update CAS ref atomically with reflog
# Usage: update-ref.sh <ref-name> <hash> [message]
#
# Arguments:
#   ref-name - Reference name (e.g., "heads/main", "tags/v1.0.0")
#   hash     - Target object hash
#   message  - Optional reflog message
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
REFS_DIR="${CAS_ROOT}/refs"
OBJECTS_DIR="${CAS_ROOT}/objects"
REFLOG_MAX_ENTRIES=100

# Input validation
if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <ref-name> <hash> [message]" >&2
    exit 1
fi

REF_NAME="$1"
NEW_HASH="$2"
MESSAGE="${3:-Update ref $REF_NAME}"

# Validate hash format
if ! [[ "$NEW_HASH" =~ ^[a-f0-9]{64}$ ]]; then
    echo "Error: Invalid hash format: $NEW_HASH" >&2
    exit 1
fi

# Validate ref name (alphanumeric, /, -, _)
if ! [[ "$REF_NAME" =~ ^[a-zA-Z0-9/_-]+$ ]]; then
    echo "Error: Invalid ref name: $REF_NAME" >&2
    echo "Allowed: alphanumeric, /, -, _" >&2
    exit 1
fi

# Check if object exists
PREFIX1="${NEW_HASH:0:2}"
PREFIX2="${NEW_HASH:2:2}"
OBJECT_PATH="${OBJECTS_DIR}/${PREFIX1}/${PREFIX2}/${NEW_HASH}"

if [[ ! -f "$OBJECT_PATH" ]] && [[ ! -f "${OBJECT_PATH}.zst" ]]; then
    echo "Error: Object not found: $NEW_HASH" >&2
    exit 1
fi

# Create ref path
REF_PATH="${REFS_DIR}/${REF_NAME}"
REF_DIR=$(dirname "$REF_PATH")
mkdir -p "$REF_DIR"

# Read old hash for reflog
if [[ -f "$REF_PATH" ]]; then
    OLD_HASH=$(cat "$REF_PATH")
else
    OLD_HASH="0000000000000000000000000000000000000000000000000000000000000000"
fi

# Atomic update using temp file + rename
TEMP_REF=$(mktemp -p "$REF_DIR")
trap "rm -f $TEMP_REF" EXIT

echo "$NEW_HASH" > "$TEMP_REF"
mv "$TEMP_REF" "$REF_PATH"

# Update reflog
REFLOG_DIR="${CAS_ROOT}/refs/logs"
REFLOG_PATH="${REFLOG_DIR}/${REF_NAME}"
REFLOG_DIR_PATH=$(dirname "$REFLOG_PATH")
mkdir -p "$REFLOG_DIR_PATH"

TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
REFLOG_ENTRY="${OLD_HASH} ${NEW_HASH} ${TIMESTAMP} ${MESSAGE}"

echo "$REFLOG_ENTRY" >> "$REFLOG_PATH"

# Trim reflog to max entries
if [[ -f "$REFLOG_PATH" ]]; then
    ENTRY_COUNT=$(wc -l < "$REFLOG_PATH")
    if [[ $ENTRY_COUNT -gt $REFLOG_MAX_ENTRIES ]]; then
        TEMP_LOG=$(mktemp)
        tail -n $REFLOG_MAX_ENTRIES "$REFLOG_PATH" > "$TEMP_LOG"
        mv "$TEMP_LOG" "$REFLOG_PATH"
    fi
fi

echo "Updated ref: $REF_NAME -> $NEW_HASH" >&2
if [[ "$OLD_HASH" != "0000000000000000000000000000000000000000000000000000000000000000" ]]; then
    echo "Previous: $OLD_HASH" >&2
fi
