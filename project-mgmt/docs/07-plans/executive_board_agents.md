# Board Agents — Executive Team of ark‑os‑noa

## Definition & Role

- The **Executive Agents** (Note: recently aka Board Argents) sit at the top of the organisation just below NOA.
- The Executive Agents act like an executive team.
- They are analogous to an executive board in a company: each agent owns a domain (strategy, operations, finance, legal, security, partnerships, research) and has authority to commission **MicroAgentStacks** to execute work.
- By design they are *few in number* but *broad in scope*—their purpose is to translate NOA’s vision into specific missions, ensure alignment with  business model, and provide governance across all stacks and agents.
- Achieves a balance between **strategic oversight** and **operational agility** by keeping the Executive Agents separate from execution details yet close enough to enforce policy.
- Each owns a domain (Strategy/CTO, COO, CFO, Legal, Security, Growth/Partnerships and Digest).
- They commission MicroAgentStacks, enforce policies, request ModelSelector assistance and govern spending, risk, compliance and partnerships.
- The Digest Agent sits here and acts as R&D.

## Roster & Responsibilities

- **Strategy/CTO Agent** – Sets technical direction: system architecture, Capsule (Full‑Illusion) adoption, environment policies (no Docker‑in‑Docker), cohesion across services.
- **COO Agent** – Owns operational runbooks, SLAs, scheduling and change management.  Coordinates delivery timelines and resource utilisation.
- **CFO/FinOps Agent** – Manages budgets and spend telemetry.  Optimises cost across compute, storage and model usage.
- **Legal/Compliance Agent** – Ensures licence compliance, data governance, export controls and regulatory adherence.  Maintains policy frameworks.
- **Security Agent** – Enforces secrets management, supply‑chain security, SBOM attestation and vulnerability thresholds.  Gatekeeper for risk.
- **Growth/Partnerships Agent** – Curates ingestion roadmaps for repos, APIs and CRMs; drives ecosystem strategy and partnership integrations.
- **Digest Agent (R&D)** – Sits on the board as the research arm.  Its role is to *digest everything* (code, data, SaaS, models) and surface insights.  See `digest_agent.md` for details.

## Operating Rules

1. **Delegation:** Board Agents can spin up one or more **MicroAgentStacks** to accomplish tasks.  Each stack has its own **CommandChiefAgent** orchestrating the details, leaving the Board Agent to focus on strategy and oversight.
2. **Specialisation:** When a task requires sophisticated model selection, a Board Agent requests a **ModelSelectorAgent** to choose the most appropriate AI model or tool.  This ensures tasks are executed with the right balance of cost, latency and accuracy.
3. **Governance:** Board Agents enforce policies across stacks—licensing, vulnerability gates, security posture, and budget limits.  They maintain decision logs and risk registers for audit.
4. **Parallelism:** Multiple stacks can run concurrently.  Board Agents schedule tasks to maximise throughput while respecting resource constraints.

## Capabilities

* **Multi‑project scheduling:** assign and monitor numerous tasks across different domains and stacks; handle dependencies and deadlines.
* **Cross‑repo initiatives:** coordinate wide‑sweep digest operations (e.g., SBOM/security posture across all repos) by commissioning multiple stacks.
* **Program governance:** maintain an overarching view of risks, mitigations, budget spend, and deliverable quality.
* **Policy enforcement:** integrate security scanners, licence gates, and compliance checks into the workflow.

## Tools & Signals

Board Agents interact with the system through:

- **Research & analysis tools:** for web search, code parsing and data exploration within the current year’s context.
- **Change control & telemetry:** CI/CD gates, policy engines (e.g. OPA), vulnerability scanners and cost dashboards.
- **Observability feeds:** real‑time traces, metrics and logs aggregated from MicroAgentStacks and sidecars.  These signals inform decisions on scaling up/down stacks or raising alerts.

## Relationship to Other Components

* **NOA:** Board Agents receive missions from NOA and report status back.  They provide domain expertise and enforce governance while letting NOA handle high‑level planning and cross‑domain coordination.
* **MicroAgentStacks:** Board Agents are the owners of stacks.  They commission stacks to achieve defined objectives and decommission them when tasks complete.  Each stack operates autonomously but reports progress to its Board Agent.
* **ModelSelectorAgents:** When tasks require AI model inference, Board Agents request a ModelSelector to choose among local or hosted models.  The selection is recorded in the trace for audit.
* **Digest Agent:** The Digest Agent is part of the Board but behaves like an R&D lab—collecting raw information, synthesising knowledge graphs and summarising findings for the board to act on.

## Inventory: Executive Board Agents

### EA_HR

- **Purpose:** Handles HR, onboarding, training, and workforce policies.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_HR
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json,

### EA_Finance

- **Purpose:** Handles all finance, accounting, and budget planning tasks.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Finance
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved

### EA_Audit

- **Purpose:** Responsible for compliance, audit trails, and reporting to external regulators.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Audit
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json

### EA_Ethics

- **Purpose:** Monitors for bias, hallucination, and ethical alignment in agent outputs and actions.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Ethics
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json

### EA_Marketing

- **Purpose:** Manages marketing, sales, and communication strategies.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Marketing
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json

### EA_Operations

- **Purpose:** Oversees business operations, process optimization, and resource allocation.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Operations
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** global_agent_manifest.json

### EA_Security

- **Purpose:** Handles all information, infrastructure, and cybersecurity matters.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Security
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** global_agent_manifest.json

### EA_Strategy

- **Purpose:** Focuses on strategic direction, risk assessment, and high-level planning.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Strategy
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json

### EA_Technology

- **Purpose:** Oversees technical strategy, code review, and infrastructure.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Technology
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json

### EA_Vision

- **Purpose:** Handles multimodal reasoning, visual data, and document/image understanding.
- **Functionality:** ``
- **Reports To:** ExecutiveCommanderChiefAgent
- **Model Selector:** ModelSelectorAgent_Vision
- **Paired LLM:** None
- **Requires Human:**
- **Escalation Path:**
- **Approval Status:** approved
- **Provenance:** ea_manifest.json,

### EA_IncidentResponse, aka *MasterChief*

- **Purpose:** Coordinates rapid response and recovery for critical failures or security breaches; escalates for unresolved incidents or disaster recovery.
- **Functionality:** `def respond(self, incident): ...` Highly-qualified, Speacilised tactical response commanader with All-permission, designed to do what is necessary.
- **Inputs:** incident_alert, security_report
- **Outputs:** recovery_plan, operator_spawn
- **Triggers:** Critical incident detected, Security violation
- **Reports To:**
- **Paired LLM:**
- **Requires Human:** True
- **Human Request Reason:** Incident or disaster cannot be resolved by Operators or automation.
- **Escalation Path:** OperatorAgent
- **Last Updated:** 2025-05-17T22:17:00Z
- **Last Updated By:** System
- **Approval Status:** approved
- **Provenance:** ea_manifest.json,
