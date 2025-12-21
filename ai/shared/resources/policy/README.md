# NOA Policy Documentation

This directory contains the authoritative policies and architectural standards for the NOA project.

## Core Governance

| File | Description |
|------|-------------|
| [01_CONSTITUTION.md](01_CONSTITUTION.md) | **The NOA Constitution**. Defines core principles, governance, and non-negotiable rules. Synced with the root `CONSTITUTION.md`. |
| [04-GOVERNANCE_ASSET-REGISTRY.md](04-GOVERNANCE_ASSET-REGISTRY.md) | Governance policy for the Asset Registry and CAS fields. |
| [04-GOVERNANCE_TASK-EXECUTION.md](04-GOVERNANCE_TASK-EXECUTION.md) | Rules for task execution, planning, and agent behavior. |
| [04-GOVERNANCE_RETENTION.md](04-GOVERNANCE_RETENTION.md) | **NEW** Unified retention and archival policy. Never delete, always archive. |

## Environment & Runtime

| File | Description |
|------|-------------|
| [02-ENV_CONTAINMENT.md](02-ENV_CONTAINMENT.md) | Policy for AppData containment and FR-001 compliance. |
| [02-ENV_HOME-vs-ROOT.md](02-ENV_HOME-vs-ROOT.md) | Definitions of `NOA_ROOT` and `NOA_HOME` path anchors. |
| [02-ENV_CANONICAL-VARS.md](02-ENV_CANONICAL-VARS.md) | **NEW** Single source of truth for all environment variables. Auto-generates env scripts. |
| [02-ENV_KERNELS.md](02-ENV_KERNELS.md) | Kernel Independence Layer (NKAL) and isolation modes. |
| [02-ARCH_AER-SPEC.md](02-ARCH_AER-SPEC.md) | Agentic Environment Runtime (AER) v2 high-level architecture. |

## Configuration & Storage

| File | Description |
|------|-------------|
| [03-CONFIG_CAS.md](03-CONFIG_CAS.md) | Content-Addressable Storage (CAS) architecture and configuration. |
| [03-CONFIG_CAS-MODULES.md](03-CONFIG_CAS-MODULES.md) | Specific policy for Module Artifacts in CAS. |
| [03-CONFIG_SCHEMA-VALIDATION.md](03-CONFIG_SCHEMA-VALIDATION.md) | JSON Schema validation policy. |
| [03-CONFIG_PACKAGE-MANAGER.md](03-CONFIG_PACKAGE-MANAGER.md) | **NEW** pnpm as canonical package manager. Shared packages, containment. |
| [03-CONFIG_WORKSPACE.md](03-CONFIG_WORKSPACE.md) | **NEW** Monorepo structure, hoisting, path to microservices. |
| [03-CONFIG_TOOLS-LIFECYCLE.md](03-CONFIG_TOOLS-LIFECYCLE.md) | **NEW** Tool version pinning, upgrade-only policy, archival. |

## Documentation Standards

| File | Description |
|------|-------------|
| [05-DOCS_WIKI-RUNBOOK.md](05-DOCS_WIKI-RUNBOOK.md) | Standards for Wiki, Pages, and Runbooks. |
