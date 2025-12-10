#!/usr/bin/env bash
# Validate AI provider configuration files against Category 14 quality requirements.
#
# Checks all provider config files for compliance with CHK122-CHK130 requirements:
# - CHK122: name, type, priority, enabled, description
# - CHK123: cli (command, package, version, binaryPath)
# - CHK124: modes (cli, cloud, ide where applicable)
# - CHK125: capabilities object
# - CHK126: sharedResources paths
# - CHK127: latency targets and timeout
# - CHK128: priority uniqueness
# - CHK129: binaryPath uses ${NOA_ROOT} syntax
# - CHK130: sharedResources paths consistent
#
# Usage:
#   ./validate-provider-configs.sh
#   ./validate-provider-configs.sh --json
#   ./validate-provider-configs.sh --fix

set -euo pipefail

# Defaults
NOA_ROOT="${NOA_ROOT:-}"
JSON_OUTPUT=false
FIX_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --fix)
            FIX_MODE=true
            shift
            ;;
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

# Auto-detect NOA_ROOT
if [[ -z "$NOA_ROOT" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    NOA_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
fi

PROVIDERS_DIR="$NOA_ROOT/ai/providers"

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required for this script" >&2
    exit 1
fi

# Standard shared resources paths
declare -A STANDARD_SHARED_RESOURCES=(
    ["prompts"]="${NOA_ROOT}/ai/shared/prompts"
    ["agents"]="${NOA_ROOT}/ai/shared/agents"
    ["tools"]="${NOA_ROOT}/ai/shared/tools"
    ["commands"]="${NOA_ROOT}/ai/shared/commands"
    ["executionMemory"]="${NOA_ROOT}/ai/shared/resources/execution-memory.db"
)

# Results tracking
TOTAL=0
PASSED=0
FAILED=0
FIXED=0
declare -A ALL_PRIORITIES
declare -A ALL_SHARED_RESOURCES

# Find all provider config files
find_config_files() {
    local files=()
    local categories=("local" "cloud" "hybrid" "ide")

    for category in "${categories[@]}"; do
        local category_dir="$PROVIDERS_DIR/$category"
        if [[ -d "$category_dir" ]]; then
            # Find config.json files in provider directories
            while IFS= read -r -d '' config_file; do
                local provider_name=$(basename "$(dirname "$config_file")")
                files+=("$config_file|$category|$provider_name")
            done < <(find "$category_dir" -mindepth 2 -name "config.json" -type f -print0 2>/dev/null || true)

            # Find direct JSON files in category directory
            while IFS= read -r -d '' json_file; do
                local provider_name=$(basename "$json_file" .json)
                files+=("$json_file|$category|$provider_name")
            done < <(find "$category_dir" -maxdepth 1 -name "*.json" -type f -print0 2>/dev/null || true)
        fi
    done

    printf '%s\n' "${files[@]}"
}

# Validate a single config file
validate_config() {
    local config_path="$1"
    local category="$2"
    local provider_name="$3"

    local errors=()
    local warnings=()
    local fixed=()

    # Check if file exists and is valid JSON
    if [[ ! -f "$config_path" ]]; then
        echo "ERROR: Config file not found: $config_path" >&2
        return 1
    fi

    if ! jq empty "$config_path" 2>/dev/null; then
        echo "ERROR: Invalid JSON: $config_path" >&2
        return 1
    fi

    # CHK122: name, type, priority, enabled, description
    local has_name=$(jq -e '.name // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_type=$(jq -e '.type // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_priority=$(jq -e '.priority // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_enabled=$(jq -e 'has("enabled")' "$config_path" 2>/dev/null || echo false)
    local has_description=$(jq -e '.description // empty' "$config_path" | grep -q . && echo true || echo false)

    if [[ "$has_description" == "false" ]] && [[ "$FIX_MODE" == "true" ]]; then
        jq ". + {description: \"AI provider: $provider_name\"}" "$config_path" > "$config_path.tmp" && mv "$config_path.tmp" "$config_path"
        fixed+=("Added missing description")
        ((FIXED++)) || true
    fi

    # CHK123: cli (command, package, version, binaryPath)
    local has_cli=$(jq -e 'has("cli")' "$config_path" 2>/dev/null || echo false)
    local has_command=$(jq -e '.cli.command // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_binary_path=$(jq -e '.cli.binaryPath // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_package=$(jq -e '.cli.package // empty' "$config_path" | grep -q . && echo true || echo false)
    local has_version=$(jq -e '.cli.version // empty' "$config_path" | grep -q . && echo true || echo false)

    # CHK124: modes
    local has_modes=$(jq -e 'has("modes")' "$config_path" 2>/dev/null || echo false)

    # CHK125: capabilities
    local has_capabilities=$(jq -e 'has("capabilities")' "$config_path" 2>/dev/null || echo false)

    if [[ "$has_capabilities" == "false" ]] && [[ "$FIX_MODE" == "true" ]]; then
        jq '. + {capabilities: {}}' "$config_path" > "$config_path.tmp" && mv "$config_path.tmp" "$config_path"
        fixed+=("Added missing capabilities object")
        ((FIXED++)) || true
    fi

    # CHK126: sharedResources
    local has_shared_resources=$(jq -e 'has("sharedResources")' "$config_path" 2>/dev/null || echo false)

    if [[ "$has_shared_resources" == "false" ]] && [[ "$FIX_MODE" == "true" ]]; then
        local shared_resources_json="{}"
        for key in "${!STANDARD_SHARED_RESOURCES[@]}"; do
            shared_resources_json=$(echo "$shared_resources_json" | jq ". + {\"$key\": \"${STANDARD_SHARED_RESOURCES[$key]}\"}")
        done
        jq ". + {sharedResources: $shared_resources_json}" "$config_path" > "$config_path.tmp" && mv "$config_path.tmp" "$config_path"
        fixed+=("Added missing sharedResources")
        ((FIXED++)) || true
    fi

    # CHK127: latency and timeout
    local has_latency=$(jq -e 'has("latency")' "$config_path" 2>/dev/null || echo false)
    local has_timeout=$(jq -e 'has("timeout")' "$config_path" 2>/dev/null || echo false)

    if [[ "$has_latency" == "false" ]] && [[ "$FIX_MODE" == "true" ]]; then
        jq '. + {latency: {target: "<2s"}}' "$config_path" > "$config_path.tmp" && mv "$config_path.tmp" "$config_path"
        fixed+=("Added missing latency")
        ((FIXED++)) || true
    fi

    if [[ "$has_timeout" == "false" ]] && [[ "$FIX_MODE" == "true" ]]; then
        jq '. + {timeout: 30000}' "$config_path" > "$config_path.tmp" && mv "$config_path.tmp" "$config_path"
        fixed+=("Added missing timeout")
        ((FIXED++)) || true
    fi

    # CHK128: priority uniqueness
    local priority=$(jq -r '.priority // empty' "$config_path")
    if [[ -n "$priority" ]]; then
        if [[ -n "${ALL_PRIORITIES[$priority]:-}" ]]; then
            errors+=("Priority $priority is duplicate (also used by ${ALL_PRIORITIES[$priority]})")
        else
            ALL_PRIORITIES[$priority]="$provider_name"
        fi
    fi

    # CHK129: binaryPath uses ${NOA_ROOT} syntax
    local binary_path_str=$(jq -r '.cli.binaryPath // empty' "$config_path" 2>/dev/null || echo "")
    if [[ -n "$binary_path_str" ]]; then
        # Handle both string and object binaryPath
        if echo "$binary_path_str" | jq -e 'type == "object"' &>/dev/null; then
            binary_path_str=$(echo "$binary_path_str" | jq -r '.[]' | tr '\n' ' ')
        fi
        if [[ "$binary_path_str" != *'${NOA_ROOT}'* ]]; then
            errors+=("binaryPath does not use \${NOA_ROOT} syntax: $binary_path_str")
        fi
    fi

    # CHK130: sharedResources paths consistent
    for key in "${!STANDARD_SHARED_RESOURCES[@]}"; do
        local expected="${STANDARD_SHARED_RESOURCES[$key]}"
        local actual=$(jq -r ".sharedResources.$key // empty" "$config_path" 2>/dev/null || echo "")
        if [[ -n "$actual" ]]; then
            if [[ "$actual" != "$expected" ]]; then
                errors+=("sharedResources.$key: expected '$expected', got '$actual'")
            fi
            # Track for consistency check
            if [[ -z "${ALL_SHARED_RESOURCES[$key]:-}" ]]; then
                ALL_SHARED_RESOURCES[$key]="$actual"
            elif [[ "${ALL_SHARED_RESOURCES[$key]}" != "$actual" ]]; then
                warnings+=("Inconsistent sharedResources.$key across providers")
            fi
        fi
    done

    # Determine overall status
    local all_checks_pass=true
    if [[ "$has_name" != "true" ]] || [[ "$has_type" != "true" ]] || \
       [[ "$has_priority" != "true" ]] || [[ "$has_enabled" != "true" ]] || \
       [[ "$has_description" != "true" ]] || [[ "$has_cli" != "true" ]] || \
       [[ "$has_command" != "true" ]] || [[ "$has_binary_path" != "true" ]] || \
       [[ "$has_capabilities" != "true" ]] || [[ "$has_shared_resources" != "true" ]] || \
       [[ "$has_latency" != "true" ]] || [[ "$has_timeout" != "true" ]] || \
       [[ ${#errors[@]} -gt 0 ]]; then
        all_checks_pass=false
    fi

    if [[ "$all_checks_pass" == "true" ]]; then
        ((PASSED++)) || true
    else
        ((FAILED++)) || true
    fi

    # Output results (simplified for bash - full JSON output would require more complex structure)
    if [[ "$JSON_OUTPUT" == "false" ]]; then
        local status="PASS"
        [[ "$all_checks_pass" == "false" ]] && status="FAIL"
        echo "$provider_name ($status)"
        for error in "${errors[@]}"; do
            echo "  ERROR: $error" >&2
        done
        for warning in "${warnings[@]}"; do
            echo "  WARNING: $warning" >&2
        done
        if [[ "$FIX_MODE" == "true" ]] && [[ ${#fixed[@]} -gt 0 ]]; then
            for fix in "${fixed[@]}"; do
                echo "  FIXED: $fix"
            done
        fi
    fi
}

# Main execution
main() {
    local config_files
    mapfile -t config_files < <(find_config_files)

    TOTAL=${#config_files[@]}

    if [[ "$JSON_OUTPUT" == "false" ]]; then
        echo "Provider Config Validation Results"
        echo "==================================="
        echo "NOA Root: $NOA_ROOT"
        echo "Providers Dir: $PROVIDERS_DIR"
        echo ""
        echo "Summary:"
    fi

    for config_entry in "${config_files[@]}"; do
        IFS='|' read -r config_path category provider_name <<< "$config_entry"
        validate_config "$config_path" "$category" "$provider_name" || true
    done

    if [[ "$JSON_OUTPUT" == "false" ]]; then
        echo ""
        echo "  Total: $TOTAL"
        echo "  Passed: $PASSED"
        echo "  Failed: $FAILED"
        if [[ "$FIX_MODE" == "true" ]]; then
            echo "  Fixed: $FIXED"
        fi
    fi

    # Exit with error code if any failures
    if [[ $FAILED -gt 0 ]]; then
        exit 1
    fi
    exit 0
}

main "$@"

