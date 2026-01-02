# 01: Platform Overview

> **Last Updated:** 2025-12-19
> **Policy Level:** FR-001 (Foundation)
> **Status:** Active

## Purpose

This document provides the foundational overview of the NOA (Neural Orchestration Architecture) platform, defining its core principles, architecture, and operational philosophy.

## Platform Vision

NOA is a unified AI orchestration platform designed to:

1. **Federate AI Providers** - Seamlessly integrate cloud, local, and hybrid AI services
2. **Enable Multi-Modal Intelligence** - Support text, code, vision, and audio processing
3. **Provide Platform Agnosticism** - Run on any device: desktop, mobile, XR, embedded
4. **Ensure Privacy-First Design** - Local-first processing with optional cloud augmentation

## Core Principles

### 1. Single Source of Truth

All configuration, policies, and documentation derive from canonical sources within the NOA repository structure:

| Domain | Canonical Location |
|--------|-------------------|
| Environment Variables | `ai/shared/resources/policy/02-ENV_CANONICAL-VARS.md` |
| Package Management | `ai/shared/resources/policy/03-CONFIG_PACKAGE-MANAGER.md` |
| Tool Lifecycle | `ai/shared/resources/policy/03-CONFIG_TOOLS-LIFECYCLE.md` |
| Retention Policy | `ai/shared/resources/policy/04-GOVERNANCE_RETENTION.md` |

### 2. Portable by Design

NOA installs entirely within `${NOA_ROOT}` with no system-wide dependencies:

```
${NOA_ROOT}/
├── opt/              # Portable runtimes (Node, Rust, Python, Go)
├── bin/              # Wrapper scripts for tools
├── config/           # Configuration files
├── ai/               # AI providers and agents
├── pkg/              # Shared packages
└── data/             # Persistent data
```

### 3. Hardware Adaptive

The platform automatically detects and adapts to hardware capabilities:

| Environment Variable | Purpose |
|---------------------|---------|
| `NOA_DEVICE_CLASS` | Device type: `desktop`, `mobile`, `xr`, `embedded`, `server` |
| `NOA_COMPUTE_PROFILE` | Compute tier: `high`, `medium`, `low`, `minimal` |
| `NOA_PLATFORM` | OS: `windows`, `darwin`, `linux`, `android` |
| `NOA_ARCH` | Architecture: `x64`, `arm64`, `arm` |

### 4. One-Click Install

Bootstrap the entire platform with a single command:

```bash
# From any directory
node scripts/bootstrap.js

# With documentation initialization
node scripts/bootstrap.js --init-docs
```

## Architecture Overview

### Plane Architecture

NOA organizes its runtime into isolated "planes":

| Plane | Purpose | Location |
|-------|---------|----------|
| **Coordinator** | Orchestration and state management | `coordinator-plane/` |
| **Sandbox** | Isolated execution environments | `sandbox-plane/` |
| **Deployed** | Production services | `deployed-plane/` |

### Provider Hierarchy

AI providers are organized by deployment model:

```
ai/providers/
├── cloud/          # Cloud-only (OpenAI, Anthropic, Gemini)
├── local/          # Self-hosted (Ollama, llama.cpp)
├── hybrid/         # Mixed mode (Cursor, Claude Desktop)
└── ide/            # IDE-integrated (VS Code Copilot)
```

### Package Structure

Shared packages follow the monorepo pattern:

```
pkg/
├── ai-providers/   # AI provider interfaces
├── schemas/        # JSON schemas
├── services/       # Shared services
├── sys/            # System packages
│   ├── core/       # Rust core runtime
│   ├── ui/         # Web UI
│   └── desktop/    # Desktop app
└── tm-*            # Task management packages
```

## Compliance

This platform adheres to:

- **FR-001**: Foundation Requirements (this document)
- **FR-002**: Security Requirements (see SECURITY.md)
- **FR-003**: Privacy Requirements (local-first processing)

## Related Documents

- [Environment Variables](./02-ENV_CANONICAL-VARS.md)
- [Package Manager Policy](./03-CONFIG_PACKAGE-MANAGER.md)
- [Workspace Configuration](./03-CONFIG_WORKSPACE.md)
- [Tools Lifecycle](./03-CONFIG_TOOLS-LIFECYCLE.md)
- [Retention Policy](./04-GOVERNANCE_RETENTION.md)

## Governance

| Role | Responsibility |
|------|---------------|
| Platform Owner | Architecture decisions, breaking changes |
| Module Owners | Domain-specific policies |
| Contributors | Implementation within policy bounds |

---

*This document serves as the entry point for understanding the NOA platform. All other policy documents reference this foundation.*
