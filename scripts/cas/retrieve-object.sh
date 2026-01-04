#!/bin/bash
# retrieve-object.sh - Retrieve object from Content-Addressed Storage (CAS)
# Usage: retrieve-object.sh <hash> [output-path]
#
# Arguments:
#   hash        - Object hash (blake3 or sha256)
#   output-path - Optional path to write object [default: stdout]
#
# Returns:
#   Object content on stdout (if no output-path)
#   Exit 0 on success, 1 on error
#
# Environment:
#   NOA_ROOT      - Root directory (default: /n/noa)
#   CAS_ROOT      - CAS directory (default: ${NOA_ROOT}/cas)
#   CAS_VERIFY    - Verify hash on read (default: true)

set -euo pipefail

# configsuration
NOA_ROOT="${NOA_ROOT:-/n/noa}"
CAS_ROOT="${CAS_ROOT:-${NOA_ROOT}/cas}"
OBJECTS_DIR="${CAS_ROOT}/objects"
CAS_VERIFY="${CAS_VERIFY:-true}"

# Input validation
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <hash> [output-path]" >&2
    exit 1
fi

HASH="$1"
OUTPUT_PATH="${2:-}"

# Validate hash format (64 hex chars for blake3/sha256)
if ! [[ "$HASH" =~ ^[a-f0-9]{64}$ ]]; then
    echo "Error: Invalid hash format: $HASH" >&2
    echo "Expected 64 hex characters (blake3 or sha256)" >&2
    exit 1
fi

# Locate object: objects/<h0h1>/<h2h3>/<full_hash>[.zst]
PREFIX1="${HASH:0:2}"
PREFIX2="${HASH:2:2}"
OBJECT_DIR="${OBJECTS_DIR}/${PREFIX1}/${PREFIX2}"
OBJECT_PATH="${OBJECT_DIR}/${HASH}"

# Check both compressed and uncompressed
if [[ -f "${OBJECT_PATH}.zst" ]]; then
    STORED_PATH="${OBJECT_PATH}.zst"
    COMPRESSED=true
elif [[ -f "$OBJECT_PATH" ]]; then
    STORED_PATH="$OBJECT_PATH"
    COMPRESSED=false
else
    echo "Error: Object not found: $HASH" >&2
    echo "Checked: $OBJECT_PATH[.zst]" >&2
    exit 1
fi

# Decompress to temp if needed for verification
if [[ "$COMPRESSED" == "true" ]]; then
    if ! command -v zstd >/dev/null 2>&1; then
        echo "Error: zstd required to decompress object" >&2
        exit 1
    fi

    if [[ "$CAS_VERIFY" == "true" ]]; then
        # Decompress to temp for verification
        TEMP_FILE=$(mktemp)
        trap "rm -f $TEMP_FILE" EXIT
        zstd -d -q -o "$TEMP_FILE" "$STORED_PATH"
        VERIFY_FILE="$TEMP_FILE"
    else
        VERIFY_FILE=""
    fi
else
    VERIFY_FILE="$STORED_PATH"
fi

# Verify integrity if enabled
if [[ "$CAS_VERIFY" == "true" ]] && [[ -n "$VERIFY_FILE" ]]; then
    if command -v b3sum >/dev/null 2>&1; then
        COMPUTED_HASH=$(b3sum "$VERIFY_FILE" | awk '{print $1}')
    elif command -v blake3 >/dev/null 2>&1; then
        COMPUTED_HASH=$(blake3 "$VERIFY_FILE" | awk '{print $1}')
    else
        # sha256sum may prefix hash with \ on Windows for paths with special chars
        COMPUTED_HASH=$(sha256sum "$VERIFY_FILE" | awk '{print $1}' | sed 's/^\\//g')
    fi

    if [[ "$COMPUTED_HASH" != "$HASH" ]]; then
        echo "Error: Hash mismatch! Object corrupted." >&2
        echo "Expected: $HASH" >&2
        echo "Computed: $COMPUTED_HASH" >&2
        exit 1
    fi
fi

# Output object
if [[ -n "$OUTPUT_PATH" ]]; then
    # Write to file
    if [[ "$COMPRESSED" == "true" ]]; then
        zstd -d -q -o "$OUTPUT_PATH" "$STORED_PATH"
    else
        cp "$STORED_PATH" "$OUTPUT_PATH"
    fi
    echo "Retrieved object to: $OUTPUT_PATH" >&2
else
    # Write to stdout
    if [[ "$COMPRESSED" == "true" ]]; then
        zstd -d -q -c "$STORED_PATH"
    else
        cat "$STORED_PATH"
    fi
fi
