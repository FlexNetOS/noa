# NOA Containment Architecture

Per **NOA Constitution §3.1**, all dependencies should reside within `noa_root`. NOA is designed to **host** desktop applications within its isolation layer.

---

## Overview

NOA hosts desktop applications through the **NOA Desktop Containment Layer (NDCL)**. This provides:
- Data isolation (all app data in `noa_root`)
- Network control (traffic through NOA proxy)
- Process sandboxing
- Unified authentication

See [Desktop App Hosting Architecture](./desktop-app-hosting.md) for full details.

---

## Hosted Desktop Applications

These applications run **inside NOA's containment layer**:

| Application | Status | Install Location | Data Location |
|-------------|--------|------------------|---------------|
| ChatGPT Desktop | 🔄 Pending | `opt/apps/chatgpt/` | `data/apps/chatgpt/` |
| Claude Desktop | 🔄 Pending | `opt/apps/claude/` | `data/apps/claude/` |
| GitHub Desktop | 🔄 Pending | `opt/apps/github-desktop/` | `data/apps/github-desktop/` |

**Launch via NOA wrappers**:
```powershell
N:\noa\bin\chatgpt.cmd        # ChatGPT Desktop (contained)
N:\noa\bin\claude-desktop.cmd # Claude Desktop (contained)
N:\noa\bin\github-desktop.cmd # GitHub Desktop (contained)
```

---

## Kernel-Required Applications

These applications require **kernel-level integration** and are architectural requirements:

### 1. Cursor IDE

**Status**: ✅ Host Application (Kernel-Required)

**Reason**: Cursor is the primary NOA development IDE and requires:
- System-level file access
- Process spawning capabilities
- IDE extension system integration
- Window management (Electron)

**Location**: `C:\Program Files\Cursor\` (system install)
**Data**: Can be redirected to `N:\noa\data\apps\cursor\`
**Install Script**: `scripts/bootstrap/installers/dev-tools/cursor.ps1`

### 2. VS Code IDE

**Status**: ✅ Host Application (Kernel-Required)

**Reason**: VS Code provides secondary/alternative IDE environment:
- System-level file access
- Process spawning capabilities
- IDE extension system integration (Copilot, extensions)
- Window management (Electron)

**Location**: `C:\Program Files\Microsoft VS Code\` (system install)
**Data**: Can be redirected to `N:\noa\data\apps\vscode\`
**Install Script**: `scripts/bootstrap/installers/dev-tools/vscode.ps1`

### 3. Docker Desktop

**Status**: ✅ Host Application (Kernel-Required)

**Reason**: Docker requires kernel-level virtualization:
- WSL2 backend (Windows)
- Hyper-V isolation
- Linux kernel access for containers
- Network namespace management

**Location**: `C:\Program Files\Docker\` (system install)
**Data**: Can be redirected to `N:\noa\data\docker\`
**Install Script**: `scripts/bootstrap/installers/dev-tools/docker.ps1`

---

## Fully Contained Tools

These tools are fully contained within `noa_root`:

| Tool | Location | Type |
|------|----------|------|
| PowerShell | `opt/powershell/` | Runtime |
| Node.js | `opt/node/` | Runtime |
| Go | `opt/go/` | Runtime |
| Python | `opt/python/` | Runtime |
| Rust | `opt/rust/` | Runtime |
| Codex CLI | `bin/codex.cmd` | AI Provider |
| Claude Code | `bin/claude.cmd` | AI Provider |
| gh CLI | `bin/gh.exe` | Tool |
| Abacus CLI | `bin/abacusai.cmd` | AI Provider |

---

## CLI Alternatives

For headless/automated workflows, use CLI alternatives:

| Desktop App | CLI Alternative | When to Use |
|-------------|-----------------|-------------|
| ChatGPT Desktop | `codex` | Automated code generation |
| Claude Desktop | `claude` | CLI-based workflows |
| GitHub Desktop | `gh` | Git automation, CI/CD |

---

## Containment Decision Tree

```
Is it a desktop application?
├─ YES → Install to $NOA_OPT/apps/, launch via NDCL wrapper
│        (ChatGPT Desktop, Claude Desktop, GitHub Desktop)
│
└─ NO → Is it a runtime dependency?
         ├─ YES → Must be in $NOA_OPT/
         │        (Node, Python, Rust, Go, etc.)
         │
         └─ NO → Does it require kernel access?
                  ├─ YES → ✅ System install (approved exception)
                  │        (Cursor, Docker)
                  │
                  └─ NO → Install to $NOA_BIN/ or $NOA_OPT/
```

---

## Data Redirection

All application data is redirected to `noa_root`:

| System Path | NOA Path |
|-------------|----------|
| `%APPDATA%\ChatGPT` | `$NOA_DATA/apps/chatgpt/` |
| `%APPDATA%\Claude` | `$NOA_DATA/apps/claude/` |
| `%LOCALAPPDATA%\GitHubDesktop` | `$NOA_DATA/apps/github-desktop/` |
| `~/.cursor-tuning/` | `$NOA_DATA/apps/cursor/tuning/` |
| `~/.docker/` | `$NOA_DATA/docker/` |

---

## Implementation Status

| Component | Status | Task |
|-----------|--------|------|
| NDCL Framework | 🔄 Pending | T-NDCL-001 |
| ChatGPT Containment | 🔄 Pending | T-NDCL-008 |
| Claude Containment | 🔄 Pending | T-NDCL-009 |
| GitHub Containment | 🔄 Pending | T-NDCL-010 |
| Network Proxy | 🔄 Pending | T-NDCL-006 |
| OAuth Proxy | 🔄 Pending | T-NDCL-007 |

---

## References

- [Desktop App Hosting Architecture](./desktop-app-hosting.md)
- [Kernel Independence Architecture](./kernel-independence.md)
- [NOA Constitution §3.1](../../CONSTITUTION.md)

---

## Audit Date

Last audited: 2025-12-09

