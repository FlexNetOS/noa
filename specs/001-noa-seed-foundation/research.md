# Research: NOA Seed Foundation

**Feature**: 001-noa-seed-foundation
**Date**: 2025-12-08
**Status**: Complete

## Overview

This document captures research decisions for the NOA Seed Foundation implementation, resolving all technical unknowns from the specification.

---

## 1. Core Runtime Technology

### Decision: Rust for Core Runtime

**Rationale**:
- Memory safety without garbage collection (critical for embedded/offline scenarios)
- Cross-platform compilation to native binaries
- Excellent performance for latency-sensitive operations
- Strong ecosystem for systems programming (tokio, serde)
- Native integration with llama.cpp via llama-cpp-rs

**Alternatives Considered**:
- **Go**: Simpler concurrency but larger binary size; GC pauses could affect latency
- **C++**: Direct llama.cpp integration but memory safety concerns
- **Zig**: Promising but ecosystem too immature

**Key Dependencies**:
- `tokio` (async runtime)
- `serde` (serialization)
- `clap` (CLI parsing)
- `llama-cpp-rs` (LLM inference)
- `rusqlite` (SQLite bindings)

---

## 2. Neural Runtime (LLM Inference)

### Decision: llama.cpp with Rust Bindings

**Rationale**:
- Pure C++ implementation, no external dependencies
- Supports GGUF format (quantized models)
- CPU inference with optional GPU acceleration
- Active community and rapid development
- Rust bindings (llama-cpp-rs) provide safe interface

**Model Selection Strategy**:
- Primary: Models <3B parameters (fit in 8GB RAM)
- Recommended: Qwen2.5-1.5B, Phi-3-mini, TinyLlama
- Fallback: Ollama wrapper for additional model support

**Alternatives Considered**:
- **Ollama**: Easier setup but additional dependency; use as optional fallback
- **vLLM**: Better throughput but heavier; overkill for single-user
- **ONNX Runtime**: Good performance but model conversion complexity

**configsuration**:
```json
{
  "default_model": "qwen2.5-1.5b-instruct-q4_k_m.gguf",
  "context_length": 4096,
  "threads": "auto",
  "gpu_layers": "auto",
  "batch_size": 512
}
```

---

## 3. Local Database & Memory

### Decision: SQLite with sqlite-vss for Vector Search

**Rationale**:
- Zero configsuration, single-file database
- Excellent read performance for memory recall
- sqlite-vss extension provides vector similarity search
- Cross-platform, embedded in binary
- WAL mode handles concurrent reads well

**Schema Strategy**:
- Main tables: `memories`, `agents`, `tasks`, `knowledge_nodes`, `embeddings`
- Vector index on embeddings for semantic search
- JSON columns for flexible metadata

**Alternatives Considered**:
- **PostgreSQL + pgvector**: Better for larger deployments; requires server setup
- **DuckDB**: Good analytics but less mature for OLTP
- **LanceDB**: Vector-native but newer, less proven

**Migration Path**:
- Start with SQLite + sqlite-vss
- Add PostgreSQL + pgvector as optional "scale-up" path
- Abstract via repository pattern for easy switching

---

## 4. P2P Networking

### Decision: libp2p (Go Implementation)

**Rationale**:
- Battle-tested in IPFS, Ethereum, Filecoin
- Built-in NAT traversal, DHT, pubsub
- Go implementation is most mature
- Transport-agnostic (TCP, QUIC, WebSocket)

**Protocol Design**:
- Discovery: mDNS for local, DHT for global (optional)
- Sync: Custom CRDT-based protocol for memory sync
- Compute: Task distribution via pubsub
- Security: TLS 1.3 + noise protocol

**Alternatives Considered**:
- **Custom TCP**: Full control but massive implementation effort
- **WebRTC**: Good for browsers but complex for native
- **ZeroMQ**: Simpler but no built-in discovery/NAT

---

## 5. Dynamic UI Framework

### Decision: Next.js 14 with React Server Components

**Rationale**:
- Server-side rendering for instant initial load
- React Server Components for dynamic content
- TypeScript-first with excellent DX
- Built-in routing and API routes
- Tailwind CSS for rapid styling

**Key Components**:
- Agent Activity Log (real-time updates)
- Memory Browser (search + navigation)
- Task Dashboard (context-aware)
- Model Status Panel
- P2P Cluster View

**Alternatives Considered**:
- **SvelteKit**: Smaller bundle but React ecosystem larger
- **Remix**: Good patterns but less mature
- **Electron**: Native feel but resource-heavy; consider for v2

**State Management**: Zustand (lightweight, TypeScript-friendly)

---

## 6. Digest Pipeline Architecture

### Decision: Python with Tree-sitter Multi-Language Parsing

**Rationale**:
- Python has best ML/NLP ecosystem
- Tree-sitter provides unified AST for all languages
- sentence-transformers for embeddings
- Easy integration with llama.cpp for summarization

**7-Step Pipeline Implementation**:

| Step | Tool | Output |
|------|------|--------|
| 1. Discover | Python stdlib + GitHub API | Source list |
| 2. Fetch | git + httpx | Local clone |
| 3. Parse | Tree-sitter | AST per file |
| 4. Analyze | sentence-transformers | Embeddings + KG |
| 5. Summarize | llama.cpp | system_card.md |
| 6. Surface | jinja2 + SQLite | profile.json, kg.json |
| 7. Secure | Syft, Grype, Gitleaks | SBOM + vulns |

**Alternatives Considered**:
- **Language-specific parsers**: More accurate but N×complexity
- **Sourcegraph SCIP**: Good indexing but heavy dependency
- **OpenAI API**: Better summaries but violates offline principle

---

## 7. Agent Framework

### Decision: Custom TypeScript Agent Definitions with JSON Protocol

**Rationale**:
- TypeScript provides type safety for agent interfaces
- JSON protocol enables language-agnostic execution
- Custom framework avoids heavy dependencies
- Aligned with constitution's self-contained principle

**Agent Hierarchy**:
```
NOA (CECCA) - Root Orchestrator
├── Board Agents (advisory)
│   ├── SecurityBoardAgent
│   ├── ComplianceBoardAgent
│   └── PerformanceBoardAgent
├── Permanent Agents
│   ├── FileIOAgent
│   ├── TerminalAgent
│   ├── RAGAgent
│   └── MicroserviceManagementAgent
└── MicroAgentStacks (dynamic)
    └── [Task-specific agent clusters]
```

**Alternatives Considered**:
- **LangChain**: Feature-rich but heavy, many external deps
- **AutoGPT**: Interesting but too autonomous, less predictable
- **CrewAI**: Good multi-agent but Python-only

---

## 8. Cross-Platform Build Strategy

### Decision: Platform-Specific Binaries with Shared Core

**Build Matrix**:

| Platform | Rust | Go | Node | Python |
|----------|------|-----|------|--------|
| Windows x64 | MSVC | Native | Bundled | Bundled |
| macOS x64/arm64 | Apple clang | Native | Bundled | Bundled |
| Linux x64 | GNU/musl | Native | Bundled | Bundled |

**Distribution**:
- Single installer per platform
- Self-contained (no system dependencies)
- Optional components downloaded on first use

**Alternatives Considered**:
- **Docker-only**: Simpler distribution but requires Docker
- **Flatpak/Snap**: Linux-only
- **WASM**: Interesting for portability but performance concerns

---

## 9. Security Model

### Decision: Defense in Depth with Sandboxed Execution

**Layers**:
1. **Process Isolation**: Agent code runs in subprocess
2. **Filesystem Sandboxing**: Agents can only access `noa_root`
3. **Network Filtering**: Allowlist for external connections
4. **Secret Management**: Vault-style encrypted storage
5. **Audit Logging**: All actions recorded

**Implementation**:
- Rust: `seccomp` on Linux, sandbox profiles on macOS
- Secrets: AES-256-GCM encrypted, key derived from user passphrase
- Audit: Append-only log with cryptographic chaining

---

## 10. Biblical Governance Integration

### Decision: Reference Implementation with Structured Rules

**Approach**:
- Store biblical principles as structured rules in `ai/governance/`
- Each rule mapped to specific agent behaviors
- Constitutional validation on agent outputs
- Human override with audit trail

**Initial Scope**:
- Character virtues mapped to agent behavior constraints
- Conflict resolution principles for multi-agent disputes
- Priority ordering based on biblical hierarchy

**Deferred**:
- Full Greek/Hebrew text transformation (requires specialized ML)
- Automated rule extraction from text

---

## Summary Table

| Decision Area | Choice | Key Reason |
|---------------|--------|------------|
| Core Runtime | Rust | Performance + safety |
| LLM Inference | llama.cpp | Offline, cross-platform |
| Database | SQLite + sqlite-vss | Zero-configs, embedded |
| P2P Layer | libp2p (Go) | Mature, NAT traversal |
| UI Framework | Next.js + React | DX, SSR, ecosystem |
| Digest Pipeline | Python + Tree-sitter | ML ecosystem, multi-lang |
| Agent Framework | Custom TypeScript | Self-contained, typed |
| Build Strategy | Platform binaries | No runtime deps |
| Security | Sandboxed processes | Defense in depth |
| Biblical Gov | Structured rules | Auditable, overridable |

---

## Open Questions (for implementation)

1. **Model download UX**: Progress UI during initial model fetch?
2. **P2P bootstrap**: How to handle first-time device pairing?
3. **Memory retention policy**: Auto-prune old memories or keep forever?
4. **Agent marketplace**: Future extension for community agents?

These will be addressed during implementation with reasonable defaults.
