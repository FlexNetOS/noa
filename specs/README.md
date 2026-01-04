# NOA Specifications Index

This directory contains all feature specifications for the NOA project, following the [Spec-Driven Development](../data/archive/sessions/project-mgmt/spec-kit/spec-driven.md) methodology.

---

## Active Specifications

| Spec ID | Name | Status | Priority | Description |
|---------|------|--------|----------|-------------|
| [001](./001-noa-seed-foundation/) | NOA Seed Foundation | 🟡 In Progress | P0-P3 | Core foundation, bootstrap, runtime, memory, agents |

---

## Architecture References

These high-level architecture documents inform the implementation specs:

| Document | Purpose | Status |
|----------|---------|--------|
| [noa_aer_spec_v2.md](./noa_aer_spec_v2.md) | Agentic Environment Runtime (AER) v2 - High-level architecture | ✅ Reference |
| [agentic_env_runtime_spec.md](./agentic_env_runtime_spec.md) | Original AER specification | ⚠️ Superseded by v2 |
| [AGENT.md](./AGENT.md) | Agent execution guidelines and 4-D methodology | ✅ Active |

---

## Spec-Kit Integration

All specs follow the [Spec-Kit](../data/archive/sessions/project-mgmt/spec-kit) template structure:

### Required Documents (per spec)

| Document | Purpose | Template |
|----------|---------|----------|
| `spec.md` | User stories, requirements, acceptance criteria | [spec-template.md](../data/archive/sessions/project-mgmt/spec-kit/templates/spec-template.md) |
| `plan.md` | Technical context, architecture, task breakdown | [plan-template.md](../data/archive/sessions/project-mgmt/spec-kit/templates/plan-template.md) |
| `tasks.md` | Detailed task list with IDs, dependencies | [tasks-template.md](../data/archive/sessions/project-mgmt/spec-kit/templates/tasks-template.md) |

### Optional Documents (per spec)

| Document | Purpose |
|----------|---------|
| `data-model.md` | Entity definitions, relationships, state transitions |
| `research.md` | Technology decisions, alternatives considered |
| `quickstart.md` | Getting started guide, installation, first run |
| `contracts/` | API contracts (OpenAPI, Protobuf, GraphQL) |
| `checklists/` | Verification checklists for quality gates |
| `memory/` | Constitutional references, governance docs |

---

## Spec Lifecycle

```
Draft → In Review → Approved → In Progress → Complete → Archived
```

| Status | Meaning |
|--------|---------|
| 🔵 Draft | Initial creation, not ready for review |
| 🟣 In Review | Ready for stakeholder review |
| 🟢 Approved | Reviewed and approved for implementation |
| 🟡 In Progress | Active implementation underway |
| ✅ Complete | All tasks done, verified |
| ⬛ Archived | Superseded or deprecated |

---

## 001-noa-seed-foundation Structure

```
001-noa-seed-foundation/
├── spec.md              # 11 user stories (US1-US11), 190 FRs
├── plan.md              # Technical context, 1070+ tasks referenced
├── tasks.md             # Detailed task list (B001-B195, T001-T875+)
├── data-model.md        # 24 entities
├── research.md          # Technology decisions
├── quickstart.md        # Getting started guide
├── kernel-independence.md # NKAL architecture
├── contracts/
│   ├── noa-core.openapi.yaml
│   ├── digest-pipeline.openapi.yaml
│   └── p2p-protocol.proto
├── checklists/
│   ├── comprehensive.md
│   ├── kernel-independence.md
│   ├── quality.md
│   ├── requirements.md
│   └── verification.md
├── configs/
│   └── tools.json
└── memory/
    └── CONSTITUTION.md
```

---

## Key Concepts from AER Architecture

The implementation specs are informed by the AER (Agentic Environment Runtime) architecture:

### Core Models (< 3B params each)

| Model | Purpose | Spec Reference |
|-------|---------|----------------|
| **EOM** | Env Orchestrator Model - planning, resource allocation | AER §9.1 |
| **TSM** | Tool & Code Synthesizer Model - code generation | AER §9.2 |
| **PSM** | Policy & Safety Model - safety enforcement | AER §9.3 |

### Architectural Layers

| Layer | Purpose | Spec Reference |
|-------|---------|----------------|
| Host Adapter | Platform-specific kernel abstraction | AER §6 |
| noa Core Microkernel | Process model, message bus, policy | AER §7 |
| llama-microkernel | LLM inference engine | AER §8 |
| User Hive Fabric | P2P memory sync across devices | AER §11 |
| Resource Mesh | Cross-device compute pooling | AER §12 |
| App Hypervisor | Desktop app hosting (NDCL) | AER §13 |

---

## Archived Specifications

| Spec ID | Name | Reason | Archive Date |
|---------|------|--------|--------------|
| [002](./.archive/002-unified-bootstrap/) | Unified Bootstrap | Merged into 001 Phase 0 | 2025-12-08 |

---

## Creating New Specs

1. Create directory: `specs/NNN-feature-name/`
2. Copy templates from `project-mgmt/spec-kit/templates/`
3. Fill in spec.md with user stories (use `/speckit.specify`)
4. Generate plan with `/speckit.plan`
5. Generate tasks with `/speckit.tasks`
6. Update this README with the new spec

---

## Related Documentation

- [NOA Constitution](../CONSTITUTION.md) - Core principles and governance
- [Spec-Kit README](../data/archive/sessions/project-mgmt/spec-kit/README.md) - Spec-driven development toolkit
- [AGENTS.md](../data/archive/sessions/project-mgmt/spec-kit/AGENTS.md) - AI agent integration guide

---

*Last Updated: 2025-12-09*

