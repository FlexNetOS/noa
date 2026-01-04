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

# Test 1: MCP SDK configsuration
echo "=== Test Group: MCP SDK ==="

run_test "MCP configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' 'true' 2>&1"

run_test "MCP has SDK configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"sdk\" in data' 2>&1"

run_test "MCP has server configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"server_configsuration\" in data' 2>&1"

run_test "MCP has client configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"client_configsuration\" in data' 2>&1"

run_test "MCP has protocol support" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"protocol_support\" in data' 2>&1"

run_test "MCP has integration points" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"integration_points\" in data' 2>&1"

run_test "MCP has security configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"security\" in data' 2>&1"

run_test "MCP has tool registration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/mcp-sdk/configs.json' '\"tool_registration\" in data' 2>&1"

echo ""

# Test 2: Qdrant configsuration
echo "=== Test Group: Qdrant ==="

run_test "Qdrant configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' 'true' 2>&1"

run_test "Qdrant has database configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' '\"database\" in data' 2>&1"

run_test "Qdrant has connection configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' '\"connection\" in data' 2>&1"

run_test "Qdrant has collections" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' '\"collections\" in data' 2>&1"

run_test "3 Qdrant collections defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' 'Object.keys(data.collections).length === 3' 2>&1"

run_test "All collections have vector_size" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' 'Object.values(data.collections).every(col => \"vector_size\" in col)' 2>&1"

run_test "All collections have distance metric" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' 'Object.values(data.collections).every(col => \"distance\" in col)' 2>&1"

run_test "Qdrant has monitoring settings" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' '\"monitoring\" in data' 2>&1"

run_test "Qdrant has backup configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/qdrant/configs.json' '\"backup\" in data' 2>&1"

echo ""

# Test 3: SQLx configsuration
echo "=== Test Group: SQLx ==="

run_test "SQLx configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' 'true' 2>&1"

run_test "SQLx database configsuration exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"database\" in data' 2>&1"

run_test "SQLx connections configsured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"connections\" in data' 2>&1"

run_test "5 database tables defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' 'Object.keys(data.tables).length === 5' 2>&1"

run_test "audit_logs table exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"audit_logs\" in data.tables' 2>&1"

run_test "budget_tracking table exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"budget_tracking\" in data.tables' 2>&1"

run_test "All tables have description" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' 'Object.values(data.tables).every(tbl => \"description\" in tbl)' 2>&1"

run_test "SQLx has schema configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"schema\" in data' 2>&1"

run_test "SQLx has queries configsured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"queries\" in data' 2>&1"

run_test "SQLx has operations configsured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/sqlx/configs.json' '\"operations\" in data' 2>&1"

echo ""

# Test 4: libp2p configsuration
echo "=== Test Group: libp2p ==="

run_test "libp2p configs is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' 'true' 2>&1"

run_test "libp2p network configsuration exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"network_configsuration\" in data' 2>&1"

run_test "libp2p has library configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"library\" in data' 2>&1"

run_test "4 protocols configsured" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' 'Object.keys(data.protocols).length === 4' 2>&1"

run_test "Kademlia protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"kademlia\" in data.protocols' 2>&1"

run_test "mDNS protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"mdns\" in data.protocols' 2>&1"

run_test "GossipSub protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"gossipsub\" in data.protocols' 2>&1"

run_test "Request-Response protocol exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"request_response\" in data.protocols' 2>&1"

run_test "libp2p has use_cases configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"use_cases\" in data' 2>&1"

run_test "libp2p has security configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"security\" in data' 2>&1"

run_test "libp2p has monitoring configsuration" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/tools/third-party/libp2p/configs.json' '\"monitoring\" in data' 2>&1"

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
