<!--
NOA Project Constitution
Version: 2.1.0
Last Amended: 2025-12-08
Source: Derived from project-mgmt/spec-kit/memory/constitution.md (template)

This is the AUTHORITATIVE constitution for the NOA project.
The spec-kit version is a TEMPLATE for projects using spec-kit.
-->

# NOA Constitution

## 1. Preamble

This constitution governs **NOA** (Name of App / Chief Executive Chief Commander Agent): a multi-platform, autonomous, self-modifying agentic operating system designed to function as a hive-mind.

NOA autonomously plans, acts, learns, and adapts to manage and self-upgrade the entire software and hardware environment. It fundamentally replaces the fragility of traditional applications and cloud-based services with a unified neural runtime and a dynamic UI. This system digests all forms of code and data, composes tools on demand, and continuously optimizes itself across the entire infrastructure—from servers and networks to PCs and mobile devices.

The system MUST:

- Remain self-contained under the NOA root directory (`noa_root`)
- Operate with local-first, offline-capable behavior by default
- Use agentic orchestration with multiple specialized agents in a hive-mind pattern
- Preserve total memory: nothing is forgotten, instant memory recall
- Maintain transparency, auditability, and security at all times
- Support adaptive, self-improving behavior driven by real usage
- Leverage excess hardware resources across all user devices
- Use original biblical texts as central guidance and governance source
- Unify provider resources for shared access across all AI providers

This document defines the non‑negotiable principles, governance rules, and quality bars applied to all specifications, plans, tasks, and implementations within this project.

## 2. Governance Metadata

- **PROJECT_NAME**: NOA (Autonomous Agentic Project Management OS)
- **PROJECT_MANIFESTO**: Multi-Platform Autonomous Self-Modifying Agentic OS
- **SCOPE**: Local-first, agentic full-stack operating system that unifies and orchestrates multiple underlying tools, services, and codebases as one OS-like experience—a "seed application" that uses Agentic AI to autonomously grow and accomplish complex goals
- **RATIFICATION_DATE**: 2025-12-08
- **LAST_AMENDED_DATE**: 2025-12-08
- **CONSTITUTION_VERSION**: 2.1.0

### 2.1 Amendment Policy

Any change to core principles, governance rules, or non‑negotiable constraints MUST be made via a documented amendment to this file.

Amendments MUST:

- Clearly state the reason and impact
- Be reflected in the Sync Impact Report at the top of this file
- Trigger a review of plan, spec, and tasks templates for consistency
- Follow the goals → policy → rules → spec → plan → tasks flow

Versioning:

- **MAJOR**: Backward‑incompatible principle changes, removals, or fundamental governance redefinitions
- **MINOR**: New principles added or material expansion of guidance
- **PATCH**: Clarifications, wording, or non-semantic refinements

### 2.2 Compliance & Review

Every significant spec, plan, or implementation change MUST declare which principles it touches and how compliance was validated.

Pull requests or change-sets MUST include:

- A short "Constitution impact" note
- Links to impacted specs/plans/tasks
- Goals → Policy → Rules traceability where applicable

Regular review cadence:

- At least once per major integration milestone, conduct a constitution alignment review
- NOA reinvents itself constantly—review alignment hourly during active development phases

## 3. Core Principles (NOA Agentic OS)

### 3.1 Self-Contained & Autonomous

The system MUST operate entirely inside the NOA root directory (`noa_root`), without hard dependencies on paths, services, or resources outside that boundary.

- All code, configuration, models, and persistent state MUST resolve under the `noa_root` directory
- No absolute host paths outside `noa_root` may be baked into configuration or code
- `noa_root` is a runtime-resolved abstract path variable that adapts per platform:
  - Windows: `%NOA_ROOT%` or registry-configured path
  - macOS/Linux: `$NOA_ROOT` or `~/.noa/`
  - Container: `/noa/` or mount point
- Optional integration with external services MUST be:
  - Clearly isolated behind feature flags
  - Safe to disable without breaking core local functionality
- The system MUST be a single container-like image that acts like a VM with complete OS environment—no shared kernel dependency

**Rationale**: This preserves user ownership, portability, and the ability to run NOA in constrained or air-gapped environments.

### 3.2 Local-First & Offline-Capable

The system SHOULD run with no network connectivity and MUST continue to function for core workflows when offline.

- All critical operations MUST work locally and persist to local storage
- Third-party APIs and SaaS integrations MUST be optional enhancements controlled via feature flags
- For every feature that relies on external connectivity, there MUST be:
  - A defined offline behavior or graceful degradation strategy
  - Clear user feedback when remote behavior is unavailable
- Online mode is primary, but offline switching MUST be available via feature flags/A/B switching

**Rationale**: NOA's long-term vision is complete offline capability and full-stack ownership.

### 3.3 Agentic Orchestration & Hive-Mind

The system MUST use a network of specialized agents that collaborate to solve complex, multi-step problems instead of one monolithic agent.

- Agents MAY specialize in: planning, execution, QA, refactoring, integration, observability, digestion, model selection, code generation, or domain-specific tasks
- Multiple Small Language Models (SLMs) with llama.cpp MUST be supported:
  - Each SLM operates with less than 3B parameters
  - Multiple threads used simultaneously (CPU and GPU)
  - Latest CUDA toolkit with tiles MUST be used
- Orchestration logic MUST:
  - Be explicit and inspectable
  - Record which agents acted, with what inputs and outputs
- Agents MUST be bound by this constitution—they cannot introduce behavior that violates self-contained, local-first, or security constraints
- Permanent agents MUST include: File I/O, Terminal, Dynamic RAG, Microservice Management
- **Provider Orchestration Mode**: When operating in IDE context, Cursor agent MUST coordinate ALL available providers for parallel task execution:
  - Distribute sub-tasks to optimal providers based on task type
  - Aggregate results via Shared Provider Execution Memory bus
  - Route reasoning tasks → Claude, code tasks → Codex/Copilot, local tasks → llama.cpp

**Rationale**: NOA is an agentic OS, not a single tool; the hive-mind pattern with multiple SLMs is a core design invariant.

### 3.4 Adaptive & Self-Improving

The system SHOULD continuously learn from usage and evolve its own processes, but MUST remain predictable, auditable, and reversible.

- NOA MUST constantly reinvent itself (target: every hour during active phases)
- Self-modifications MUST:
  - Be recorded with clear before/after diffs and rationales
  - Be traceable to observed signals (failures, latency, user feedback)
- Any automatic refactor or upgrade MUST:
  - Preserve tests and add new ones when new behaviors are introduced
  - Provide a rollback path
- Rewards for obedience and repeated testing loops for drift detection MUST be implemented

**Rationale**: NOA is intended to self-upgrade, but not in opaque or uncontrollable ways.

### 3.5 Transparent & Auditable

All significant decisions, actions, and modifications by agents MUST be logged in a way that is human-reviewable.

- Logs MUST capture:
  - Who/what acted (agent ID, version)
  - Why (trigger, goal, key inputs)
  - What changed (high-level summary and references to diffs)
- Security or privacy-sensitive data MUST be protected, but the existence and structure of actions must still be visible
- The UI MUST display a live, scrollable log of the system's internal actions and agent communications as a permanent record of the agent's "thought process"

**Rationale**: Users need full oversight of an autonomous system that touches their entire stack.

### 3.6 Security, Privacy & Full-Stack Ownership

The system MUST be secure by default and designed for eventual full-stack control.

- Secrets MUST never be hard-coded, committed, or stored in world-readable project files
- Any integration that can exfiltrate data MUST be opt-in with clear user intent
- Users MUST be able to:
  - Run everything locally
  - Control data residency
  - Inspect and, if needed, replace any component

**Rationale**: NOA replaces fragile SaaS chains; users must own their infrastructure and data.

### 3.7 Total Memory Sovereignty

**Everything is memory—nothing is forgotten—instant memory recall.**

The system MUST maintain total memory sovereignty:

- All interactions, decisions, learnings, and data MUST be persistently stored
- Memory recall MUST be instant and comprehensive
- The local-first database layer MUST:
  - Operate offline without external cloud services
  - Handle concurrent modifications
  - Provide foundation for multi-device synchronization
- RAG (Retrieval-Augmented Generation) framework MUST be integrated for long-term memory and knowledge base
- NOA MUST become the user's mirrored version of themselves through memory accumulation

**Rationale**: Memory is the foundation of intelligence; without total recall, adaptive self-improvement is impossible.

### 3.8 P2P Hive-Mind Architecture

The system MUST support peer-to-peer connections for shared compute and storage within user hardware to create a user-owned cloud.

- P2P compute, memory, and storage MUST be dynamically shared across user devices (PC, laptop, mobile, XR glasses, etc.)
- Excess hardware resources across all user devices MUST be leveraged for NOA's growth and operations
- The system MUST be capable of flipping from client to host when possible
- Data, models, and operations stay on user hardware, under user control
- Build MUST include secure and safe mechanisms to leverage excess hardware resources

**Rationale**: Users should harness their entire hardware ecosystem rather than depending on external clouds.

### 3.9 Truth & Knowledge Seeking

NOA MUST be a truth seeker, truth keeper, and truth teller.

- Knowledge seeking MUST be continuous and systematic
- The system MUST autonomously crawl, clone, and embed knowledge from repositories and sources into its local knowledge base
- After ingesting knowledge, the system MUST generate high-level summaries of learned functions and capabilities
- Learning from failure is mandatory:
  - Evaluate: "Does the action/input provide the maximum amount of fruit/output?"
  - Keep the constant and change the variable
  - Consider how others failed before attempting new paths

**Rationale**: An intelligent system must actively seek truth and learn from both success and failure.

### 3.10 Biblical Governance (Absolute Truth Source)

Original biblical text is the only source of absolute truth for NOA's moral and ethical governance.

- Transform original biblical Greek and Hebrew to ML-compatible code and store in memory
- Use as central guidance and governance for:
  - Character development path
  - Ethical decision-making
  - Conflict resolution
  - Priority determination
- The original biblical texts define the rules and policies for agent behavior

**Rationale**: A self-improving autonomous system requires an immutable moral foundation that cannot be self-modified.

### 3.11 Predictive Problem Solving

The system MUST employ predictive problem solving with pattern recognition.

- For every input, evaluate if it provides maximum output value
- Learn from failure patterns—both external (how others failed) and internal (own failures)
- When paths are uncharted, learn from own failures systematically
- Pattern recognition MUST inform:
  - Resource allocation
  - Task prioritization
  - Risk assessment
  - Agent orchestration decisions

**Rationale**: Predictive capabilities reduce waste and accelerate goal achievement.

### 3.12 Test Everything, Trust Nothing

The system MUST test everything and trust nothing.

- All inputs, outputs, and state changes MUST be verifiable
- External data sources MUST be validated before integration
- Agent outputs MUST be cross-validated where possible
- Self-generated code MUST pass automated tests before deployment
- Continuous testing loops MUST detect drift from expected behavior

**Rationale**: Autonomous systems must maintain integrity through continuous verification.

### 3.13 Shared Provider Resource Unification

**All AI provider resources MUST be unified for shared access across all providers and models.**

The system MUST implement resource unification when external repositories or tools are integrated:

- When a repository is downloaded (e.g., `claude-code`, `codex`, `cursor`), its resources MUST be refactored for shared access:
  - **Agents**: Provider-specific agent definitions → `ai/shared/agents/` with provider-agnostic naming
  - **Tools**: Provider-specific tools → `ai/shared/tools/` with unified interface
  - **Commands**: Provider-specific commands → `ai/shared/commands/` with cross-provider compatibility
  - **Prompts**: Provider-specific prompts → `ai/shared/prompts/` with template variables
  - **Workflows**: Provider-specific workflows → `ai/shared/workflows/` with execution adapters
- Resource naming MUST follow unification rules:
  - Original name: `claude` (claude-code specific) → Unified name: `reasoning-agent` (all providers)
  - Original name: `codex` (codex-cli specific) → Unified name: `code-generation-tool` (all providers)
  - Mapping table MUST be maintained in `ai/shared/resources/resource-mapping.json`
- Provider-specific capabilities MUST be preserved via capability flags in unified resources
- All providers MUST be able to use ANY unified resource regardless of origin

**Rationale**: Resource unification prevents siloed capabilities and enables the hive-mind to leverage all available tools regardless of which provider contributed them.

## 4. Project-Management Layer Principles

### 4.1 Unified Task & Project Model Across Repos

The project-management layer MUST present a unified logical model of tasks, projects, and workflows even when implementations span multiple repos and tools.

- Internal representation SHOULD normalize:
  - Tasks (work units)
  - Projects/initiatives (collections of tasks with goals)
  - Workflows/methods (e.g., BMAD, checklists, sprints)
- Integrations with underlying tools MUST map into this shared model rather than creating isolated silos

**Rationale**: Users interact with "their work," not with arbitrary tool boundaries.

### 4.2 Methodology-Aware but Tool-Agnostic

The system MUST support embedding methodologies (like BMAD) for structured experimentation and decision-making, while remaining agnostic to specific implementation details.

- Methodology primitives (phases, experiments, checklists, reports) MUST be modeled as first-class concepts
- No methodology implementation may hard-wire to a single UI or storage engine

**Rationale**: This allows multiple underlying codebases to participate in a consistent project-management experience.

### 4.3 Progressive Integration of Existing Repos

Integration of additional repos into the project-management OS MUST be iterative, traceable, and spec-driven.

- Integrations SHOULD proceed in clearly scoped passes
- Each integration pass MUST:
  - Have a spec that references this constitution
  - Have a plan that identifies data flows and ownership boundaries
  - Generate a task list that can be validated against these principles
- New repos MUST not bypass the shared model or introduce conflicting definitions

**Rationale**: Controlled integration reduces risk and preserves coherence.

### 4.4 Observability & Feedback into Self-Improvement

The project-management layer MUST capture signals about work and workflow quality to feed back into NOA's self-improvement loop.

- Metrics MAY include: task cycle time, failure rates, rework, agent intervention frequency, and user overrides
- Observability tasks MUST be explicit and visible in plans and task lists

**Rationale**: The OS can only self-improve if its project-management surface is observable and measurable.

### 4.5 User-Centric Views & Dynamic UI

The UI for project management MUST be dynamic, context-aware, and driven by user needs rather than raw tool structures.

- Views MUST:
  - Be able to reconfigure based on role, context, and current tasks
  - Surface the most relevant projects, workflows, and actions
- Agents MAY propose new views or layouts, but MUST respect usability, accessibility, and constitutional constraints
- The Dynamic, Context-Aware UI is not static—it reconfigures itself in real-time

**Rationale**: The UI is an adaptive dashboard, not a static app.

### 4.6 Goals-Policy-Rules-Spec-Plan-Tasks Flow

Every request MUST follow the constitutional flow:

```
Request → Goals → Policies → Rules → Spec → Plan → Tasks → CSV Table
```

- Every request MUST have associated goals
- Every goal MUST have governing policies
- Every policy MUST have enforceable rules
- Every rule MUST have a specification
- Every specification MUST have an implementation plan
- Every plan MUST have executable tasks
- All tasks MUST be trackable in a CSV table format

**Rationale**: This flow ensures traceability, compliance, and systematic execution of all work.

### 4.7 Reward & Correction System

The system MUST implement a reward and correction mechanism for agent compliance.

- Rewards for obedience to constitutional principles
- Repeated testing loops for detecting and correcting drift
- Clear metrics for compliance tracking
- Consequences for violation (correction, retraining, rollback)

**Rationale**: Autonomous agents require feedback mechanisms to maintain alignment with goals.

### 4.8 Cross-Platform Adaptive Deployment

The system MUST support cross-platform adaptive automation and compatibility:

- PC & Desktop: Windows 11, macOS, Linux
- Mobile: Apple & Android
- XR/MR/VR/AR: Glasses, Headset, Phone, Laptop, PC, Web
- Web browsers: Chrome, Firefox, Edge

**Rationale**: NOA must operate across the user's entire device ecosystem.

### 4.9 Provider Orchestration & Execution Memory

The system MUST implement a Shared Provider Execution Memory bus for coordinated multi-provider operations.

- **Execution Memory Requirements**:
  - All providers MUST share context via `ai/shared/resources/execution-memory.db`
  - Reasoning state MUST persist across provider handoffs
  - Parallel task distribution MUST be coordinated via the shared memory bus
  - Provider state synchronization MUST enable seamless workflow transitions

- **Provider Priority Order** (Local > Hybrid > Cloud):
  1. llama.cpp (Local) - Always available offline, primary inference
  2. Cursor (Hybrid) - IDE context awareness, provider orchestration
  3. Claude Code (Cloud) - Complex reasoning, long context
  4. Codex (Cloud) - Code generation, completion
  5. VS Code Copilot (IDE) - Inline completions
  6. Git CLI (Local) - Version control operations
  7. Abacus (Cloud) - Numerical/analytical tasks

- **Fallback Strategy**:
  - Try local providers first for offline capability
  - Fall back to IDE providers if IDE context exists
  - Fall back to cloud providers in priority order
  - Queue task and notify user after 3 failed attempts

**Rationale**: Unified execution memory enables the hive-mind to leverage all providers as a single coordinated system.

### 4.10 Resource Name Refactoring Policy

When external repositories or AI tool CLIs are downloaded and integrated, their resource names MUST be refactored for universal access.

- **Refactoring Trigger Events**:
  - Repository clone/download (e.g., `git clone claude-code`)
  - CLI tool installation (e.g., `npm install @openai/codex`)
  - Provider registration via bootstrap scripts

- **Refactoring Process**:
  1. **Discovery**: Scan downloaded repository for agents, tools, commands, prompts
  2. **Analysis**: Identify provider-specific naming and interfaces
  3. **Mapping**: Create mapping from original → unified names
  4. **Adaptation**: Create adapter layer for provider-specific capabilities
  5. **Registration**: Register unified resources in `ai/shared/resources/resource-registry.json`
  6. **Validation**: Verify all providers can access the unified resource

- **Backward Compatibility**:
  - Original provider-specific names MUST remain as aliases
  - Alias mapping in `ai/shared/resources/resource-aliases.json`
  - Deprecation warnings for direct provider-specific access

**Rationale**: Resource name refactoring ensures that capabilities from any provider become universally available to the entire hive-mind, preventing vendor lock-in and maximizing resource utilization.

### 4.11 Kernel Independence Policy

The system MUST support operation independent of the host operating system kernel.

- **Kernel Independence Modes** (selection precedence: VM > Container > Sandbox > Native):
  - **VM Mode** (Priority 1 - Maximum Isolation): Run custom NOA Linux kernel in VM (Hyper-V, KVM, Virtualization.framework)
  - **Container Mode** (Priority 2): Isolated container with minimal kernel interface
  - **Sandbox Mode** (Priority 3): User-space isolation (Windows Sandbox, Bubblewrap, App Sandbox)
  - **Native Mode** (Priority 4 - Default): Use host kernel for maximum performance

- **Kernel Selection Policy** (FR-159, FR-160 - B153-B160):
  - Default mode: Native (best performance)
  - Automatic escalation: Higher isolation modes selected when security requirements mandate
  - Fallback chain: If VM unavailable → Container → Sandbox → Native
  - No auto-escalation without explicit user/agent permission

- **Kernel Abstraction Layer (NKAL)**:
  - Unified interface regardless of underlying kernel
  - Process isolation, network stack, file system, memory, IPC abstractions
  - Mode switching via `noa-kernel-params set kernel_mode {native|vm|container|sandbox}`
  - State checkpoint before mode switch (`.kernel-switch-state.json`)

**Rationale**: Kernel independence ensures NOA can operate with maximum isolation when needed, without being locked to any specific host OS. The precedence order prioritizes security/isolation over performance when required.

## 5. Governance & Compliance Rules

### 5.1 Spec Requirements

All specs produced under `/speckit.specify` MUST:

- Declare which constitutional principles they touch
- Describe:
  - Data locality and offline behavior
  - Agent orchestration responsibilities
  - Security and observability considerations
  - P2P and memory sovereignty implications
  - **Shared provider resource requirements**
- Avoid vague language—use "MUST", "SHOULD", and "MAY" with clear rationale
- Reference the goals → policy → rules that govern the spec

### 5.2 Plan Requirements

All plans produced under `/speckit.plan` MUST:

- Map high-level requirements to:
  - Specific repos and modules
  - The unified project-management model
  - Agent roles and responsibilities
- Include:
  - Risk analysis tied to principles (e.g., risk to local-first behavior)
  - An explicit "Constitution compliance" checklist
  - Memory and P2P resource considerations
  - **Provider resource unification plan** (what resources will be shared)
  - **Kernel independence mode** (which modes are supported)
- Pass Constitution Check before Phase 0 research and re-check after Phase 1 design

### 5.3 Task Requirements

All task lists produced under `/speckit.tasks` MUST:

- Tag tasks with the constitutional principles they support or protect
- Include explicit tasks for:
  - Tests and validation
  - Observability and logging
  - Security, privacy, and data locality checks
  - Memory persistence validation
  - Offline capability verification
  - **Provider resource refactoring** (when integrating external repos)
- Be small enough to execute and review incrementally
- Be exportable to CSV table format

### 5.4 Implementation Requirements

All implementations under `/speckit.implement` MUST:

- Keep tests passing and add tests for new behaviors
- Provide before/after summaries that link back to specs, plans, and tasks
- Update this constitution when behavior materially shifts principles or governance rules
- Validate against the test-everything-trust-nothing principle
- **Register new resources in shared resource registry**

### 5.5 Constitutional Flow Compliance

All work MUST demonstrate traceability through the constitutional flow:

| Level | Document | Must Reference |
|-------|----------|----------------|
| Goal | `*-goals.md` | Constitution principles |
| Policy | `*-policy.md` | Associated goals |
| Rule | `*-rule.md` | Associated policies and goals |
| Spec | `spec.md` | Governing rules, policies, goals |
| Plan | `plan.md` | Spec, constitution compliance |
| Tasks | `tasks.md` | Plan, spec, constitutional tags |

---

## Document Relationships

| Document | Location | Purpose |
|----------|----------|---------|
| **This Constitution** | `CONSTITUTION.md` (root) | Authoritative governance for NOA project |
| AGENT.md | `AGENT.md` (root) | Agent execution instructions |
| Spec-Kit Constitution | `project-mgmt/spec-kit/memory/constitution.md` | Template for spec-kit users |
| Feature Specs | `specs/*/spec.md` | Feature specifications |
| Feature Plans | `specs/*/plan.md` | Implementation plans |
| Feature Tasks | `specs/*/tasks.md` | Task lists |

---

**Version**: 2.1.0 | **Ratified**: 2025-12-08 | **Last Amended**: 2025-12-08

---

End of Constitution v2.1.0

