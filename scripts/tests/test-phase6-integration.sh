#!/bin/bash
# test-phase6-integration.sh - Integration tests for Phase 6 (Third-Party Integrations)
# Usage: test-phase6-integration.sh [--verbose]
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

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper
run_test() {
    local name="$1"
    local command="$2"

    echo -n "Test: $name ... "

    if [[ "$VERBOSE" == "true" ]]; then
        echo ""
        echo "  Command: $command"
    fi

    if eval "$command" > /tmp/test-output.txt 2>&1; then
        echo "✓ PASS"
        ((TESTS_PASSED++)) || true
        if [[ "$VERBOSE" == "true" ]]; then
            cat /tmp/test-output.txt | sed 's/^/    /'
        fi
    else
        echo "✗ FAIL"
        ((TESTS_FAILED++)) || true
        echo "  Error output:"
        cat /tmp/test-output.txt | sed 's/^/    /'
    fi

    rm -f /tmp/test-output.txt
}

echo "=== Phase 6 Integration Tests ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Test 1: MCP SDK Configuration
echo "=== Test Group: MCP SDK ==="

run_test "MCP config is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' 'true' 2>&1"

run_test "MCP has SDK configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"sdk\" in data' 2>&1"

run_test "MCP has server configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"server_configuration\" in data' 2>&1"

run_test "MCP has client configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"client_configuration\" in data' 2>&1"

run_test "MCP has protocol support" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"protocol_support\" in data' 2>&1"

run_test "MCP has integration points" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"integration_points\" in data' 2>&1"

run_test "MCP has security configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"security\" in data' 2>&1"

run_test "MCP has tool registration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/config.json' '\"tool_registration\" in data' 2>&1"

echo ""

# Test 2: Qdrant Configuration
echo "=== Test Group: Qdrant ==="

run_test "Qdrant config is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' 'true' 2>&1"

run_test "Qdrant has database configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' '\"database\" in data' 2>&1"

run_test "Qdrant has connection configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' '\"connection\" in data' 2>&1"

run_test "Qdrant has collections" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' '\"collections\" in data' 2>&1"

run_test "3 Qdrant collections defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' 'Object.keys(data.collections).length === 3' 2>&1"

run_test "All collections have vector_size" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' 'Object.values(data.collections).every(col => \"vector_size\" in col)' 2>&1"

run_test "All collections have distance metric" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' 'Object.values(data.collections).every(col => \"distance\" in col)' 2>&1"

run_test "Qdrant has monitoring settings" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' '\"monitoring\" in data' 2>&1"

run_test "Qdrant has backup configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/config.json' '\"backup\" in data' 2>&1"

echo ""

# Test 3: SQLx Configuration
echo "=== Test Group: SQLx ==="

run_test "SQLx config is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' 'true' 2>&1"

run_test "SQLx database configuration exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"database\" in data' 2>&1"

run_test "SQLx connections configured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"connections\" in data' 2>&1"

run_test "5 database tables defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' 'Object.keys(data.tables).length === 5' 2>&1"

run_test "audit_logs table exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"audit_logs\" in data.tables' 2>&1"

run_test "budget_tracking table exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"budget_tracking\" in data.tables' 2>&1"

run_test "All tables have description" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' 'Object.values(data.tables).every(tbl => \"description\" in tbl)' 2>&1"

run_test "SQLx has schema configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"schema\" in data' 2>&1"

run_test "SQLx has queries configured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"queries\" in data' 2>&1"

run_test "SQLx has operations configured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/config.json' '\"operations\" in data' 2>&1"

echo ""

# Test 4: libp2p Configuration
echo "=== Test Group: libp2p ==="

run_test "libp2p config is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' 'true' 2>&1"

run_test "libp2p network configuration exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"network_configuration\" in data' 2>&1"

run_test "libp2p has library configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"library\" in data' 2>&1"

run_test "4 protocols configured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' 'Object.keys(data.protocols).length === 4' 2>&1"

run_test "Kademlia protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"kademlia\" in data.protocols' 2>&1"

run_test "mDNS protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"mdns\" in data.protocols' 2>&1"

run_test "GossipSub protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"gossipsub\" in data.protocols' 2>&1"

run_test "Request-Response protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"request_response\" in data.protocols' 2>&1"

run_test "libp2p has use_cases configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"use_cases\" in data' 2>&1"

run_test "libp2p has security configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"security\" in data' 2>&1"

run_test "libp2p has monitoring configuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/config.json' '\"monitoring\" in data' 2>&1"

echo ""

# Test 5: Documentation
echo "=== Test Group: Documentation ==="

run_test "Integration guide exists" \
  "[[ -f '$NOA_ROOT/tools/third-party/INTEGRATION_GUIDE.md' ]]"

run_test "Integration guide is not empty" \
  "[[ -s '$NOA_ROOT/tools/third-party/INTEGRATION_GUIDE.md' ]]"

echo ""

# Summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "✓ All Phase 6 integration tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
