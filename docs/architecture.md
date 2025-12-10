# NOA Architecture Overview

This document summarizes the major components of the NOA Seed Foundation and points to detailed design docs in `docs/architecture/`.

## Control Planes
- **Sandbox Plane** – local-first execution for development and validation.
- **Deployed Plane** – promoted workloads with controlled blast radius.
- **Coordinator Plane** – promotion, analytics, and rollback orchestration.

## Core Services
- **Kernel Independence (NKAL)** – `sys/core/src/kernel/` abstracts host dependencies (native, VM, container, sandbox). See `docs/architecture/kernel-independence.md`.
- **Providers** – shared execution memory and provider registry for LLM backends (Claude, Codex, Cursor, Git, Abacus, llama.cpp).
- **Agents & Orchestration** – task/goal decomposition and collaborative reasoning over shared memory and context stores.
- **Modules & CAS** – content-addressable registry for binaries, tools, agents, and services with integrity verification.

## Data Plane
- **Memory** – embeddings, vector search, and recall pipelines.
- **Digest** – ingestion and summarization pipelines (code, docs, signals).
- **State** – SQLite-based execution memory bus; registry databases for modules and artifacts.

## Experience Layer
- **CLI** – `noa` command for init, start, status, providers, modules, planes, promotions, healing, models, tasks, and goals.
- **UI** – Next.js dashboard (status, connectors, marketplace stubs, P2P pairing).
- **Desktop Containment** – NDCL wrappers for desktop apps (ChatGPT, Claude, GitHub Desktop, Cursor/VS Code).

## Security & Governance
- **Constitutional Guardrails** – principles 3.1, 3.5, 3.6, 3.12 enforced via prompts, policies, and validation layers.
- **Isolation** – app data redirection, network proxying, OAuth proxy, and provider-specific rate limits.
- **Verification** – hashes, reports, gap scans, and truth-gate checks to validate releases.

## Deployment Path
1. Run bootstrap/setup scripts to create portable toolchains and shared resources.
2. Configure providers and shared execution memory.
3. Start core services (`noa start`) and validate health.
4. Promote artifacts through planes using coordinator policies.

For deeper dives, consult:
- `docs/architecture/kernel-independence.md`
- `docs/architecture/desktop-app-hosting.md`
- `docs/architecture/appdata-containment.md`
- `docs/architecture/replay_memory.md`
- `docs/architecture/toolkengpt.md`
