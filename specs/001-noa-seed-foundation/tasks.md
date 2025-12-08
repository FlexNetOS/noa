# Tasks: NOA Seed Foundation

**Input**: Design documents from `/specs/001-noa-seed-foundation/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story?] [Principle?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: US1-US8 from spec.md
- **[Principle]**: Constitutional principle tag (§3.1-§4.6)

## Constitutional Principle Tags

| Tag | Principle |
|-----|-----------|
| §3.1 | Self-Contained & Autonomous |
| §3.2 | Local-First & Offline-Capable |
| §3.3 | Agentic Orchestration |
| §3.4 | Adaptive & Self-Improving |
| §3.5 | Transparent & Auditable |
| §3.6 | Security & Privacy |
| §3.7 | Total Memory Sovereignty |
| §3.8 | P2P Hive-Mind |
| §3.12 | Test Everything, Trust Nothing |
| §4.6 | Goals-Policy-Rules Flow |

## User Story Summary

| Story | Title | Priority | MVP |
|-------|-------|----------|-----|
| US1 | Initialize NOA Seed Environment | P1 | ✅ |
| US2 | Multi-SLM Neural Runtime | P1 | ✅ |
| US3 | Total Memory Sovereignty | P1 | ✅ |
| US4 | Digest Everything Pipeline | P2 | |
| US5 | Dynamic Context-Aware UI | P2 | |
| US6 | P2P Hive-Mind Device Federation | P2 | |
| US7 | Autonomous Agent Orchestration | P2 | |
| US8 | Self-Improvement & Code Modification | P3 | |

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and monorepo structure

- [ ] T001 §3.1 Create monorepo directory structure per plan.md in `noa_root/`
- [ ] T002 [P] §3.1 Initialize Rust workspace with Cargo.toml (all crates: api, embedder, trainer, indexer, ui, agent, common) in `noa_root/sys/core/Cargo.toml`
- [ ] T003 [P] §3.1 Initialize Go module for P2P services in `noa_root/p2p/go.mod`
- [ ] T004 [P] §3.1 Initialize TypeScript/Next.js project for UI in `noa_root/sys/ui/package.json`
- [ ] T005 [P] §3.1 Initialize Python project for digest pipeline in `noa_root/sys/digest/pyproject.toml`
- [ ] T006 [P] Configure linting: rustfmt, golangci-lint, eslint, ruff in `.config/`
- [ ] T007 [P] Create cross-platform build scripts in `scripts/bash/build.sh` and `scripts/powershell/build.ps1`
- [ ] T008 §3.1 Create environment configuration templates in `config/`
- [ ] T009 [P] Setup GitHub Actions CI pipeline in `.github/workflows/ci.yml`
- [ ] T010 Create README.md with quickstart instructions in `noa_root/README.md`

---

## Phase 2: Foundational - Database & Storage Infrastructure

**Purpose**: Core storage infrastructure required by ALL user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Storage Components Setup

- [ ] T012 §3.2 Create data directory structure (`data/memory/`, `data/knowledge/`, `data/embeddings/`, `data/artifacts/`) in `noa_root/data/`
- [ ] T013 [P] Setup Private OCI Registry configuration in `containers/oci/registry.yaml`
- [ ] T014 [P] Setup MinIO S3-compatible storage config in `config/minio.yaml`
- [ ] T015 [P] Setup Postgres/SQLite configuration in `config/database.yaml`
- [ ] T016 [P] Setup Qdrant vector store configuration in `config/qdrant.yaml`
- [ ] T017 [P] Setup Quickwit hybrid search configuration in `config/quickwit.yaml`

### Database Schema (SQLite Primary)

- [ ] T018 §3.2 Create complete SQLite schema with all 14 entities from data-model.md in `init/migrations/001_initial.sql`
- [ ] T019 [P] §3.7 Create Memory table with JSON metadata and checksum in `init/migrations/001_initial.sql`
- [ ] T020 [P] §3.7 Create Embedding table with vector column (384-dim) in `init/migrations/001_initial.sql`
- [ ] T021 [P] §3.3 Create Agent table with stack_code support (mas_*, gen_mas) in `init/migrations/001_initial.sql`
- [ ] T022 [P] §3.5 Create AgentLog table (append-only audit) in `init/migrations/001_initial.sql`
- [ ] T023 [P] §3.3 Create Task table with priority, retry_count in `init/migrations/001_initial.sql`
- [ ] T024 [P] Create TaskEvent table with lifecycle events in `init/migrations/001_initial.sql`
- [ ] T025 [P] §3.3 Create MicroAgentStack table with 5-stage lifecycle in `init/migrations/001_initial.sql`
- [ ] T026 [P] Create Capsule table with manifest checksum in `init/migrations/001_initial.sql`
- [ ] T027 [P] Create KnowledgeNode table (type: function/class/module/file/repo/concept) in `init/migrations/001_initial.sql`
- [ ] T028 [P] Create KnowledgeEdge table (relationship: calls/imports/extends/implements/contains/references) in `init/migrations/001_initial.sql`
- [ ] T029 [P] Create DigestSource table (type: repository/file/api/document) in `init/migrations/001_initial.sql`
- [ ] T030 [P] Create Model table with provider, parameters, context_length in `init/migrations/001_initial.sql`
- [ ] T031 [P] §3.8 Create Device table with platform, peer_id, resources in `init/migrations/001_initial.sql`
- [ ] T032 [P] §3.8 Create SyncState table with vector clocks in `init/migrations/001_initial.sql`

### Additional Database Tables (from plan.md)

- [ ] T033 [P] Create traces table (run_id, action, input, output, duration_ms) in `init/migrations/001_initial.sql`
- [ ] T034 [P] Create claims table (statement, evidence, verified, timestamp) in `init/migrations/001_initial.sql`
- [ ] T035 [P] Create evidence table (claim_id, source, hash, timestamp) in `init/migrations/001_initial.sql`
- [ ] T036 [P] Create metrics table (name, value, unit, timestamp) in `init/migrations/001_initial.sql`
- [ ] T037 Create database indexes per data-model.md in `init/migrations/002_indexes.sql`

### Vector Storage Setup

- [ ] T038 §3.7 Integrate sqlite-vss extension for vector search in `init/migrations/003_vectors.sql`
- [ ] T039 [P] Create HNSW index on embeddings vector column in `init/migrations/003_vectors.sql`
- [ ] T040 [P] Setup pgvector extension (optional PostgreSQL scale-up) in `init/migrations/pg/001_pgvector.sql`

### CSV & Data Table Standards

- [ ] T041 §3.5 Implement CSV export service for all entities in `sys/core/src/export/csv_export.rs`
- [ ] T042 [P] Define CSV schema for Agent Directory in `config/schemas/csv/agent_directory.yaml`
- [ ] T043 [P] Define CSV schema for Task Tables in `config/schemas/csv/task_tables.yaml`
- [ ] T044 [P] Define CSV schema for Claims/Evidence Ledger in `config/schemas/csv/claims_evidence.yaml`
- [ ] T045 [P] Define CSV schema for Metrics/Traces in `config/schemas/csv/metrics_traces.yaml`

### Configuration Standards

- [ ] T046 Define unified JSON/YAML config schema in `config/schemas/config_schema.json`
- [ ] T047 [P] Implement config validation with rich metadata support in `sys/core/src/config/validator.rs`
- [ ] T048 [P] Implement config lineage/provenance tracking in `sys/core/src/config/lineage.rs`
- [ ] T049 [P] Create default config templates in `config/templates/`

### Rust Core Foundation

- [ ] T050 §3.1 Define core error types and Result wrapper in `sys/core/src/error.rs`
- [ ] T051 [P] §3.2 Implement configuration loader from JSON/YAML in `sys/core/src/config/mod.rs`
- [ ] T052 [P] §3.5 Implement structured logging with tracing in `sys/core/src/logging.rs`
- [ ] T053 §3.2 Implement SQLite connection pool with rusqlite in `sys/core/src/db/pool.rs`
- [ ] T054 §3.1 Define repository trait pattern in `sys/core/src/db/repository.rs`
- [ ] T055 §3.2 Implement database migration runner in `sys/core/src/db/migrations.rs`

### API Foundation

- [ ] T056 §3.2 Implement HTTP server with axum in `sys/core/src/api/server.rs`
- [ ] T057 [P] Implement health check endpoint GET /api/v1/health in `sys/core/src/api/routes/health.rs`
- [ ] T058 [P] §3.6 Implement request validation middleware in `sys/core/src/api/middleware/validation.rs`
- [ ] T059 [P] §3.5 Implement request logging middleware in `sys/core/src/api/middleware/logging.rs`
- [ ] T060 [P] §3.5 Implement OpenTelemetry tracing middleware in `sys/core/src/api/middleware/telemetry.rs`

### CLI Foundation

- [ ] T061 §3.1 Create CLI entry point with clap in `sys/core/src/main.rs`
- [ ] T062 [P] Implement `noa init` command in `sys/core/src/cli/init.rs`
- [ ] T063 [P] Implement `noa start` command in `sys/core/src/cli/start.rs`
- [ ] T064 [P] Implement `noa status` command in `sys/core/src/cli/status.rs`
- [ ] T065 [P] Implement `noa stop` command in `sys/core/src/cli/stop.rs`
- [ ] T066 [P] Implement `noa db check` command in `sys/core/src/cli/db.rs`
- [ ] T067 [P] Implement `noa db export` command in `sys/core/src/cli/db.rs`

### Observability Foundation

- [ ] T068 §3.5 Setup tracing-subscriber for log collection in `sys/core/src/observability/logging.rs`
- [ ] T069 [P] Setup opentelemetry OTLP export in `sys/core/src/observability/telemetry.rs`
- [ ] T070 [P] Setup prometheus metrics export in `sys/core/src/observability/metrics.rs`
- [ ] T071 [P] Create observability config in `config/observability.yaml`

**Checkpoint**: Foundation ready - Continue to Shared Provider setup ✅

---

## Phase 2.5: Shared Provider Execution Memory (Critical)

**Purpose**: Enable multiple model providers to share memory, context, and execute tasks collaboratively

**⚠️ CRITICAL**: This phase implements FR-037 through FR-042 for multi-provider collaborative execution

### Database Schema (Provider Entities)

- [ ] T417 §3.3 Create Provider table in `init/migrations/004_providers.sql`
- [ ] T418 [P] Create SharedExecutionContext table in `init/migrations/004_providers.sql`
- [ ] T419 [P] Create ProviderTask table in `init/migrations/004_providers.sql`
- [ ] T420 [P] Create provider indexes (name, priority, status) in `init/migrations/004_providers.sql`

### Shared Memory Bus

- [ ] T421 §3.3 Create SharedProviderMemory bus architecture in `sys/core/src/providers/shared_memory.rs`
- [ ] T422 [P] Implement context creation and lifecycle in `sys/core/src/providers/context_manager.rs`
- [ ] T423 [P] Implement context synchronization across providers in `sys/core/src/providers/sync.rs`
- [ ] T424 §3.7 Implement shared execution memory persistence in `sys/core/src/providers/persistence.rs`

### Provider Framework

- [ ] T425 §3.3 Define ProviderTrait base interface in `sys/core/src/providers/base.rs`
- [ ] T426 [P] Implement provider registry and discovery in `sys/core/src/providers/registry.rs`
- [ ] T427 [P] Implement provider health checking in `sys/core/src/providers/health.rs`
- [ ] T428 [P] Implement provider priority selection in `sys/core/src/providers/selector.rs`

### Local Providers (llama.cpp - 5+ models)

- [ ] T429 §3.2 Implement llama.cpp multi-model shared context in `sys/core/src/providers/llama/shared.rs`
- [ ] T430 [P] Implement llama.cpp model pool (5+ concurrent) in `sys/core/src/providers/llama/pool.rs`
- [ ] T431 [P] Implement llama.cpp context forwarding in `sys/core/src/providers/llama/context.rs`

### Cloud/CLI Providers

- [ ] T432 §3.3 Implement Claude Code provider (CLI + Cloud + IDE) in `sys/core/src/providers/claude/mod.rs`
- [ ] T433 [P] Implement Claude CLI wrapper in `sys/core/src/providers/claude/cli.rs`
- [ ] T434 [P] Implement Claude Cloud API client in `sys/core/src/providers/claude/cloud.rs`
- [ ] T435 [P] Implement Claude IDE extension bridge in `sys/core/src/providers/claude/ide.rs`

- [ ] T436 §3.3 Implement Codex provider (CLI + Cloud + IDE) in `sys/core/src/providers/codex/mod.rs`
- [ ] T437 [P] Implement Codex CLI wrapper in `sys/core/src/providers/codex/cli.rs`
- [ ] T438 [P] Implement Codex Cloud API client in `sys/core/src/providers/codex/cloud.rs`

- [ ] T439 §3.3 Implement VS Code Copilot IDE provider in `sys/core/src/providers/copilot/mod.rs`
- [ ] T440 [P] Implement Copilot extension communication in `sys/core/src/providers/copilot/extension.rs`

- [ ] T441 §3.3 Implement Git CLI provider in `sys/core/src/providers/git/mod.rs`
- [ ] T442 [P] Implement Git command execution in `sys/core/src/providers/git/commands.rs`

- [ ] T443 §3.3 Implement Cursor provider (IDE + CLI + Cloud) in `sys/core/src/providers/cursor/mod.rs`
- [ ] T444 [P] Implement Cursor CLI wrapper in `sys/core/src/providers/cursor/cli.rs`
- [ ] T445 [P] Implement Cursor Cloud API client in `sys/core/src/providers/cursor/cloud.rs`
- [ ] T446 [P] Implement Cursor IDE extension bridge in `sys/core/src/providers/cursor/ide.rs`

- [ ] T447 §3.3 Implement Abacus provider (CLI + Cloud) in `sys/core/src/providers/abacus/mod.rs`
- [ ] T448 [P] Implement Abacus CLI wrapper in `sys/core/src/providers/abacus/cli.rs`
- [ ] T449 [P] Implement Abacus Cloud API client in `sys/core/src/providers/abacus/cloud.rs`

### Collaborative Reasoning Engine

- [ ] T450 §3.3 Implement collaborative reasoning orchestrator in `sys/core/src/providers/collaborative/orchestrator.rs`
- [ ] T451 [P] Implement parallel task distribution in `sys/core/src/providers/collaborative/distributor.rs`
- [ ] T452 [P] Implement result aggregation and consensus in `sys/core/src/providers/collaborative/aggregator.rs`
- [ ] T453 [P] Implement provider state synchronization in `sys/core/src/providers/collaborative/state_sync.rs`

### Provider Configuration

- [ ] T454 Create provider configuration schema in `config/schemas/providers.yaml`
- [ ] T455 [P] Create default provider configs in `config/providers/`
- [ ] T456 [P] Implement provider feature flags in `config/features.json`

### CLI Commands (Providers)

- [ ] T457 Implement `noa providers list` command in `sys/core/src/cli/providers.rs`
- [ ] T458 [P] Implement `noa providers status` command in `sys/core/src/cli/providers.rs`
- [ ] T459 [P] Implement `noa providers enable/disable` command in `sys/core/src/cli/providers.rs`
- [ ] T460 [P] Implement `noa providers test` command in `sys/core/src/cli/providers.rs`

### API Endpoints (Providers)

- [ ] T461 Implement GET /api/v1/providers endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T462 [P] Implement POST /api/v1/providers/{id}/enable endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T463 [P] Implement GET /api/v1/providers/context endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T464 [P] Implement POST /api/v1/providers/execute endpoint in `sys/core/src/api/routes/providers.rs`

**Phase 2.5 Acceptance Criteria**:
- [ ] Minimum 5 llama.cpp models running concurrently with shared context
- [ ] All 8 provider types integrated (llama.cpp, Claude, Codex, Copilot, Git, Cursor, Abacus)
- [ ] Providers can reason together on complex tasks
- [ ] Shared execution memory persists across sessions

**Checkpoint**: Provider infrastructure ready - User story implementation can now begin ✅

---

## Phase 3: User Story 1 - Initialize NOA Seed Environment (P1) 🎯 MVP

**Goal**: Create self-contained environment under `noa_root` with all components

**Independent Test**: Run `noa init` and verify directory structure, database, and binary accessibility

### Directory Creation (US1)

- [ ] T072 [US1] §3.1 Implement directory structure creation in `sys/core/src/init/structure.rs`
- [ ] T073 [P] [US1] §3.1 Define directory constants (sys/, p2p/, opt/, init/, containers/, config/, bin/, ai/, data/) in `sys/core/src/init/paths.rs`
- [ ] T074 [P] [US1] Create sys/core/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T075 [P] [US1] Create sys/services/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T076 [P] [US1] Create sys/ui/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T077 [P] [US1] Create sys/digest/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T078 [P] [US1] Create sys/kernel/ subdirectory structure in `sys/core/src/init/structure.rs`

### Config & Database Init (US1)

- [ ] T079 [US1] §3.2 Implement default config generation in `sys/core/src/init/config.rs`
- [ ] T080 [US1] §3.2 Implement database initialization in `sys/core/src/init/database.rs`
- [ ] T081 [P] [US1] Generate ai-providers.json default config in `sys/core/src/init/config.rs`
- [ ] T082 [P] [US1] Generate noa-server.json default config in `sys/core/src/init/config.rs`
- [ ] T083 [P] [US1] Generate features.json default feature flags in `sys/core/src/init/config.rs`
- [ ] T084 [P] [US1] Generate models.json default model registry in `sys/core/src/init/config.rs`

### Services (US1)

- [ ] T085 [US1] §3.1 Implement InitService with full initialization flow in `sys/core/src/services/init_service.rs`
- [ ] T086 [P] [US1] §3.6 Implement permission checking and setup in `sys/core/src/services/init_service.rs`
- [ ] T087 [P] [US1] §3.1 Implement binary path registration in `sys/core/src/services/init_service.rs`

### Bootstrap Scripts (US1)

- [ ] T088 [US1] Create bootstrap dirs.sh script in `init/bootstrap/dirs.sh`
- [ ] T089 [P] [US1] Create bootstrap deps.sh script in `init/bootstrap/deps.sh`
- [ ] T090 [P] [US1] Create bootstrap models.sh script in `init/bootstrap/models.sh`
- [ ] T091 [P] [US1] Create Windows init script noa-init.ps1 in `init/noa-init.ps1`

### CLI Commands (US1)

- [ ] T092 [US1] Enhance `noa init` with --root and --force flags in `sys/core/src/cli/init.rs`
- [ ] T093 [P] [US1] Implement initialization progress display in `sys/core/src/cli/init.rs`
- [ ] T094 [P] [US1] Implement verification output in `sys/core/src/cli/init.rs`

### API Endpoints (US1)

- [ ] T095 [P] [US1] Implement GET /api/v1/system/info endpoint in `sys/core/src/api/routes/system.rs`
- [ ] T096 [P] [US1] Implement GET /api/v1/system/health with database check in `sys/core/src/api/routes/system.rs`

**US1 Acceptance Criteria**:
- [ ] Directory structure created with correct permissions
- [ ] Local database operational
- [ ] Works fully offline

---

## Phase 4: User Story 2 - Multi-SLM Neural Runtime (P1) 🎯 MVP

**Goal**: Run multiple SLMs locally with <2s response on standard hardware

**Independent Test**: Load model, send prompt, verify response within latency target

### Rust ML Stack Setup (US2)

- [ ] T097 [US2] §3.2 Add burn-rs/burn to Cargo.toml for training in `sys/core/Cargo.toml`
- [ ] T098 [P] [US2] Add candle-core and candle-transformers for inference in `sys/core/Cargo.toml`
- [ ] T099 [P] [US2] Add tokenizers crate for HuggingFace tokenization in `sys/core/Cargo.toml`
- [ ] T100 [P] [US2] Add qdrant-client for vector store in `sys/core/Cargo.toml`
- [ ] T101 [P] [US2] Configure tch backend for CUDA 13.1+ in `sys/core/Cargo.toml`
- [ ] T102 [P] [US2] Configure wgpu backend for cross-platform GPU in `sys/core/Cargo.toml`

### Models & Data Layer (US2)

- [ ] T103 [US2] §3.2 Implement Model entity repository in `sys/core/src/db/repositories/model_repository.rs`
- [ ] T104 [P] [US2] Define model configuration schema in `config/ai-providers.json`
- [ ] T105 [P] [US2] Create GGUF model loader interface in `sys/core/src/neural/model_loader.rs`

### Neural Runtime Core (US2)

- [ ] T106 [US2] §3.2 Integrate llama-cpp-rs bindings in `sys/core/src/neural/llama_backend.rs`
- [ ] T107 [US2] Implement model loading with GPU layer auto-detection in `sys/core/src/neural/model_loader.rs`
- [ ] T108 [P] [US2] Implement context management for inference in `sys/core/src/neural/context.rs`
- [ ] T109 [US2] Implement inference engine with streaming in `sys/core/src/neural/inference.rs`
- [ ] T110 [P] [US2] Implement quantization detection and layer offloading in `sys/core/src/neural/hardware.rs`

### Multi-GPU Support (US2) - FR-047 to FR-050

- [ ] T478 §3.2 [US2] Implement CUDA device enumeration in `sys/core/src/neural/cuda_devices.rs`
- [ ] T479 [P] [US2] Implement multi-GPU layer distribution in `sys/core/src/neural/multi_gpu.rs`
- [ ] T480 [P] [US2] Implement tensor parallelism for model sharding in `sys/core/src/neural/tensor_parallel.rs`
- [ ] T481 [P] [US2] Implement NVLink detection and utilization in `sys/core/src/neural/nvlink.rs`
- [ ] T482 [P] [US2] Add GPU memory pooling across devices in `sys/core/src/neural/gpu_pool.rs`
- [ ] T483 §3.2 [US2] Implement CUDA 13.1+ tiles configuration in `sys/core/src/neural/cuda_tiles.rs`
- [ ] T484 [P] [US2] Add multi-GPU load balancing in `sys/core/src/neural/gpu_scheduler.rs`
- [ ] T485 [P] [US2] Implement GPU health monitoring in `sys/core/src/neural/gpu_health.rs`

### Candle Inference Layer (US2)

- [ ] T111 [US2] Implement Candle embedding service in `sys/core/crates/embedder/src/lib.rs`
- [ ] T112 [P] [US2] Support models: Llama, Mistral, Qwen, BGE/E5 in `sys/core/crates/embedder/src/models.rs`
- [ ] T113 [P] [US2] Implement safetensors/GGUF export in `sys/core/src/neural/export.rs`

### Model Selection (US2)

- [ ] T114 [US2] §3.3 Implement ModelSelectorAgent base logic in `sys/core/src/agents/model_selector.rs`
- [ ] T115 [P] [US2] Define model selection criteria (task type, resources) in `sys/core/src/agents/model_selector.rs`
- [ ] T116 [P] [US2] Implement model benchmarking utility in `sys/core/src/neural/benchmark.rs`

### 13 Specialized ModelSelectorAgents (US2)

- [ ] T465 [P] [US2] Implement ModelSelectorAgent_Audit in `ai/agents/model_selectors/audit.ts`
- [ ] T466 [P] [US2] Implement ModelSelectorAgent_DataStack in `ai/agents/model_selectors/data_stack.ts`
- [ ] T467 [P] [US2] Implement ModelSelectorAgent_DevOps in `ai/agents/model_selectors/devops.ts`
- [ ] T468 [P] [US2] Implement ModelSelectorAgent_Ethics in `ai/agents/model_selectors/ethics.ts`
- [ ] T469 [P] [US2] Implement ModelSelectorAgent_Finance in `ai/agents/model_selectors/finance.ts`
- [ ] T470 [P] [US2] Implement ModelSelectorAgent_HR in `ai/agents/model_selectors/hr.ts`
- [ ] T471 [P] [US2] Implement ModelSelectorAgent_LegalCompliance in `ai/agents/model_selectors/legal.ts`
- [ ] T472 [P] [US2] Implement ModelSelectorAgent_Marketing in `ai/agents/model_selectors/marketing.ts`
- [ ] T473 [P] [US2] Implement ModelSelectorAgent_Operations in `ai/agents/model_selectors/operations.ts`
- [ ] T474 [P] [US2] Implement ModelSelectorAgent_Security in `ai/agents/model_selectors/security.ts`
- [ ] T475 [P] [US2] Implement ModelSelectorAgent_Strategy in `ai/agents/model_selectors/strategy.ts`
- [ ] T476 [P] [US2] Implement ModelSelectorAgent_Technology in `ai/agents/model_selectors/technology.ts`
- [ ] T477 [P] [US2] Implement ModelSelectorAgent_Vision in `ai/agents/model_selectors/vision.ts`

### Services (US2)

- [ ] T118 [US2] Implement NeuralService with model management in `sys/core/src/services/neural_service.rs`
- [ ] T119 [P] [US2] Implement model download with progress in `sys/core/src/services/model_download.rs`

### CLI Commands (US2)

- [ ] T120 [US2] Implement `noa models list` command in `sys/core/src/cli/models.rs`
- [ ] T121 [P] [US2] Implement `noa models download` command in `sys/core/src/cli/models.rs`
- [ ] T122 [P] [US2] Implement `noa models verify` command in `sys/core/src/cli/models.rs`
- [ ] T123 [US2] Implement `noa ask` command for inference in `sys/core/src/cli/ask.rs`
- [ ] T124 [P] [US2] Implement `noa models benchmark` command in `sys/core/src/cli/models.rs`

### API Endpoints (US2)

- [ ] T125 [P] [US2] Implement GET /api/v1/models endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T126 [P] [US2] Implement POST /api/v1/models/download endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T127 [US2] Implement POST /api/v1/inference endpoint in `sys/core/src/api/routes/inference.rs`
- [ ] T128 [P] [US2] Implement POST /api/v1/inference/stream SSE endpoint in `sys/core/src/api/routes/inference.rs`
- [ ] T129 [P] [US2] Implement POST /api/v1/models/benchmark endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T130 [P] [US2] Implement POST /api/v1/models/ingest endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T527 [P] [US2] Implement POST /api/v1/models/{id}/load endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T528 [P] [US2] Implement POST /api/v1/models/{id}/unload endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T529 [P] [US2] Implement GET /api/v1/models/{id}/status endpoint in `sys/core/src/api/routes/models.rs`

**US2 Acceptance Criteria**:
- [ ] Response within 2 seconds on standard hardware
- [ ] ModelSelectorAgent routes tasks to optimal model
- [ ] Dynamic quantization based on available resources

---

## Phase 5: User Story 3 - Total Memory Sovereignty (P1) 🎯 MVP

**Goal**: Remember everything with instant recall (<500ms)

**Independent Test**: Create memory, close system, reopen, verify instant recall

### Models & Data Layer (US3)

- [ ] T131 [US3] §3.7 Implement Memory entity repository in `sys/core/src/db/repositories/memory_repository.rs`
- [ ] T132 [P] [US3] §3.7 Implement Embedding entity repository in `sys/core/src/db/repositories/embedding_repository.rs`
- [ ] T133 [US3] §3.7 Integrate sqlite-vss for vector search in `sys/core/src/db/vector_search.rs`

### Embedding Pipeline (US3)

- [ ] T134 [US3] §3.7 Implement embedding generation with Candle in `sys/core/src/memory/embeddings.rs`
- [ ] T135 [P] [US3] Create embedding model loader (MiniLM-384 or BGE/E5) in `sys/core/src/memory/embedding_model.rs`
- [ ] T136 [US3] §3.7 Implement semantic search with HNSW index in `sys/core/src/memory/semantic_search.rs`
- [ ] T137 [P] [US3] Implement embedding cache (model_version + input_hash + params_hash) in `sys/core/src/memory/cache.rs`
- [ ] T138 [P] [US3] Implement batch embedding requests in `sys/core/src/memory/embeddings.rs`

### Vector Store Integration (US3)

- [ ] T139 [US3] §3.7 Implement Qdrant client wrapper in `sys/core/src/vector/qdrant_client.rs`
- [ ] T140 [P] [US3] Implement vector upsert with metadata in `sys/core/src/vector/qdrant_client.rs`
- [ ] T141 [P] [US3] Implement vector search with filters in `sys/core/src/vector/qdrant_client.rs`

### Services (US3)

- [ ] T142 [US3] §3.7 Implement MemoryService with CRUD operations in `sys/core/src/services/memory_service.rs`
- [ ] T143 [P] [US3] Implement memory checksum validation in `sys/core/src/services/memory_service.rs`
- [ ] T144 [US3] §3.7 Implement SearchService with semantic + keyword in `sys/core/src/services/search_service.rs`

### CLI Commands (US3)

- [ ] T145 [US3] Implement `noa memory create` command in `sys/core/src/cli/memory.rs`
- [ ] T146 [P] [US3] Implement `noa memory search` command in `sys/core/src/cli/memory.rs`
- [ ] T147 [P] [US3] Implement `noa memory list` command in `sys/core/src/cli/memory.rs`
- [ ] T148 [P] [US3] Implement `noa memory get` command in `sys/core/src/cli/memory.rs`

### API Endpoints (US3)

- [ ] T149 [US3] §3.7 Implement POST /api/v1/memories endpoint in `sys/core/src/api/routes/memories.rs`
- [ ] T150 [P] [US3] Implement GET /api/v1/memories/{id} endpoint in `sys/core/src/api/routes/memories.rs`
- [ ] T151 [P] [US3] Implement GET /api/v1/memories endpoint with pagination in `sys/core/src/api/routes/memories.rs`
- [ ] T152 [US3] §3.7 Implement POST /api/v1/memories/search endpoint in `sys/core/src/api/routes/memories.rs`

**US3 Acceptance Criteria**:
- [ ] Instant recall of past conversations
- [ ] Search results in <500ms
- [ ] Memory syncs across P2P devices (when US6 complete)

---

## Phase 6: User Story 4 - Digest Everything Pipeline (P2)

**Goal**: Ingest and understand any codebase or data source

**Independent Test**: Point digest at GitHub repo, verify output artifacts

### Backend Pipeline Services (US4)

- [ ] T153 [US4] Implement IntakeService (Stage 1) in `sys/core/src/services/digest/intake.rs`
- [ ] T154 [P] [US4] Implement ClassifierService (Stage 2) - languages, licenses in `sys/core/src/services/digest/classifier.rs`
- [ ] T155 [P] [US4] Implement GraphExtractService (Stage 3) - kg.json, system_card in `sys/core/src/services/digest/graph_extract.rs`
- [ ] T156 [P] [US4] Implement EmbeddingsService (Stage 4) - pgvector/Qdrant in `sys/core/src/services/digest/embeddings.rs`
- [ ] T157 [P] [US4] Implement EnvSynthesisService (Stage 5) - Dockerfiles, compose, K8s in `sys/core/src/services/digest/env_synthesis.rs`
- [ ] T158 [P] [US4] Implement SafetyService (Stage 6) - SBOM, Grype, Gitleaks in `sys/core/src/services/digest/safety.rs`
- [ ] T159 [P] [US4] Implement RunnerService (Stage 7) - build, test, demo in `sys/core/src/services/digest/runner.rs`
- [ ] T160 [P] [US4] Implement IntegratorService (Stage 8) - SDKs, telemetry in `sys/core/src/services/digest/integrator.rs`
- [ ] T161 [P] [US4] Implement RegistrarService (Stage 9) - storage, registry in `sys/core/src/services/digest/registrar.rs`

### Models & Data Layer (US4)

- [ ] T162 [US4] Implement DigestSource repository in `sys/core/src/db/repositories/digest_repository.rs`
- [ ] T163 [P] [US4] Implement KnowledgeNode repository in `sys/core/src/db/repositories/knowledge_node_repository.rs`
- [ ] T164 [P] [US4] Implement KnowledgeEdge repository in `sys/core/src/db/repositories/knowledge_edge_repository.rs`

### Digest Pipeline (Python) (US4)

- [ ] T165 [US4] §3.4 Implement Discover stage in `sys/digest/src/stages/discover.py`
- [ ] T166 [P] [US4] Implement Fetch stage with git clone in `sys/digest/src/stages/fetch.py`
- [ ] T167 [US4] Implement Parse stage with tree-sitter in `sys/digest/src/stages/parse.py`
- [ ] T168 [P] [US4] §3.4 Implement Analyze stage for embeddings in `sys/digest/src/stages/analyze.py`
- [ ] T169 [P] [US4] Implement Summarize stage in `sys/digest/src/stages/summarize.py`
- [ ] T170 [US4] Implement Surface stage for output generation in `sys/digest/src/stages/surface.py`
- [ ] T171 [US4] §3.6 Implement Secure stage with Gitleaks/Trivy in `sys/digest/src/stages/secure.py`

### Multi-Language Parsing (US4)

- [ ] T172 [P] [US4] Implement Python AST parser in `sys/digest/src/parsers/python_parser.py`
- [ ] T173 [P] [US4] Implement TypeScript parser with ts-morph in `sys/digest/src/parsers/typescript_parser.py`
- [ ] T174 [P] [US4] Implement Rust parser with syn in `sys/digest/src/parsers/rust_parser.py`
- [ ] T175 [P] [US4] Implement Go parser with go/ast in `sys/digest/src/parsers/go_parser.py`
- [ ] T176 [P] [US4] Implement Java parser in `sys/digest/src/parsers/java_parser.py`

### Output Generation (US4)

- [ ] T177 [US4] Generate profile.json output in `sys/digest/src/output/profile.py`
- [ ] T178 [P] [US4] Generate system_card.md output in `sys/digest/src/output/system_card.py`
- [ ] T179 [P] [US4] Generate kg.json knowledge graph in `sys/digest/src/output/knowledge_graph.py`
- [ ] T180 [P] [US4] §3.6 Generate SBOM with Syft in `sys/digest/src/output/sbom.py`

### Event Bus (US4)

- [ ] T181 [US4] Implement Redis Streams event bus in `sys/core/src/events/redis_streams.rs`
- [ ] T182 [P] [US4] Implement workflow DAG engine in `sys/core/src/events/workflow_engine.rs`
- [ ] T183 [P] [US4] Implement digest job queue in `sys/core/src/services/digest_queue.rs`

### Services (US4)

- [ ] T184 [US4] Implement DigestService orchestrator in `sys/core/src/services/digest_service.rs`

### CLI Commands (US4)

- [ ] T185 [US4] Implement `noa digest <url>` command in `sys/core/src/cli/digest.rs`
- [ ] T186 [P] [US4] Implement `noa digest status` command in `sys/core/src/cli/digest.rs`
- [ ] T187 [P] [US4] Implement `noa digest security-report` command in `sys/core/src/cli/digest.rs`
- [ ] T188 [P] [US4] Implement `noa knowledge search` command in `sys/core/src/cli/knowledge.rs`

### API Endpoints (US4)

- [ ] T189 [US4] Implement POST /api/v1/digest endpoint per OpenAPI in `sys/core/src/api/routes/digest.rs`
- [ ] T190 [P] [US4] Implement GET /api/v1/digest/{jobId} endpoint in `sys/core/src/api/routes/digest.rs`
- [ ] T191 [P] [US4] Implement GET /api/v1/digest/{jobId}/artifacts endpoint in `sys/core/src/api/routes/digest.rs`

### Digest Source Endpoints (US4) - per digest-pipeline.openapi.yaml

- [ ] T509 [US4] Implement GET /api/v1/digest/sources endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T510 [P] [US4] Implement POST /api/v1/digest/sources endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T511 [P] [US4] Implement GET /api/v1/digest/sources/{id} endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T512 [P] [US4] Implement GET /api/v1/digest/sources/{id}/profile endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T513 [P] [US4] Implement GET /api/v1/digest/sources/{id}/system-card endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T514 [P] [US4] Implement GET /api/v1/digest/sources/{id}/sbom endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T515 [P] [US4] Implement GET /api/v1/digest/sources/{id}/security endpoint in `sys/core/src/api/routes/digest_sources.rs`

### Knowledge Graph Endpoints (US4) - per digest-pipeline.openapi.yaml

- [ ] T516 [US4] Implement GET /api/v1/knowledge/nodes endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T517 [P] [US4] Implement GET /api/v1/knowledge/nodes/{id} endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T518 [P] [US4] Implement GET /api/v1/knowledge/edges endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T519 [P] [US4] Implement POST /api/v1/knowledge/query endpoint in `sys/core/src/api/routes/knowledge.rs`

**US4 Acceptance Criteria**:
- [ ] Produces profile.json, system_card.md, kg.json, SBOM, security report
- [ ] Multi-language parsing (Python, TypeScript, Rust, Go, Java)
- [ ] Vulnerabilities flagged with severity

---

## Phase 7: User Story 5 - Dynamic Context-Aware UI (P2)

**Goal**: Fluid, agent-driven interface that adapts to context

**Independent Test**: Switch tasks and verify UI reconfigures appropriately

### UI Foundation (US5)

- [ ] T192 [US5] Initialize Next.js app with TypeScript in `sys/ui/`
- [ ] T193 [P] [US5] Setup Tailwind CSS and shadcn/ui components in `sys/ui/`
- [ ] T194 [P] [US5] Implement API client for backend in `sys/ui/src/lib/api.ts`
- [ ] T195 [US5] Implement WebSocket connection for real-time updates in `sys/ui/src/lib/websocket.ts`

### Core Components (US5)

- [ ] T196 [US5] Implement main layout with sidebar in `sys/ui/src/components/layout/MainLayout.tsx`
- [ ] T197 [P] [US5] Implement navigation component in `sys/ui/src/components/layout/Navigation.tsx`
- [ ] T198 [P] [US5] §3.5 Implement activity log component in `sys/ui/src/components/ActivityLog.tsx`
- [ ] T199 [US5] Implement context detection service in `sys/ui/src/services/contextDetector.ts`

### Admin Console Features (US5)

- [ ] T200 [US5] Implement Jobs Dashboard in `sys/ui/src/pages/admin/Jobs.tsx`
- [ ] T201 [P] [US5] Implement Capsules View in `sys/ui/src/pages/admin/Capsules.tsx`
- [ ] T202 [P] [US5] Implement Artifacts Explorer in `sys/ui/src/pages/admin/Artifacts.tsx`
- [ ] T203 [P] [US5] Implement SBOM & Security view in `sys/ui/src/pages/admin/Security.tsx`
- [ ] T204 [P] [US5] Implement Model Registry view in `sys/ui/src/pages/admin/Models.tsx`
- [ ] T205 [P] [US5] Implement CRM Controls view in `sys/ui/src/pages/admin/CRM.tsx`

### Settings Dashboard (US5)

- [ ] T206 [US5] Implement Settings Dashboard architecture in `sys/ui/src/pages/Settings.tsx`
- [ ] T207 [P] [US5] Implement AI/LLM Settings panel in `sys/ui/src/components/settings/AISettings.tsx`
- [ ] T208 [P] [US5] Implement Providers Settings panel in `sys/ui/src/components/settings/ProvidersSettings.tsx`
- [ ] T209 [P] [US5] Implement IDE/Editor Settings panel in `sys/ui/src/components/settings/IDESettings.tsx`
- [ ] T210 [P] [US5] Implement CLI Settings panel in `sys/ui/src/components/settings/CLISettings.tsx`
- [ ] T211 [P] [US5] Implement Sync Settings panel in `sys/ui/src/components/settings/SyncSettings.tsx`
- [ ] T212 [P] [US5] Implement Memory Settings panel in `sys/ui/src/components/settings/MemorySettings.tsx`
- [ ] T213 [P] [US5] Implement Security Settings panel in `sys/ui/src/components/settings/SecuritySettings.tsx`
- [ ] T214 [P] [US5] Implement Privacy Settings panel in `sys/ui/src/components/settings/PrivacySettings.tsx`
- [ ] T215 [P] [US5] Implement Theme Settings panel in `sys/ui/src/components/settings/ThemeSettings.tsx`

### Cross-Platform Settings Sync (US5)

- [ ] T216 [US5] §3.8 Implement settings sync service in `sys/ui/src/services/settingsSync.ts`
- [ ] T217 [P] [US5] Implement sync scope (global/per-device/per-project) in `sys/ui/src/services/settingsSync.ts`
- [ ] T218 [P] [US5] Implement conflict resolution strategy in `sys/ui/src/services/settingsSync.ts`

### Dynamic UI Features (US5)

- [ ] T219 [US5] Implement widget registry in `sys/ui/src/components/widgets/WidgetRegistry.ts`
- [ ] T220 [P] [US5] Implement drag-and-drop widget layout in `sys/ui/src/components/widgets/WidgetGrid.tsx`
- [ ] T221 [P] [US5] Implement widget persistence in `sys/ui/src/services/widgetPersistence.ts`
- [ ] T222 [P] [US5] Implement contextual UI auto-adapt in `sys/ui/src/services/contextualUI.ts`
- [ ] T223 [P] [US5] Implement user presets save/switch in `sys/ui/src/services/presets.ts`
- [ ] T224 [P] [US5] Implement AI-assisted insights in `sys/ui/src/services/insights.ts`

### Model-Agnostic Chat Interface (US5)

- [ ] T225 [US5] Implement chat interface component in `sys/ui/src/components/Chat.tsx`
- [ ] T226 [P] [US5] Implement streaming response display in `sys/ui/src/components/ChatMessage.tsx`
- [ ] T227 [P] [US5] Implement markdown rendering in `sys/ui/src/components/MarkdownRenderer.tsx`
- [ ] T228 [US5] Implement provider abstraction for model switching in `sys/ui/src/services/providerClient.ts`
- [ ] T229 [P] [US5] Implement context persistence across devices in `sys/ui/src/services/contextPersistence.ts`

**US5 Acceptance Criteria**:
- [ ] UI surfaces relevant tools based on context
- [ ] Live scrollable activity log
- [ ] Real-time updates via WebSocket
- [ ] Settings sync across IDE, CLI, web

---

## Phase 8: User Story 6 - P2P Hive-Mind Device Federation (P2)

**Goal**: User-owned P2P cloud sharing compute, memory, storage

**Independent Test**: Connect two devices, verify discovery and resource sharing

### P2P Core (Go) (US6)

- [ ] T230 [US6] §3.8 Initialize libp2p host in `p2p/pkg/node/host.go`
- [ ] T231 [P] [US6] §3.8 Implement mDNS discovery in `p2p/pkg/discovery/mdns.go`
- [ ] T232 [P] [US6] Implement DHT discovery in `p2p/pkg/discovery/dht.go`
- [ ] T233 [P] [US6] §3.6 Implement TLS encryption for connections in `p2p/pkg/security/tls.go`
- [ ] T234 [US6] §3.8 Implement peer connection management in `p2p/pkg/node/peers.go`

### Device Management (US6)

- [ ] T235 [US6] §3.8 Implement Device repository in `sys/core/src/db/repositories/device_repository.rs`
- [ ] T236 [P] [US6] Implement device registration in `sys/core/src/services/device_service.rs`
- [ ] T237 [US6] §3.8 Implement resource capability detection in `p2p/pkg/resources/capabilities.go`

### P2P Discovery Service (US6) - per p2p-protocol.proto

- [ ] T501 [US6] §3.8 Implement Discovery.Announce RPC in `p2p/pkg/discovery/announce.go`
- [ ] T502 [P] [US6] Implement Discovery.FindPeers RPC in `p2p/pkg/discovery/find_peers.go`
- [ ] T503 [P] [US6] Implement Discovery.Ping RPC in `p2p/pkg/discovery/ping.go`
- [ ] T504 [P] [US6] Generate Go code from p2p-protocol.proto in `p2p/pkg/protocol/`

### Sync Protocol (US6) - per p2p-protocol.proto

- [ ] T238 [US6] §3.8 Implement SyncState repository in `sys/core/src/db/repositories/sync_repository.rs`
- [ ] T239 [US6] §3.8 Implement vector clock sync in `p2p/pkg/sync/vector_clock.go`
- [ ] T240 [P] [US6] Implement CRDT data types in `p2p/pkg/sync/crdt.go`
- [ ] T241 [P] [US6] Implement delta sync protocol in `p2p/pkg/sync/delta.go`
- [ ] T242 [P] [US6] Implement conflict resolution in `p2p/pkg/sync/conflict.go`
- [ ] T243 [US6] §3.7 Implement memory sync protocol in `p2p/pkg/sync/memory_sync.go`
- [ ] T505 [US6] Implement Sync.GetState RPC in `p2p/pkg/sync/get_state.go`
- [ ] T506 [P] [US6] Implement Sync.PushChanges RPC in `p2p/pkg/sync/push_changes.go`
- [ ] T507 [P] [US6] Implement Sync.PullChanges RPC in `p2p/pkg/sync/pull_changes.go`
- [ ] T508 [P] [US6] Implement Sync.ResolveConflict RPC in `p2p/pkg/sync/resolve_conflict.go`

### Distributed Storage (US6)

- [ ] T244 [US6] §3.8 Implement CAS (Content-Addressable Storage) in `p2p/pkg/storage/cas.go`
- [ ] T245 [P] [US6] Implement replication protocol in `p2p/pkg/storage/replication.go`

### P2P Storage Service (US6) - per p2p-protocol.proto

- [ ] T509 [US6] §3.8 Implement Storage.Store RPC in `p2p/pkg/storage/store.go`
- [ ] T510 [P] [US6] Implement Storage.Retrieve RPC in `p2p/pkg/storage/retrieve.go`
- [ ] T511 [P] [US6] Implement Storage.Exists RPC in `p2p/pkg/storage/exists.go`
- [ ] T512 [P] [US6] Implement Storage.List RPC in `p2p/pkg/storage/list.go`
- [ ] T513 [P] [US6] Implement Storage.Replicate RPC in `p2p/pkg/storage/replicate.go`

### Task Distribution (US6)

- [ ] T246 [US6] §3.3 Implement task offloading protocol in `p2p/pkg/tasks/offload.go`
- [ ] T247 [P] [US6] Implement resource-based routing in `p2p/pkg/tasks/router.go`
- [ ] T248 [P] [US6] Implement distributed compute scheduler in `p2p/pkg/compute/scheduler.go`
- [ ] T249 [P] [US6] Implement worker node in `p2p/pkg/compute/worker.go`

### P2P Compute Service (US6) - per p2p-protocol.proto

- [ ] T514 [US6] §3.3 Implement Compute.SubmitTask RPC in `p2p/pkg/compute/submit_task.go`
- [ ] T515 [P] [US6] Implement Compute.GetTaskStatus RPC in `p2p/pkg/compute/get_task_status.go`
- [ ] T516 [P] [US6] Implement Compute.CancelTask RPC in `p2p/pkg/compute/cancel_task.go`
- [ ] T517 [P] [US6] Implement Compute.StreamOutput RPC (streaming) in `p2p/pkg/compute/stream_output.go`

### CLI Commands (US6)

- [ ] T250 [US6] Implement `noa p2p info` command in `sys/core/src/cli/p2p.rs`
- [ ] T251 [P] [US6] Implement `noa p2p connect` command in `sys/core/src/cli/p2p.rs`
- [ ] T252 [P] [US6] Implement `noa p2p status` command in `sys/core/src/cli/p2p.rs`
- [ ] T253 [P] [US6] Implement `noa p2p ping` command in `sys/core/src/cli/p2p.rs`
- [ ] T254 [P] [US6] Implement `noa p2p reset` command in `sys/core/src/cli/p2p.rs`

### API Endpoints (US6)

- [ ] T255 [US6] Implement GET /api/v1/p2p/info endpoint in `sys/core/src/api/routes/p2p.rs`
- [ ] T256 [P] [US6] Implement GET /api/v1/p2p/peers endpoint in `sys/core/src/api/routes/p2p.rs`
- [ ] T257 [P] [US6] Implement POST /api/v1/p2p/connect endpoint in `sys/core/src/api/routes/p2p.rs`

**US6 Acceptance Criteria**:
- [ ] Devices discover each other on same network
- [ ] Tasks distributed based on available resources
- [ ] Graceful degradation when device goes offline

---

## Phase 9: User Story 7 - Autonomous Agent Orchestration (P2)

**Goal**: Coordinate specialized agents for complex multi-step problems

**Independent Test**: Submit complex goal, observe decomposition and agent collaboration

### Agent Framework (US7)

- [ ] T258 [US7] §3.3 Implement Agent repository in `sys/core/src/db/repositories/agent_repository.rs`
- [ ] T259 [P] [US7] §3.5 Implement AgentLog repository in `sys/core/src/db/repositories/agentlog_repository.rs`
- [ ] T260 [US7] §3.3 Implement Task repository in `sys/core/src/db/repositories/task_repository.rs`
- [ ] T261 [P] [US7] Implement TaskEvent repository in `sys/core/src/db/repositories/task_event_repository.rs`

### Core Agents (US7)

- [ ] T262 [US7] §3.3 Implement BaseAgent trait in `sys/core/src/agents/base.rs`
- [ ] T263 [P] [US7] Implement FileIOAgent in `sys/core/src/agents/file_io.rs`
- [ ] T264 [P] [US7] Implement TerminalAgent in `sys/core/src/agents/terminal.rs`
- [ ] T265 [P] [US7] Implement RAGAgent in `sys/core/src/agents/rag.rs`
- [ ] T266 [P] [US7] Implement MicroserviceManagementAgent in `sys/core/src/agents/microservice_mgmt.rs`

### Executive Board Agents (US7)

- [ ] T267 [US7] §3.3 Implement ExecutiveCommanderChiefAgent in `sys/core/src/agents/executive/commander.rs`
- [ ] T268 [P] [US7] Implement EA_HR Agent in `sys/core/src/agents/executive/hr.rs`
- [ ] T269 [P] [US7] Implement EA_Finance Agent in `sys/core/src/agents/executive/finance.rs`
- [ ] T270 [P] [US7] Implement EA_Audit Agent in `sys/core/src/agents/executive/audit.rs`
- [ ] T271 [P] [US7] Implement EA_Ethics Agent in `sys/core/src/agents/executive/ethics.rs`
- [ ] T272 [P] [US7] Implement EA_Operations Agent in `sys/core/src/agents/executive/operations.rs`
- [ ] T273 [P] [US7] Implement EA_Security Agent in `sys/core/src/agents/executive/security.rs`
- [ ] T274 [P] [US7] Implement EA_Technology Agent in `sys/core/src/agents/executive/technology.rs`

### MicroAgentStack (US7)

- [ ] T275 [US7] §3.3 Implement MicroAgentStack repository in `sys/core/src/db/repositories/stack_repository.rs`
- [ ] T276 [US7] §3.3 Implement 5-stage lifecycle (bootstrap→execute→validate→package→archive) in `sys/core/src/orchestration/stack_lifecycle.rs`
- [ ] T277 [P] [US7] Implement CommanderChiefAgent in `sys/core/src/agents/commander.rs`
- [ ] T278 [P] [US7] Implement stack naming (mas_* reusable, gen_mas disposable) in `sys/core/src/orchestration/stack_naming.rs`
- [ ] T279 [P] [US7] Implement stack workspace (in/work/out/logs) in `sys/core/src/orchestration/stack_workspace.rs`

### Orchestration (US7)

- [ ] T280 [US7] §3.3 Implement goal decomposition in `sys/core/src/orchestration/decomposer.rs`
- [ ] T281 [P] [US7] Implement task scheduler with priority queue in `sys/core/src/orchestration/scheduler.rs`
- [ ] T282 [US7] §3.3 Implement orchestration engine (CECCA) in `sys/core/src/orchestration/engine.rs`
- [ ] T283 [P] [US7] Implement retry and escalation logic in `sys/core/src/orchestration/retry.rs`

### Capsule Architecture (US7)

- [ ] T284 [US7] Implement Capsule sidecars (build-proxy, service-mirror, policy-agent, telemetry) in `containers/capsules/sidecars/`
- [ ] T285 [P] [US7] Create noa-core.yaml capsule spec in `containers/capsules/noa-core.yaml`
- [ ] T286 [P] [US7] Create noa-digest.yaml capsule spec in `containers/capsules/noa-digest.yaml`

### CRM Strangler Pattern (US7)

- [ ] T287 [US7] Implement CRM Strangler Proxy service in `sys/core/src/services/crm_strangler.rs`
- [ ] T288 [P] [US7] Implement shadow mode in `sys/core/src/services/crm_strangler.rs`
- [ ] T289 [P] [US7] Implement write-through mode in `sys/core/src/services/crm_strangler.rs`
- [ ] T290 [P] [US7] Implement instant rollback in `sys/core/src/services/crm_strangler.rs`

### Services (US7)

- [ ] T291 [US7] §3.3 Implement AgentService in `sys/core/src/services/agent_service.rs`
- [ ] T292 [P] [US7] Implement TaskService in `sys/core/src/services/task_service.rs`
- [ ] T293 [US7] §3.3 Implement OrchestrationService in `sys/core/src/services/orchestration_service.rs`

### CLI Commands (US7)

- [ ] T294 [US7] Implement `noa agents list` command in `sys/core/src/cli/agents.rs`
- [ ] T295 [P] [US7] Implement `noa tasks list` command in `sys/core/src/cli/tasks.rs`
- [ ] T296 [P] [US7] Implement `noa goal submit` command in `sys/core/src/cli/goal.rs`
- [ ] T297 [P] [US7] §3.5 Implement `noa logs` command in `sys/core/src/cli/logs.rs`
- [ ] T298 [P] [US7] Implement `noa capsule spawn` command in `sys/core/src/cli/capsule.rs`
- [ ] T299 [P] [US7] Implement `noa crm toggle` command in `sys/core/src/cli/crm.rs`

### API Endpoints (US7)

- [ ] T300 [US7] Implement GET /api/v1/agents endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T520 [P] [US7] Implement GET /api/v1/agents/{id} endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T521 [P] [US7] Implement POST /api/v1/agents/{id}/start endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T522 [P] [US7] Implement POST /api/v1/agents/{id}/stop endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T523 [P] [US7] Implement GET /api/v1/agents/{id}/logs endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T301 [P] [US7] Implement GET /api/v1/tasks endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T524 [P] [US7] Implement POST /api/v1/tasks endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T525 [P] [US7] Implement GET /api/v1/tasks/{id} endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T526 [P] [US7] Implement PATCH /api/v1/tasks/{id} endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T302 [US7] Implement POST /api/v1/goals endpoint in `sys/core/src/api/routes/goals.rs`
- [ ] T303 [P] [US7] Implement GET /api/v1/orchestration/status endpoint in `sys/core/src/api/routes/orchestration.rs`
- [ ] T304 [P] [US7] Implement POST /api/v1/capsule/spawn endpoint in `sys/core/src/api/routes/capsule.rs`
- [ ] T305 [P] [US7] Implement POST /api/v1/crm/toggle endpoint in `sys/core/src/api/routes/crm.rs`
- [ ] T306 [P] [US7] Implement POST /api/v1/crm/rollback endpoint in `sys/core/src/api/routes/crm.rs`

**US7 Acceptance Criteria**:
- [ ] Complex goals decomposed into tasks
- [ ] 98% task completion rate at 200 concurrent
- [ ] Proper escalation on failure

---

## Phase 10: User Story 8 - Self-Improvement & Code Modification (P3)

**Goal**: Continuously improve by analyzing performance and modifying own code

**Independent Test**: Trigger self-analysis, verify improvement proposals with rollback capability

### Dynamic Graphs Framework (US8)

- [ ] T307 [US8] §3.4 Implement Dynamic Graph base framework in `sys/core/src/graphs/base.rs`
- [ ] T308 [P] [US8] Implement EFG (Environment Function Graph) in `sys/core/src/graphs/efg.rs`
- [ ] T309 [P] [US8] Implement DSG (Dynamic Software Graph) in `sys/core/src/graphs/dsg.rs`
- [ ] T310 [P] [US8] Implement DHG (Dynamic Hardware Graph) in `sys/core/src/graphs/dhg.rs`
- [ ] T311 [P] [US8] Implement DPG (Dynamic Process Graph) in `sys/core/src/graphs/dpg.rs`
- [ ] T312 [P] [US8] Implement DRG (Dynamic Resource Graph) in `sys/core/src/graphs/drg.rs`
- [ ] T313 [P] [US8] Implement DSeCG (Dynamic Security Graph) in `sys/core/src/graphs/dsecg.rs`
- [ ] T314 [P] [US8] Implement DPeG (Dynamic Performance Graph) in `sys/core/src/graphs/dpeg.rs`
- [ ] T315 [P] [US8] Implement DEG (Dynamic Error Graph) in `sys/core/src/graphs/deg.rs`
- [ ] T316 [P] [US8] Implement DKG (Dynamic Knowledge Graph) in `sys/core/src/graphs/dkg.rs`

### Specialized CECCA Cells (US8)

- [ ] T317 [US8] §3.4 Implement CC_CONST (Constitution/Signer) in `sys/core/src/cecca/cells/constitution.rs`
- [ ] T318 [P] [US8] Implement CC_QUORUM (Virtual Board) in `sys/core/src/cecca/cells/quorum.rs`
- [ ] T319 [P] [US8] Implement CC_TRUTH (Truth-Gate) in `sys/core/src/cecca/cells/truth_gate.rs`
- [ ] T320 [US8] §3.4 Implement CC_STEM_REPL (Replicator) in `sys/core/src/cecca/cells/stem_repl.rs`
- [ ] T321 [P] [US8] Implement CC_STEM_DIFF (Differentiator) in `sys/core/src/cecca/cells/stem_diff.rs`
- [ ] T322 [US8] §3.4 Implement CC_CHOP (Capsule Surgeon) in `sys/core/src/cecca/cells/chop.rs`
- [ ] T323 [P] [US8] Implement CC_ARBITER (Promotion Arbiter) in `sys/core/src/cecca/cells/arbiter.rs`
- [ ] T324 [P] [US8] Implement CC_AUDIT (Global Auditor) in `sys/core/src/cecca/cells/audit.rs`
- [ ] T325 [P] [US8] Implement CC_LICENSE (License Gate) in `sys/core/src/cecca/cells/license.rs`

### Knowledge Capsules (KPLANE) (US8)

- [ ] T326 [US8] Implement KIDX_CAP (CAS Index & Blob Store) in `sys/core/src/knowledge/kidx.rs`
- [ ] T327 [P] [US8] Implement KSCHEMA_CAP (Schema Registry) in `sys/core/src/knowledge/kschema.rs`
- [ ] T328 [P] [US8] Implement KMETRICS_CAP (Metrics & Events) in `sys/core/src/knowledge/kmetrics.rs`
- [ ] T329 [P] [US8] Implement KDIR_CAP (Canonical Directory) in `sys/core/src/knowledge/kdir.rs`
- [ ] T330 [P] [US8] Implement KSNAP_CAP (Snapshot & Restore) in `sys/core/src/knowledge/ksnap.rs`
- [ ] T331 [P] [US8] Implement KCRASH_CAP (Crash Forensics) in `sys/core/src/knowledge/kcrash.rs`

### Shared Model Provider Techniques (US8)

- [ ] T332 [US8] §3.4 Implement ToolkenGPT (pre-trained tokens for toolken) in `sys/core/src/ml/toolkengpt.rs`
- [ ] T333 [P] [US8] Implement Replay Memory Cache in `sys/core/src/ml/replay_cache.rs`
- [ ] T334 [P] [US8] Implement EWC (Elastic Weight Consolidation) in `sys/core/src/ml/ewc.rs`
- [ ] T335 [P] [US8] Implement Progressive Neural Network adapters in `sys/core/src/ml/progressive_nn.rs`
- [ ] T336 [P] [US8] Implement Meta-Learning (MAML) wrapper in `sys/core/src/ml/maml.rs`
- [ ] T337 [P] [US8] Implement Neuromodulation techniques in `sys/core/src/ml/neuromodulation.rs`

### Self-Analysis (US8)

- [ ] T338 [US8] §3.4 Implement performance metrics collection in `sys/core/src/self_improve/metrics.rs`
- [ ] T339 [P] [US8] Implement efficiency analysis in `sys/core/src/self_improve/analyzer.rs`
- [ ] T340 [US8] §3.4 Implement improvement proposal generation in `sys/core/src/self_improve/proposals.rs`

### Safety & Rollback (US8)

- [ ] T341 [US8] §3.12 Implement pre-modification snapshot in `sys/core/src/self_improve/snapshot.rs`
- [ ] T342 [P] [US8] §3.12 Implement test runner for modifications in `sys/core/src/self_improve/test_runner.rs`
- [ ] T343 [US8] §3.12 Implement automatic rollback on test failure in `sys/core/src/self_improve/rollback.rs`

### Evidence-Based Execution Policy (US8)

- [ ] T344 [US8] §3.12 Implement Order of Truth Sources in `sys/core/src/policy/truth_sources.rs`
- [ ] T345 [P] [US8] Implement Hard Stop Rule in `sys/core/src/policy/hard_stop.rs`
- [ ] T346 [P] [US8] Implement Triple-Verify Rule in `sys/core/src/policy/triple_verify.rs`
- [ ] T347 [P] [US8] Implement Gap Hunt Rule in `sys/core/src/policy/gap_hunt.rs`

### AMPK Mode (Resource Scarcity) (US8)

- [ ] T348 [US8] §3.4 Implement AMPK mode detection in `sys/core/src/autonomy/ampk.rs`
- [ ] T349 [P] [US8] Implement throttle and quiesce logic in `sys/core/src/autonomy/ampk.rs`
- [ ] T350 [P] [US8] Implement prioritize and resume logic in `sys/core/src/autonomy/ampk.rs`

### Autonomy Loop (US8)

- [ ] T351 [US8] §3.4 Implement SENSE→DECIDE→UPDATE loop in `sys/core/src/autonomy/autonomy_loop.rs`
- [ ] T352 [P] [US8] Implement hourly self-reinvention scheduler in `sys/core/src/autonomy/scheduler.rs`

### Human Co-Improvement (US8)

- [ ] T353 [US8] §3.4 Implement improvement approval workflow in `sys/core/src/self_improve/approval.rs`
- [ ] T354 [P] [US8] Implement improvement audit log in `sys/core/src/self_improve/audit.rs`

### VHDX Integration (US8)

- [ ] T355 [US8] Implement VHDX stack packaging in `sys/core/src/vhdx/packaging.rs`
- [ ] T356 [P] [US8] Implement nested VHDX support in `sys/core/src/vhdx/nested.rs`
- [ ] T357 [P] [US8] Implement VHDX snapshot/rollback in `sys/core/src/vhdx/snapshot.rs`

### CLI Commands (US8)

- [ ] T358 [US8] Implement `noa improve analyze` command in `sys/core/src/cli/improve.rs`
- [ ] T359 [P] [US8] Implement `noa improve propose` command in `sys/core/src/cli/improve.rs`
- [ ] T360 [P] [US8] Implement `noa improve apply` command in `sys/core/src/cli/improve.rs`
- [ ] T361 [P] [US8] Implement `noa improve rollback` command in `sys/core/src/cli/improve.rs`

**US8 Acceptance Criteria**:
- [ ] 25+ Dynamic Graphs operational
- [ ] CECCA cells functional
- [ ] Improvement proposals include before/after comparison
- [ ] Automatic rollback on test failure
- [ ] All changes logged with rationale

---

## Phase 11: Project Management Integration

**Purpose**: BMAD, PRP, Spec-Kit, DSPy, Ruler integrations

### BMAD Integration

- [ ] T362 Implement BMAD workflow orchestrator in `ai/agents/bmad/orchestrator.ts`
- [ ] T363 [P] Implement BMAD analyst agent in `ai/agents/bmad/analyst.ts`
- [ ] T364 [P] Implement BMAD architect agent in `ai/agents/bmad/architect.ts`
- [ ] T365 [P] Implement BMAD po (product owner) agent in `ai/agents/bmad/po.ts`
- [ ] T366 [P] Implement BMAD dev agent in `ai/agents/bmad/dev.ts`
- [ ] T367 [P] Create BMAD document templates in `ai/agents/bmad/templates/`

### PRP Integration

- [ ] T368 Implement PRP workflow engine in `ai/agents/prp/workflow.ts`
- [ ] T369 [P] Implement PRP signal system (30+ signals) in `ai/agents/prp/signals.ts`
- [ ] T370 [P] Implement PRP LOOP MODE execution in `ai/agents/prp/loop_mode.ts`
- [ ] T371 [P] Implement robo-system-analyst agent in `ai/agents/prp/system_analyst.ts`
- [ ] T372 [P] Implement robo-aqa agent in `ai/agents/prp/aqa.ts`

### Spec-Kit Integration

- [ ] T373 Implement Spec-Kit provider auto-detection in `ai/agents/speckit/provider_detect.ts`
- [ ] T374 [P] Implement Spec-Kit command generation in `ai/agents/speckit/commands.ts`
- [ ] T375 [P] Create Spec-Kit CLI wrapper in `sys/core/src/cli/speckit.rs`

### DSPy Training Integration

- [ ] T376 Implement DSPy training pipeline in `sys/digest/src/training/dspy_trainer.py`
- [ ] T377 [P] Implement DSPy modules (Predict, ChainOfThought, ReAct) in `sys/digest/src/training/modules.py`
- [ ] T378 [P] Implement DSPy optimizers (MIPROv2, COPRO) in `sys/digest/src/training/optimizers.py`
- [ ] T379 [P] Implement PRD-based training data loader in `sys/digest/src/training/prd_loader.py`

### Ruler Integration

- [ ] T380 Create unified .ruler/AGENTS.md for all providers in `ruler/AGENTS.md`
- [ ] T381 [P] Implement TDD enforcement rules in `ruler/rules/tdd.md`
- [ ] T382 [P] Implement code quality rules in `ruler/rules/quality.md`
- [ ] T383 [P] Implement CI integration rules in `ruler/rules/ci.md`
- [ ] T384 [P] Implement AI commit attribution in `ruler/rules/attribution.md`

---

## Phase 12: Polish & Cross-Cutting Concerns

**Purpose**: Final integration, documentation, and quality assurance

### External Connectors (Feature-Flagged)

- [ ] T385 Implement connector framework in `sys/core/src/connectors/base.rs`
- [ ] T386 [P] Implement GitHub connector in `sys/core/src/connectors/github.rs`
- [ ] T387 [P] Implement OpenAI connector in `sys/core/src/connectors/openai.rs`
- [ ] T388 [P] Implement Claude connector in `sys/core/src/connectors/claude.rs`
- [ ] T389 [P] Implement Email (SMTP/IMAP) connector in `sys/core/src/connectors/email.rs`

### Secondary Layer Adapters

- [ ] T390 Implement adapter toggle framework in `sys/core/src/adapters/toggle.rs`
- [ ] T391 [P] Implement AD_DKR (Docker) adapter in `sys/core/src/adapters/docker.rs`
- [ ] T392 [P] Implement AD_GH (GitHub) adapter in `sys/core/src/adapters/github.rs`
- [ ] T393 [P] Implement AD_MINIO (MinIO) adapter in `sys/core/src/adapters/minio.rs`

### Kernel Independence Layer

- [ ] T394 Implement KernelAbstraction trait in `sys/core/src/kernel/abstraction.rs`
- [ ] T395 [P] Implement file operations (cross-platform) in `sys/core/src/kernel/file.rs`
- [ ] T396 [P] Implement process operations in `sys/core/src/kernel/process.rs`
- [ ] T397 [P] Implement network operations in `sys/core/src/kernel/network.rs`
- [ ] T398 [P] Implement platform detection in `sys/core/src/kernel/platform.rs`

### System Prompts & Grammars

- [ ] T399 Create noa-system.md system prompt in `ai/shared/prompts/noa-system.md`
- [ ] T400 [P] Create agent-coordinator.md prompt in `ai/shared/prompts/agent-coordinator.md`
- [ ] T401 [P] Create digest-analyst.md prompt in `ai/shared/prompts/digest-analyst.md`
- [ ] T402 [P] Create json.gbnf grammar in `ai/grammars/json.gbnf`
- [ ] T403 [P] Create structured-output.gbnf grammar in `ai/grammars/structured-output.gbnf`

### Documentation

- [ ] T404 [P] Update README.md with final quickstart in `noa_root/README.md`
- [ ] T405 [P] Generate API documentation from OpenAPI in `docs/api/`
- [ ] T406 [P] Create architecture documentation in `docs/architecture.md`
- [ ] T407 [P] Create contributing guide in `CONTRIBUTING.md`
- [ ] T408 [P] Create setup guides (windows.md, macos.md, linux.md) in `docs/setup/`

### Integration Testing

- [ ] T409 §3.12 Create end-to-end test suite for US1-US3 (MVP) in `tests/e2e/`
- [ ] T410 [P] §3.12 Create integration tests for US4-US7 in `tests/integration/`
- [ ] T411 [P] §3.12 Create performance benchmarks in `tests/benchmarks/`

### Universal Task Execution Policy Artifacts (§9)

Per [universal_task_execution_policy.md](../../project-mgmt/docs/05-policy/universal_task_execution_policy.md):

- [ ] T486 Create HASHES.txt generation script in `scripts/bash/generate-hashes.sh`
- [ ] T487 [P] Create SHA-256 hash generation for all key artifacts in `noa_root/test-results/HASHES.txt`
- [ ] T488 [P] Create FINAL_REPORT.md template in `noa_root/test-results/FINAL_REPORT.md`
- [ ] T489 [P] Create COVERAGE.md template (requirements → artifacts → tests map) in `noa_root/test-results/COVERAGE.md`
- [ ] T490 [P] Create REPRO.md template (exact environment, commands) in `noa_root/test-results/REPRO.md`
- [ ] T491 [P] Create EVIDENCE_LEDGER.md template in `noa_root/test-results/EVIDENCE_LEDGER.md`
- [ ] T492 Implement Triple-Verification Protocol (Pass A/B/C) automation in `scripts/bash/triple-verify.sh`
- [ ] T493 [P] Create Truth Gate checklist automation in `scripts/bash/truth-gate.sh`
- [ ] T494 [P] Implement Gap Hunt scan automation in `scripts/bash/gap-scan.sh`
- [ ] T495 [P] Create Claims Table generator in `scripts/bash/claims-table.sh`

### Rich Metadata & Schema Verification

- [ ] T496 Implement metadata validator (id, created_at, updated_at, checksum) in `sys/core/src/validation/metadata.rs`
- [ ] T497 [P] Implement embedding validator (model, vector, source_type, source_id) in `sys/core/src/validation/embedding.rs`
- [ ] T498 [P] Implement config schema validation against `config/schemas/` in `sys/core/src/validation/config.rs`
- [ ] T499 [P] Implement index verification for all database tables in `sys/core/src/validation/indexes.rs`
- [ ] T500 [P] Implement checksum generation and verification in `sys/core/src/validation/checksum.rs`

### Open Questions Resolution (from research.md)

- [ ] T530 Implement model download progress UI in `sys/ui/src/components/models/DownloadProgress.tsx`
- [ ] T531 [P] Implement P2P first-time device pairing wizard in `sys/ui/src/components/p2p/PairingWizard.tsx`
- [ ] T532 [P] Implement memory retention policy configuration in `config/memory-retention.json`
- [ ] T533 [P] Implement memory auto-prune service in `sys/core/src/services/memory_retention.rs`
- [ ] T534 [P] Create agent marketplace stub API in `sys/core/src/api/routes/marketplace.rs`
- [ ] T535 [P] Create agent marketplace stub UI in `sys/ui/src/components/marketplace/`

### Build & Release

- [ ] T412 Create cross-platform build scripts in `scripts/bash/release.sh`
- [ ] T413 [P] Create installer scripts (PowerShell, Bash) in `scripts/install/`
- [ ] T414 [P] Setup release automation in `.github/workflows/release.yml`

### Configuration Validation

- [ ] T415 §3.1 Implement config validation CLI command in `sys/core/src/cli/config.rs`
- [ ] T416 [P] Create default configuration templates in `config/templates/`

---

## Dependency Graph

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundation - Database & Storage)
    ↓
Phase 2.5 (Shared Provider Execution Memory) ← CRITICAL: Multi-provider collaborative execution
    ↓
┌───────────────────────────────────────┐
│         MVP (P1 User Stories)          │
│  ┌─────────────────────────────────┐   │
│  │ Phase 3: US1 (Init)             │   │
│  │            ↓                    │   │
│  │ Phase 4: US2 (Neural) ←────────┐│   │
│  │    + 13 ModelSelectorAgents    ││   │
│  │            ↓                   ││   │
│  │ Phase 5: US3 (Memory) ←────────┘│   │
│  └─────────────────────────────────┘   │
└───────────────────────────────────────┘
    ↓ MVP Complete
┌───────────────────────────────────────┐
│       P2 User Stories (Parallel)       │
│  Phase 6: US4 (Digest)    ←─┐          │
│  Phase 7: US5 (UI)        ←─┤ Independent│
│  Phase 8: US6 (P2P)       ←─┤          │
│  Phase 9: US7 (Agents)    ←─┘          │
└───────────────────────────────────────┘
    ↓
Phase 10: US8 (Self-Improve) ← Requires US7
    ↓
Phase 11 (Project Mgmt Integration)
    ↓
Phase 12 (Polish)
```

### Provider Integration Flow

```
Phase 2.5 Provider Setup:
    llama.cpp (5+ models) ───┐
    Claude Code (CLI/Cloud/IDE) ───┤
    Codex (CLI/Cloud/IDE) ───┤
    VS Code Copilot (IDE) ───┼──> Shared Execution Memory Bus ──> Collaborative Reasoning
    Git CLI ───┤
    Cursor (IDE/CLI/Cloud) ───┤
    Abacus (CLI/Cloud) ───┘
```

## Parallel Execution Opportunities

### Within Foundation (Phase 2)
```
T018 (Schema) → T019-T036 (Tables) [PARALLEL]
T050 (Errors) → T051-T055 (Config/Logging/DB) [PARALLEL]
T056 (Server) → T057-T060 (Endpoints/Middleware) [PARALLEL]
T061 (CLI) → T062-T067 (Commands) [PARALLEL]
```

### Within MVP Stories
```
US1: T072 → T073-T078 [PARALLEL] → T079
US2: T106-T110 [PARALLEL] → T114 → T118
US3: T131 → T132-T133 [PARALLEL] → T134-T138 [PARALLEL]
```

### P2 Stories (Fully Parallel)
```
US4 (Digest) ─────────┐
US5 (UI)    ─────────┼─── All execute in parallel
US6 (P2P)   ─────────┤
US7 (Agents) ────────┘
```

---

## Implementation Strategy

### MVP First (P1 Stories: US1, US2, US3)

**Goal**: Working seed environment with local intelligence and memory

1. Complete Phase 1-2 (Setup + Foundation)
2. Implement US1 (Init) - Self-contained environment
3. Implement US2 (Neural) - Local inference
4. Implement US3 (Memory) - Persistent recall

**MVP Deliverable**: `noa init && noa ask "Hello" && noa memory create/search`

### Incremental Delivery (P2 Stories)

After MVP, implement P2 stories in parallel:
- **US4**: Digest pipeline for knowledge acquisition
- **US5**: UI for user interaction
- **US6**: P2P for device federation
- **US7**: Agent orchestration for complex tasks

### Advanced Features (P3)

Finally, implement self-improvement (US8) which requires stable agent framework (US7).

---

## Summary

| Metric | Count |
|--------|-------|
| **Total Tasks** | 544 |
| **Phase 1 (Setup)** | 10 |
| **Phase 2 (Foundation)** | 60 |
| **Phase 2.5 (Shared Providers)** | 48 |
| **US1 Tasks** | 25 |
| **US2 Tasks** | 58 (includes 13 ModelSelectorAgents + 8 Multi-GPU + 3 model endpoints) |
| **US3 Tasks** | 22 |
| **US4 Tasks** | 50 (includes 11 digest source/knowledge endpoints) |
| **US5 Tasks** | 38 |
| **US6 Tasks** | 49 (includes 21 P2P protocol service tasks: Discovery 4, Sync 4, Compute 4, Storage 5 + 4 infra) |
| **US7 Tasks** | 56 (includes 7 agent/task endpoints) |
| **US8 Tasks** | 55 |
| **Phase 11 (Project Mgmt)** | 23 |
| **Phase 12 (Polish)** | 53 (includes 21 Execution Policy + Open Questions tasks) |
| **Parallelizable [P]** | 365 (67%) |

### New Requirements Covered (FR-037 to FR-050)
- **Shared Provider Execution Memory**: ✅ 48 tasks (FR-037 to FR-042)
- **Collaborative Reasoning**: ✅ T450-T453
- **8 Provider Types**: ✅ llama.cpp, Claude, Codex, Copilot, Git, Cursor, Abacus
- **5+ Concurrent Local Models**: ✅ T429-T431
- **Multi-GPU Support**: ✅ 8 tasks T478-T485 (FR-047 to FR-050)
- **CUDA 13.1+ Tiles**: ✅ T483

### Contract Coverage (from contracts/)
- **noa-core.openapi.yaml**: ✅ Health, Memory, Agents, Tasks, Models, System endpoints
- **digest-pipeline.openapi.yaml**: ✅ Sources, Jobs, Knowledge, Security endpoints
- **p2p-protocol.proto**: ✅ All 4 services covered:
  - Discovery (T501-T503): Announce, FindPeers, Ping
  - Sync (T505-T508): GetState, PushChanges, PullChanges, ResolveConflict
  - Storage (T509-T513): Store, Retrieve, Exists, List, Replicate
  - Compute (T514-T517): SubmitTask, GetTaskStatus, CancelTask, StreamOutput

### Universal Task Execution Policy Artifacts (§9)
- **Hash Generation**: ✅ T486-T487 (SHA-256 for all artifacts)
- **Verification Artifacts**: ✅ T488-T491 (FINAL_REPORT, COVERAGE, REPRO, EVIDENCE_LEDGER)
- **Triple-Verification**: ✅ T492 (Pass A/B/C automation)
- **Truth Gate**: ✅ T493 (§4 compliance automation)
- **Gap Hunt**: ✅ T494 (coverage scan)
- **Rich Metadata**: ✅ T496-T500 (validators for metadata, embeddings, configs, indexes, checksums)

### Open Questions Addressed (from research.md)
- **Model Download UX**: ✅ T530 (progress UI)
- **P2P Bootstrap**: ✅ T531 (first-time pairing wizard)
- **Memory Retention Policy**: ✅ T532-T533 (config + auto-prune)
- **Agent Marketplace**: ✅ T534-T535 (stub API + UI)

### MVP Scope (US1 + US2 + US3)
- **Tasks**: 223 (Foundation + Providers + 3 stories + Multi-GPU)
- **Estimated Duration**: 8-10 weeks with 2 developers

### Full Implementation
- **Tasks**: 535
- **Estimated Duration**: 20-24 weeks with 2-4 developers
