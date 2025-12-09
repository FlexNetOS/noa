#!/bin/bash
#
# NOA Kernel Mode Control (B139-B141)
#
# Set and manage kernel isolation mode for NOA on Unix systems.
# Implements noa-kernel-params set kernel_mode {native|vm|container|sandbox}
#
# Usage:
#   ./noa-kernel-mode.sh get
#   ./noa-kernel-mode.sh set <mode>
#   ./noa-kernel-mode.sh detect
#   ./noa-kernel-mode.sh status

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"

CONFIG_PATH="$NOA_ROOT/config/kernel-mode.json"

get_current_mode() {
    if [[ -f "$CONFIG_PATH" ]]; then
        jq -r '.mode // "native"' "$CONFIG_PATH" 2>/dev/null || echo "native"
    else
        echo "native"
    fi
}

test_kvm() {
    [[ -e /dev/kvm ]] && [[ -r /dev/kvm ]]
}

test_virtfw() {
    # macOS Virtualization.framework
    [[ "$(uname -s)" == "Darwin" ]] && [[ "$(sysctl -n kern.hv_support 2>/dev/null)" == "1" ]]
}

test_docker() {
    command -v docker &>/dev/null && docker info &>/dev/null
}

test_podman() {
    command -v podman &>/dev/null
}

test_sandbox() {
    # Check for bubblewrap or firejail on Linux
    if [[ "$(uname -s)" == "Linux" ]]; then
        command -v bwrap &>/dev/null || command -v firejail &>/dev/null
    else
        # macOS has sandbox-exec
        command -v sandbox-exec &>/dev/null
    fi
}

detect_best_mode() {
    local os=$(uname -s)

    case "$os" in
        Linux)
            if test_kvm; then
                echo "vm"
            elif test_docker || test_podman; then
                echo "container"
            elif test_sandbox; then
                echo "sandbox"
            else
                echo "native"
            fi
            ;;
        Darwin)
            if test_virtfw; then
                echo "vm"
            elif test_docker; then
                echo "container"
            elif test_sandbox; then
                echo "sandbox"
            else
                echo "native"
            fi
            ;;
        *)
            echo "native"
            ;;
    esac
}

get_capabilities() {
    local vm_ok="false"
    local container_ok="false"
    local sandbox_ok="false"

    # Check VM capability
    if test_kvm || test_virtfw; then
        vm_ok="true"
    fi

    # Check container capability
    if test_docker || test_podman; then
        container_ok="true"
    fi

    # Check sandbox capability
    if test_sandbox; then
        sandbox_ok="true"
    fi

    echo "{\"native\":true,\"vm\":$vm_ok,\"container\":$container_ok,\"sandbox\":$sandbox_ok}"
}

set_kernel_mode() {
    local new_mode="$1"
    local caps=$(get_capabilities)

    if [[ "$new_mode" == "auto" ]]; then
        new_mode=$(detect_best_mode)
        echo -e "${CYAN}Auto-detected mode: $new_mode${NC}"
    fi

    # Check if mode is available
    local mode_available=$(echo "$caps" | jq -r ".$new_mode")
    if [[ "$mode_available" != "true" ]]; then
        echo -e "${RED}[ERROR] Mode '$new_mode' is not available on this system${NC}"
        echo -e "${YELLOW}Available modes:${NC}"
        echo "$caps" | jq -r 'to_entries | .[] | select(.value == true) | "  - \(.key)"'
        exit 1
    fi

    # Save configuration
    mkdir -p "$(dirname "$CONFIG_PATH")"

    cat > "$CONFIG_PATH" << EOF
{
  "mode": "$new_mode",
  "setAt": "$(date -Iseconds)",
  "capabilities": $caps
}
EOF

    # Set environment variable
    export NOA_KERNEL_MODE="$new_mode"

    echo -e "${GREEN}[OK] Kernel mode set to: $new_mode${NC}"

    # Mode-specific instructions
    case "$new_mode" in
        vm)
            echo ""
            echo -e "${YELLOW}VM mode active. Use these commands:${NC}"
            if [[ "$(uname -s)" == "Linux" ]]; then
                echo -e "${GRAY}  Create VM:  ./sys/kernel/linux/vm/noa-vm.sh create${NC}"
                echo -e "${GRAY}  Start VM:   ./sys/kernel/linux/vm/noa-vm.sh start${NC}"
            else
                echo -e "${GRAY}  Create VM:  ./sys/kernel/macos/vm/noa-vm.sh create${NC}"
                echo -e "${GRAY}  Start VM:   ./sys/kernel/macos/vm/noa-vm.sh start${NC}"
            fi
            ;;
        container)
            echo ""
            echo -e "${YELLOW}Container mode active. Ensure Docker/Podman is running.${NC}"
            ;;
        sandbox)
            echo ""
            echo -e "${YELLOW}Sandbox mode active.${NC}"
            ;;
    esac
}

show_status() {
    echo -e "${CYAN}NOA Kernel Mode Status${NC}"
    echo -e "${CYAN}═══════════════════════${NC}"
    echo ""

    local current_mode=$(get_current_mode)
    local color="${YELLOW}"
    [[ "$current_mode" != "native" ]] && color="${GREEN}"
    echo -e "Current Mode: ${color}$current_mode${NC}"
    echo ""

    echo -e "${YELLOW}Available Modes:${NC}"

    local caps=$(get_capabilities)

    for mode in native vm container sandbox; do
        local available=$(echo "$caps" | jq -r ".$mode")
        local status="[--]"
        local color="${GRAY}"
        local current=""

        if [[ "$available" == "true" ]]; then
            status="[OK]"
            color="${GREEN}"
        fi

        [[ "$mode" == "$current_mode" ]] && current=" (current)"

        local details=""
        case "$mode" in
            native) details="Direct host execution" ;;
            vm)
                if [[ "$(uname -s)" == "Linux" ]]; then
                    details="KVM/QEMU isolation"
                else
                    details="Virtualization.framework"
                fi
                ;;
            container) details="Docker/Podman containers" ;;
            sandbox) details="Lightweight sandbox" ;;
        esac

        echo -e "  ${color}$status $mode$current${NC}"
        echo -e "      ${GRAY}$details${NC}"
    done

    echo ""
    echo -e "${CYAN}Best Available: $(detect_best_mode)${NC}"
}

# Main
ACTION="${1:-status}"
MODE="${2:-auto}"

case "$ACTION" in
    get)
        get_current_mode
        ;;
    set)
        set_kernel_mode "$MODE"
        ;;
    detect)
        best=$(detect_best_mode)
        echo -e "${CYAN}Best available mode: $best${NC}"
        echo ""
        echo -e "${YELLOW}Capabilities detected:${NC}"
        get_capabilities | jq -r 'to_entries | .[] | "\(.key): \(.value)"' | while read line; do
            mode=$(echo "$line" | cut -d: -f1)
            value=$(echo "$line" | cut -d: -f2 | tr -d ' ')
            if [[ "$value" == "true" ]]; then
                echo -e "  ${GREEN}[OK]${NC} $mode"
            else
                echo -e "  ${GRAY}[--] $mode${NC}"
            fi
        done
        ;;
    status)
        show_status
        ;;
    *)
        echo "Usage: $0 {get|set|detect|status} [mode]"
        echo ""
        echo "Actions:"
        echo "  get      Show current kernel mode"
        echo "  set      Set kernel mode (native|vm|container|sandbox|auto)"
        echo "  detect   Auto-detect best available mode"
        echo "  status   Show full status and capabilities"
        exit 1
        ;;
esac

