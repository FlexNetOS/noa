#!/bin/bash
# validate-configss.sh - Validate all JSON configsurations
# Usage: validate-configss.sh [--verbose]
# Respects NOA Constitution §3.1 - uses portable tools from noa_root

set -euo pipefail

# Source portable tool resolver
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "$SCRIPT_DIR/../lib/noa-tools.sh" ]]; then
    source "$SCRIPT_DIR/../lib/noa-tools.sh"
else
    echo "ERROR: noa-tools.sh not found. Run from NOA repository." >&2
    exit 1
fi

VERBOSE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Counters
TOTAL=0
PASSED=0
FAILED=0

# Validate JSON syntax using portable tools
validate_json() {
    local file="$1"
    ((TOTAL++)) || true

    if [[ "$VERBOSE" == "true" ]]; then
        echo -n "Validating: $file ... "
    fi

    # Check if file exists
    if [[ ! -f "$file" ]]; then
        ((FAILED++)) || true
        echo "✗ FAIL: $file (file not found)"
        return 1
    fi

    # Use noa_jq from portable tools (has fallback to node/python)
    if noa_jq 'empty' "$file" >/dev/null 2>&1; then
        ((PASSED++)) || true
        [[ "$VERBOSE" == "true" ]] && echo "✓ PASS"
        return 0
    else
        ((FAILED++)) || true
        echo "✗ FAIL: $file"
        noa_jq 'empty' "$file" 2>&1 | head -3 || true
        return 1
    fi
}

echo "=== NOA configsuration Validation ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "NOA_ROOT: $NOA_ROOT"
echo "Platform: $NOA_PLATFORM"
echo ""

# Validate Phase 3 configss
echo "[Phase 3] CAS & Data Plane"
validate_json "$NOA_ROOT/configss/base/cas/configs.json"
validate_json "$NOA_ROOT/cas/gc/gc_rules.json"
validate_json "$NOA_ROOT/cas/registry/models.json"
validate_json "$NOA_ROOT/cas/registry/prompts.json"
validate_json "$NOA_ROOT/cas/registry/snapshots.json"
validate_json "$NOA_ROOT/cas/registry/binaries.json"
validate_json "$NOA_ROOT/cas/registry/packages.json"
validate_json "$NOA_ROOT/configss/base/cache/cache-policies.json"
echo ""

# Validate Phase 4 configss
echo "[Phase 4] System Core & Policy"
validate_json "$NOA_ROOT/sys/core/identity/identity.json"
validate_json "$NOA_ROOT/sys/core/enforcement/policy.json"
validate_json "$NOA_ROOT/sys/core/audit/audit-configs.json"
validate_json "$NOA_ROOT/sys/core/registry/registry.json"
validate_json "$NOA_ROOT/sys/core/scheduler/configs.json"
echo ""

# Validate Phase 5 configss
echo "[Phase 5] Resource Registry"
validate_json "$NOA_ROOT/data/resources/registry.json"
echo ""

# Validate Phase 6 configss
echo "[Phase 6] Third-Party Integrations"
validate_json "$NOA_ROOT/tools/third-party/mcp-sdk/configs.json"
validate_json "$NOA_ROOT/tools/third-party/qdrant/configs.json"
validate_json "$NOA_ROOT/tools/third-party/sqlx/configs.json"
validate_json "$NOA_ROOT/tools/third-party/libp2p/configs.json"
echo ""

# Summary
echo "=== Validation Summary ==="
echo "Total configss:  $TOTAL"
echo "Passed:         $PASSED"
echo "Failed:         $FAILED"
echo ""

if [[ $FAILED -eq 0 ]]; then
    echo "✓ All configsurations valid!"
    exit 0
else
    echo "✗ Some configsurations failed validation"
    exit 1
fi
