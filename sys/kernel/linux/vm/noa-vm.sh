#!/bin/bash
#
# NOA KVM/QEMU VM Management (B127)
#
# Manages NOA VM instances on Linux using KVM/QEMU.
# Supports create, start, stop, status, and destroy operations.
#
# Usage:
#   ./noa-vm.sh create [--name NAME] [--image PATH] [--memory MB] [--cpus N]
#   ./noa-vm.sh start [--name NAME]
#   ./noa-vm.sh stop [--name NAME]
#   ./noa-vm.sh status [--name NAME]
#   ./noa-vm.sh destroy [--name NAME]
#   ./noa-vm.sh list
#   ./noa-vm.sh console [--name NAME]

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
MEMORY=512
CPUS=2
IMAGE_PATH=""

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

# VM directories
VM_DIR="$NOA_ROOT/sys/kernel/vms"
RUN_DIR="$NOA_ROOT/sys/kernel/run"

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
        --force|-f)
            FORCE=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Default image path
if [[ -z "$IMAGE_PATH" ]]; then
    IMAGE_PATH="$NOA_ROOT/sys/kernel/images/noa-linux.qcow2"
fi

# VM-specific paths
VM_DISK="$VM_DIR/$VM_NAME/$VM_NAME.qcow2"
VM_PID="$RUN_DIR/$VM_NAME.pid"
VM_MONITOR="$RUN_DIR/$VM_NAME.monitor"
VM_CONSOLE="$RUN_DIR/$VM_NAME.console"

check_kvm() {
    if [[ ! -e /dev/kvm ]]; then
        echo -e "${RED}KVM not available. Ensure virtualization is enabled in BIOS.${NC}"
        exit 1
    fi

    if [[ ! -r /dev/kvm ]] || [[ ! -w /dev/kvm ]]; then
        echo -e "${RED}No access to /dev/kvm. Add user to kvm group: sudo usermod -aG kvm \$USER${NC}"
        exit 1
    fi
}

check_qemu() {
    if ! command -v qemu-system-x86_64 &> /dev/null; then
        echo -e "${RED}QEMU not found. Install it: sudo apt install qemu-system-x86${NC}"
        exit 1
    fi
}

ensure_dirs() {
    mkdir -p "$VM_DIR/$VM_NAME"
    mkdir -p "$RUN_DIR"
}

is_running() {
    [[ -f "$VM_PID" ]] && kill -0 "$(cat "$VM_PID")" 2>/dev/null
}

create_vm() {
    echo -e "${CYAN}Creating NOA VM: $VM_NAME${NC}"

    # Check if running
    if is_running; then
        echo -e "${RED}VM is running. Stop it first.${NC}"
        exit 1
    fi

    # Check image
    if [[ ! -f "$IMAGE_PATH" ]]; then
        echo -e "${RED}VM image not found: $IMAGE_PATH${NC}"
        echo -e "${YELLOW}Build the image first with: ./sys/kernel/images/build-alpine.sh${NC}"
        exit 1
    fi

    ensure_dirs

    # Copy image
    if [[ -f "$VM_DISK" ]] && [[ -z "${FORCE:-}" ]]; then
        echo -e "${YELLOW}VM disk already exists. Use --force to replace.${NC}"
        exit 1
    fi

    echo -e "${GRAY}  Copying VM disk...${NC}"
    cp "$IMAGE_PATH" "$VM_DISK"

    # Resize if needed (add 1GB headroom)
    qemu-img resize "$VM_DISK" +1G 2>/dev/null || true

    echo -e "${GREEN}  [OK] VM created: $VM_NAME${NC}"
    echo -e "${GRAY}       Memory: ${MEMORY}MB${NC}"
    echo -e "${GRAY}       CPUs: $CPUS${NC}"
    echo -e "${GRAY}       Disk: $VM_DISK${NC}"
}

start_vm() {
    echo -e "${CYAN}Starting VM: $VM_NAME${NC}"

    if is_running; then
        echo -e "${YELLOW}VM is already running${NC}"
        return
    fi

    if [[ ! -f "$VM_DISK" ]]; then
        echo -e "${RED}VM disk not found. Create the VM first.${NC}"
        exit 1
    fi

    ensure_dirs

    local START_TIME=$(date +%s.%N)

    # Start QEMU with KVM
    qemu-system-x86_64 \
        -name "$VM_NAME" \
        -machine q35,accel=kvm \
        -cpu host \
        -smp "$CPUS" \
        -m "$MEMORY" \
        -drive file="$VM_DISK",format=qcow2,if=virtio \
        -netdev user,id=net0,hostfwd=tcp::2222-:22 \
        -device virtio-net-pci,netdev=net0 \
        -nographic \
        -serial unix:"$VM_CONSOLE",server,nowait \
        -monitor unix:"$VM_MONITOR",server,nowait \
        -pidfile "$VM_PID" \
        -daemonize

    # Wait for boot
    echo -e "${GRAY}  Waiting for boot...${NC}"
    local TIMEOUT=30
    local ELAPSED=0
    while [[ $ELAPSED -lt $TIMEOUT ]]; do
        if nc -z localhost 2222 2>/dev/null; then
            break
        fi
        sleep 1
        ((ELAPSED++))
    done

    local END_TIME=$(date +%s.%N)
    local BOOT_TIME=$(echo "$END_TIME - $START_TIME" | bc)

    if [[ $ELAPSED -ge $TIMEOUT ]]; then
        echo -e "${YELLOW}  [WARN] Boot timeout - VM may still be starting${NC}"
    else
        echo -e "${GREEN}  [OK] VM started in ${BOOT_TIME}s${NC}"
    fi

    echo -e "${GRAY}  SSH: ssh -p 2222 root@localhost${NC}"
}

stop_vm() {
    echo -e "${CYAN}Stopping VM: $VM_NAME${NC}"

    if ! is_running; then
        echo -e "${YELLOW}VM is not running${NC}"
        return
    fi

    # Try graceful shutdown first
    if [[ -S "$VM_MONITOR" ]]; then
        echo "system_powerdown" | nc -U "$VM_MONITOR" -q 1 2>/dev/null || true
        sleep 3
    fi

    # Force kill if still running
    if is_running; then
        kill "$(cat "$VM_PID")" 2>/dev/null || true
    fi

    rm -f "$VM_PID" "$VM_MONITOR" "$VM_CONSOLE"

    echo -e "${GREEN}  [OK] VM stopped${NC}"
}

status_vm() {
    echo -e "${CYAN}NOA VM Status: $VM_NAME${NC}"

    if is_running; then
        local PID=$(cat "$VM_PID")
        echo -e "  State: ${GREEN}Running${NC}"
        echo -e "  ${GRAY}PID: $PID${NC}"

        # Check SSH
        if nc -z localhost 2222 2>/dev/null; then
            echo -e "  ${GRAY}SSH: Available on port 2222${NC}"
        fi

        # Memory usage
        if [[ -f "/proc/$PID/status" ]]; then
            local MEM=$(grep VmRSS /proc/$PID/status | awk '{print $2}')
            echo -e "  ${GRAY}Memory: $((MEM / 1024))MB${NC}"
        fi
    else
        echo -e "  State: ${YELLOW}Stopped${NC}"
    fi

    if [[ -f "$VM_DISK" ]]; then
        local SIZE=$(du -h "$VM_DISK" | cut -f1)
        echo -e "  ${GRAY}Disk: $VM_DISK ($SIZE)${NC}"
    fi
}

destroy_vm() {
    echo -e "${RED}Destroying VM: $VM_NAME${NC}"

    if is_running; then
        echo -e "${GRAY}  Stopping VM first...${NC}"
        stop_vm
    fi

    if [[ -d "$VM_DIR/$VM_NAME" ]]; then
        rm -rf "$VM_DIR/$VM_NAME"
    fi

    rm -f "$VM_PID" "$VM_MONITOR" "$VM_CONSOLE"

    echo -e "${GREEN}  [OK] VM destroyed${NC}"
}

list_vms() {
    echo -e "${CYAN}NOA VMs:${NC}"

    if [[ ! -d "$VM_DIR" ]]; then
        echo -e "  ${GRAY}No VMs found${NC}"
        return
    fi

    echo -e "  ${GRAY}NAME\t\tSTATE\t\tDISK${NC}"

    for vm_path in "$VM_DIR"/*; do
        if [[ -d "$vm_path" ]]; then
            local name=$(basename "$vm_path")
            local state="stopped"
            local pid_file="$RUN_DIR/$name.pid"

            if [[ -f "$pid_file" ]] && kill -0 "$(cat "$pid_file")" 2>/dev/null; then
                state="running"
            fi

            local disk_size=""
            if [[ -f "$vm_path/$name.qcow2" ]]; then
                disk_size=$(du -h "$vm_path/$name.qcow2" | cut -f1)
            fi

            if [[ "$state" == "running" ]]; then
                echo -e "  ${GREEN}$name${NC}\t\t$state\t\t$disk_size"
            else
                echo -e "  $name\t\t$state\t\t$disk_size"
            fi
        fi
    done
}

console_vm() {
    echo -e "${CYAN}Connecting to VM console: $VM_NAME${NC}"

    if ! is_running; then
        echo -e "${RED}VM is not running${NC}"
        exit 1
    fi

    if [[ ! -S "$VM_CONSOLE" ]]; then
        echo -e "${RED}Console socket not found${NC}"
        exit 1
    fi

    echo -e "${YELLOW}Press Ctrl+] to exit${NC}"
    socat -,raw,echo=0 UNIX-CONNECT:"$VM_CONSOLE"
}

# Main
check_kvm
check_qemu

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
    console)
        console_vm
        ;;
    *)
        echo "Usage: $0 {create|start|stop|status|destroy|list|console} [options]"
        echo ""
        echo "Options:"
        echo "  --name, -n NAME    VM name (default: noa-vm)"
        echo "  --image, -i PATH   Source image path"
        echo "  --memory, -m MB    Memory in MB (default: 512)"
        echo "  --cpus, -c N       Number of CPUs (default: 2)"
        echo "  --force, -f        Force operation"
        exit 1
        ;;
esac

