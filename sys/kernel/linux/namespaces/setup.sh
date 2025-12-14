#!/bin/bash
#
# NOA Linux Namespace Isolation Setup (B128)
#
# Configures namespace isolation for NOA processes.
# Supports network, PID, mount, and user namespaces.
#
# Usage:
#   ./setup.sh [--network] [--pid] [--mount] [--user]

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../../.." && pwd)}"

# Configuration
NAMESPACE_DIR="$NOA_ROOT/sys/namespace"
NETNS_NAME="noa"
BRIDGE_NAME="noa0"
BRIDGE_IP="10.10.0.1/24"

# Parse arguments
SETUP_NETWORK=false
SETUP_PID=false
SETUP_MOUNT=false
SETUP_USER=false

for arg in "$@"; do
    case "$arg" in
        --network) SETUP_NETWORK=true ;;
        --pid) SETUP_PID=true ;;
        --mount) SETUP_MOUNT=true ;;
        --user) SETUP_USER=true ;;
        --all) SETUP_NETWORK=true; SETUP_PID=true; SETUP_MOUNT=true; SETUP_USER=true ;;
        *) echo -e "${RED}Unknown option: $arg${NC}"; exit 1 ;;
    esac
done

# If no specific namespace requested, show status
if ! $SETUP_NETWORK && ! $SETUP_PID && ! $SETUP_MOUNT && ! $SETUP_USER; then
    echo -e "${CYAN}NOA Namespace Status${NC}"
    echo ""

    # Check network namespace
    if ip netns list 2>/dev/null | grep -q "^$NETNS_NAME"; then
        echo -e "  Network namespace ($NETNS_NAME): ${GREEN}EXISTS${NC}"
    else
        echo -e "  Network namespace ($NETNS_NAME): ${YELLOW}NOT CONFIGURED${NC}"
    fi

    # Check bridge
    if ip link show "$BRIDGE_NAME" &>/dev/null; then
        echo -e "  Bridge ($BRIDGE_NAME): ${GREEN}EXISTS${NC}"
    else
        echo -e "  Bridge ($BRIDGE_NAME): ${YELLOW}NOT CONFIGURED${NC}"
    fi

    # Check mount namespace
    if [[ -d "$NAMESPACE_DIR/mnt" ]]; then
        echo -e "  Mount namespace dir: ${GREEN}EXISTS${NC}"
    else
        echo -e "  Mount namespace dir: ${YELLOW}NOT CONFIGURED${NC}"
    fi

    # Check user namespace support
    if [[ -f /proc/sys/kernel/unprivileged_userns_clone ]]; then
        userns=$(cat /proc/sys/kernel/unprivileged_userns_clone)
        if [[ "$userns" == "1" ]]; then
            echo -e "  User namespaces: ${GREEN}ENABLED${NC}"
        else
            echo -e "  User namespaces: ${YELLOW}DISABLED${NC}"
        fi
    else
        echo -e "  User namespaces: ${GREEN}ENABLED (no restriction)${NC}"
    fi

    echo ""
    echo "Usage: $0 [--network] [--pid] [--mount] [--user] [--all]"
    exit 0
fi

check_root() {
    if [[ $EUID -ne 0 ]]; then
        echo -e "${RED}This script requires root privileges for namespace configuration.${NC}"
        echo "Run with: sudo $0 $*"
        exit 1
    fi
}

setup_network_namespace() {
    echo -e "${CYAN}Setting up network namespace...${NC}"

    # Create network namespace
    if ! ip netns list | grep -q "^$NETNS_NAME"; then
        ip netns add "$NETNS_NAME"
        echo -e "  ${GREEN}[OK]${NC} Created namespace: $NETNS_NAME"
    else
        echo -e "  ${YELLOW}[SKIP]${NC} Namespace already exists: $NETNS_NAME"
    fi

    # Create bridge
    if ! ip link show "$BRIDGE_NAME" &>/dev/null; then
        ip link add name "$BRIDGE_NAME" type bridge
        ip addr add "$BRIDGE_IP" dev "$BRIDGE_NAME"
        ip link set "$BRIDGE_NAME" up
        echo -e "  ${GREEN}[OK]${NC} Created bridge: $BRIDGE_NAME"
    else
        echo -e "  ${YELLOW}[SKIP]${NC} Bridge already exists: $BRIDGE_NAME"
    fi

    # Create veth pair
    VETH_HOST="veth-noa-h"
    VETH_NS="veth-noa-n"

    if ! ip link show "$VETH_HOST" &>/dev/null; then
        ip link add "$VETH_HOST" type veth peer name "$VETH_NS"
        ip link set "$VETH_HOST" master "$BRIDGE_NAME"
        ip link set "$VETH_HOST" up
        ip link set "$VETH_NS" netns "$NETNS_NAME"

        # Configure namespace side
        ip netns exec "$NETNS_NAME" ip addr add 10.10.0.2/24 dev "$VETH_NS"
        ip netns exec "$NETNS_NAME" ip link set "$VETH_NS" up
        ip netns exec "$NETNS_NAME" ip link set lo up
        ip netns exec "$NETNS_NAME" ip route add default via 10.10.0.1

        echo -e "  ${GREEN}[OK]${NC} Created veth pair"
    else
        echo -e "  ${YELLOW}[SKIP]${NC} Veth pair already exists"
    fi

    # Enable IP forwarding
    if [[ $(cat /proc/sys/net/ipv4/ip_forward) != "1" ]]; then
        echo 1 > /proc/sys/net/ipv4/ip_forward
        echo -e "  ${GREEN}[OK]${NC} Enabled IP forwarding"
    fi

    # Setup NAT
    if ! iptables -t nat -C POSTROUTING -s 10.10.0.0/24 -j MASQUERADE &>/dev/null; then
        iptables -t nat -A POSTROUTING -s 10.10.0.0/24 -j MASQUERADE
        echo -e "  ${GREEN}[OK]${NC} Added NAT rule"
    fi

    echo -e "${GREEN}Network namespace configured.${NC}"
    echo -e "${GRAY}  Run commands in namespace: ip netns exec $NETNS_NAME <command>${NC}"
}

setup_mount_namespace() {
    echo -e "${CYAN}Setting up mount namespace directories...${NC}"

    mkdir -p "$NAMESPACE_DIR/mnt"
    mkdir -p "$NAMESPACE_DIR/mnt/proc"
    mkdir -p "$NAMESPACE_DIR/mnt/sys"
    mkdir -p "$NAMESPACE_DIR/mnt/dev"
    mkdir -p "$NAMESPACE_DIR/mnt/tmp"
    mkdir -p "$NAMESPACE_DIR/mnt/run"
    mkdir -p "$NAMESPACE_DIR/mnt/noa"

    # Create mount script
    cat > "$NAMESPACE_DIR/enter-mnt.sh" << 'EOF'
#!/bin/bash
# Enter mount namespace with isolated mounts
unshare --mount --propagation private /bin/bash
EOF
    chmod +x "$NAMESPACE_DIR/enter-mnt.sh"

    echo -e "  ${GREEN}[OK]${NC} Created mount namespace directories"
    echo -e "${GRAY}  Enter with: $NAMESPACE_DIR/enter-mnt.sh${NC}"
}

setup_pid_namespace() {
    echo -e "${CYAN}Setting up PID namespace...${NC}"

    # Create wrapper script
    cat > "$NAMESPACE_DIR/enter-pid.sh" << 'EOF'
#!/bin/bash
# Enter PID namespace (requires root or CAP_SYS_ADMIN)
exec unshare --pid --fork --mount-proc /bin/bash "$@"
EOF
    chmod +x "$NAMESPACE_DIR/enter-pid.sh"

    echo -e "  ${GREEN}[OK]${NC} Created PID namespace wrapper"
    echo -e "${GRAY}  Enter with: sudo $NAMESPACE_DIR/enter-pid.sh${NC}"
}

setup_user_namespace() {
    echo -e "${CYAN}Setting up user namespace...${NC}"

    # Check if user namespaces are enabled
    if [[ -f /proc/sys/kernel/unprivileged_userns_clone ]]; then
        local userns
        userns=$(cat /proc/sys/kernel/unprivileged_userns_clone)
        if [[ "$userns" != "1" ]]; then
            echo -e "  ${YELLOW}[WARN]${NC} Unprivileged user namespaces disabled"
            echo -e "  ${GRAY}Enable with: echo 1 | sudo tee /proc/sys/kernel/unprivileged_userns_clone${NC}"
        fi
    fi

    # Create UID/GID mapping files
    mkdir -p "$NAMESPACE_DIR/userns"

    # Create wrapper script
    cat > "$NAMESPACE_DIR/enter-userns.sh" << 'EOF'
#!/bin/bash
# Enter user namespace with UID/GID mapping
# Maps current user to root inside namespace

# Get current UID/GID
HOST_UID=$(id -u)
HOST_GID=$(id -g)

# Create new user namespace
exec unshare --user --map-root-user /bin/bash "$@"
EOF
    chmod +x "$NAMESPACE_DIR/enter-userns.sh"

    echo -e "  ${GREEN}[OK]${NC} Created user namespace wrapper"
    echo -e "${GRAY}  Enter with: $NAMESPACE_DIR/enter-userns.sh${NC}"
}

# Main
check_root

mkdir -p "$NAMESPACE_DIR"

if $SETUP_NETWORK; then
    setup_network_namespace
fi

if $SETUP_MOUNT; then
    setup_mount_namespace
fi

if $SETUP_PID; then
    setup_pid_namespace
fi

if $SETUP_USER; then
    setup_user_namespace
fi

echo ""
echo -e "${GREEN}Namespace setup complete.${NC}"

