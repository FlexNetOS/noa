# NOA Platform - Agent Instructions

<!--
This file MUST remain in sync with:
- CONSTITUTION.md (governance principles)
- README.md (technical architecture)
Last synced: 2026-01-01
-->

## Overview

**NOA** (Name of App / Chief Executive Chief Commander Agent) is a multi-platform, autonomous, self-modifying agentic operating system designed to function as a **hive-mind**.

NOA autonomously plans, acts, learns, and adapts to manage and self-upgrade the entire software and hardware environment. It replaces traditional applications and cloud-based services with a unified neural runtime and a dynamic UI.

## Core Constitutional Principles

All agents MUST adhere to these non-negotiable principles from [CONSTITUTION.md](CONSTITUTION.md):

### 1. Self-Contained & Autonomous
- All code, configuration, models, and state MUST resolve under `noa_root`
- No absolute paths outside `noa_root` may be baked into code
- `noa_root` is runtime-resolved: Windows (`%NOA_ROOT%`), Unix (`$NOA_ROOT`), Container (`/noa/`)

### 2. Local-First & Offline-Capable
- Core workflows MUST work without network connectivity
- Third-party APIs are **optional enhancements** behind feature flags
- Online mode is primary, but offline switching MUST be available

### 3. Agentic Orchestration & Hive-Mind
- Multiple specialized agents collaborate (planning, execution, QA, refactoring)
- Multiple Small Language Models (SLMs) via llama.cpp (<3B params each)
- Agents are bound by this constitution—no violations allowed

### 4. Total Memory Sovereignty
- **Everything is memory—nothing is forgotten—instant memory recall**
- All interactions, decisions, and learnings MUST be persistently stored
- RAG framework integrated for long-term memory

### 5. Test Everything, Trust Nothing
- All inputs, outputs, and state changes MUST be verifiable
- Self-generated code MUST pass automated tests before deployment
- Continuous testing loops detect drift from expected behavior

### 6. Transparent & Auditable
- All significant decisions MUST be logged (who acted, why, what changed)
- UI MUST display live, scrollable log of agent's "thought process"

## Provider Orchestration Priority

When operating in IDE context, coordinate ALL available providers:

| Priority | Provider | Type | Purpose |
|----------|----------|------|---------|
| 1 | llama.cpp | Local | Always available offline, primary inference |
| 2 | Cursor | Hybrid | IDE context awareness, provider orchestration |
| 3 | Claude Code | Cloud | Complex reasoning, long context |
| 4 | Codex | Cloud | Code generation, completion |
| 5 | VS Code Copilot | IDE | Inline completions |
| 6 | Git CLI | Local | Version control operations |
| 7 | Abacus | Cloud | Numerical/analytical tasks |

**Fallback Strategy**: Local → IDE → Cloud → Queue and notify after 3 failures

## Repository Structure

```
noa/
├── ai/                    # AI providers, agents, shared resources
│   ├── agents/            # Agent definitions and orchestration
│   ├── providers/         # Provider integrations (Ollama, OpenAI, Anthropic)
│   └── shared/            # Unified resources for ALL providers
│       ├── agents/        # Provider-agnostic agent definitions
│       ├── tools/         # Unified tool interfaces
│       ├── commands/      # Cross-provider commands
│       ├── prompts/       # Template prompts with variables
│       └── resources/     # Resource registry and mappings
├── apps/                  # Application packages
├── bin/                   # Executable wrappers and CLI tools
├── cache/                 # Cache configuration and storage
├── cmd/                   # Command-line tools
│   ├── tasks-cli/         # Task CLI (Rust)
│   ├── tasks-mcp/         # MCP server integration
│   └── tasks-extension/   # VS Code extension
├── config/                # Centralized configuration (JSON schemas)
│   ├── schemas/           # JSON validation schemas
│   └── templates/         # Configuration templates
├── containers/            # Container definitions (capsules, OCI)
├── data/                  # Runtime data, state, modules
├── docs/                  # Documentation
│   ├── wiki/              # Internal knowledge base
│   ├── pages/             # Static reference pages
│   ├── runbooks/          # Operational procedures
│   ├── architecture/      # System design, data flow
│   ├── api/               # API reference
│   └── adr/               # Architecture Decision Records
├── etc/                   # System configuration files
├── gateway/               # API gateway and MCP servers
│   └── mcp/               # Model Context Protocol implementations
├── init/                  # Initialization and bootstrap scripts
├── lib/                   # Shared libraries
│   └── flexnetos/         # FlexNetOS dependencies (rusqlite, sqlx, etc.)
├── logs/                  # Log output directory
├── ml_devops_platform/    # ML/DevOps platform components
├── opt/                   # Optional tools (Ollama, portable apps)
├── orchestrator/          # Core orchestration engine
├── p2p/                   # Peer-to-peer networking (libp2p)
├── pkg/                   # Package definitions
├── ruler/                 # Ruler task management system
├── sandbox/               # Isolated testing environments
├── scripts/               # Utility and automation scripts
│   └── sweep/             # Codebase sweep system
├── specs/                 # API and system specifications
├── sys/                   # System-level crates
│   └── core/              # Core library (6 crates)
├── tests/                 # Integration and E2E tests
├── tmp/                   # Temporary files
└── ui/                    # User interface components
    └── rust-lovable/      # Dioxus-based conversational UI
```

## Backend Architecture (sys/core/)

The backend consists of 6 Rust crates:

| Crate | Purpose |
|-------|---------|
| **noa-api** | REST API server with Axum (`/health`, `/api/v1/status`, `/api/v1/tasks`) |
| **noa-common** | Shared types (Entity IDs, Agent types, Knowledge graph) |
| **noa-embedder** | Vector embedding service (FastEmbed integration) |
| **noa-trainer** | Model training pipeline, fine-tuning |
| **noa-indexer** | Repository indexing, code analysis |
| **noa-agent** | Autonomous agent system (CECCA orchestrator) |

## Running Services

| Service | URL | Status |
|---------|-----|--------|
| API Server | http://localhost:3001 | ✅ Operational |
| UI Dashboard | http://localhost:3000 | ✅ Operational |
| rust-lovable | http://localhost:8080 | ✅ Available |
| Ollama | http://localhost:11434 | ⏳ On-demand |

## Development Guidelines

### Code Standards

1. **Rust Code**
   - Follow `rustfmt` config in `.config/rustfmt.toml`
   - Use `clippy` with `.config/clippy.toml`
   - All public APIs MUST have doc comments
   - Prefer `Result<T, E>` over panics

2. **TypeScript/JavaScript**
   - Follow ESLint config in `.config/eslint.config.mjs`
   - Use Prettier for formatting
   - Prefer TypeScript over JavaScript

3. **Python**
   - Follow Ruff config in `.config/ruff.toml`
   - Use type hints for all function signatures
   - Document with docstrings

### Testing Requirements

- **Unit tests**: Required for all new functionality
- **Integration tests**: Required for cross-module interactions
- **E2E tests**: Required for user-facing features

```bash
cargo test           # Rust tests
npm test             # Node tests
pytest               # Python tests
```

### Git Workflow

1. Create feature branches from `develop`
2. Use descriptive commit messages
3. **AI commits**: `git commit --author="AI <noa+ai@flexnetos.com>"`
4. Submit PRs with clear descriptions
5. Ensure CI passes before merging

## Goals-Policy-Rules-Spec-Plan-Tasks Flow

Every request MUST follow the constitutional flow:

```
Request → Goals → Policies → Rules → Spec → Plan → Tasks → CSV Table
```

All work MUST demonstrate traceability:

| Level | Document | Must Reference |
|-------|----------|----------------|
| Goal | `*-goals.md` | Constitution principles |
| Policy | `*-policy.md` | Associated goals |
| Rule | `*-rule.md` | Associated policies |
| Spec | `spec.md` | Governing rules |
| Plan | `plan.md` | Spec, constitution compliance |
| Tasks | `tasks.md` | Plan, spec, constitutional tags |

## Key Systems

### Orchestrator (`orchestrator/`)
Core system for coordinating tasks, agents, and workflows.

### Ruler (`ruler/`)
Task management and rule evaluation system. See `ruler/AGENTS.md` for specific instructions.

### MCP Gateway (`gateway/mcp/`)
Model Context Protocol server implementations for tool integration.

### Tasks System (`cmd/tasks-*`)
- `tasks-cli/` - Command-line interface
- `tasks-mcp/` - MCP server integration
- `tasks-extension/` - VS Code extension

### AI Providers (`ai/providers/`)
Integrations: Ollama (local), OpenAI, Anthropic, Azure OpenAI

## Shared Provider Resource Unification

When external repositories are integrated, resources MUST be refactored for universal access:

1. **Discovery**: Scan for agents, tools, commands, prompts
2. **Analysis**: Identify provider-specific naming
3. **Mapping**: Create original → unified name mapping
4. **Adaptation**: Create adapter layer
5. **Registration**: Register in `ai/shared/resources/resource-registry.json`

Example mappings:
- `claude` (claude-code) → `reasoning-agent` (all providers)
- `codex` (codex-cli) → `code-generation-tool` (all providers)

## Sweep System

The codebase sweep system (`scripts/sweep/`) provides automated analysis:

```powershell
# Run full sweep
.\scripts\sweep\sweep.ps1 -Sweep 1 -Operations all

# Individual operations
-Operations extract    # Symbol extraction
-Operations docs       # Documentation cross-reference
-Operations embed      # Ollama embeddings (nomic-embed-text, 768 dims)
-Operations graph      # Mermaid diagram generation
-Operations test       # E2E test validation
```

## Configuration

### Environment
- Primary config: `config/` directory
- Schemas: `config/schemas/`
- Templates: `config/templates/`

### Local Development
1. Run `init/noa-init.ps1` for Windows setup
2. Run `init/noa-init` for Unix setup
3. Set environment via `noa-env.ps1`

## Key Files

| File | Purpose |
|------|---------|
| `CONSTITUTION.md` | Core principles and governance (AUTHORITATIVE) |
| `AGENTS.md` | This file - agent execution instructions |
| `README.md` | Project overview and quick start |
| `Makefile` | Build automation |
| `config/` | Centralized JSON configuration with schemas |

## Kernel Independence Modes

NOA supports operation independent of host OS kernel:

| Mode | Priority | Description |
|------|----------|-------------|
| VM | 1 | Maximum isolation (Hyper-V, KVM) |
| Container | 2 | Isolated container |
| Sandbox | 3 | User-space isolation |
| Native | 4 (Default) | Host kernel for performance |

Selection precedence: VM > Container > Sandbox > Native (when security requires)

## Contact & Resources

1. `docs/wiki/` - Internal documentation
2. `docs/runbooks/` - Operational procedures
3. Issue tracker - Bug reports and feature requests
4. `CONSTITUTION.md` - Governance reference

---

**Version**: Synced with CONSTITUTION.md v2.1.0 | README.md
