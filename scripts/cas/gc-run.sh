#!/bin/bash
# gc-run.sh - Run CAS garbage collection
# Usage: gc-run.sh [--dry-run] [--force]
#
# Options:
#   --dry-run - Show what would be deleted without deleting
#   --force   - Skip safety checks and delete immediately
#
# Environment:
#   NOA_ROOT      - Root directory (auto-detected via noa-tools.sh)
#   CAS_ROOT      - CAS directory (default: ${NOA_ROOT}/cas)

set -euo pipefail

# Source noa-tools.sh for cross-platform path resolution
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "$SCRIPT_DIR/../lib/noa-tools.sh" ]]; then
    source "$SCRIPT_DIR/../lib/noa-tools.sh"
else
    # Fallback if noa-tools.sh not available
    NOA_ROOT="${NOA_ROOT:-/n/noa}"
fi

# configsuration
CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
OBJECTS_DIR="${CAS_ROOT}/objects"
REFS_DIR="${CAS_ROOT}/refs"
TAGS_DIR="${CAS_ROOT}/tags"
GC_configs="${CAS_ROOT}/gc/gc_rules.json"

DRY_RUN=false
FORCE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        *)
            echo "Unknown option: $1" >&2
            echo "Usage: $0 [--dry-run] [--force]" >&2
            exit 1
            ;;
    esac
done

# Load GC configsuration (simplified - would use jq in production)
MIN_AGE_DAYS=7
if [[ -f "$GC_configs" ]]; then
    echo "Using GC configs: $GC_configs" >&2
fi

# Find all objects
echo "Scanning objects..." >&2
TOTAL_OBJECTS=0
REFERENCED_OBJECTS=0
UNREFERENCED_OBJECTS=0

# Build set of referenced objects from refs and tags
REFERENCED_FILE=$(mktemp)
trap "rm -f $REFERENCED_FILE" EXIT

# Collect hashes from refs
if [[ -d "$REFS_DIR" ]]; then
    find "$REFS_DIR" -type f ! -path "*/logs/*" -exec cat {} \; >> "$REFERENCED_FILE" 2>/dev/null || true
fi

# Collect hashes from tags
if [[ -d "$TAGS_DIR" ]]; then
    find "$TAGS_DIR" -type f -exec grep -oP '"object":\s*"\K[a-f0-9]{64}' {} \; >> "$REFERENCED_FILE" 2>/dev/null || true
fi

# Sort and unique
sort -u "$REFERENCED_FILE" -o "$REFERENCED_FILE"
REFERENCED_COUNT=$(wc -l < "$REFERENCED_FILE")

echo "Found $REFERENCED_COUNT referenced objects" >&2

# Scan all objects
DELETED_COUNT=0
DELETED_SIZE=0
CURRENT_TIME=$(date +%s)
MIN_AGE_SECONDS=$((MIN_AGE_DAYS * 86400))

if [[ -d "$OBJECTS_DIR" ]]; then
    while IFS= read -r -d '' OBJECT_FILE; do
        ((TOTAL_OBJECTS++)) || true

        # Extract hash from path
        HASH=$(basename "$OBJECT_FILE" .zst)

        # Check if referenced
        if grep -q "^${HASH}$" "$REFERENCED_FILE"; then
            ((REFERENCED_OBJECTS++)) || true
            continue
        fi

        # Check age
        if [[ -f "$OBJECT_FILE" ]]; then
            FILE_TIME=$(stat -c%Y "$OBJECT_FILE" 2>/dev/null || stat -f%m "$OBJECT_FILE" 2>/dev/null || echo 0)
            AGE_SECONDS=$((CURRENT_TIME - FILE_TIME))

            if [[ $AGE_SECONDS -lt $MIN_AGE_SECONDS ]] && [[ "$FORCE" != "true" ]]; then
                continue
            fi
        fi

        # Unreferenced and old enough - candidate for deletion
        ((UNREFERENCED_OBJECTS++)) || true

        FILE_SIZE=$(stat -c%s "$OBJECT_FILE" 2>/dev/null || stat -f%z "$OBJECT_FILE" 2>/dev/null || echo 0)
        DELETED_SIZE=$((DELETED_SIZE + FILE_SIZE))

        if [[ "$DRY_RUN" == "true" ]]; then
            echo "Would delete: $HASH ($(numfmt --to=iec $FILE_SIZE 2>/dev/null || echo ${FILE_SIZE}B))"
        else
            rm -f "$OBJECT_FILE"
            ((DELETED_COUNT++)) || true
        fi
    done < <(find "$OBJECTS_DIR" -type f \( -name "*[a-f0-9]" -o -name "*.zst" \) -print0)
fi

# Summary
echo "" >&2
echo "=== GC Summary ===" >&2
echo "Total objects:        $TOTAL_OBJECTS" >&2
echo "Referenced:           $REFERENCED_OBJECTS" >&2
echo "Unreferenced:         $UNREFERENCED_OBJECTS" >&2

if [[ "$DRY_RUN" == "true" ]]; then
    echo "Would delete:         $UNREFERENCED_OBJECTS objects ($(numfmt --to=iec $DELETED_SIZE 2>/dev/null || echo ${DELETED_SIZE}B))" >&2
    echo "" >&2
    echo "Run without --dry-run to actually delete" >&2
else
    echo "Deleted:              $DELETED_COUNT objects ($(numfmt --to=iec $DELETED_SIZE 2>/dev/null || echo ${DELETED_SIZE}B))" >&2
fi
