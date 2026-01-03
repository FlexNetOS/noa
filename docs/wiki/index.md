# NOA Wiki

This wiki provides conceptual documentation and reference material for the NOA platform.

> **Auto-Generated Documentation**: This wiki can be automatically generated and updated using the Litho documentation generator.
> Run `noa wiki generate-full` for complete regeneration or `noa wiki generate-incremental` for changed files only.
> Manual edits wrapped in `<!-- provider:add-manual-edit -->` markers are preserved during regeneration.

## Overview

NOA (Neural Orchestration Architecture) is an AI-native operating system designed for autonomous agent orchestration, local-first computation, and total memory sovereignty.

## Quick Start

```bash
# Generate full documentation
noa wiki generate-full

# Generate only for changed files
noa wiki generate-incremental

# Check generation status
noa wiki status

# Validate documentation
noa wiki validate
```

## Core Concepts

### Constitutional Authority
NOA operates under a constitutional framework defined in [CONSTITUTION.md](../../CONSTITUTION.md). All agents and systems must comply with constitutional requirements.

### Agent System
The agent system provides:
- **Commander-Chief**: Executive orchestrator for task decomposition
- **File-IO Agent**: File system operations with sandboxing
- **Terminal Agent**: Secure command execution with whitelisting
- **RAG Agent**: Retrieval-augmented generation for knowledge access

See [Agent Documentation](../agents/AGENT.md) for details.

### Microkernel Architecture
NOA follows a microkernel design principle:
- Minimal trusted core in `sys/`
- Tools exposed via Model Context Protocol (MCP)
- Content-addressable storage for reproducibility
- Sandboxed execution for all tasks

### Data Sovereignty
All data remains under user control:
- Local SQLite database (24 entities)
- Local inference via llama.cpp
- No external dependencies for core functionality
- P2P networking for distributed compute (planned)

## Navigation

### Reference
- [Architecture](../architecture/) — System design documents
- [Guides](../guides/) — How-to guides
- [Agents](../agents/) — Agent system documentation
- [ML DevOps](../ml-devops/) — ML platform documentation

### Codebase & Crates
- [Codebase Map](codebase_map.md) — Full file listing of the repository
- [Crate Plans](crates/index.md) — Detailed plans and documentation for system crates
- [Internal Crates](internal-crates/) — Documentation for internal crate dependencies

### Generated Documentation
- [API Reference](generated/api/) — Auto-generated API documentation
- [Crate Docs](generated/crates/) — Rustdoc for all crates
- [Diagrams](generated/diagrams/) — Architecture diagrams (Mermaid)

## Documentation Generation

This wiki uses **Litho** (wiki-rs) for AI-powered documentation generation.

### Multi-Pass Pipeline
| Pass | Subagent | Purpose |
|------|----------|---------|
| 1 | RustCrateScannerAgent | Structure analysis (sequential) |
| 2 | RustClippyAgent | Code analysis (parallel) |
| 3 | RustDocAgent | Doc generation (parallel) |
| 4 | RustFmtAgent | Validation (parallel) |

### Provider Fallback Chain
1. **llama.cpp** (local) - qwen2.5-coder:1.5b
2. **copilot** - IDE integration
3. **anthropic** - Cloud fallback
4. **openai** - Secondary cloud
5. **git** - Template-based fallback

### Manual Edit Preservation
Wrap manual edits to preserve them during regeneration:

```markdown
<!-- provider:add-manual-edit -->
Your custom content here will be preserved.
<!-- /provider:add-manual-edit -->
```

---

*This wiki is part of the NOA documentation system. Last generated: (never)*
