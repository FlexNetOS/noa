# NOA macOS VM Management (B131-B133)

## Overview

NOA VM management on macOS using Apple's Virtualization.framework.

**Requirements:**
- macOS 11 (Big Sur) or later
- Apple Silicon (M1/M2/M3) or Intel Mac with VT-x
- Xcode Command Line Tools

## Limitations (Darwin Kernel Differences)

1. **No nested virtualization on Apple Silicon** - VMs cannot run their own hypervisors
2. **Limited GPU passthrough** - No direct GPU access in VMs
3. **Network isolation** - NAT networking only, no bridged mode without special configuration
4. **File sharing** - VirtioFS available but requires specific kernel support in guest

## Quick Start

### Check Virtualization Support

```bash
# Check if hypervisor is supported
sysctl -n kern.hv_support
# Returns 1 if supported

# Check macOS version
sw_vers -productVersion
```

### Using the VM Manager

```bash
# Create a VM
./noa-vm.sh create --name noa-vm --image ~/noa-linux.img

# Start the VM
./noa-vm.sh start --name noa-vm

# Check status
./noa-vm.sh status --name noa-vm

# SSH into VM (once booted)
ssh -p 2222 root@localhost

# Stop the VM
./noa-vm.sh stop --name noa-vm

# Destroy the VM
./noa-vm.sh destroy --name noa-vm
```

## Image Requirements

The VM image must be:
- **Format**: RAW disk image (`.img` or `.raw`)
- **Architecture**: arm64 for Apple Silicon, x86_64 for Intel
- **Bootloader**: UEFI compatible
- **Kernel**: Linux with virtio drivers

### Building a Compatible Image

See `sys/kernel/images/build-alpine-macos.sh` for an automated build script.

## Network Configuration

Default configuration uses user-mode networking (NAT):
- Guest can access internet through host
- Port forwarding: `hostfwd=tcp::2222-:22` (SSH)
- No inbound connections without port forwarding

## Storage

VirtioFS is used for shared folders:
- Fast file sharing between host and guest
- Requires VirtioFS kernel module in guest

## Swift API Usage

For programmatic control, see `noa-vm.swift`:

```swift
import Virtualization

// Create VM configuration
let config = VZVirtualMachineConfiguration()
config.cpuCount = 2
config.memorySize = 512 * 1024 * 1024 // 512 MB

// Add storage
let diskURL = URL(fileURLWithPath: "noa-linux.img")
let disk = try VZDiskImageStorageDeviceAttachment(url: diskURL, readOnly: false)
let storage = VZVirtioBlockDeviceConfiguration(attachment: disk)
config.storageDevices = [storage]

// Add networking
let network = VZNATNetworkDeviceAttachment()
let netDevice = VZVirtioNetworkDeviceConfiguration()
netDevice.attachment = network
config.networkDevices = [netDevice]

// Create and start VM
let vm = VZVirtualMachine(configuration: config)
try await vm.start()
```

## Troubleshooting

### "Hypervisor not supported"

Ensure:
1. macOS 11+ is installed
2. Running on real hardware (not a VM)
3. SIP doesn't block hypervisor access

### "Permission denied"

Check entitlements:
```bash
codesign -d --entitlements :- /path/to/noa-vm
```

Should include `com.apple.security.virtualization`.

### VM won't boot

1. Verify image format is RAW
2. Check UEFI bootloader is present
3. Verify architecture matches (arm64 vs x86_64)

## References

- [Apple Virtualization.framework Documentation](https://developer.apple.com/documentation/virtualization)
- [Creating a Linux VM with Swift](https://developer.apple.com/documentation/virtualization/running_linux_in_a_virtual_machine)

