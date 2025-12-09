# NOA Desktop Application Hosting Architecture

**Status**: Design
**Created**: 2025-12-09
**Constitutional Reference**: §3.1 Self-Containment, §4.11 Kernel Independence

---

## Overview

NOA is designed to **host** desktop applications (ChatGPT Desktop, GitHub Desktop, Claude Desktop) within its isolation layer. This document describes the architecture for running GUI applications inside NOA's controlled environment.

---

## Hosted Desktop Applications

| Application | Type | Runtime | Data Path |
|-------------|------|---------|-----------|
| **ChatGPT Desktop** | Electron | Node.js + Chromium | `$NOA_DATA/apps/chatgpt/` |
| **GitHub Desktop** | Electron | Node.js + Chromium | `$NOA_DATA/apps/github-desktop/` |
| **Claude Desktop** | Electron | Node.js + Chromium | `$NOA_DATA/apps/claude/` |
| **Cursor IDE** | Electron | Node.js + Chromium | `$NOA_DATA/apps/cursor/` |
| **Docker Desktop** | Native + Electron | Go + Node.js | `$NOA_DATA/docker/` |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    NOA Application Layer                         │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  ChatGPT  │  Claude  │  GitHub  │  Cursor  │  Docker    │   │
│  │  Desktop  │  Desktop │  Desktop │   IDE    │  Desktop   │   │
│  └──────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│              NOA Desktop Containment Layer (NDCL)                │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  • Display forwarding (GPU-accelerated)                   │   │
│  │  • Network isolation/proxy                                │   │
│  │  • File system redirection ($XDG_* → $NOA_ROOT)          │   │
│  │  • Process isolation (sandbox/container)                  │   │
│  │  • Unified authentication (OAuth proxy)                   │   │
│  └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│               NOA Kernel Abstraction Layer (NKAL)                │
├─────────────────────────────────────────────────────────────────┤
│                    Host Kernel (Native/VM)                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Desktop Containment Layer (NDCL)

### 1. Display Forwarding

Run GUI applications inside NOA's isolation with GPU acceleration.

#### Windows (Hyper-V + GPU-PV)
```powershell
# Enable GPU partitioning for Hyper-V VM
Set-VMGpuPartitionAdapter -VMName "NOA-Desktop" -MinPartitionVRAM 1073741824

# Or use RDP for display
$NoaVm = Get-VM -Name "NOA-Desktop"
Enable-VMIntegrationService -VMName $NoaVm.Name -Name "Guest Service Interface"
```

#### Linux (X11/Wayland forwarding)
```bash
# X11 forwarding to container
docker run -e DISPLAY=$DISPLAY \
           -v /tmp/.X11-unix:/tmp/.X11-unix \
           --gpus all \
           noa-desktop:latest

# Or Wayland socket forwarding
docker run -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
           -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/$WAYLAND_DISPLAY \
           noa-desktop:latest
```

#### macOS (Virtualization + VNC)
```bash
# Use VNC for display forwarding from VM
./sys/kernel/macos/vm/noa-vm.sh --vnc-display :1
```

### 2. Network Isolation

All app network traffic routes through NOA's P2P layer.

```
┌───────────────────────────────────────────────────────────────┐
│                      Internet                                   │
└───────────────────────────┬───────────────────────────────────┘
                            │
┌───────────────────────────▼───────────────────────────────────┐
│                  NOA Network Proxy                              │
│  • Traffic inspection & logging                                │
│  • Rate limiting                                                │
│  • P2P routing (when available)                                 │
│  • OAuth token injection                                        │
└───────────────────────────┬───────────────────────────────────┘
                            │
┌───────────────────────────▼───────────────────────────────────┐
│            Desktop Apps (ChatGPT, Claude, GitHub)               │
└───────────────────────────────────────────────────────────────┘
```

### 3. File System Redirection

Redirect all application data to `noa_root`.

| App Standard Path | NOA Redirect |
|-------------------|--------------|
| `%APPDATA%\ChatGPT` | `$NOA_DATA/apps/chatgpt/` |
| `%APPDATA%\Claude` | `$NOA_DATA/apps/claude/` |
| `%LOCALAPPDATA%\GitHubDesktop` | `$NOA_DATA/apps/github-desktop/` |
| `~/.cursor-tuning/` | `$NOA_DATA/apps/cursor/tuning/` |
| `~/.docker/` | `$NOA_DATA/docker/` |

**Implementation**: Environment variable injection + bind mounts

```powershell
# Windows: Set environment before launch
$env:APPDATA = "$env:NOA_DATA\apps"
$env:LOCALAPPDATA = "$env:NOA_DATA\apps"
$env:USERPROFILE = "$env:NOA_ROOT\home"

# Launch with redirected paths
& "$env:NOA_OPT\apps\chatgpt\ChatGPT.exe"
```

```bash
# Linux/macOS: Bind mount + env
docker run \
    -v $NOA_DATA/apps/chatgpt:/home/user/.config/ChatGPT \
    -e HOME=/home/user \
    -e XDG_CONFIG_HOME=/home/user/.config \
    -e XDG_DATA_HOME=/home/user/.local/share \
    noa-chatgpt:latest
```

### 4. Process Isolation

Isolate desktop apps from the host system.

#### Windows
```powershell
# Use AppContainer or Job Objects
$jobObject = [System.Diagnostics.Process]::Start($psi).JobObject
$jobObject.SetLimits(@{
    ActiveProcessLimit = 100
    MemoryLimit = 4GB
    WorkingSetLimit = 2GB
})
```

#### Linux
```bash
# Use bubblewrap for sandboxing
bwrap \
    --bind $NOA_DATA/apps/chatgpt /home/user/.config/ChatGPT \
    --ro-bind /usr /usr \
    --dev /dev \
    --proc /proc \
    --unshare-all \
    --share-net \
    -- /opt/ChatGPT/chatgpt
```

### 5. Unified Authentication

Proxy OAuth flows through NOA's authentication service.

```
┌──────────────────────────────────────────────────────────────┐
│                  Desktop App (ChatGPT)                         │
│                         │                                      │
│                         ▼                                      │
│               OAuth Login Request                              │
└─────────────────────────┬────────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────────┐
│              NOA OAuth Proxy Service                           │
│  • Intercepts auth redirects                                   │
│  • Stores tokens in NOA credential store                       │
│  • Injects tokens into app config                              │
│  • Token refresh management                                    │
└─────────────────────────┬────────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────────┐
│                   OpenAI / Anthropic / GitHub                  │
└──────────────────────────────────────────────────────────────┘
```

---

## Installation Process

### 1. Download to NOA Containment

```powershell
# Download ChatGPT Desktop to NOA opt
Invoke-WebRequest -Uri "https://chatgpt.com/download" -OutFile "$env:NOA_OPT/apps/chatgpt-installer.exe"

# Install to NOA (non-system install)
& "$env:NOA_OPT/apps/chatgpt-installer.exe" /PORTABLE /DIR="$env:NOA_OPT/apps/chatgpt"
```

### 2. Create Launch Wrapper

```powershell
# N:\noa\bin\chatgpt.cmd
@echo off
setlocal

:: Redirect app data to NOA
set APPDATA=%NOA_DATA%\apps
set LOCALAPPDATA=%NOA_DATA%\apps
set USERPROFILE=%NOA_ROOT%\home

:: Configure network proxy (optional)
set HTTP_PROXY=http://localhost:8888
set HTTPS_PROXY=http://localhost:8888

:: Launch with isolation
start "" "%NOA_OPT%\apps\chatgpt\ChatGPT.exe" %*
```

### 3. Register with NOA Desktop Manager

```json
// config/desktop-apps.json
{
  "apps": {
    "chatgpt": {
      "name": "ChatGPT Desktop",
      "installPath": "${NOA_OPT}/apps/chatgpt",
      "executable": "ChatGPT.exe",
      "dataPath": "${NOA_DATA}/apps/chatgpt",
      "networkIsolation": true,
      "gpuEnabled": true,
      "autoUpdate": false
    },
    "claude": {
      "name": "Claude Desktop",
      "installPath": "${NOA_OPT}/apps/claude",
      "executable": "Claude.exe",
      "dataPath": "${NOA_DATA}/apps/claude",
      "networkIsolation": true,
      "gpuEnabled": true,
      "autoUpdate": false
    },
    "github-desktop": {
      "name": "GitHub Desktop",
      "installPath": "${NOA_OPT}/apps/github-desktop",
      "executable": "GitHubDesktop.exe",
      "dataPath": "${NOA_DATA}/apps/github-desktop",
      "networkIsolation": false,
      "gpuEnabled": false,
      "autoUpdate": false
    }
  }
}
```

---

## Directory Structure

```
noa_root/
├── opt/
│   └── apps/                    # Desktop app installations
│       ├── chatgpt/             # ChatGPT Desktop
│       ├── claude/              # Claude Desktop
│       ├── github-desktop/      # GitHub Desktop
│       └── cursor/              # Cursor IDE (if portable)
├── data/
│   ├── apps/                    # App data (redirected from system)
│   │   ├── chatgpt/
│   │   │   ├── config/
│   │   │   ├── cache/
│   │   │   └── logs/
│   │   ├── claude/
│   │   └── github-desktop/
│   └── docker/                  # Docker data
│       ├── images/
│       └── containers/
├── bin/
│   ├── chatgpt.cmd              # ChatGPT launcher wrapper
│   ├── claude-desktop.cmd       # Claude Desktop launcher
│   └── github-desktop.cmd       # GitHub Desktop launcher
└── sys/
    └── desktop/                 # Desktop containment layer
        ├── ndcl/                # NDCL implementation
        ├── proxy/               # Network proxy service
        └── auth/                # OAuth proxy service
```

---

## Platform-Specific Notes

### Windows

- **Portable Installation**: Most Electron apps support `/PORTABLE` flag
- **Data Redirection**: Use environment variables + symbolic links
- **GPU Access**: Native GPU access in sandboxed mode
- **Network Proxy**: Use WinDivert or system proxy settings

### Linux

- **Flatpak/AppImage**: Many apps available as portable formats
- **Data Redirection**: XDG directories + bind mounts
- **GPU Access**: NVIDIA Container Toolkit or Mesa passthrough
- **Network Proxy**: iptables/nftables + transparent proxy

### macOS

- **App Bundles**: Can be relocated to `$NOA_OPT/apps/`
- **Data Redirection**: `$HOME` override + `defaults write` for app-specific paths
- **GPU Access**: Metal API passthrough in VM (limited)
- **Network Proxy**: `scutil` proxy configuration

---

## Security Considerations

1. **Sandboxing**: All apps run in isolated process space
2. **Network Filtering**: Traffic inspected at NOA proxy
3. **Credential Isolation**: OAuth tokens stored in NOA credential vault
4. **Update Control**: Auto-updates disabled; updates managed by NOA
5. **Data Isolation**: No access to system files outside `noa_root`

---

## Tasks to Implement

The following tasks need to be added to `tasks.md`:

1. **T-NDCL-001**: Create `sys/desktop/ndcl/` directory structure
2. **T-NDCL-002**: Implement desktop app launcher wrappers for Windows
3. **T-NDCL-003**: Implement desktop app launcher wrappers for Linux
4. **T-NDCL-004**: Implement desktop app launcher wrappers for macOS
5. **T-NDCL-005**: Create `config/desktop-apps.json` registry
6. **T-NDCL-006**: Implement network proxy service for desktop apps
7. **T-NDCL-007**: Implement OAuth proxy service
8. **T-NDCL-008**: Create installation scripts for ChatGPT Desktop
9. **T-NDCL-009**: Create installation scripts for Claude Desktop
10. **T-NDCL-010**: Create installation scripts for GitHub Desktop
11. **T-NDCL-011**: Implement GPU passthrough for container/VM modes
12. **T-NDCL-012**: Create X11/Wayland forwarding for Linux container mode
13. **T-NDCL-013**: Create RDP/VNC forwarding for Windows VM mode
14. **T-NDCL-014**: Test desktop app data isolation

---

## References

- [Electron Portable Mode](https://www.electronjs.org/docs/latest/api/app#appgetpath-name)
- [bubblewrap](https://github.com/containers/bubblewrap)
- [Windows AppContainer](https://docs.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)
- [GPU Passthrough](https://wiki.archlinux.org/title/VFIO)

