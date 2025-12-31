# Backend — Services & Infrastructure of ark‑os‑noa

## Purpose

The **backend** of ark‑os‑noa comprises all of the runtime services and infrastructure that turn high‑level plans into concrete work.  It includes the event bus, microservices that implement the **Expanded Digest Pipeline**, sidecars that enable the **Capsule** pattern, and internal data stores.  Together, these components provide a robust, scalable and secure environment for executing tasks, orchestrated by NOA and the Board Agents.

## Services & Microservices

### Core Pipeline Services

The digest‑everything pipeline is decomposed into a series of microservices, each responsible for a discrete stage.  Running them as independent services ensures that each can scale, fail and be updated independently, which is aligned with microservice best practices【43537238352704†L1068-L1088】.

1. **Intake Service:** Receives digest requests; validates inputs (repo URLs, API endpoints, model lists); creates provenance records and initializes workspace directories.
2. **Classifier Service:** Detects programming languages, build systems, service types (CLI, API, library) and licences.  Produces a `profile.json` summarising the source.
3. **Graph Extract Service:** Parses code and schemas to build call graphs, data flow graphs and config surfaces.  Supports multi‑language parsing (Python, JS/TS, Go, Rust, Java).  Outputs `kg.json` and `system_card.md`.
4. **Embeddings Service:** Generates embeddings for code and documentation using models selected by ModelSelectorAgents.  Upserts vectors to pgvector or Qdrant.
5. **Env Synthesis Service:** Emits Dockerfiles, docker‑compose YAML, Kubernetes manifests, `.env.example`, `Makefile` targets and config schemas.  Ensures reproducible builds using outer BuildKit (no DinD).
6. **Safety Service:** Runs SBOM generation (Syft), vulnerability scans (Grype/Trivy), secret scans (Gitleaks) and static analysis (Semgrep).  Applies policy gates; stops the pipeline on critical issues.
7. **Runner Service:** Builds and runs the source in a controlled container environment; executes existing tests or generates smoke tests; produces `demo.md`.
8. **Integrator Service:** Generates adapters (SDKs for Python, Node, Go), telemetry hooks and policy stubs; prepares packaging instructions.
9. **Registrar Service:** Writes outputs and metadata to storage (registry, MinIO, Postgres); registers embeddings; updates indexes for search.

### Auxiliary Services

* **CRM Strangler Proxy:** Provides a transparent layer between internal clients and an external CRM.  It records requests/responses, supports *shadow* and *write‑through* modes, and allows incremental internal re‑implementation of CRM features.
* **Model Serving:** Hosts local models using frameworks like llama.cpp, Ollama or vLLM.  Exposes endpoints for inference and embedding generation.  Each model server is packaged in its own container with health checks.
* **Gateway API:** A FastAPI service exposing endpoints: `/digest`, `/capsule/spawn`, `/crm/toggle`, `/models/ingest`, `/models/benchmark`, `/admin/*`.  Acts as the single entry point for external clients and the front‑end.

## Event Bus & Orchestration

* **Redis Streams:** Provides the primary event bus for inter‑service communication.  Services consume and produce events in a decoupled fashion.  The bus also supports message persistence and backpressure.
* **NATS (optional):** A lightweight publish/subscribe system for high fan‑out or cross‑cluster communication.  Enabled via a feature flag.
* **Workflow Engine:** A simple DAG engine built on Redis to coordinate pipeline tasks with retries and backoff.  Temporal or Argo Workflows can be integrated later for more sophisticated orchestrations.

## Capsule Sidecars

All containers run inside a “Capsule” to simulate container‑in‑container and Kubernetes‑in‑Kubernetes workflows without the security and performance drawbacks【716409907369096†L1037-L1067】.  Capsule sidecars include:

1. **Build‑Proxy:** A lightweight service that proxies inner `docker build` and `nerdctl` commands to the outer BuildKit daemon.  It exposes a local socket inside the Capsule but forwards build requests externally, avoiding duplicate layer storage.
2. **Service‑Mirror:** Watches inner service definitions and publishes corresponding services in the outer service mesh with mTLS and SLO configurations.  This allows inner services to be reachable and observable from the outer plane.
3. **Policy Agent (OPA):** Enforces egress rules, resource quotas, and other policies at the Capsule boundary.  It integrates with eBPF to block unauthorised traffic.
4. **Telemetry Agent:** Collects traces, metrics and logs from the inner services and sidecars.  It forwards data to the central observability stack with proper trace‑ID propagation.
5. **vcluster (optional):** Provides a lightweight Kubernetes API server inside the Capsule for tools that require kubectl.  It maps pods to the parent cluster’s nodes without duplicating container runtimes.

## Data Stores

* **OCI Registry:** Stores container images, compiled outputs and Capsule definitions.  The registry uses content‑addressed storage and enforces immutable tags.
* **MinIO:** Stores large artefacts, zipped deliverables, SBOMs and data sets.  Supports versioning and server‑side encryption.
* **Postgres (+ Supabase):** Maintains metadata (profiles, system cards, run logs), traces, job statuses and vector search indices.  Supabase provides developer APIs and pgvector integration.
* **Vector Store:** For embeddings.  The backend can be `pgvector` in Postgres or an external Qdrant instance.  A feature flag chooses which driver to enable.

## Security & Compliance

The backend enforces numerous policies:

- **No DinD:** Build operations are forwarded to outer BuildKit/containerd; containers run with user namespaces and seccomp, preventing container‑root escalation【43537238352704†L1068-L1088】.
- **Licence & vulnerability gates:** The Safety service halts builds on critical issues; the Board Agents define accepted licence lists and vulnerability thresholds.
- **Secrets management:** Secrets are never stored in environment variables.  They are mounted as files via Vault or similar systems, and sidecars are responsible for retrieving them.
- **Audit trails:** Every API call, pipeline event and model selection decision logs context (who, what, when, rationale).  These logs live in Postgres and are tied to run IDs.

## Development & Testing

* **Makefile:** Provides convenience targets (`make up`, `make down`, `make logs`, `make demo`, `make scan`, `make lock-images`) for developers.  It ensures consistent environment setup and teardown.
* **Docker‑Compose:** Defines services and dependencies; profiles enable optional components like NATS, Supabase and vcluster.  Compose is used for local development.  For production, manifests under `k8s/` can be applied to a Kubernetes cluster.
* **Automated tests:** Unit and integration tests run within the Runner Service; security scanners run in the Safety Service.  CI pipelines (to be implemented post‑launch) build images, run tests, generate SBOMs and publish artefacts.

By modularising the backend into clear services and infrastructure layers, ark‑os‑noa achieves the flexibility of microservices with the discipline of reproducible builds and strong security controls.
