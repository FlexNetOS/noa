#!/bin/bash
# test-gc-function.sh - Debug script for GC function test

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/noa-tools.sh"

CAS_SCRIPTS="${NOA_ROOT}/scripts/cas"
CACHE_SCRIPTS="${NOA_ROOT}/scripts/cache"

echo "NOA_ROOT: $NOA_ROOT"
echo "CAS_SCRIPTS: $CAS_SCRIPTS"
echo "Testing gc-run.sh..."

# Run GC directly
OUTPUT=$(bash "$CAS_SCRIPTS/gc-run.sh" --dry-run 2>&1)
echo "Output:"
echo "$OUTPUT"
echo ""
echo "Checking for 'GC Summary'..."
if echo "$OUTPUT" | grep -q 'GC Summary'; then
    echo "PASS: Found 'GC Summary'"
else
    echo "FAIL: Did not find 'GC Summary'"
fi
