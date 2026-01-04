# NOA Framework Conflicts Analysis

**Date:** 2026-01-02
**Purpose:** Compare Phase 1 & 2 implementation against NOA policy framework
**Status:** 🔍 Analysis Complete

---

## Executive Summary

After reviewing the NOA policy documents (`ai/shared/resources/policy/`), several **conflicts and gaps** were identified between the implementation and the canonical NOA framework requirements.

**Critical Findings:**
1. ✅ **Aligned:** 3-layer configs, provider structure, gateway MCP
2. ⚠️ **Conflict:** CAS location and structure
3. ⚠️ **Gap:** Missing sys/core microkernel subdirectories
4. ⚠️ **Gap:** Constitution compliance not enforced
5. ⚠️ **Gap:** NOA_ROOT vs NOA_HOME not distinguished

---

## 1. CAS (Content-Addressed Storage) Conflicts

### Policy Requirement (03-configs_CAS.md)

**Canonical CAS Structure:**
```
${NOA_ROOT}/
  cas/                            # AUTHORITATIVE CAS root
    objects/                      # Immutable blobs (Merkle)
      ab/12cd34ef...              # Hash prefixes
    refs/                         # Mutable pointers
      latest-kernel
      active-configs
      agent-commander
    tags/                         # Named stable references
      v1.0.0
      stable
    registry/                     # Catalog of CAS objects
      models.json
      prompts.json
    gc/                           # GC rules + orphan tracking
    merkle/                       # Precomputed DAG structures
```

### Current Implementation

**What we created:**
```
data/cas/                         # ❌ WRONG LOCATION
  blobs/                          # ✅ Correct concept
  refs/                           # ✅ Correct concept
  index/                          # ⚠️ Should be in cas/registry/
  gc/                             # ✅ Correct
```

**Location mismatch:**
- Policy: `${NOA_ROOT}/cas/`
- Implementation: `${NOA_ROOT}/data/cas/`

### Conflict Resolution

**Decision:** Move CAS to canonical location

**Action Required:**
1. Move `data/cas/` → `cas/`
2. Restructure:
   - `cas/objects/` (was `blobs/`)
   - `cas/refs/` (keep)
   - `cas/tags/` (add)
   - `cas/registry/` (was `index/`)
   - `cas/gc/` (keep)
   - `cas/merkle/` (add)
3. Update all references in configss
4. Document `data/cas/` as DEPRECATED

---

## 2. Provider Structure Conflicts

### Policy Requirement (03-configs_CAS.md)

**Canonical Provider Location:**
```
${NOA_ROOT}/
  providers/                      # ✅ We implemented this
    provider.llamacpp.json
    provider.codex.json
    provider.claude.json
```

### Current Implementation

**What we created:**
```
providers/
  local/
    llama_cpp/
  remote/
    claude_code_cli/
    codex_cli/
  shared/
    kv-cache/
    embedding-cache/
  pool/
    scheduler/
    router/
    budget-manager/
```

**Analysis:**
- ✅ Location correct (`providers/`)
- ✅ Structure enhanced beyond policy (good!)
- ⚠️ Policy shows flat provider files, we have subdirs
- ✅ Our structure is MORE detailed (acceptable)

### Resolution

**Decision:** KEEP current structure (enhancement)

**Rationale:**
- Policy shows minimal structure
- Our implementation provides better organization
- Subdirectories for local/remote/shared/pool add value
- No actual conflict, just enhancement

---

## 3. configss vs configs Conflict

### Policy Framework

**Constitution references:**
- Section 3.8: "Centralized, immutable configsuration baseline"
- Section 3.9: "Mutable semantic layer on top of baseline"
- Section 3.10: "Enforcement layer validates and compiles"

**CAS Policy references:**
- "Immutable base + AI-Native mutable layer"
- "3-layer hybrid configsuration model"

### Current State

**Old location (still exists):**
- `configs/` - Original configss (DEPRECATED but not removed)

**New location (we created):**
- `configss/` - 3-layer architecture
  - `configss/base/` - Immutable
  - `configss/semantic/` - Mutable
  - `configss/enforcement/` - Validation

### Resolution

**Decision:** KEEP `configss/` (plural), DEPRECATE `configs/` (singular)

**Action Required:**
1. Add deprecation notice to `configs/README.md`
2. Document migration path in constitution
3. Plan `configs/` removal after full migration

---

## 4. sys/core Microkernel Gap

### Policy Requirement (02-ARCH_AER-SPEC.md)

**Expected sys/core structure:**
```
sys/core/
  identity/                       # Users/devices/org roles
  policy/                         # Capability tokens, allow/deny
  secrets/                        # Secret mediation API
  audit/                          # Append-only audit
  scheduler/                      # Task graph runtime
  world_model/                    # Machine-readable SSoT
  registry/                       # Tool/model/server registry
  enforcement/                    # Validators/guardrails
```

### Current Implementation

**What exists:**
```
sys/core/
  [Rust workspace with 6 crates]
  - noa-api
  - noa-common
  - noa-embedder
  - noa-trainer
  - noa-indexer
  - noa-agent
```

**What's missing:**
- ❌ No subdirectories (identity, policy, secrets, etc.)
- ❌ Registry not implemented
- ❌ Audit logging not structured
- ❌ Scheduler not in sys/core
- ❌ World model not defined

### Resolution

**Decision:** CREATE missing sys/core subdirectories

**Action Required:**
1. Create subdirectories in sys/core
2. Move scheduler configs from providers/pool → sys/core/scheduler
3. Implement registry database
4. Set up audit logging structure
5. Define world model schema

---

## 5. NOA_ROOT vs NOA_HOME Gap

### Policy Requirement (03-configs_CAS.md)

**Distinction:**
- `NOA_ROOT` = Ecosystem base install anchor (shared, persistent)
- `NOA_HOME` = Active instance directory (versioned/runtime-specific)
- May be same for single-folder installs
- Must be different for multi-version setups

### Current Implementation

**What we use:**
- `${NOA_ROOT}` - Used throughout all configss ✅
- `${NOA_HOME}` - NOT used ❌

**Examples from our configss:**
```json
"path": "${NOA_ROOT}/providers/local"
"database": "${NOA_ROOT}/data/registry.db"
```

### Resolution

**Decision:** INTRODUCE `${NOA_HOME}` for runtime-specific paths

**Mapping:**
```
# Shared, persistent (use NOA_ROOT)
${NOA_ROOT}/cas/
${NOA_ROOT}/providers/
${NOA_ROOT}/configss/base/
${NOA_ROOT}/data/

# Instance-specific (use NOA_HOME)
${NOA_HOME}/logs/
${NOA_HOME}/cache/
${NOA_HOME}/settings/resolved/
${NOA_HOME}/tmp/
```

**Action Required:**
1. Update configss to use NOA_HOME where appropriate
2. Document variable distinction
3. Set default: `NOA_HOME=${NOA_ROOT}` for single-folder installs

---

## 6. Constitution Compliance Gaps

### Policy Requirement (01_CONSTITUTION.md)

**Key Principles:**

**3.1 Self-Contained & Autonomous:**
- System MUST operate entirely inside NOA root
- No absolute host paths outside noa_root
- Runtime-resolved abstract path variable

**Status:** ✅ Compliant (we use `${NOA_ROOT}`)

**3.2 Local-First & Offline-Capable:**
- Core operations MUST work locally
- Third-party APIs optional
- Offline behavior defined

**Status:** ⚠️ Partial (we have Redis/Qdrant dependencies)

**3.3 Agentic Orchestration & Hive-Mind:**
- Network of specialized agents
- Multiple SLMs (<3B params) via llama.cpp
- Permanent agents: File I/O, Terminal, Dynamic RAG, Microservice Management

**Status:** ⚠️ Partial (gateway ready, agents not yet implemented)

**3.4 Adaptive & Self-Improving:**
- NOA MUST constantly reinvent itself (target: hourly)
- Self-modifications recorded with diffs
- Rollback path required

**Status:** ❌ Not implemented

**3.5 Transparent & Auditable:**
- All actions logged (who, why, what changed)
- Human-reviewable

**Status:** ⚠️ Partial (audit configs exists, not enforced)

### Resolution

**Decision:** DOCUMENT constitution gaps, implement incrementally

**Immediate Actions:**
1. Add offline fallbacks to Redis/Qdrant configss
2. Document audit enforcement in Phase 4
3. Plan self-improvement loop for later phase

---

## 7. Gateway vs Framework Alignment

### AgentGateway Integration

**Our Implementation:**
- AgentGateway deployed at `gateway/mcp/agentgateway/`
- MCP and A2A protocol support
- RBAC with capabilities
- Routing, registry, authz layers

**Framework Requirements:**
- ✅ Aligns with AER "monolith with microservices"
- ✅ MCP/A2A are compatible with agent orchestration
- ✅ Gateway acts as message bus for internal microservices

**Analysis:** ✅ GOOD ALIGNMENT

**Enhancements Needed:**
1. Link gateway to sys/core/registry
2. Integrate gateway with sys/core/audit
3. Use gateway for provider orchestration (Constitution 3.3)

---

## 8. Data Directory Structure Conflicts

### Policy Requirement (03-configs_CAS.md)

**Canonical data structure:**
```
${NOA_ROOT}/
  data/
    indexes/
      global.idx.json
      agent.idx.json
    knowledge/
      docs/
    logs/                         # ⚠️ Should be at root level
```

### Current Implementation

**What exists:**
```
data/
  appdata/
  apps/
  archive/
  cache/                          # ⚠️ Should be at root level
  mamba/
  modules/
  pnpm/
  state/
```

**Also exists:**
```
cache/                            # At root level
logs/                             # At root level
```

**Conflicts:**
- `data/cache/` vs `cache/` (duplicate)
- `data/logs/` (doesn't exist) vs `logs/` (exists)
- Missing `data/indexes/`
- Missing `data/knowledge/`

### Resolution

**Decision:** RESTRUCTURE data directory per policy

**Target structure:**
```
${NOA_ROOT}/
  cas/                            # Moved from data/cas/
  data/
    indexes/                      # NEW
      global.idx.json
      agent.idx.json
    knowledge/                    # NEW
      docs/
    db/
      postgres/
      sqlite/
    vectors/
    object-store/
  cache/                          # Keep at root
  logs/                           # Keep at root
```

**Action Required:**
1. Create `data/indexes/`
2. Create `data/knowledge/`
3. Remove `data/cache/` (use root `cache/`)
4. Move `data/cas/` → `cas/`
5. Clean up legacy dirs (appdata, pnpm, etc.)

---

## 9. Sandbox Structure Gap

### Policy Requirement (README.md Architecture)

**Expected:**
```
sandbox/
  runtime/
    runners/
    workspaces/
    mounts/
    network/
    limits/
  snapshots/
  policies/
```

### Current Implementation

**What exists:**
```
sandbox/                          # Directory exists (empty)
```

**What we have in configss:**
```
configss/base/sandbox-definitions/
  default-profiles.json           # ✅ Sandbox profiles defined
```

### Resolution

**Decision:** CREATE sandbox runtime structure

**Action Required:**
1. Create subdirectories in `sandbox/`
2. Link to `configss/base/sandbox-definitions/`
3. Implement sandbox runtime (Phase 3+)

---

## 10. Summary of Conflicts

### Critical (Must Fix)

1. **CAS Location** - Move `data/cas/` → `cas/`
2. **CAS Structure** - Rename `blobs/` → `objects/`, add `tags/`, `merkle/`
3. **sys/core Structure** - Add missing subdirectories
4. **Data Directory** - Add `indexes/`, `knowledge/`

### High Priority (Should Fix)

5. **NOA_HOME Variable** - Introduce for instance-specific paths
6. **configs Deprecation** - Mark `configs/` as deprecated
7. **Sandbox Runtime** - Create directory structure
8. **Offline Fallbacks** - Add to Redis/Qdrant configss

### Medium Priority (Plan for Later)

9. **Constitution Enforcement** - Implement audit, self-improvement
10. **Gateway Integration** - Link to sys/core/registry and audit
11. **Agent Implementation** - Permanent agents per Constitution 3.3

---

## 11. Recommended Fix Order

### Phase 2.5: Critical Fixes (Before Phase 3)

**Priority 1: CAS Restructure**
```bash
# 1. Create canonical CAS structure
mkdir -p cas/{objects,refs,tags,registry,gc,merkle}

# 2. Move data/cas content (if any)
# Note: data/cas/ was created but may be empty

# 3. Update all configs references
# Update: configss, gateway, providers configss
```

**Priority 2: sys/core Structure**
```bash
# 1. Create microkernel subdirectories
mkdir -p sys/core/{identity,policy,secrets,audit,scheduler,world_model,registry,enforcement}

# 2. Create initial configss for each
# (Will implement in Phase 4)
```

**Priority 3: Data Directory**
```bash
# 1. Create missing directories
mkdir -p data/{indexes,knowledge/docs,db/postgres,db/sqlite,vectors,object-store}

# 2. Remove deprecated locations
# Plan removal of data/cache (use root cache/)
```

**Priority 4: Variable Distinction**
```bash
# 1. Document NOA_ROOT vs NOA_HOME
# 2. Update configss to use NOA_HOME for:
#    - logs/, cache/, tmp/, settings/resolved/
```

### Phase 3: Data Plane (Updated)

With corrected CAS structure at `cas/` (not `data/cas/`)

### Phase 4: sys/core Registry (Enhanced)

With all subdirectories created and linked to gateway

---

## 12. Updated Architecture Diagram

```
${NOA_ROOT}/
├─ cas/                           # ✅ CANONICAL (not data/cas/)
│  ├─ objects/                    # Immutable blobs (was blobs/)
│  ├─ refs/                       # Mutable pointers
│  ├─ tags/                       # Named references
│  ├─ registry/                   # Catalog (was index/)
│  ├─ gc/                         # Garbage collection
│  └─ merkle/                     # DAG structures
│
├─ configss/                       # ✅ 3-layer (not configs/)
│  ├─ base/
│  ├─ semantic/
│  └─ enforcement/
│
├─ providers/                     # ✅ Enhanced structure
│  ├─ local/
│  ├─ remote/
│  ├─ shared/
│  └─ pool/
│
├─ sys/core/                      # ⚠️ NEEDS subdirectories
│  ├─ identity/                   # NEW
│  ├─ policy/                     # NEW
│  ├─ secrets/                    # NEW
│  ├─ audit/                      # NEW
│  ├─ scheduler/                  # NEW
│  ├─ world_model/                # NEW
│  ├─ registry/                   # NEW
│  └─ enforcement/                # NEW
│
├─ gateway/                       # ✅ Implemented
│  └─ mcp/
│
├─ data/                          # ⚠️ NEEDS indexes/, knowledge/
│  ├─ indexes/                    # NEW
│  ├─ knowledge/                  # NEW
│  ├─ db/
│  ├─ vectors/
│  └─ object-store/
│
├─ sandbox/                       # ⚠️ NEEDS runtime structure
│  ├─ runtime/                    # NEW
│  ├─ snapshots/                  # NEW
│  └─ policies/                   # Link to configss/base/
│
├─ cache/                         # ✅ At root (correct)
├─ logs/                          # ✅ At root (correct)
└─ settings/                      # ✅ Runtime configss
```

---

## 13. Constitution Compliance Checklist

### Self-Contained & Autonomous (3.1)
- ✅ All configss use `${NOA_ROOT}`
- ⚠️ Need `${NOA_HOME}` for instance-specific
- ✅ No hardcoded absolute paths

### Local-First & Offline-Capable (3.2)
- ⚠️ Redis dependency (need offline fallback)
- ⚠️ Qdrant dependency (need offline fallback)
- ✅ Core structure can work locally

### Agentic Orchestration & Hive-Mind (3.3)
- ✅ Gateway ready for agent coordination
- ❌ Permanent agents not yet implemented
- ❌ Provider orchestration mode not configsured
- ⚠️ llama.cpp integration pending

### Adaptive & Self-Improving (3.4)
- ❌ Self-modification not implemented
- ❌ Hourly reinvention not configsured
- ❌ Rollback mechanism not implemented

### Transparent & Auditable (3.5)
- ⚠️ Audit configss exist
- ❌ Audit enforcement not active
- ❌ Decision logging not implemented

**Compliance Score:** 2/5 ⚠️

---

## 14. Action Plan

### Immediate (Before Phase 3)

1. **Fix CAS Location**
   - Create `cas/` at root
   - Update all references
   - Document migration

2. **Create sys/core Structure**
   - Add all subdirectories
   - Create placeholder configss
   - Plan Phase 4 implementation

3. **Update Data Directory**
   - Add `indexes/` and `knowledge/`
   - Document structure
   - Clean legacy dirs

4. **Introduce NOA_HOME**
   - Document variable distinction
   - Update instance-specific paths
   - Set default for single-folder

### Phase 3 (Updated)

- Implement CAS with correct location (`cas/` not `data/cas/`)
- Use `objects/` instead of `blobs/`
- Add `tags/` and `merkle/` support

### Phase 4 (Enhanced)

- Implement sys/core/registry
- Link gateway to registry
- Implement audit enforcement
- Add policy enforcement

---

## 15. Documentation Updates Needed

### Update These Files:

1. **IMPLEMENTATION_SUMMARY.md**
   - Add conflicts section
   - Document CAS location fix
   - Update architecture diagram

2. **Fix Plan (07-plans/)**
   - Add Phase 2.5 (critical fixes)
   - Update Phase 3 with correct CAS location
   - Update Phase 4 with sys/core structure

3. **README.md**
   - Update architecture section
   - Add NOA_ROOT vs NOA_HOME explanation
   - Document configs/ → configss/ migration

4. **Gateway Integration (INTEGRATION.md)**
   - Link to sys/core/registry
   - Document audit integration
   - Add constitution compliance notes

---

**Status:** 🔍 Analysis Complete | ⚠️ Critical Conflicts Identified | 📋 Action Plan Ready

**Next Step:** Implement Phase 2.5 (Critical Fixes) before proceeding to Phase 3
