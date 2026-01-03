#!/bin/bash
# test-phase5-integration.sh - Integration tests for Phase 5 (Resource Registry)
# Usage: test-phase5-integration.sh [--verbose]
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

echo "=== Phase 5 Integration Tests ==="
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Test 1: Resource Registry
echo "=== Test Group: Resource Registry ==="

run_test "Registry config is valid JSON" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'true' 2>&1"

run_test "Registry has required sections" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' '[\"version\", \"metadata\", \"agent_templates\", \"tool_definitions\", \"prompt_templates\", \"deployment_workflows\"].every(k => k in data)' 2>&1"

run_test "Registry version is valid" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'typeof data.version === \"string\" && data.version.split(\".\").length === 3' 2>&1"

echo ""

# Test 2: Agent Templates
echo "=== Test Group: Agent Templates ==="

run_test "3 agent templates defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.keys(data.agent_templates).length === 3' 2>&1"

run_test "claude_code_agent template exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' '\"claude_code_agent\" in data.agent_templates' 2>&1"

run_test "All templates have required fields" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.agent_templates).every(tmpl => [\"id\", \"name\", \"version\", \"provider\", \"capabilities\", \"configuration\"].every(k => k in tmpl))' 2>&1"

run_test "All template IDs start with tmpl:" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.agent_templates).every(tmpl => tmpl.id.startsWith(\"tmpl:\"))' 2>&1"

run_test "All templates have capabilities" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.agent_templates).every(tmpl => tmpl.capabilities.length > 0)' 2>&1"

echo ""

# Test 3: Tool Definitions
echo "=== Test Group: Tool Definitions ==="

run_test "5 tool groups defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.keys(data.tool_definitions).length === 5' 2>&1"

run_test "All tool groups have required fields" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.tool_definitions).every(grp => [\"id\", \"name\", \"tools\"].every(k => k in grp))' 2>&1"

run_test "All tool IDs start with tool_group:" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.tool_definitions).every(grp => grp.id.startsWith(\"tool_group:\"))' 2>&1"

run_test "All tools have required fields" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.tool_definitions).flatMap(grp => grp.tools).every(tool => [\"id\", \"name\", \"description\", \"type\", \"capabilities_required\"].every(k => k in tool))' 2>&1"

run_test "All individual tool IDs start with tool:" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.tool_definitions).flatMap(grp => grp.tools).every(tool => tool.id.startsWith(\"tool:\"))' 2>&1"

run_test "14 total tools defined across all groups" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.tool_definitions).flatMap(grp => grp.tools).length === 14' 2>&1"

echo ""

# Test 4: Prompt Templates
echo "=== Test Group: Prompt Templates ==="

run_test "3 prompt templates defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.keys(data.prompt_templates).length === 3' 2>&1"

run_test "All prompt templates have required fields" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.prompt_templates).every(tmpl => [\"id\", \"name\", \"template\", \"variables\"].every(k => k in tmpl))' 2>&1"

echo ""

# Test 5: Deployment Workflows
echo "=== Test Group: Deployment Workflows ==="

run_test "2 deployment workflows defined" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.keys(data.deployment_workflows).length === 2' 2>&1"

run_test "deploy_agent workflow exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' '\"deploy_agent\" in data.deployment_workflows' 2>&1"

run_test "deploy_model workflow exists" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' '\"deploy_model\" in data.deployment_workflows' 2>&1"

run_test "All workflows have required fields" \
  "node '$NOA_ROOT/scripts/tests/json-test.js' 'N:/noa/data/resources/registry.json' 'Object.values(data.deployment_workflows).every(wf => [\"id\", \"name\", \"steps\"].every(k => k in wf))' 2>&1"

echo ""

# Test 6: Deployment Scripts
echo "=== Test Group: Deployment Scripts ==="

run_test "deploy-agent.sh exists" \
  "[[ -f '$NOA_ROOT/scripts/agents/deploy-agent.sh' ]]"

run_test "deploy-agent.sh is executable" \
  "[[ -x '$NOA_ROOT/scripts/agents/deploy-agent.sh' ]]"

run_test "deploy-model.sh exists" \
  "[[ -f '$NOA_ROOT/scripts/models/deploy-model.sh' ]]"

run_test "deploy-model.sh is executable" \
  "[[ -x '$NOA_ROOT/scripts/models/deploy-model.sh' ]]"

echo ""

# Summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "✓ All Phase 5 integration tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
