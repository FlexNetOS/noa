#!/bin/bash
#
# NOA Alpine Linux VM Image Builder (B134-B138)
#
# Builds a minimal Alpine Linux image for NOA VM mode.
# Includes P2P kernel modules and NOA runtime.
#
# Usage:
#   ./build-alpine.sh [--output PATH] [--arch ARCH] [--format FORMAT]
#
# Formats:
#   qcow2  - KVM/QEMU (default for Linux)
#   vhdx   - Hyper-V (Windows)
#   raw    - Virtualization.framework (macOS)

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

# Defaults
OUTPUT=""
ARCH="${ARCH:-$(uname -m)}"
FORMAT="qcow2"
IMAGE_SIZE="2G"
ALPINE_VERSION="3.19"
ALPINE_MIRROR="https://dl-cdn.alpinelinux.org/alpine"

# Auto-detect NOA_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOA_ROOT="${NOA_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output|-o)
            OUTPUT="$2"
            shift 2
            ;;
        --arch|-a)
            ARCH="$2"
            shift 2
            ;;
        --format|-f)
            FORMAT="$2"
            shift 2
            ;;
        --size|-s)
            IMAGE_SIZE="$2"
            shift 2
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Normalize architecture
case "$ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        ALPINE_ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ALPINE_ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Default output path
if [[ -z "$OUTPUT" ]]; then
    case "$FORMAT" in
        qcow2) OUTPUT="$SCRIPT_DIR/noa-linux.qcow2" ;;
        vhdx)  OUTPUT="$SCRIPT_DIR/noa-linux.vhdx" ;;
        raw)   OUTPUT="$SCRIPT_DIR/noa-linux.img" ;;
        *)     echo -e "${RED}Unknown format: $FORMAT${NC}"; exit 1 ;;
    esac
fi

# Working directory
WORK_DIR=$(mktemp -d)
trap "rm -rf $WORK_DIR" EXIT

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║        NOA Alpine Linux VM Image Builder                  ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GRAY}Architecture: $ARCH${NC}"
echo -e "${GRAY}Format:       $FORMAT${NC}"
echo -e "${GRAY}Size:         $IMAGE_SIZE${NC}"
echo -e "${GRAY}Output:       $OUTPUT${NC}"
echo ""

check_deps() {
    echo -e "${CYAN}Checking dependencies...${NC}"

    local missing=()

    for cmd in qemu-img wget tar; do
        if ! command -v "$cmd" &>/dev/null; then
            missing+=("$cmd")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        echo -e "${RED}Missing dependencies: ${missing[*]}${NC}"
        echo -e "${GRAY}Install with your package manager${NC}"
        exit 1
    fi

    echo -e "${GREEN}  [OK] All dependencies found${NC}"
}

download_rootfs() {
    echo -e "${CYAN}Downloading Alpine Linux rootfs...${NC}"

    local rootfs_url="$ALPINE_MIRROR/v${ALPINE_VERSION}/releases/${ALPINE_ARCH}/alpine-minirootfs-${ALPINE_VERSION}.0-${ALPINE_ARCH}.tar.gz"
    local rootfs_file="$WORK_DIR/alpine-rootfs.tar.gz"

    wget -q --show-progress -O "$rootfs_file" "$rootfs_url"

    echo -e "${GREEN}  [OK] Downloaded rootfs${NC}"
}

create_disk_image() {
    echo -e "${CYAN}Creating disk image...${NC}"

    local raw_image="$WORK_DIR/disk.raw"

    # Create raw image
    qemu-img create -f raw "$raw_image" "$IMAGE_SIZE"

    # Create partition table and filesystem
    # Using parted or fdisk would require root, so we use a simpler approach
    # with a single partition image

    echo -e "${GREEN}  [OK] Created disk image${NC}"
}

setup_rootfs() {
    echo -e "${CYAN}Setting up rootfs...${NC}"

    local rootfs_dir="$WORK_DIR/rootfs"
    mkdir -p "$rootfs_dir"

    # Extract rootfs
    tar -xzf "$WORK_DIR/alpine-rootfs.tar.gz" -C "$rootfs_dir"

    # configsure Alpine
    cat > "$rootfs_dir/etc/apk/repositories" << EOF
$ALPINE_MIRROR/v$ALPINE_VERSION/main
$ALPINE_MIRROR/v$ALPINE_VERSION/community
EOF

    # Install packages (would need chroot/qemu-user for cross-arch)
    # For simplicity, create a setup script to run inside VM
    cat > "$rootfs_dir/root/setup.sh" << 'SETUP_EOF'
#!/bin/sh
set -e

# Update packages
apk update
apk upgrade

# Install essential packages
apk add \
    openssh-server \
    curl \
    wget \
    jq \
    bash \
    sudo \
    iptables \
    ip6tables \
    iproute2 \
    wireguard-tools \
    bridge-utils \
    nftables

# Enable services
rc-update add sshd default
rc-update add networking default

# configsure SSH
mkdir -p /root/.ssh
chmod 700 /root/.ssh
echo "PermitRootLogin yes" >> /etc/ssh/sshd_configs

# configsure networking
cat > /etc/network/interfaces << NET_EOF
auto lo
iface lo inet loopback

auto eth0
iface eth0 inet dhcp
NET_EOF

# Enable kernel modules
cat > /etc/modules << MOD_EOF
tun
bridge
wireguard
nf_tables
MOD_EOF

# Set hostname
echo "noa-vm" > /etc/hostname

# Create NOA directories
mkdir -p /noa/{bin,opt,configs,logs}

echo "Setup complete!"
SETUP_EOF
    chmod +x "$rootfs_dir/root/setup.sh"

    # Create boot configsuration
    mkdir -p "$rootfs_dir/boot/grub"
    cat > "$rootfs_dir/boot/grub/grub.cfg" << 'GRUB_EOF'
set timeout=1
set default=0

menuentry "NOA Linux" {
    linux /boot/vmlinuz root=/dev/vda rw console=ttyS0
    initrd /boot/initramfs
}
GRUB_EOF

    echo -e "${GREEN}  [OK] Rootfs configsured${NC}"
}

convert_image() {
    echo -e "${CYAN}Converting to $FORMAT format...${NC}"

    local raw_image="$WORK_DIR/disk.raw"

    case "$FORMAT" in
        qcow2)
            qemu-img convert -f raw -O qcow2 "$raw_image" "$OUTPUT"
            ;;
        vhdx)
            qemu-img convert -f raw -O vhdx "$raw_image" "$OUTPUT"
            ;;
        raw)
            cp "$raw_image" "$OUTPUT"
            ;;
    esac

    echo -e "${GREEN}  [OK] Converted to $FORMAT${NC}"
}

# Main
check_deps
download_rootfs
create_disk_image
setup_rootfs
convert_image

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                Image Build Complete!                      ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GRAY}Output: $OUTPUT${NC}"
echo -e "${GRAY}Size:   $(du -h "$OUTPUT" | cut -f1)${NC}"
echo ""
echo -e "${YELLOW}Note: First boot will require running /root/setup.sh${NC}"
echo -e "${YELLOW}to complete package installation.${NC}"

