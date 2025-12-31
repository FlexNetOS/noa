# NOA Wiki

This wiki provides conceptual documentation and reference material for the NOA platform.

## Overview

NOA (Neural Orchestration Architecture) is an AI-native operating system designed for autonomous agent orchestration, local-first computation, and total memory sovereignty.

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

- [Architecture](../architecture/) — System design documents
- [Guides](../guides/) — How-to guides
- [Agents](../agents/) — Agent system documentation
- [ML DevOps](../ml-devops/) — ML platform documentation

## Codebase & Crates

- [Codebase Map](codebase_map.md) — Full file listing of the repository
- [Crate Plans](crates/index.md) — Detailed plans and documentation for system crates

---

*This wiki is part of the NOA documentation system.*
