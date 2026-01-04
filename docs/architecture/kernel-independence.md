# NOA Kernel Independence Architecture

## Overview

NOA's Kernel Independence Layer (NKAL) enables operation across different isolation modes, from native execution to full VM isolation. This document describes the architecture, modes, and platform-specific implementations.

## Kernel Selection Precedence (FR-159, FR-160)

**Priority Order**: VM > Container > Sandbox > Native

| Priority | Mode | Isolation Level | Use Case |
|----------|------|-----------------|----------|
| 1 (Highest) | **VM** | Maximum | Untrusted code, security-critical operations |
| 2 | **Container** | High | Service deployment, reproducible builds |
| 3 | **Sandbox** | Medium | Quick testing, untrusted scripts |
| 4 (Default) | **Native** | None | Development, maximum performance |

**Selection Logic**:
- Default: Native mode (best performance)
- Automatic escalation: Security requirements trigger higher isolation
- Fallback chain: VM → Container → Sandbox → Native (if higher modes unavailable)
- Mode switch command: `noa-kernel-params set kernel_mode {native|vm|container|sandbox}`

## Design Principles

Per NOA Constitution §4.11:

1. **Portability**: Run on any platform without modification
2. **Isolation**: Support multiple isolation levels for security
3. **Performance**: Minimize overhead in native mode
4. **Fallback**: Graceful degradation when features unavailable

## Isolation Modes

### 1. Native Mode

Direct execution on the host operating system.

**Characteristics:**
- Zero overhead
- Full hardware access
- Shared kernel with host
- Fastest performance

**Use Cases:**
- Development
- Trusted environments
- Performance-critical workloads

### 2. VM Mode

Full isolation via hypervisor.

**Platforms:**
| Platform | Hypervisor | Image Format |
|----------|------------|--------------|
| Windows | Hyper-V | VHDX |
| Linux | KVM/QEMU | QCOW2 |
| macOS | Virtualization.framework | RAW/DMG |

**Characteristics:**
- Complete isolation
- Custom kernel
- Dedicated resources
- < 3 second boot time (target)

**Use Cases:**
- Untrusted code execution
- Security-sensitive operations
- Cross-platform testing

### 3. Container Mode

Lightweight isolation via containers.

**Runtimes:**
- Docker (rootless preferred)
- Podman (rootless by default)
- containerd

**Characteristics:**
- Shared kernel
- Process-level isolation
- Fast startup
- Efficient resource usage

**Use Cases:**
- Service deployment
- Reproducible builds
- CI/CD pipelines

### 4. Sandbox Mode

Minimal isolation for testing.

**Platforms:**
| Platform | Technology |
|----------|------------|
| Windows | Windows Sandbox |
| Linux | bubblewrap, firejail |
| macOS | App Sandbox |

**Characteristics:**
- Ephemeral environment
- Quick setup/teardown
- Limited persistence

**Use Cases:**
- Quick testing
- Untrusted file inspection
- Isolated experiments

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      NOA Application                             │
├─────────────────────────────────────────────────────────────────┤
│                      NKAL Interface                              │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  detect_capabilities() → PlatformCapabilities            │   │
│  │  set_mode(KernelMode) → Result<()>                       │   │
│  │  initialize() → Result<()>                               │   │
│  │  best_available_mode() → KernelMode                      │   │
│  └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                      Mode Drivers                                │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐ │
│  │   Native   │  │     VM     │  │ Container  │  │ Sandbox  │ │
│  │   Driver   │  │   Driver   │  │   Driver   │  │  Driver  │ │
│  └────────────┘  └────────────┘  └────────────┘  └──────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                   Platform Adapters                              │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Windows: Hyper-V, Windows Sandbox, Docker Desktop         │ │
│  │  Linux: KVM/QEMU, Docker, Podman, bubblewrap               │ │
│  │  macOS: Virtualization.framework, Docker Desktop           │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Platform-Specific Details

### Windows

#### Hyper-V (VM Mode)

**Requirements:**
- Windows 10/11 Pro, Enterprise, or Education
- Hardware virtualization enabled in BIOS
- Hyper-V feature enabled

**Image:**
- Format: VHDX (Generation 2)
- OS: Alpine Linux (minimal)
- Size: < 500MB

**Scripts:**
- `sys/kernel/windows/hyperv/noa-vm.ps1` - VM management
- `sys/kernel/windows/hyperv/create-vm.ps1` - VM creation
- `sys/kernel/windows/hyperv/start-vm.ps1` - VM startup

#### Windows Sandbox (Sandbox Mode)

**Requirements:**
- Windows 10/11 Pro, Enterprise
- Containers-DisposableClientVM feature enabled

**configsuration:**
- `sys/kernel/windows/sandbox/noa.wsb` - Sandbox profile

### Linux

#### KVM/QEMU (VM Mode)

**Requirements:**
- KVM kernel module loaded
- `/dev/kvm` accessible
- QEMU installed

**Image:**
- Format: QCOW2
- OS: Alpine Linux (minimal)
- Size: < 500MB

**Scripts:**
- `sys/kernel/linux/vm/noa-vm.sh` - VM management
- `sys/kernel/linux/vm/create-vm.sh` - VM creation

#### Namespace Isolation

**Scripts:**
- `sys/kernel/linux/namespaces/setup.sh` - Namespace configsuration

### macOS

#### Virtualization.framework (VM Mode)

**Requirements:**
- macOS 11+ (Big Sur or later)
- Apple Silicon or Intel with VT-x

**Image:**
- Format: RAW or DMG
- OS: Alpine Linux (minimal)

**Scripts:**
- `sys/kernel/macos/vm/noa-vm.swift` - VM management (Swift)

## VM Image Specification

### Base Image Requirements

1. **Minimal Linux Distribution** (Alpine recommended)
2. **Required Packages:**
   - P2P kernel modules (tun, bridge, wireguard)
   - Network filtering (nf_tables)
   - SSH server (for management)
3. **NOA Runtime:**
   - NOA binaries pre-installed
   - configsuration from host
4. **Performance:**
   - Boot time < 3 seconds
   - Memory footprint < 256MB base

### Image Build Process

```bash
# Build Alpine-based image
./sys/kernel/images/build-alpine.sh

# Customize for platform
./sys/kernel/images/customize-windows.sh  # VHDX
./sys/kernel/images/customize-linux.sh    # QCOW2
./sys/kernel/images/customize-macos.sh    # RAW/DMG
```

## configsuration

### Environment Variables

```bash
# Set kernel mode
export NOA_KERNEL_MODE=native|vm|container|sandbox

# VM-specific
export NOA_VM_IMAGE=/path/to/image.qcow2
export NOA_VM_MEMORY=512M
export NOA_VM_CPUS=2

# Container-specific
export NOA_CONTAINER_IMAGE=noa-runtime:latest
export NOA_CONTAINER_RUNTIME=podman|docker
```

### configsuration File

```json
{
  "kernel": {
    "mode": "auto",
    "preferredMode": "vm",
    "fallbackMode": "container",
    "autoDetect": true
  },
  "vm": {
    "imagePath": "${NOA_ROOT}/sys/kernel/images/noa-linux.qcow2",
    "memory": "512M",
    "cpus": 2,
    "bootTimeout": 30
  },
  "container": {
    "image": "noa-runtime:latest",
    "runtime": "auto"
  }
}
```

## API Reference

### Rust API

```rust
use noa::kernel::{Nkal, KernelMode, Nkalconfigs};

// Auto-detect and initialize
let nkal = Nkal::new()?;

// Check capabilities
let caps = nkal.capabilities();
println!("Hypervisor: {:?}", caps.hypervisor);
println!("Best mode: {:?}", nkal.best_available_mode());

// Set specific mode
let mut nkal = Nkal::with_configs(Nkalconfigs {
    preferred_mode: KernelMode::VM,
    auto_detect: false,
    ..Default::default()
})?;

nkal.initialize()?;
```

### CLI Commands

```bash
# Check capabilities
noa kernel status

# Set mode
noa kernel set-mode vm

# Get current mode
noa kernel get-mode

# Initialize with specific mode
noa kernel init --mode=container
```

## Testing

### Unit Tests

```bash
cargo test -p noa-kernel
```

### Integration Tests

```bash
# Test VM boot (requires hypervisor)
./tests/kernel/test-vm-boot.sh

# Test container mode
./tests/kernel/test-container.sh

# Full isolation test
./tests/kernel/test-isolation.sh
```

### CI/CD

See `.github/workflows/kernel-tests.yml` for automated testing across platforms.

## Performance Benchmarks

| Mode | Boot Time | Memory Overhead | CPU Overhead |
|------|-----------|-----------------|--------------|
| Native | 0ms | 0MB | 0% |
| VM | < 3s | 256MB | 5-10% |
| Container | < 1s | 50MB | 2-5% |
| Sandbox | < 5s | 100MB | 5% |

## Troubleshooting

### Hyper-V Not Detected

```powershell
# Enable Hyper-V
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
```

### KVM Permission Denied

```bash
# Add user to kvm group
sudo usermod -aG kvm $USER
# Logout and login again
```

### Container Runtime Not Found

```bash
# Install Podman (recommended)
sudo apt install podman

# Or Docker
sudo apt install docker.io
sudo usermod -aG docker $USER
```

## References

- [NOA Constitution §3.11](../../CONSTITUTION.md#311-kernel-independence)
- [Hyper-V Documentation](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/)
- [KVM Documentation](https://www.linux-kvm.org/page/Documents)
- [Apple Virtualization.framework](https://developer.apple.com/documentation/virtualization)

