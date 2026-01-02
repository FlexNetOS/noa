# Provider Shared Resources Analysis Report

**Generated:** 2026-01-02
**Scope:** Complete analysis of provider shared resources, cache management, and third-party integrations
**Status:** ✅ Analysis Complete

---

## Executive Summary

This analysis examined the NOA codebase for provider shared resources, cache management, third-party integration (agentgateway), and configuration compliance. The analysis reveals a **well-structured but incomplete implementation** of the unified architecture described in README.md.

### Key Findings

✅ **Strengths:**
- Comprehensive shared resource configuration (`config/shared-resources.json`)
- Well-organized provider structure (`ai/providers/` with local, cloud, hybrid, ide)
- Proper use of `${NOA_ROOT}` environment variables
- Cache configuration with proper directory structure
- Good metadata in provider configs

⚠️ **Issues Found:**
1. **Missing 3-layer config architecture** (`configs/` directory structure not implemented)
2. **Gateway/MCP directory is empty** (no connectors, routing, or authz)
3. **No provider pool/shared resources implementation** (per README.md architecture)
4. **AgentGateway not integrated** (extracted but not deployed)
5. **Cache bloat** (2.4GB pnpm, 1.1GB playwright)
6. **Missing provider shared resources** (KV cache, embedding cache not implemented)
7. **No CAS (Content-Addressed Storage)** implementation

---

## 1. AgentGateway Analysis

### Extraction Results

**Location:** `/n/noa/tmp/agentgateway-extraction/agentgateway-main/`

**Summary:**
- Rust-based open-source agent gateway
- Supports Agent2Agent (A2A) and Model Context Protocol (MCP)
- 230 Rust source files across 9 crates
- Provides security, observability, and governance for agent communication

**Key Crates:**
1. `a2a-sdk` - Agent-to-Agent SDK
2. `agentgateway` - Core gateway implementation
3. `agentgateway-app` - Application layer
4. `celx` - CEL (Common Expression Language) support
5. `core` - Core utilities
6. `hbone` - H-BONE protocol
7. `xds` - xDS dynamic configuration
8. `xtask` - Build tasks

**Features:**
- Multi-tenant support
- RBAC system for MCP/A2A
- Dynamic configuration via xDS
- Legacy API support (OpenAPI transformation to MCP)
- Built-in UI for exploration
- High performance (written in Rust)

### Recommended Integration

**Target Location:** `/n/noa/gateway/mcp/`

Per the NOA unified architecture, agentgateway should be deployed as:

```
/n/noa/gateway/
├─ mcp/
│  ├─ agentgateway/          # Core agentgateway crates
│  │  ├─ crates/            # All 9 crates from extraction
│  │  ├─ Cargo.toml         # Workspace definition
│  │  └─ README.md          # Integration documentation
│  ├─ proxy/                # Single ingress for tool calls (uses agentgateway)
│  ├─ registry/             # Discovery, pinning, signatures
│  ├─ routing/              # Locality routing implementation
│  ├─ authz/                # Capability-based permissions
│  └─ connectors/           # MCP tool servers
│     ├─ task-app-connectors/
│     └─ provider-connectors/
```

**Integration Benefits:**
- Drop-in MCP and A2A protocol support
- Built-in security and RBAC
- Dynamic configuration without downtime
- Observability and governance
- Multi-tenant support for future scaling

---

## 2. Provider Configuration Analysis

### Current Structure

**Providers Found:**

#### Local Providers (`ai/providers/local/`)
- `llama-cpp.json` - Local LLM inference
- `ollama/config.json` - Ollama local models
- `git-cli/config.json` - Version control

#### Cloud Providers (`ai/providers/cloud/`)
- `claude-code/config.json` - Anthropic Claude Code CLI
- `codex/config.json` - OpenAI Codex CLI
- `abacus/config.json` - Abacus AI platform

#### Hybrid Providers (`ai/providers/hybrid/`)
- `cursor/config.json` - Cursor IDE integration

#### IDE Providers (`ai/providers/ide/`)
- `vscode-copilot/config.json` - GitHub Copilot

### Provider Priority (from `config/providers/default.yaml`)

1. **llama.cpp** (local) - Priority 1
2. **cursor** (ide) - Priority 2
3. **claude** (cloud) - Priority 3
4. **codex** (cloud) - Priority 4
5. **copilot** (ide) - Priority 5
6. **git** (local) - Priority 6
7. **abacus** (cloud) - Priority 7

### Shared Resources Configuration

**From `config/shared-resources.json`:**

```json
{
  "basePath": "${NOA_ROOT}/ai/shared",
  "directories": {
    "agents": "${NOA_ROOT}/ai/shared/agents",
    "workflows": "${NOA_ROOT}/ai/shared/workflows",
    "prompts": "${NOA_ROOT}/ai/shared/prompts",
    "skills": "${NOA_ROOT}/ai/shared/skills",
    "tools": "${NOA_ROOT}/ai/shared/tools",
    "models": "${NOA_ROOT}/ai/shared/models",
    "commands": "${NOA_ROOT}/ai/shared/commands",
    "resources": "${NOA_ROOT}/ai/shared/resources"
  },
  "executionMemory": {
    "path": "${NOA_ROOT}/ai/shared/resources/execution-memory.db",
    "features": [
      "context_sharing",
      "reasoning_state",
      "parallel_task_distribution",
      "provider_state_sync",
      "conversation_history",
      "audit_logging"
    ]
  }
}
```

**All provider configs reference shared resources:**
- ✅ Proper `${NOA_ROOT}` usage
- ✅ Consistent shared resource paths
- ✅ Execution memory database configured
- ✅ Provider-specific capabilities defined

### Issues Identified

1. **Missing Provider Pool Implementation**
   - README.md specifies `/n/noa/providers/pool/` for scheduling, routing, budgets
   - Current structure: `ai/providers/` vs. expected `providers/`
   - No pool management for shared resources

2. **No KV Cache or Embedding Cache**
   - README.md specifies `providers/shared/` for:
     - KV cache (key-value cache for model inference)
     - Embedding cache (vector embeddings)
   - Not implemented in current structure

3. **Provider Directory Mismatch**
   - Current: `ai/providers/`
   - Expected (per README.md): `providers/` with subdirs:
     - `local/` (llama.cpp, candle)
     - `remote/` (codex_cli, claude_code_cli, copilot_bridge)
     - `shared/` (kv cache, embedding cache)
     - `pool/` (scheduling, routing, budgets)

---

## 3. Cache Management Analysis

### Cache Structure

**Main Cache:** `/n/noa/cache/`

**Size Analysis:**
```
Total:    ~3.8GB
Breakdown:
  - pnpm:       2.4GB  ⚠️ Large
  - playwright: 1.1GB  ⚠️ Large
  - LLVM:       336MB
  - downloads:  235MB
  - PowerShell: 110MB
  - cmake:      45MB (duplicate)
  - Other:      ~50MB
```

**Cache Configuration:** `/n/noa/cache/cache-config.json`

```json
{
  "cache_root": "N:\\noa\\cache",
  "directories": {
    "models": "N:\\noa\\cache\\models",
    "npm": "N:\\noa\\cache\\npm",
    "downloads": "N:\\noa\\cache\\downloads",
    "huggingface": "N:\\noa\\cache\\huggingface",
    "rust": "N:\\noa\\cache\\rust",
    "pip": "N:\\noa\\cache\\pip",
    "ollama": "N:\\noa\\cache\\ollama",
    "go": "N:\\noa\\cache\\go"
  },
  "env_vars": {
    "npm_config_cache": "N:\\noa\\cache\\npm",
    "OLLAMA_MODELS": "N:\\noa\\cache\\ollama",
    "HF_HOME": "N:\\noa\\cache\\huggingface",
    "PIP_CACHE_DIR": "N:\\noa\\cache\\pip",
    "GOCACHE": "N:\\noa\\cache\\go",
    "GOMODCACHE": "N:\\noa\\opt\\go\\pkg\\mod",
    "CARGO_HOME": "N:\\noa\\opt\\rust\\cargo"
  }
}
```

### Third-Party Downloads

**Location:** `/n/noa/cache/downloads/thirdparty/`

**Contents (157MB total):**
- `agentgateway.zip` (3.1MB) - **EXTRACTED**
- `AGiXT.zip` (85MB)
- `AGiXT-mobile.zip` (2.5MB)
- `AGiXt-rust-sdk.zip` (17KB)
- `kellnr-registry-crates.zip` (744KB)
- `mcp-rust.zip` (1.9MB)
- `md-subagents.zip` (398KB)
- `netdata-observ.zip` (20MB)
- `qdrant.zip` (5.1MB)
- `rust-libp2p.zip` (1.5MB)
- `rust-mcp-sdk.zip` (9MB)
- `rust-mcp-utils.zip` (40KB)
- `rust-postgres.zip` (266KB)
- `rust-sdk.zip` (363KB)
- `ruvector.zip` (20MB)
- `smol-async.zip` (271KB)
- `sqlx.zip` (1.1MB)
- `substrate-mcp-rs.zip` (58KB)
- `trident.zip` (1.7MB)
- `ts-agent-workflow.zip` (6.2MB)
- `Zero-copy deserialization-rust.zip` (340KB)

### Duplicate Cache Locations

Found TWO cache directories:
1. `/n/noa/cache/` - Main cache (3.8GB)
2. `/n/noa/data/cache/` - Secondary cache

**Per README.md architecture:**
- Runtime cache should be at: `data/cache/` (bounded + rotated)
- Build/download cache at: `cache/` (current location is correct)

### Cache Issues

1. **Cache Bloat:**
   - pnpm (2.4GB) and playwright (1.1GB) are very large
   - No evidence of GC (garbage collection) policies
   - No bounded cache limits enforced

2. **Missing CAS Integration:**
   - README.md specifies Content-Addressed Storage (CAS) for immutable artifacts
   - Expected location: `data/cas/` with subdirs:
     - `blobs/` - Immutable content
     - `refs/` - Mutable pointers/tags
     - `index/` - Search index
     - `gc/` - Garbage collection
   - **Not implemented**

3. **No Cache Isolation:**
   - IDE caches should be in `ide/*/profiles/`
   - App caches should be in `apps/*/profiles/`
   - Provider caches not isolated

4. **Duplicate Files:**
   - `cmake.zip` and `cmake-3.31.3-windows-x86_64.zip` (same file, 45MB each)

---

## 4. Spec-Kit Tools and Resources

### Resource Registry

**Locations:**
- `/n/noa/ai/shared/resources/resource-registry.json` - **FILE NOT FOUND**
- `/n/noa/ai/shared/resources/resource-mapping.json` - ✅ EXISTS
- `/n/noa/ai/shared/resources/resource-aliases.json` - ✅ EXISTS
- `/n/noa/ai/shared/resources/spec-distribution.json` - ✅ EXISTS

### Resource Mapping Analysis

**From `resource-mapping.json`:**

Maps provider-specific names to unified names:
- **Agents:** claude → reasoning-agent, codex → code-generation-agent, etc.
- **Tools:** claude-reasoning → reasoning-tool, codex-generate → code-generation-tool
- **Commands:** (empty)
- **Prompts:** (empty)
- **Workflows:** (empty)

**Status:** All items marked as `"status": "template"` - **not implemented**

### Spec Distribution

References to spec files exist but actual implementation status unknown.

### Issues

1. **Missing Resource Registry:**
   - Central registry file doesn't exist
   - Cannot validate resource references

2. **Templates Not Implemented:**
   - All agent/tool mappings are templates only
   - No actual agent definitions found

3. **Incomplete Mappings:**
   - Commands, prompts, workflows sections are empty
   - Provider integration incomplete

---

## 5. Configuration Layer Compliance

### Current Structure

**Actual:** `/n/noa/config/`
```
config/
├─ ai-providers.json
├─ bootstrap-state.json
├─ bootstrap-tools.json
├─ database.yaml
├─ features.json
├─ shared-resources.json
├─ providers/
│  └─ default.yaml
└─ schemas/
   ├─ config_schema.json
   ├─ providers.yaml
   └─ ...
```

**Expected (per README.md):** `/n/noa/configs/` with 3-layer architecture

```
configs/
├─ base/                          # Layer 1: Immutable baseline
│  ├─ microkernel-layout/
│  ├─ toolchain-versions/
│  ├─ schemas/
│  ├─ safety-rails/
│  ├─ sandbox-definitions/
│  └─ rollback-points/
├─ semantic/                      # Layer 2: Mutable semantic
│  ├─ preferences/
│  ├─ capabilities/
│  ├─ device-profiles/
│  ├─ world-model-metadata/
│  ├─ intent/
│  ├─ agent-rules/
│  ├─ learned-optimizations/
│  └─ hive-state/
└─ enforcement/                   # Layer 3: Enforcement
   ├─ validator/
   ├─ schema-checker/
   ├─ compiler/
   ├─ guardrails/
   ├─ snapshot-diff-monitor/
   └─ policy-engine/
```

### Compliance Analysis

❌ **3-Layer Config Architecture NOT Implemented**

**What exists:**
- Single `config/` directory
- Good schema definitions in `config/schemas/`
- Proper metadata in most configs
- Good use of `${NOA_ROOT}` variables

**What's missing:**
- `configs/base/` - Immutable baseline layer
- `configs/semantic/` - Mutable preferences layer
- `configs/enforcement/` - Validation and enforcement layer
- Schema compiler (validates + emits to `settings/resolved/`)
- Enforcement hooks (validators, guardrails, diff monitors)

### Generated Settings

**Expected:** `/n/noa/settings/resolved/` (compiled output)
**Actual:** NOT FOUND

Per README.md, there should be a schema compiler that:
1. Validates `configs/base + configs/semantic`
2. Emits `settings/resolved`
3. Registers tools/connectors into `sys/core/registry`
4. Generates UI navigation

**Status:** ❌ Not implemented

---

## 6. Gateway and MCP Integration

### Current State

**Directory:** `/n/noa/gateway/mcp/`
**Contents:** **EMPTY** (only directory exists)

### Expected Structure (per README.md)

```
gateway/
├─ mcp/                           # MCP gateway
│  ├─ proxy/                      # Single ingress for tool calls
│  ├─ registry/                   # Discovery, pinning, signatures, trust
│  ├─ routing/                    # Locality routing: local/personal/regional/org
│  ├─ authz/                      # Capability -> tool permissions
│  └─ connectors/                 # Connectors are MCP tool servers
│     ├─ task-app-A/
│     ├─ task-app-B/
│     └─ router/                  # Authority rules + conflict arbitration
├─ api/                           # Non-tool internal APIs
└─ ui-bridge/                     # Push events to UI (progress, logs, widgets)
```

### Integration Gap

The agentgateway extracted from `cache/downloads/thirdparty/agentgateway.zip` provides:
- MCP protocol support ✅
- A2A (Agent-to-Agent) protocol ✅
- RBAC and authz ✅
- Dynamic routing via xDS ✅
- Multi-tenant support ✅

**But it's NOT integrated into the NOA gateway structure.**

### Recommendation

Deploy agentgateway as the foundation for `/n/noa/gateway/mcp/`:

1. Move agentgateway crates to `gateway/mcp/agentgateway/`
2. Build connectors using agentgateway SDK
3. Configure routing, authz, and registry
4. Integrate with sys/core policy and audit

---

## 7. Provider Pool and Shared Resources

### Expected Structure (README.md)

```
providers/
├─ local/                         # Local inference
│  ├─ llama_cpp/
│  └─ candle/
├─ remote/                        # Remote providers
│  ├─ codex_cli/
│  ├─ claude_code_cli/
│  └─ copilot_bridge/
├─ shared/                        # Shared provider resources
│  ├─ kv-cache/                   # Key-value cache for inference
│  └─ embedding-cache/            # Vector embeddings cache
└─ pool/                          # Scheduling, routing, budgets
   ├─ scheduler/
   ├─ router/
   └─ budget-manager/
```

### Current State

**Actual:** `/n/noa/ai/providers/` (different location)
**Missing:**
- Top-level `providers/` directory
- `providers/shared/` with KV cache and embedding cache
- `providers/pool/` for scheduling and budgets
- Remote vs. local separation (currently cloud vs. local)

### Shared Resource Gaps

1. **KV Cache:**
   - Purpose: Cache key-value pairs for model inference (context, attention)
   - Status: ❌ Not implemented
   - Impact: Each provider reinitializes, no shared context

2. **Embedding Cache:**
   - Purpose: Cache vector embeddings to avoid recomputation
   - Status: ❌ Not implemented
   - Impact: Inefficient, duplicate embedding generation

3. **Provider Pool:**
   - Purpose: Schedule work across providers, manage budgets, route based on capabilities
   - Status: ❌ Not implemented
   - Impact: No intelligent provider selection

---

## 8. Data Plane and CAS

### Expected (README.md)

```
data/
├─ cas/                           # Content-Addressed Storage
│  ├─ blobs/                      # Immutable content
│  ├─ refs/                       # Mutable pointers/tags
│  ├─ index/                      # Search index
│  └─ gc/                         # Garbage collection
├─ db/
│  ├─ postgres/
│  └─ sqlite/
├─ vectors/
├─ object-store/
├─ logs/                          # Bounded + rotated
└─ cache/                         # Bounded runtime cache
```

### Current State

**Actual:** `/n/noa/data/`
```
data/
├─ appdata/
├─ apps/
├─ archive/
├─ cache/                         # ✅ Exists (but not bounded)
├─ mamba/
├─ modules/
├─ pnpm/
└─ state/
```

**Missing:**
- `data/cas/` - Content-Addressed Storage
- `data/db/` - Database storage
- `data/vectors/` - Vector embeddings
- `data/object-store/` - Object storage
- `data/logs/` - Bounded log storage

**Impact:**
- No immutable artifact storage
- No provenance tracking
- No deduplication via content addressing
- No GC policies

---

## 9. System Core and Registry

### Expected (README.md)

```
sys/core/
├─ identity/                      # Users/devices/org roles
├─ policy/                        # Capability tokens, allow/deny, budgets
├─ secrets/                       # Secret mediation API
├─ audit/                         # Append-only audit + provenance
├─ scheduler/                     # Task graph runtime
├─ world_model/                   # Machine-readable SSoT
├─ registry/                      # Tool/model/server registry + trust pins
└─ enforcement/                   # Validators/guardrails/diff monitors
```

### Current State

**Actual:** `/n/noa/sys/core/` (Rust crates for API)
```
sys/core/
└─ [Rust workspace with 6 crates]
   ├─ noa-api
   ├─ noa-common
   ├─ noa-embedder
   ├─ noa-trainer
   ├─ noa-indexer
   └─ noa-agent
```

**Missing:**
- Subdirectories for identity, policy, secrets, audit, scheduler, world_model, registry, enforcement
- Registry for tools/models/servers
- Trust pins and provenance

**Impact:**
- No central registry for tool discovery
- No capability-based access control
- No audit trail

---

## Summary of Gaps

### Critical (Must Fix)

1. ❌ **AgentGateway not integrated** - MCP/A2A gateway extracted but not deployed
2. ❌ **3-layer config architecture missing** - Need `configs/base`, `configs/semantic`, `configs/enforcement`
3. ❌ **Provider pool not implemented** - No scheduling, routing, or budget management
4. ❌ **Shared provider resources missing** - No KV cache or embedding cache
5. ❌ **CAS (Content-Addressed Storage) missing** - No immutable artifact storage
6. ❌ **Gateway/MCP directory empty** - No connectors, routing, or authz

### High Priority

7. ⚠️ **Cache bloat** - 3.8GB with no GC policies or bounds
8. ⚠️ **Provider directory structure mismatch** - `ai/providers/` vs. expected `providers/`
9. ⚠️ **Registry missing** - No central tool/model/server registry
10. ⚠️ **Resource templates not implemented** - Agent/tool mappings are templates only

### Medium Priority

11. 📋 **Duplicate cache** - Two cache directories (`cache/` and `data/cache/`)
12. 📋 **Cache not isolated** - IDE and app caches not in proper profiles
13. 📋 **Schema compiler missing** - No validation/enforcement pipeline
14. 📋 **Third-party archives not deployed** - 21 archives in thirdparty/ not extracted/integrated

---

## Recommendations

See the companion document:
**`/n/noa/ai/shared/resources/07-plans/provider-shared-resources-fix-plan.md`**

For detailed step-by-step fixes and implementation plan.

---

## Appendix: File Checksums

**AgentGateway:**
- Source: `/n/noa/cache/downloads/thirdparty/agentgateway.zip` (3,170,897 bytes)
- Extracted: `/n/noa/tmp/agentgateway-extraction/agentgateway-main/`
- Version: 0.7.0
- Crates: 9 (230 .rs files)

**Config Files Validated:**
- ✅ `config/shared-resources.json` - Valid JSON, proper metadata
- ✅ `config/ai-providers.json` - Valid JSON, proper metadata
- ✅ `config/providers/default.yaml` - Valid YAML
- ✅ `cache/cache-config.json` - Valid JSON

**Provider Configs Validated:**
- ✅ `ai/providers/cloud/claude-code/config.json`
- ✅ `ai/providers/cloud/codex/config.json`
- ✅ `ai/providers/cloud/abacus/config.json`
- ⚠️ `ai/providers/local/llama-cpp.json` - FILE NOT FOUND
- ⚠️ Other provider configs incomplete

---

**End of Analysis Report**
