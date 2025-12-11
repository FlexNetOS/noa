# Combined Framework & Architecture of ark‑os‑noa

## High‑Level Overview

**ark‑os‑noa** is an **agentic AI platform** designed to realise ElementArk/DeFlex’s business model.  It combines hierarchical organisational patterns (NOA → Board Agents → MicroAgentStacks → microservices) with modern infrastructure techniques (Capsule/Full‑Illusion pattern, private data plane, event bus) and an adaptable AI layer (ModelSelectorAgents and Digest Agent).  The result is a **“hive mind”** of specialised agents capable of digesting, reasoning about and producing artefacts across software, data and SaaS systems.

## Layers & Hierarchy

### 1. Strategy & Orchestration Layer

- **NOA:** The ExecutiveCommanderChiefAgent at the top.  Transforms business goals into actionable plans, assigns Board Agents, sets policies, and monitors execution.
- **Board Agents:** Domain‑specific executives (Strategy/CTO, COO, CFO/FinOps, Legal/Compliance, Security, Growth/Partnerships, Digest).  Each can commission work via MicroAgentStacks and request ModelSelector assistance.

### 2. Execution Layer

- **MicroAgentStacks:** On‑demand work pods orchestrated by a CommandChiefAgent.  Each stack contains Operators, Adapters and Guards and runs through a defined lifecycle (Bootstrap → Execute → Validate → Package → Archive).  Stacks interact with external sources (repos, CRMs, APIs) and internal services via Adapters.
- **Expanded Digest Pipeline:** A set of microservices (Intake, Classifier, Graph Extract, Embeddings, Env Synthesis, Safety, Runner, Integrator, Registrar) that perform the actual work.  Each is loosely coupled via an event bus and runs inside the Capsule environment.  CRM Strangler and Model Serving are additional services.

### 3. Infrastructure Layer

- **Capsule Architecture (Full Illusion):** Encapsulates stacks and services in a sandbox that forwards build operations and network traffic to the outer runtime.  Capsule sidecars (Build‑Proxy, Service‑Mirror, Policy Agent, Telemetry Agent, optionally vcluster) provide the illusion of Docker‑in‑Docker and Kubernetes‑in‑Kubernetes without their drawbacks【716409907369096†L1037-L1067】.
- **Event Bus & Orchestration:** Redis Streams (primary) and optional NATS enable asynchronous communication.  A workflow engine coordinates the pipeline steps, handling retries and backoff.
- **Data Plane:** Private OCI registry, MinIO, Postgres (+ pgvector/Supabase) and optionally Qdrant.  This plane stores everything from container images to embeddings and ensures data stays within the trust boundary.
- **Observability & Security:** OTel tracing, Prometheus metrics, policy agents, SBOM/vulnerability scanners and secrets management.  The **no DinD** policy and user namespaces reduce privilege escalation risk【43537238352704†L1068-L1088】.

## How the Pieces Fit Together

1. **Goal Intake:** A high‑level goal arrives.  NOA normalises it into a WorkPlan and determines which Board Agents are responsible.
2. **Board Planning:** Board Agents refine the goal, assign budgets, define SLAs and set policies.  They request MicroAgentStacks and ModelSelectorAgents as needed.
3. **Stack Deployment:** For each task, a MicroAgentStack is spawned.  The stack uses Adapters to fetch sources (repos, CRMs), Operators to parse/analyse, and Guards to enforce policies.  Microservices implement the digest pipeline, orchestrated via the event bus.
4. **Model Selection & Execution:** When a service or operator needs AI inference (embeddings, summarisation, code explanation), it calls a ModelSelectorAgent.  The selected model is executed via local model servers or remote APIs.
5. **Data Persistence:** Outputs from each step (SBOMs, graphs, embeddings, demos) are persisted via the Data Plane.  The Registrar Service updates indexes and metadata.
6. **Completion & Reporting:** Once tasks finish, the stack packages results into a zip and compiled PDF, publishes them to MinIO and the registry, and updates Postgres.  NOA receives a report and archives the run.

## Why This Architecture?

1. **Modularity & Scalability:** By decomposing functionality into microservices and agents, ark‑os‑noa can scale horizontally and update components independently—avoiding the pitfalls of monolithic systems【43537238352704†L1068-L1088】.
2. **Security & Compliance:** The Capsule pattern, no DinD policy, private data plane and sidecar enforcement minimise the attack surface.  SBOMs, licences and vulnerability scans ensure supply‑chain integrity.
3. **Intelligence & Adaptability:** ModelSelectorAgents enable adaptive AI usage; the Digest Agent builds knowledge graphs and embeddings; the board can ingest CRMs and SaaS systems without downtime using the strangler proxy.
4. **Auditability & Provenance:** Every decision, model selection and action is logged in Postgres and associated with a run ID.  Artefacts are content‑addressed and signed.  This supports post‑mortems, compliance and future learning.

## Extensibility

* **New Board roles:** Additional executives (e.g. Marketing, HR) can be added by extending the roster and defining their domains and policies.
* **Additional microservices:** New processing stages (e.g. code transformers, simulation engines) can be plugged into the pipeline without redesigning the whole system.
* **Hybrid deployment:** While Compose is used locally, Kubernetes manifests (`k8s/`) can be applied to a cluster; the same Capsule pattern applies.
* **Model & Connector expansions:** New AI models are registered via the ModelSelector; new connectors are implemented by Adapters to integrate more SaaS or data sources.

The **Combined Framework & Architecture** unifies strategic planning, microservice execution, security and AI into a cohesive system.  It is intentionally modular to allow continuous growth and improvement.
