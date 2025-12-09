# NOA Kernel Independence Strategy

**Status**: Design
**Last Updated**: 2025-12-08
**Spec**: 001-noa-seed-foundation
**Constitutional Reference**: §4.11 Kernel Independence Policy

---

## Overview

NOA requires the ability to operate independently of the host operating system kernel. This document outlines the strategy for achieving kernel independence across all platforms.

---

## Goals

1. **Primary Goal**: Run NOA with full functionality using the host OS kernel
2. **Secondary Goal**: Run NOA independently in its own isolated environment
3. **Ultimate Goal**: NOA as a standalone OS that can boot directly on hardware

---

## Platform Strategies

### Windows

#### Level 1: Windows Kernel (Default)
- Use Windows kernel for all operations
- Leverage Windows APIs, services, and networking
- Full integration with Windows ecosystem

#### Level 2: Hyper-V Isolation
- Run NOA in a lightweight Hyper-V VM
- Custom Linux kernel inside VM
- Near-native performance with hardware virtualization
- **Implementation**: `sys/kernel/hyperv/noa-vm.ps1`

```powershell
# Create NOA VM with minimal Linux kernel
New-VM -Name "NOA-Kernel" -MemoryStartupBytes 4GB -Generation 2
Set-VMProcessor -VMName "NOA-Kernel" -Count 4
Add-VMHardDiskDrive -VMName "NOA-Kernel" -Path "$NOA_ROOT/sys/kernel/noa-disk.vhdx"
```

#### Level 3: UEFI Boot (Future)
- NOA as a UEFI application
- Boots before Windows
- Bare-metal access when needed

### Linux

#### Level 1: Linux Kernel (Default)
- Use host Linux kernel
- Leverage kernel namespaces for isolation
- Full access to kernel parameters and modules

#### Level 2: Container Isolation
- Run NOA in rootless containers
- Custom kernel via `--privileged` mode
- OCI-compatible deployment

#### Level 3: Custom Kernel (Future)
- NOA-optimized Linux kernel
- Custom modules for P2P networking
- Minimal boot with initramfs

### macOS

#### Level 1: macOS Kernel (Default)
- Use Darwin/XNU kernel
- Leverage macOS APIs and networking
- Sandbox-compliant operation

#### Level 2: Virtualization Framework
- Run NOA in Apple Virtualization.framework VM
- Linux kernel inside VM
- Apple Silicon native support

---

## Kernel Components

### NOA Kernel Abstraction Layer (NKAL)

The NKAL provides a unified interface regardless of the underlying kernel:

```
┌─────────────────────────────────────────┐
│              NOA Applications           │
├─────────────────────────────────────────┤
│         NOA Kernel Abstraction Layer    │
├────────────┬────────────┬───────────────┤
│  Windows   │   Linux    │    macOS      │
│   Kernel   │   Kernel   │    Kernel     │
│   (Native/ │  (Native/  │   (Native/    │
│   Hyper-V) │  Custom)   │   VM)         │
└────────────┴────────────┴───────────────┘
```

### NKAL Components

| Component | Windows | Linux | macOS |
|-----------|---------|-------|-------|
| Process Isolation | Job Objects / Hyper-V | Namespaces / cgroups | Sandbox / VM |
| Network Stack | WinSock / TAP | Socket / TUN | Socket / utun |
| File System | NTFS / VHD | ext4 / overlay | APFS / dmg |
| Memory | VirtualAlloc / VM | mmap / hugepages | mmap / VM |
| IPC | Named Pipes / VM | Unix Sockets / VM | XPC / VM |

---

## Implementation Phases

### Phase 1: Unified Scripts (Current)
- All scripts available on all platforms
- Platform detection and adaptation
- Common interface, platform-specific implementation

### Phase 2: NKAL Implementation
- Create `sys/core/src/kernel/` abstraction
- Rust-based kernel interface
- Platform-specific backends

### Phase 3: VM-Based Independence
- Lightweight VM images for each platform
- Custom Linux kernel for NOA
- Shared file system between host and VM

### Phase 4: Bare-Metal Support (Future)
- UEFI bootloader for NOA
- Minimal OS layer
- Direct hardware access

---

## Directory Structure

```
sys/
├── kernel/
│   ├── abstraction/       # NKAL interface definitions
│   ├── windows/           # Windows kernel interface
│   │   ├── hyperv/        # Hyper-V VM management
│   │   ├── drivers/       # TAP, WinDivert, etc.
│   │   └── native/        # Direct Windows API
│   ├── linux/             # Linux kernel interface
│   │   ├── modules/       # Kernel modules
│   │   ├── namespaces/    # Namespace management
│   │   └── custom/        # Custom kernel builds
│   ├── macos/             # macOS kernel interface
│   │   ├── vm/            # Virtualization.framework
│   │   └── native/        # Darwin API
│   ├── images/            # Pre-built VM images
│   │   ├── noa-linux.qcow2
│   │   ├── noa-linux.vhdx
│   │   └── noa-linux.dmg
│   └── params/            # Kernel parameters
│       └── current.json
└── namespace/             # Isolation runtime
```

---

## VM Image Specifications

### NOA Linux VM Image

- **Base**: Alpine Linux (minimal)
- **Kernel**: 6.6 LTS with P2P modules
- **Size**: < 500MB compressed
- **Boot time**: < 3 seconds

### Included Components

1. **Kernel modules**:
   - `tun` - P2P tunneling
   - `bridge` - Network bridging
   - `nf_tables` - Firewall/NAT
   - `wireguard` - Encrypted P2P

2. **Userspace**:
   - BusyBox utilities
   - NOA runtime binaries
   - P2P networking stack

### Building the Image

```bash
# Build NOA Linux image
./sys/kernel/build-image.sh --target all

# Output:
#   sys/kernel/images/noa-linux.qcow2  (QEMU/KVM)
#   sys/kernel/images/noa-linux.vhdx   (Hyper-V)
#   sys/kernel/images/noa-linux.dmg    (macOS VM)
```

---

## Configuration

### Enable Kernel Independence Mode

```powershell
# Windows - Enable Hyper-V isolation
.\scripts\noa-kernel-params.ps1 -Action set -Param kernel_mode -Value "hyperv"

# Launch NOA in isolated VM
.\scripts\noa.ps1 -Mode Isolated
```

```bash
# Linux - Enable container isolation
./scripts/noa-kernel-params set kernel_mode container

# Launch NOA in namespace
./scripts/noa-namespace run ./bin/noa-server
```

---

## Security Considerations

1. **VM Escape Prevention**: Minimal attack surface in VM images
2. **Memory Isolation**: Hardware-enforced separation
3. **Network Isolation**: Virtual network adapters only
4. **File System**: Copy-on-write with integrity checks

---

## Performance

| Mode | Overhead | Use Case |
|------|----------|----------|
| Native | 0% | Development, trusted environments |
| Container | 2-5% | Production, multi-tenant |
| VM | 5-15% | Maximum isolation, untrusted code |
| Bare-metal | 0% | Dedicated NOA hardware |

---

## Constitutional Compliance

This document implements:
- **§3.1 Self-Contained & Autonomous**: "no shared kernel dependency"
- **§4.8 Cross-Platform Adaptive Deployment**: All platforms supported
- **§4.11 Kernel Independence Policy**: Full implementation strategy

---

## Roadmap

- [ ] **Q1**: Complete unified scripts (all platforms)
- [ ] **Q2**: NKAL Rust implementation
- [ ] **Q3**: VM image builder
- [ ] **Q4**: Hyper-V/Virtualization.framework integration
- [ ] **Future**: UEFI bootloader research

---

## References

- [Hyper-V Architecture](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/)
- [Linux Namespaces](https://man7.org/linux/man-pages/man7/namespaces.7.html)
- [Apple Virtualization Framework](https://developer.apple.com/documentation/virtualization)
- [UEFI Specification](https://uefi.org/specifications)

