# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION]
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION]
**Project Type**: [single/web/mobile - determines source structure]
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION]
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION]
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

| Principle | Compliant | Notes |
|-----------|-----------|-------|
| 3.1 Self-Contained & Autonomous | ☐ Yes / ☐ N/A | All paths under `noa_root` directory? |
| 3.2 Local-First & Offline-Capable | ☐ Yes / ☐ N/A | Works without network? Graceful degradation? |
| 3.3 Agentic Orchestration | ☐ Yes / ☐ N/A | Uses multi-agent pattern? SLM compatible? |
| 3.4 Adaptive & Self-Improving | ☐ Yes / ☐ N/A | Supports learning loops? Rollback path? |
| 3.5 Transparent & Auditable | ☐ Yes / ☐ N/A | All actions logged? Human-reviewable? |
| 3.6 Security & Privacy | ☐ Yes / ☐ N/A | No hardcoded secrets? Data residency controlled? |
| 3.7 Total Memory Sovereignty | ☐ Yes / ☐ N/A | Persistent storage? Instant recall? |
| 3.8 P2P Hive-Mind | ☐ Yes / ☐ N/A | Supports multi-device? Resource sharing? |
| 3.12 Test Everything | ☐ Yes / ☐ N/A | Verification built-in? |
| 3.13 Shared Provider Resource Unification | ☐ Yes / ☐ N/A | Resources refactored for shared access? |
| 4.9 Provider Orchestration | ☐ Yes / ☐ N/A | Uses shared execution memory? |
| 4.10 Resource Name Refactoring | ☐ Yes / ☐ N/A | Provider names unified? |
| 4.11 Kernel Independence | ☐ Yes / ☐ N/A | Supports multiple kernel modes? |

### Constitutional Flow Traceability

| Level | Reference | Status |
|-------|-----------|--------|
| Goal | [Link to goal] | ☐ Linked |
| Policy | [Link to policy] | ☐ Linked |
| Rule | [Link to rule] | ☐ Linked |
| Spec | [Link to spec] | ☐ Linked |

### Provider Resource Unification Plan

*Required when integrating external repositories or AI provider tools (§3.13, §4.10)*

| External Resource | Origin | Unified Name | Shared Location | Status |
|-------------------|--------|--------------|-----------------|--------|
| [e.g., claude-reasoning] | claude-code repo | reasoning-agent | ai/shared/agents/ | ☐ Refactored |
| [e.g., codex-generate] | codex-cli repo | code-generation-tool | ai/shared/tools/ | ☐ Refactored |

**Execution Memory Integration**: ☐ Yes / ☐ N/A
- Shared context path: `ai/shared/resources/execution-memory.db`
- Provider state sync: ☐ Enabled / ☐ N/A

### Kernel Independence Mode

*Required for features that may require isolation (§4.11)*

| Mode | Supported | Notes |
|------|-----------|-------|
| Native | ☐ Yes / ☐ N/A | Default mode, host kernel |
| VM | ☐ Yes / ☐ N/A | Hyper-V / KVM / Virtualization.framework |
| Container | ☐ Yes / ☐ N/A | Docker / Podman |
| Sandbox | ☐ Yes / ☐ N/A | User-space isolation |

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
