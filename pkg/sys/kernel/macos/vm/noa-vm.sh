#!/bin/bash
#
# NOA macOS VM Management (B131)
#
# Manages NOA VM instances on macOS using Virtualization.framework
# via the `tart` CLI tool (open-source, uses native APIs).
#
# Dependencies:
#   brew install cirruslabs/cli/tart
#
# Usage:
#   ./noa-vm.sh create [--name NAME] [--image PATH]
#   ./noa-vm.sh start [--name NAME]
#   ./noa-vm.sh stop [--name NAME]
#   ./noa-vm.sh status [--name NAME]
#   ./noa-vm.sh list
#   ./noa-vm.sh ssh [--name NAME]

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

# Defaults
VM_NAME="noa-vm"
IMAGE_PATH=""
MEMORY=512
CPUS=2

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

# Parse arguments
ACTION="${1:-}"
shift || true

while [[ $# -gt 0 ]]; do
    case "$1" in
        --name|-n)
            VM_NAME="$2"
            shift 2
            ;;
        --image|-i)
            IMAGE_PATH="$2"
            shift 2
            ;;
        --memory|-m)
            MEMORY="$2"
            shift 2
            ;;
        --cpus|-c)
            CPUS="$2"
            shift 2
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

check_macos() {
    if [[ "$(uname -s)" != "Darwin" ]]; then
        echo -e "${RED}This script is for macOS only.${NC}"
        exit 1
    fi
}

check_hypervisor() {
    local hv_support=$(sysctl -n kern.hv_support 2>/dev/null || echo "0")
    if [[ "$hv_support" != "1" ]]; then
        echo -e "${RED}Hypervisor not supported on this Mac.${NC}"
        echo -e "${GRAY}Ensure you're running macOS 11+ on supported hardware.${NC}"
        exit 1
    fi
}

check_tart() {
    if ! command -v tart &>/dev/null; then
        echo -e "${YELLOW}tart CLI not found. Installing...${NC}"

        if command -v brew &>/dev/null; then
            brew install cirruslabs/cli/tart
        else
            echo -e "${RED}Homebrew not found. Install tart manually:${NC}"
            echo "brew install cirruslabs/cli/tart"
            exit 1
        fi
    fi
}

create_vm() {
    echo -e "${CYAN}Creating NOA VM: $VM_NAME${NC}"

    # Check if VM exists
    if tart list | grep -q "^$VM_NAME"; then
        echo -e "${YELLOW}VM already exists: $VM_NAME${NC}"
        echo -e "${GRAY}Use 'destroy' first to recreate.${NC}"
        return
    fi

    if [[ -n "$IMAGE_PATH" ]] && [[ -f "$IMAGE_PATH" ]]; then
        # Import from OCI image or local file
        echo -e "${GRAY}  Importing from: $IMAGE_PATH${NC}"
        tart clone "$IMAGE_PATH" "$VM_NAME"
    else
        # Use a base Linux image
        echo -e "${GRAY}  Creating from ghcr.io/cirruslabs/ubuntu:latest${NC}"
        tart clone ghcr.io/cirruslabs/ubuntu:latest "$VM_NAME"
    fi

    # Configure resources
    tart set "$VM_NAME" --memory "$MEMORY" --cpu "$CPUS"

    echo -e "${GREEN}  [OK] VM created: $VM_NAME${NC}"
    echo -e "${GRAY}       Memory: ${MEMORY}MB${NC}"
    echo -e "${GRAY}       CPUs: $CPUS${NC}"
}

start_vm() {
    echo -e "${CYAN}Starting VM: $VM_NAME${NC}"

    if ! tart list | grep -q "^$VM_NAME"; then
        echo -e "${RED}VM not found: $VM_NAME${NC}"
        exit 1
    fi

    # Start in background
    tart run "$VM_NAME" --no-graphics &
    local pid=$!

    echo -e "${GRAY}  Waiting for VM to boot...${NC}"
    sleep 5

    # Get IP
    local ip=$(tart ip "$VM_NAME" 2>/dev/null || echo "")

    if [[ -n "$ip" ]]; then
        echo -e "${GREEN}  [OK] VM started${NC}"
        echo -e "${GRAY}  IP: $ip${NC}"
        echo -e "${GRAY}  SSH: ssh admin@$ip${NC}"
    else
        echo -e "${YELLOW}  VM starting (IP not yet available)${NC}"
    fi
}

stop_vm() {
    echo -e "${CYAN}Stopping VM: $VM_NAME${NC}"

    tart stop "$VM_NAME" 2>/dev/null || true

    echo -e "${GREEN}  [OK] VM stopped${NC}"
}

status_vm() {
    echo -e "${CYAN}NOA VM Status: $VM_NAME${NC}"

    if ! tart list | grep -q "^$VM_NAME"; then
        echo -e "  State: ${YELLOW}NOT FOUND${NC}"
        return
    fi

    # Check if running
    local ip=$(tart ip "$VM_NAME" 2>/dev/null || echo "")

    if [[ -n "$ip" ]]; then
        echo -e "  State: ${GREEN}Running${NC}"
        echo -e "  ${GRAY}IP: $ip${NC}"
    else
        echo -e "  State: ${YELLOW}Stopped${NC}"
    fi

    # Show config
    tart get "$VM_NAME" 2>/dev/null | grep -E "cpu|memory" | while read line; do
        echo -e "  ${GRAY}$line${NC}"
    done
}

destroy_vm() {
    echo -e "${RED}Destroying VM: $VM_NAME${NC}"

    tart stop "$VM_NAME" 2>/dev/null || true
    tart delete "$VM_NAME" 2>/dev/null || true

    echo -e "${GREEN}  [OK] VM destroyed${NC}"
}

list_vms() {
    echo -e "${CYAN}NOA VMs:${NC}"

    local vms=$(tart list 2>/dev/null | grep "^noa" || true)

    if [[ -z "$vms" ]]; then
        echo -e "  ${GRAY}No NOA VMs found${NC}"
        return
    fi

    echo "$vms"
}

ssh_vm() {
    local ip=$(tart ip "$VM_NAME" 2>/dev/null || echo "")

    if [[ -z "$ip" ]]; then
        echo -e "${RED}VM not running or IP not available${NC}"
        exit 1
    fi

    echo -e "${CYAN}Connecting to $VM_NAME ($ip)...${NC}"
    ssh -o StrictHostKeyChecking=no "admin@$ip"
}

# Main
check_macos
check_hypervisor
check_tart

case "$ACTION" in
    create)
        create_vm
        ;;
    start)
        start_vm
        ;;
    stop)
        stop_vm
        ;;
    status)
        status_vm
        ;;
    destroy)
        destroy_vm
        ;;
    list)
        list_vms
        ;;
    ssh)
        ssh_vm
        ;;
    *)
        echo "Usage: $0 {create|start|stop|status|destroy|list|ssh} [options]"
        echo ""
        echo "Options:"
        echo "  --name, -n NAME    VM name (default: noa-vm)"
        echo "  --image, -i PATH   Source image path"
        echo "  --memory, -m MB    Memory in MB (default: 512)"
        echo "  --cpus, -c N       Number of CPUs (default: 2)"
        exit 1
        ;;
esac

