#!/bin/bash
#
# Generate bootstrap installation report.
#
# Creates a detailed report of the NOA bootstrap installation including
# all installed tools, versions, and configuration status.
#
# Usage:
#   ./generate-report.sh [--noa-root DIR] [--output PATH]
#
# Arguments:
#   --noa-root DIR    NOA root directory (default: auto-detect)
#   --output PATH     Path to save the report (default: logs/bootstrap/report.md)
#
# Example:
#   ./generate-report.sh
#   ./generate-report.sh --output /tmp/report.md

set -euo pipefail

# Parse arguments
NOA_ROOT=""
OUTPUT_PATH=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --noa-root)
            NOA_ROOT="$2"
            shift 2
            ;;
        --output)
            OUTPUT_PATH="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [--noa-root DIR] [--output PATH]"
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 1
            ;;
    esac
done

# Auto-detect NOA_ROOT
if [[ -z "$NOA_ROOT" ]]; then
    if [[ -n "${NOA_ROOT:-}" ]]; then
        NOA_ROOT="$NOA_ROOT"
    else
        SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
        NOA_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
    fi
fi

if [[ -z "$OUTPUT_PATH" ]]; then
    OUTPUT_PATH="$NOA_ROOT/logs/bootstrap/report.md"
fi

# Ensure output directory exists
OUTPUT_DIR="$(dirname "$OUTPUT_PATH")"
mkdir -p "$OUTPUT_DIR"

echo -e "\033[36mGenerating NOA Bootstrap Report...\033[0m"

# Detect platform
PLATFORM="$(uname -s)"
PLATFORM_VERSION="$(uname -r)"

# Build report
REPORT="# NOA Bootstrap Installation Report

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')
**NOA Root**: \`$NOA_ROOT\`
**Platform**: $PLATFORM ($PLATFORM_VERSION)

---

## Environment Summary

| Component | Status | Details |
|-----------|--------|---------|"

# Check toolchains
check_tool() {
    local name="$1"
    local cmd="$2"

    if version=$($cmd 2>&1 | head -n1); then
        echo "| $name | ✅ Installed | \`$version\` |"
    else
        echo "| $name | ❌ Not Found | - |"
    fi
}

REPORT+="
$(check_tool "Git" "git --version")
$(check_tool "Rust" "rustc --version")
$(check_tool "Go" "go version")
$(check_tool "Node.js" "node --version")
$(check_tool "Python" "python3 --version")
$(check_tool "jq" "jq --version")
$(check_tool "ripgrep" "rg --version")"

REPORT+="

---

## Directory Structure

| Directory | Exists | Contents |
|-----------|--------|----------|"

# Check directories
check_dir() {
    local dir="$1"
    local path="$NOA_ROOT/$dir"

    if [[ -d "$path" ]]; then
        local count=$(find "$path" -type f 2>/dev/null | wc -l | tr -d ' ')
        echo "| \`$dir\` | ✅ | $count files |"
    else
        echo "| \`$dir\` | ❌ | Not created |"
    fi
}

REPORT+="
$(check_dir "bin")
$(check_dir "config")
$(check_dir "ai")
$(check_dir "ai/shared")
$(check_dir "ai/providers")
$(check_dir "logs")
$(check_dir "specs")
$(check_dir "cache")"

REPORT+="

---

## AI Providers

| Provider | Type | Config Exists |
|----------|------|---------------|"

# Check AI providers
for provider_type in local cloud hybrid ide; do
    provider_dir="$NOA_ROOT/ai/providers/$provider_type"
    if [[ -d "$provider_dir" ]]; then
        for config in $(find "$provider_dir" -name "config.json" 2>/dev/null); do
            provider_name="$(basename "$(dirname "$config")")"
            REPORT+="
| $provider_name | $provider_type | ✅ |"
        done
    fi
done

REPORT+="

---

## Shared Resources

| Resource | Path | Status |
|----------|------|--------|"

# Check shared resources
check_resource() {
    local name="$1"
    local path="$2"
    local full_path="$NOA_ROOT/$path"

    if [[ -e "$full_path" ]]; then
        echo "| $name | \`$path\` | ✅ |"
    else
        echo "| $name | \`$path\` | ❌ |"
    fi
}

REPORT+="
$(check_resource "Agent definitions" "ai/shared/agents")
$(check_resource "Workflow definitions" "ai/shared/workflows")
$(check_resource "Prompts" "ai/shared/prompts")
$(check_resource "Tools" "ai/shared/tools")
$(check_resource "Skills" "ai/shared/skills")
$(check_resource "Models" "ai/shared/models")
$(check_resource "Commands" "ai/shared/commands")
$(check_resource "Execution Memory DB" "ai/shared/resources/execution-memory.db")
$(check_resource "Resource Registry" "ai/shared/resources/resource-registry.json")"

REPORT+="

---

## Configuration Files

| Config | Path | Valid |
|--------|------|-------|"

# Check config files
check_config() {
    local cfg="$1"
    local path="$NOA_ROOT/$cfg"
    local name="$(basename "$cfg")"

    if [[ -f "$path" ]]; then
        if jq empty "$path" 2>/dev/null; then
            echo "| $name | \`$cfg\` | ✅ Valid JSON |"
        else
            echo "| $name | \`$cfg\` | ⚠️ Invalid JSON |"
        fi
    else
        echo "| $name | \`$cfg\` | ❌ Not found |"
    fi
}

REPORT+="
$(check_config "config/ai-providers.json")
$(check_config "config/shared-resources.json")
$(check_config "config/bootstrap-state.json")
$(check_config "config/bootstrap-tools.json")"

REPORT+="

---

## Next Steps

1. Run \`./scripts/bootstrap/verify/verify-all.sh\` to verify installation
2. Run \`./scripts/bootstrap/verify/smoke-test.sh\` to test toolchains
3. Source the environment: \`. ./noa-env.sh\`
4. Start using NOA commands

---

*Report generated by NOA Bootstrap*"

# Save report
echo "$REPORT" > "$OUTPUT_PATH"

echo -e "\033[32mReport saved to: $OUTPUT_PATH\033[0m"
echo ""
echo -e "\033[33mPreview:\033[0m"
echo "${REPORT:0:1000}"
echo "..."

