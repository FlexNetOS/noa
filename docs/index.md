---
layout: default
title: NOA Documentation
---

# NOA Documentation

Welcome to the NOA project documentation hub. This is the central navigation point for all project documentation.

---

## 🚀 Quick Start

| Document | Description |
|----------|-------------|
| [Quickstart Guide](guides/quickstart.md) | Get started with NOA in minutes |
| [Automation Guide](guides/automation.md) | Advanced automation workflows |

---

## 📚 Documentation Sections

### Guides
Practical how-to guides for users and developers.

- [quickstart.md](guides/quickstart.md) — Environment setup and first steps
- [automation.md](guides/automation.md) — Code reviews, deployment, workflows
- [next-steps.md](guides/next-steps.md) — Workspace ID setup, configuration

### Agents
Agent system documentation and CLI reference.

- [AGENT.md](agents/AGENT.md) — Constitutional authority and operational protocols
- [cli-reference.md](agents/cli-reference.md) — Agent CLI quick reference
- [implementation.md](agents/implementation.md) — Agent architecture and implementation

### Architecture
System design and architectural decisions.

- [architecture/](architecture/) — Core architecture documents
- [architecture/policy/](architecture/policy/) — Governance and policy documents
- [architecture/plans/](architecture/plans/) — Planning documents and audits

### Setup
Platform-specific setup guides.

- [setup/bootstrap-complete-guide.md](setup/bootstrap-complete-guide.md) — Complete bootstrap guide
- [setup/local-inference.md](setup/local-inference.md) — Local model inference
- [setup/windows.md](setup/windows.md) — Windows-specific setup

### ML DevOps
Machine learning and DevOps platform documentation.

- [ml-devops/](ml-devops/) — Complete ML DevOps documentation (26 files)
- [ml-devops/QUICKSTART.md](ml-devops/QUICKSTART.md) — ML DevOps quickstart
- [ml-devops/ARCHITECTURE.md](ml-devops/ARCHITECTURE.md) — Platform architecture
- [ml-devops/MODEL_INVENTORY.md](ml-devops/MODEL_INVENTORY.md) — Model inventory with CAS integration

### Runbooks
Operational procedures and runbooks.

- [runbooks/CAS_OPERATIONS.md](runbooks/CAS_OPERATIONS.md) — CAS operations runbook (complete guide)

### Status
Project status and history.

- [status/STATUS-HISTORY.md](status/STATUS-HISTORY.md) — Consolidated status history
- [PHASE_6_COMPLETE.md](../PHASE_6_COMPLETE.md) — Phase 6 implementation summary
- [PHASE_5_COMPLETE.md](../PHASE_5_COMPLETE.md) — Phase 5 implementation summary
- [PHASE_4_COMPLETE.md](../PHASE_4_COMPLETE.md) — Phase 4 implementation summary
- [PHASE_3_COMPLETE.md](../PHASE_3_COMPLETE.md) — Phase 3 implementation summary
- [PHASE_2.5_COMPLETE.md](../PHASE_2.5_COMPLETE.md) — Phase 2.5 framework alignment
- [IMPLEMENTATION_SUMMARY.md](../IMPLEMENTATION_SUMMARY.md) — Phase 1 & 2 summary

---

## 📁 Directory Structure

```
docs/
├── agents/           # Agent system documentation
├── adr/              # Architecture Decision Records
├── api/              # API reference documentation
├── architecture/     # System architecture docs
├── guides/           # User and developer guides
├── ml-devops/        # ML DevOps platform docs
├── pages/            # Static documentation pages
├── reference/        # Quick reference materials
├── runbooks/         # Operational runbooks
├── schema/           # Schema definitions
├── setup/            # Platform setup guides
├── status/           # Project status history
└── wiki/             # Wiki content
```

---

## 🔗 Key Resources

| Resource | Location |
|----------|----------|
| Main README | [../README.md](../README.md) |
| Constitution | [../CONSTITUTION.md](../CONSTITUTION.md) |
| Security Policy | [../SECURITY.md](../SECURITY.md) |
| License | [../LICENSE](../LICENSE) |

---

## 📖 By Audience

### New Users
| Priority | Document | Purpose |
|----------|----------|---------|
| ⭐⭐⭐ | [Quickstart](guides/quickstart.md) | Get started quickly |
| ⭐⭐ | [Windows Setup](setup/windows.md) | Windows-specific setup |

### Developers
| Priority | Document | Purpose |
|----------|----------|---------|
| ⭐⭐⭐ | [Agent Implementation](agents/implementation.md) | Agent architecture |
| ⭐⭐⭐ | [Architecture](architecture/) | System design |
| ⭐⭐ | [Automation Guide](guides/automation.md) | Workflow automation |

### DevOps
| Priority | Document | Purpose |
|----------|----------|---------|
| ⭐⭐⭐ | [ML DevOps](ml-devops/) | Platform operations |
| ⭐⭐⭐ | [CAS Operations](runbooks/CAS_OPERATIONS.md) | CAS operational runbook |
| ⭐⭐ | [Runbooks](runbooks/) | Operational procedures |
| ⭐⭐ | [Model Inventory](ml-devops/MODEL_INVENTORY.md) | Model management with CAS |

---

## 🆕 Recent Updates

### Phase 6: Third-Party Tool Integration (2026-01-02)

**Phase 6 Complete**: Third-party integrations configured

- ✅ MCP SDK integration (protocol implementation, tool discovery)
- ✅ Qdrant integration (3 vector collections, semantic search, RAG)
- ✅ SQLx integration (5 database tables, audit archival, analytics)
- ✅ libp2p integration (4 protocols, P2P networking, distributed CAS)
- ✅ Security configurations (capability requirements, audit logging)
- ✅ Complete integration guide

**Key Documents**:
- [PHASE_6_COMPLETE.md](../PHASE_6_COMPLETE.md) - Complete implementation summary
- [Integration Guide](../tools/third-party/INTEGRATION_GUIDE.md) - Setup and usage guide

### Phase 5: Resource Registry & Agent Templates (2026-01-02)

**Phase 5 Complete**: Resource management implementation

- ✅ Resource registry (3 agent templates, 18 tool definitions, 3 prompt templates)
- ✅ Agent deployment workflow (6-step automated deployment)
- ✅ Model deployment workflow (5-step CAS deployment)
- ✅ Tool definition schema (JSON Schema validation)
- ✅ Resource quotas (agent limits, budget defaults)
- ✅ Deployment scripts (agent & model)
- ✅ Complete documentation

**Key Documents**:
- [PHASE_5_COMPLETE.md](../PHASE_5_COMPLETE.md) - Complete implementation summary
- [Resource Registry](../data/resources/registry.json) - Central resource catalog

### Phase 4: System Core & Policy (2026-01-02)

**Phase 4 Complete**: Trusted microkernel implementation

- ✅ Identity management (3 principals, 4 roles, 7 capabilities)
- ✅ Policy enforcement (6 policy categories, 17 rules)
- ✅ Audit logging (8 event categories, SOC2/ISO27001 compliant)
- ✅ System registry (5 services, 3 providers, 4 resources)
- ✅ Task scheduler (5 scheduled tasks)
- ✅ Capability-based RBAC with default deny
- ✅ Complete documentation

**Key Documents**:
- [PHASE_4_COMPLETE.md](../PHASE_4_COMPLETE.md) - Complete implementation summary
- [Identity README](../sys/core/identity/README.md) - Identity system guide
- [Audit README](../sys/core/audit/README.md) - Audit system guide

### Phase 3: CAS & Data Plane (2026-01-02)

**Phase 3 Complete**: Content-Addressed Storage implementation

- ✅ CAS layer with blake3 hashing and zstd compression
- ✅ 7 utility scripts for object operations
- ✅ 5 specialized registries (models, prompts, snapshots, binaries, packages)
- ✅ Garbage collection with reachability analysis
- ✅ Bounded cache management (6 cache types, 25GB total)
- ✅ 35 automated tests (100% pass rate)
- ✅ Complete operational runbook

**Key Documents**:
- [CAS Operations Runbook](runbooks/CAS_OPERATIONS.md) - Complete operational guide
- [PHASE_3_COMPLETE.md](../PHASE_3_COMPLETE.md) - Full implementation summary
- [CAS README](../cas/README.md) - Framework documentation
- [CAS Scripts README](../scripts/cas/README.md) - Utility scripts guide

---

*Last Updated: 2026-01-02*
