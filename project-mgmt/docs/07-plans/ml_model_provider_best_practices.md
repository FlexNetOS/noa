
# Machine Learning & Model Provider Code Best Practices

> Working doc: opinionated, machine-first, designed for automation and multi-model providers.

---

## 1. Core Principles

1. **Determinism where possible**
   - Same inputs → same outputs (given same model + seed + data).
   - Fix random seeds for training and evaluation (`numpy`, `torch`, `random`, etc.).
   - Pin library versions in `requirements.txt` / `pyproject.toml` / lockfiles.

2. **Separation of concerns**
   - Data loading, feature engineering, model definition, training loop, evaluation, and deployment kept in separate modules.
   - No business logic inside training scripts; no training logic inside API handlers.

3. **Configuration, not code forks**
   - All dynamic behavior controlled via config (YAML/JSON/env vars), not by editing code.
   - One code path, many configurations.

4. **Reproducibility over cleverness**
   - Always be able to answer: “Which code, which data, which config produced this model or result?”

5. **Fail loud, fail fast**
   - Validate inputs and configs on startup.
   - Explicit errors > silent fallbacks.

---

## 2. Repository Structure

Add predictable layout such as:

```text
repo/
  README.md
  LICENSE
  pyproject.toml / requirements.txt
  src/
    app/                # API / services
    core/               # core abstractions (models, data, pipelines)
    data/               # dataset schemas, loaders
    features/           # feature engineering
    models/             # model architectures & wrappers
    training/           # training loops, schedules, callbacks
    evaluation/         # metrics, test sets, eval scripts
    providers/          # external model providers (OpenAI, local LLM, etc.)
  configs/
    data/
    training/
    evaluation/
    deployment/
  scripts/
    train_*.py
    evaluate_*.py
    export_*.py
  tests/
    unit/
    integration/
    regression/
  infra/
    docker/
    k8s/
    terraform/
  docs/
    architecture.md
    data_contracts.md
    api_reference.md
    model_cards/
  .github/
    workflows/
```

Key rules:
- **No code in the repo root** except `__main__.py` or entry shims.
- **No “magic scripts”** with everything in one file.

---

## 3. Coding Standards (Language-Agnostic)

1. **Style & linting**
   - Use a formatter (**Black**, **Prettier**), linter (**Ruff**, **ESLint**), and type checker (**mypy**, **pyright**, **tsc**).
   - Enforce in CI; treat lint/type errors as build failures.

2. **Type hints / static typing**
   - Add explicit types for public functions, model interfaces, and data structures.
   - Use `TypedDict` / `dataclasses` / `pydantic` models for request/response schemas.

3. **Small, composable functions**
   - Functions do one job, return one abstraction (not 12 things).
   - Avoid deeply nested conditionals; prefer explicit early returns.

4. **No hard-coded paths, secrets, or credentials**
   - Use env vars or config files.
   - Secrets managed via a vault (HashiCorp, AWS Secrets Manager, etc.).

5. **Logging**
   - Use structured logging (`json` logs).
   - Log levels: `DEBUG` (dev only), `INFO` (normal ops), `WARN`, `ERROR`.
   - Correlate logs by request ID / trace ID.

6. **Error handling**
   - Catch and wrap external errors (network, providers) in domain-specific exceptions.
   - Never silently drop exceptions; either handle or propagate.

---

## 4. Data & Metadata Best Practices (Machine-First)

1. **Schema-first**
   - Define schemas for datasets and model IO in a machine-readable format:
     - JSON Schema
     - Pydantic models
     - Protocol Buffers / Avro
   - Validate data on ingest and before training.

2. **Strict metadata**
   - Always attach:
     - `source` (system/component)
     - `timestamp` (with timezone or UTC)
     - `version` (schema version, data pipeline version)
     - `units` (for numeric values)
   - Maintain `data_contracts.md` describing schemas, units, ranges, and invariants.

3. **Immutable raw data**
   - Raw data is append-only, never mutated.
   - Derived/cleaned datasets are stored as separate artifacts with lineage metadata.

4. **Consistent formats**
   - For tabular/time-series: Parquet/Arrow with explicit schema.
   - For JSON APIs: minimize optional fields; document them.

5. **Quality checks**
   - Outlier detection, missing value rates, category drift, schema drift.
   - Enforce checks in CI for data pipelines (e.g., Great Expectations, custom checks).

---

## 5. Model Development Best Practices

### 5.1. Training

- **Config-driven training**
  - Hyperparameters (lr, batch size, scheduler, model name) live in config, not hard-coded.
  - Support config overrides via CLI flags.

- **Device & precision**
  - Single abstraction for device management (CPU/GPU/MPS/TPU).
  - Explicitly control precision (fp32, bf16, fp16) and log it.

- **Checkpoints**
  - Save:
    - model weights
    - optimizer state
    - scheduler state
    - training step/epoch
    - config + code commit hash
  - Store in a registry or artifact store (S3/GCS/MinIO).

- **Early stopping & monitoring**
  - Track validation metrics.
  - Early stop on plateau or overfitting, with clear rules.

### 5.2. Evaluation

- **Standardized metrics**
  - Use a single metrics module across training and offline evaluation.
  - Document metric definitions (e.g., what exactly is “accuracy” here?).

- **Golden datasets**
  - Maintain a small, hand-curated test set that must never be touched by training.
  - Use it for regression tests when models or providers change.

- **Reproducible evaluation scripts**
  - CLI entry point like:
    ```bash
    python -m evaluation.run --config configs/eval/model_X.yaml
    ```
  - No notebook-only logic; notebooks are for exploration, not production.

---

## 6. Model Versioning & Registry

1. **Semantic model versions**
   - `MAJOR.MINOR.PATCH`:
     - MAJOR: architecture/behavior changes.
     - MINOR: performance improvements with same interface.
     - PATCH: bug fixes / parameter tweaks.

2. **Model registry**
   - Store:
     - model version
     - training data snapshot ID(s)
     - config fingerprint
     - code commit hash
     - evaluation metrics
     - deployment status (staging/prod/deprecated)

3. **Immutable models, mutable aliases**
   - Model artifacts immutable (`model:sentiment:1.3.2`).
   - Aliases (`sentiment:latest`, `sentiment:prod`) point to versions.

4. **Rollback**
   - Always be able to roll back by switching alias to previous stable model.

---

## 7. Model Provider Interface Best Practices

This applies to external LLM APIs, internal microservices, and local model servers.

### 7.1. Request/Response Contracts

- Define strict, typed interfaces:
  - Example (pseudo-schema):
    ```json
    {
      "model": "string",
      "input": "string | array",
      "metadata": {
        "request_id": "string",
        "tenant_id": "string",
        "trace_id": "string"
      },
      "params": {
        "temperature": "float",
        "max_tokens": "int",
        "top_p": "float"
      }
    }
    ```
- Responses should include:
  - `model_version`
  - `latency_ms`
  - `usage` (tokens, cost estimate)
  - `error` object (if applicable)

### 7.2. Timeouts, retries, and idempotency

- **Timeouts**
  - Enforce client-side timeouts for provider calls.
  - Reasonable defaults, configurable per route.

- **Retries**
  - Retry only on retry-safe errors (e.g., 429, network issues).
  - Exponential backoff and jitter.
  - Avoid retry storms; cap max retries.

- **Idempotency**
  - For non-idempotent calls that can be safely retried (e.g., generation with explicit request ID), use idempotency keys.

### 7.3. Routing & multi-provider support

- Abstract provider calls behind a common interface:
  - `ProviderClient.generate()` / `ProviderClient.embed()` / `ProviderClient.chat()`.
- Provider routing strategies:
  - Failover: if provider A fails, fallback to B.
  - Tiered: cheap models first, premium models on demand.
  - Policy-based: route by tenant, data sensitivity, or jurisdiction.

- Centralize:
  - provider configs (keys, URLs, models)
  - rate limits
  - routing policies

### 7.4. Observability

- Log for every provider call:
  - request ID
  - provider + model
  - latency
  - status (success, error type)
  - usage metrics (tokens, cost)
- Export metrics to monitoring:
  - QPS, error rates, P95/P99 latency.
  - Per-provider and per-model breakdowns.

---

## 8. LLM & Prompt Engineering Best Practices

1. **Prompt templates as first-class artifacts**
   - Store prompts in version-controlled files, not embedded strings.
   - Use named placeholders: `{{user_input}}`, `{{context}}`, `{{tools}}`.

2. **Explicit instructions**
   - System messages define role, constraints, and allowed operations.
   - Clearly specify:
     - format (JSON, Markdown, etc.)
     - what to avoid (no speculation, no unsafe actions)
     - how to handle uncertainty (say “not sure” instead of guessing)

3. **Structured outputs**
   - Prefer JSON or well-defined Markdown sections that machines can parse.
   - Validate outputs and fail if schema is broken.

4. **Guardrails**
   - Safety filters on input + output.
   - Policies for:
     - PII handling
     - disallowed content
     - hallucination-sensitive domains (legal, medical, finance)

5. **Few-shot examples**
   - Include canonical examples for each pattern:
     - good vs bad responses
     - edge cases
   - Keep example set small, curated, and testable.

---

## 9. Testing Strategy

1. **Unit tests**
   - For pure functions: feature transforms, tokenization, schema validation, metrics.
   - High coverage for all deterministic logic.

2. **Integration tests**
   - Test E2E small flows:
     - data → model → output
     - API -> provider -> response
   - Use mock providers or sandbox keys.

3. **Model regression tests**
   - Golden test cases with expected behavior (or ranges).
   - If behavior changes, require human review + approval.

4. **Data contract tests**
   - Check that external producers still obey schemas.
   - Run in CI for ETL / ingestion pipelines.

5. **Load & performance tests**
   - Benchmark throughput and latency at expected and worst-case traffic.

---

## 10. Performance & Scaling

1. **Batching**
   - Batch requests to models when possible to improve throughput.
   - Tune batch sizes experimentally.

2. **Caching**
   - Cache deterministic model results when safe (e.g., embeddings for documents).
   - Use cache keys that include:
     - model version
     - input hash
     - parameters hash

3. **Resource management**
   - GPU utilization monitoring.
   - Limit concurrent requests per device.
   - Use queues for high-load scenarios.

4. **Cost awareness**
   - Track per-model and per-tenant cost.
   - Budgets and alerts for overages.

---

## 11. Security & Privacy

1. **Least privilege**
   - API keys scoped to minimum required permissions.
   - Separate keys per environment (dev/staging/prod).

2. **Data minimization**
   - Don’t send more data to providers than necessary.
   - Strip PII where possible; tokenize or hash IDs.

3. **Encryption**
   - TLS in transit.
   - Encryption at rest for sensitive data and model artifacts.

4. **Access control**
   - Role-based access to:
     - production models,
     - data,
     - config.
   - Audit logs for sensitive operations (e.g., model promotion to prod).

5. **Compliance**
   - Document data flows.
   - Tag data by jurisdiction or sensitivity level.

---

## 12. Deployment & CI/CD

1. **Immutable builds**
   - Build once, deploy same artifact across environments.
   - Docker images pinned to base image digests, not floating tags.

2. **Config per environment**
   - No code changes between dev/staging/prod.
   - Config injected via env vars or config maps.

3. **Progressive rollout**
   - Canary releases or traffic splitting:
     - e.g., 5% → 20% → 50% → 100%.
   - Automatic rollback on health check failures.

4. **CI checks**
   - Linting + formatting
   - Static typing
   - Unit + integration tests
   - Security scans (SAST/DAST)
   - Data pipeline tests (if applicable)

5. **Model deployment pipeline**
   - Triggered when:
     - model artifact is registered & approved
     - evaluation metrics exceed baseline
   - Pipeline:
     - fetch model artifact
     - spin up new instances
     - run smoke tests
     - perform canary rollout
     - promote or rollback

---

## 13. Documentation & Governance

1. **Model cards**
   - For each model:
     - purpose and domain
     - training data summary
     - known limitations
     - metrics and benchmarks
     - ethical considerations

2. **Runbooks**
   - Operational runbooks for:
     - incident response
     - provider outages
     - rollbacks
     - retraining procedures

3. **Architecture docs**
   - High-level diagrams for:
     - data flow
     - model serving topology
     - provider routing
     - observability stack

4. **Change management**
   - Every major change (models, pipelines, providers) tied to:
     - ticket/issue
     - changelog entry
     - owner

---

## 14. Anti-Patterns to Avoid

- One-off scripts that:
  - mix data munging, training, evaluation, and deployment in one file.
- Undocumented magic constants and thresholds.
- Using notebooks for production logic.
- Relying only on “latest” model without versioning or aliasing.
- Silent provider fallbacks that dramatically change behavior without traceability.
- Ignoring token usage and cost metrics until the bill arrives.

---

## Best Practices for Writing Modular and Reusable Code.

- 𝐌𝐚𝐢𝐧𝐭𝐚𝐢𝐧𝐚𝐛𝐢𝐥𝐢𝐭𝐲
• Follow standards (PEP 8!).
• Refactor consistently.
• Use code reviews to catch issues early.

- 𝐃𝐞𝐬𝐢𝐠𝐧
• Loose coupling → minimal dependency.
• High cohesion → one purpose, done well.
• Single Responsibility Principle → clarity and quality.

- 𝐃𝐨𝐜𝐮𝐦𝐞𝐧𝐭𝐚𝐭𝐢𝐨𝐧
• Clear README files.
• Comments explaining non-obvious logic.

- 𝐑𝐞𝐮𝐬𝐚𝐛𝐢𝐥𝐢𝐭𝐲 = 𝐒𝐩𝐞𝐞𝐝 + 𝐐𝐮𝐚𝐥𝐢𝐭𝐲
• Write generic, reusable modules.
• Use trusted libraries where possible.

- 𝐓𝐞𝐬𝐭𝐢𝐧𝐠:
• Unit tests for each module.
• High coverage.
• Mock dependencies where needed.

- 𝐒𝐭𝐫𝐮𝐜𝐭𝐮𝐫𝐞 & 𝐂𝐥𝐚𝐫𝐢𝐭𝐲
• Consistent naming.
• Small, focused functions.
• Refactor repeatedly.

- 𝐄𝐫𝐫𝐨𝐫 𝐇𝐚𝐧𝐝𝐥𝐢𝐧𝐠 𝐓𝐡𝐚𝐭 𝐒𝐚𝐯𝐞𝐬 𝐘𝐨𝐮
• Graceful exceptions.
• Meaningful logging.
• Fail-safes to prevent cascading failures.

- 𝐕𝐞𝐫𝐬𝐢𝐨𝐧 𝐂𝐨𝐧𝐭𝐫𝐨𝐥 𝐓𝐡𝐚𝐭 𝐒𝐜𝐚𝐥𝐞𝐬
• Git for tracking.
• Smart branching strategies.
• Clear commit messages.

- 𝐒𝐞𝐜𝐮𝐫𝐢𝐭𝐲 𝐌𝐚𝐭𝐭𝐞𝐫𝐬
• Input validation.
• Dependency hygiene.
• Regular audits.

- 𝐏𝐞𝐫𝐟𝐨𝐫𝐦𝐚𝐧𝐜𝐞 𝐎𝐩𝐭𝐢𝐦𝐢𝐳𝐚𝐭𝐢𝐨𝐧
• Profile before optimizing.
• Improve where it matters.
• Use caching smartly.


---

This file is intended as a living standard: extend it with project-specific rules, but keep the core principles intact: **reproducible, observable, secure, cost-aware, and model-agnostic.**
