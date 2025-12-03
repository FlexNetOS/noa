# [PROJECT_NAME] Constitution
<!-- Example: Spec Constitution, TaskFlow Constitution, etc. -->

## Core Principles

### [PRINCIPLE_1_NAME]
<!-- Example: I. Library-First -->
[PRINCIPLE_1_DESCRIPTION]
<!-- Example: Every feature starts as a standalone library; Libraries must be self-contained, independently testable, documented; Clear purpose required - no organizational-only libraries -->

### [PRINCIPLE_2_NAME]
<!-- Example: II. CLI Interface -->
[PRINCIPLE_2_DESCRIPTION]
<!-- Example: Every library exposes functionality via CLI; Text in/out protocol: stdin/args → stdout, errors → stderr; Support JSON + human-readable formats -->

### [PRINCIPLE_3_NAME]
<!-- Example: III. Test-First (NON-NEGOTIABLE) -->
[PRINCIPLE_3_DESCRIPTION]
<!-- Example: TDD mandatory: Tests written → User approved → Tests fail → Then implement; Red-Green-Refactor cycle strictly enforced -->

### [PRINCIPLE_4_NAME]
<!-- Example: IV. Integration Testing -->
[PRINCIPLE_4_DESCRIPTION]
<!-- Example: Focus areas requiring integration tests: New library contract tests, Contract changes, Inter-service communication, Shared schemas -->

### [PRINCIPLE_5_NAME]
<!-- Example: V. Observability, VI. Versioning & Breaking Changes, VII. Simplicity -->
[PRINCIPLE_5_DESCRIPTION]
<!-- Example: Text I/O ensures debuggability; Structured logging required; Or: MAJOR.MINOR.BUILD format; Or: Start simple, YAGNI principles -->

## [SECTION_2_NAME]
<!-- Example: Additional Constraints, Security Requirements, Performance Standards, etc. -->

[SECTION_2_CONTENT]
<!-- Example: Technology stack requirements, compliance standards, deployment policies, etc. -->

## [SECTION_3_NAME]
<!-- Example: Development Workflow, Review Process, Quality Gates, etc. -->

[SECTION_3_CONTENT]
<!-- Example: Code review requirements, testing gates, deployment approval process, etc. -->

## Governance
<!-- Example: Constitution supersedes all other practices; Amendments require documentation, approval, migration plan -->

[GOVERNANCE_RULES]
<!-- Example: All PRs/reviews must verify compliance; Complexity must be justified; Use [GUIDANCE_FILE] for runtime development guidance -->

**Version**: [CONSTITUTION_VERSION] | **Ratified**: [RATIFICATION_DATE] | **Last Amended**: [LAST_AMENDED_DATE]
<!-- Example: Version: 2.1.1 | Ratified: 2025-06-13 | Last Amended: 2025-07-16 -->

<!--
Sync Impact Report
Version change: 0.0.0 → 1.0.0

Modified principles:
- N/A (initial constitution)

Added sections:
- Preamble
- Governance Metadata
- Core Principles
- Project-Management Layer Principles
- Governance & Compliance

Removed sections:
- N/A

Templates requiring updates:
- ⚠ /templates/plan-template.md (align plan checks with NOA self-contained, local-first, agentic rules)
- ⚠ /templates/spec-template.md (ensure specs always declare data locality, agent orchestration, and offline behavior)
- ⚠ /templates/tasks-template.md (add task categories for observability, offline readiness, P2P/cloud independence, and self-improvement loops)
- ⚠ /templates/commands/constitution.md (confirm wording matches this versioned constitution)
- ⚠ /templates/commands/specify.md (ensure it references this constitution for constraints)
- ⚠ /templates/commands/plan.md (enforce plan checks against constitution)
- ⚠ /templates/commands/tasks.md (ensure tasks are tagged and grouped by constitutional principles)
- ⚠ /templates/commands/implement.md (require validation against constitution before major merges)
- ⚠ README.md or project quickstart docs (document NOA constraints and project-mgmt focus)

Deferred items:
- TODO(RATIFICATION_DATE): Set actual original adoption date once agreed
-->

# NOA Project-Management – Project Constitution

## 1. Preamble

This constitution governs the NOA Project-Managemen: a local-first,
agentic operating system that functions as a hive-mind to orchestrate
and continuously improve a unified project-management environment.

The system MUST:
- Remain self-contained under the current user’s directory (`$user`),
- Prefer local-first, offline-capable behavior,
- Use agentic orchestration rather than single-purpose tools,
- Preserve transparency, auditability, and security,
- Support adaptive, self-improving behavior driven by real usage.

This document defines the non‑negotiable principles, governance rules,
and quality bars applied to all specifications, plans, tasks, and
implementations within this project.

## 2. Governance Metadata

- PROJECT_NAME: NOA Project-Management
- SCOPE: Local-first, agentic project-management layer that unifies and
  orchestrates multiple underlying tools and codebases as one OS-like
  experience for the user.
- RATIFICATION_DATE: TODO(RATIFICATION_DATE): Set the original adoption
  date for this constitution.
- LAST_AMENDED_DATE: 2025-12-02
- CONSTITUTION_VERSION: 1.0.0

### 2.1 Amendment Policy

- Any change to core principles, governance rules, or non‑negotiable
  constraints MUST be made via a documented amendment to this file.
- Amendments MUST:
  - Clearly state the reason and impact.
  - Be reflected in the Sync Impact Report at the top of this file.
  - Trigger a review of plan, spec, and tasks templates for consistency.
- Versioning:
  - MAJOR: Backward‑incompatible principle changes or removals.
  - MINOR: New principles or material expansion of guidance.
  - PATCH: Clarifications or wording changes that do not alter meaning.

### 2.2 Compliance & Review

- Every significant spec, plan, or implementation change MUST declare
  which principles it touches and how compliance was validated.
- Pull requests or change-sets MUST include:
  - A short “Constitution impact” note.
  - Links to impacted specs/plans/tasks.
- Regular review cadence:
  - At least once per major integration milestone (e.g., adding or
    refactoring a new repo into the project-management OS), conduct a
    constitution alignment review.

## 3. Core Principles (NOA Agentic OS)

### 3.1 Self-Contained & Autonomous

The system MUST operate entirely inside the current user’s directory
(`$user`), without hard dependencies on paths, services, or resources
outside that boundary.

- All code, configuration, models, and persistent state MUST resolve
  under the `$user` directory.
- No absolute host paths (e.g., `C:\Users\OtherUser\...`, `N:\shared\...`)
  may be baked into configuration or code.
- Any optional integration with external services MUST be:
  - Clearly isolated behind feature flags.
  - Safe to disable without breaking core local functionality.

Rationale: This preserves user ownership, portability, and the ability
to run NOA in constrained or air-gapped environments.

### 3.2 Local-First & Offline-Capable

The system SHOULD run with no network connectivity and MUST continue
to function for core project-management workflows when offline.

- All critical operations (creating/updating tasks, projects, views,
  and history) MUST work locally and persist to local storage.
- Third-party APIs and SaaS integrations MUST be optional enhancements
  controlled via feature flags or configuration.
- For every feature that relies on external connectivity, there MUST be:
  - A defined offline behavior or graceful degradation strategy.
  - Clear user feedback when remote behavior is unavailable.

Rationale: NOA’s long-term vision is complete offline capability and
full-stack ownership.

### 3.3 Agentic Orchestration & Hive-Mind

The system MUST use a network of specialized agents that collaborate
to solve complex, multi-step problems instead of one monolithic agent.

- Agents MAY specialize in planning, execution, QA, refactoring,
  integration, observability, or domain-specific tasks.
- Orchestration logic MUST:
  - Be explicit and inspectable.
  - Record which agents acted, with what inputs and outputs.
- Agents MUST be bound by this constitution: they cannot introduce
  behavior that violates self-contained, local-first, or security
  constraints.

Rationale: NOA is an agentic OS, not a single tool; the hive-mind
pattern is a core design invariant.

### 3.4 Adaptive & Self-Improving

The system SHOULD continuously learn from usage and evolve its own
processes, but MUST remain predictable, auditable, and reversible.

- Self-modifications MUST:
  - Be recorded with clear before/after diffs and rationales.
  - Be traceable to observed signals (e.g., failures, latency,
    user feedback).
- Any automatic refactor or upgrade MUST:
  - Preserve tests and add new ones when new behaviors are introduced.
  - Provide a rollback path.

Rationale: NOA is intended to self-upgrade, but not in opaque or
uncontrollable ways.

### 3.5 Transparent & Auditable

All significant decisions, actions, and modifications by agents MUST
be logged in a way that is human-reviewable.

- Logs MUST capture:
  - Who/what acted (agent ID, version).
  - Why (trigger, goal, key inputs).
  - What changed (high-level summary and references to diffs).
- Security or privacy-sensitive data MUST be protected, but the
existence and structure of actions must still be visible.

Rationale: Users need full oversight of an autonomous system that
touches their entire stack.

### 3.6 Security, Privacy & Full-Stack Ownership

The system MUST be secure by default and designed for eventual
full-stack control.

- Secrets MUST never be hard-coded, committed, or stored in
world-readable project files.
- Any integration that can exfiltrate data MUST be opt-in with clear
user intent.
- Users MUST be able to:
  - Run everything locally.
  - Control data residency.
  - Inspect and, if needed, replace any component.

Rationale: NOA replaces fragile SaaS chains; users must own their
infrastructure and data.

## 4. Project-Management Layer Principles

### 4.1 Unified Task & Project Model Across Repos

The project-management layer MUST present a unified logical model of
tasks, projects, and workflows even when implementations span multiple
repos and tools.

- Internal representation SHOULD normalize:
  - Tasks (work units).
  - Projects/initiatives (collections of tasks with goals).
  - Workflows/methods (e.g., BMAD, checklists, sprints).
- Integrations with underlying tools (e.g., existing apps, libraries,
or services) MUST map into this shared model rather than creating
isolated silos.

Rationale: Users interact with “their work,” not with arbitrary tool
boundaries.

### 4.2 Methodology-Aware (e.g., BMAD) but Tool-Agnostic

The system MUST support embedding methodologies like BMAD for
structured experimentation and decision-making, while remaining
agnostic to specific implementation details of any one repo.

- Methodology primitives (phases, experiments, checklists, reports)
MUST be modeled as first-class concepts layered onto tasks and
projects.
- No methodology implementation may hard-wire to a single UI or
storage engine; they MUST be accessible via the shared model.

Rationale: This allows multiple underlying codebases to participate
in a consistent project-management experience.

### 4.3 Progressive Integration of Existing Repos

Integration of additional repos into the project-management OS MUST be
iterative, traceable, and spec-driven.

- Integrations SHOULD proceed in clearly scoped passes (e.g., two repos
per Spec Kit cycle) rather than big-bang merges.
- Each integration pass MUST:
  - Have a spec that references this constitution.
  - Have a plan that identifies data flows and ownership boundaries.
  - Generate a task list that can be validated against these principles.
- New repos MUST not bypass the shared model or introduce conflicting
definitions of core concepts (task, project, workflow).

Rationale: Controlled integration reduces risk and preserves coherence.

### 4.4 Observability & Feedback into Self-Improvement

The project-management layer MUST capture signals about work and
workflow quality to feed back into NOA’s self-improvement loop.

- Metrics MAY include: task cycle time, failure rates, rework, agent
intervention frequency, and user overrides.
- Observability tasks MUST be explicit and visible in plans and task
lists, not implicit side effects.

Rationale: The OS can only self-improve if its project-management
surface is observable and measurable.

### 4.5 User-Centric Views & Dynamic UI

The UI for project management MUST be dynamic, context-aware, and
driven by user needs rather than raw tool structures.

- Views MUST:
  - Be able to reconfigure based on role, context, and current tasks.
  - Surface the most relevant projects, workflows, and actions.
- Agents MAY propose new views or layouts, but they MUST respect
usability, accessibility, and constitutional constraints.

Rationale: The UI is an adaptive dashboard, not a static app.

## 5. Governance & Compliance Rules

### 5.1 Spec Requirements

All specs produced under `/speckit.specify` MUST:

- Declare which constitutional principles they touch.
- Describe:
  - Data locality and offline behavior.
  - Agent orchestration responsibilities.
  - Security and observability considerations.
- Avoid vague language:
  - Use “MUST”, “SHOULD”, and “MAY” with clear rationale.

### 5.2 Plan Requirements

All plans produced under `/speckit.plan` MUST:

- Map high-level requirements to:
  - Specific repos and modules.
  - The unified project-management model.
  - Agent roles and responsibilities.
- Include:
  - Risk analysis tied to principles (e.g., risk to local-first
    behavior).
  - An explicit “Constitution compliance” checklist.

### 5.3 Task Requirements

All task lists produced under `/speckit.tasks` MUST:

- Tag tasks with the constitutional principles they support or protect.
- Include explicit tasks for:
  - Tests and validation.
  - Observability and logging.
  - Security, privacy, and data locality checks.
- Be small enough to execute and review incrementally.

### 5.4 Implementation Requirements

All implementations under `/speckit.implement` MUST:

- Keep tests passing and add tests for new behaviors.
- Provide before/after summaries that link back to specs, plans, and
tasks.
- Update this constitution when behavior materially shifts principles
or governance rules.

---

End of Constitution v1.0.0
