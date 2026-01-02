# Provider Shared Resources Fix Implementation Plan

**Version:** 1.0.0
**Created:** 2026-01-02
**Status:** 📋 Ready for Implementation
**Prerequisites:** Analysis report completed

---

## Overview

This plan addresses all critical and high-priority issues identified in the provider shared resources analysis. Implementation is organized into phases with clear dependencies and validation steps.

---

## Phase 1: Foundation - Directory Structure and 3-Layer Config

**Priority:** 🔴 Critical
**Duration:** High
**Dependencies:** None

### 1.1 Create 3-Layer Configuration Architecture

**Goal:** Implement the immutable base, mutable semantic, and enforcement layers.

```bash
# Create directory structure
mkdir -p configs/base/{microkernel-layout,toolchain-versions,schemas,safety-rails,sandbox-definitions,rollback-points}
mkdir -p configs/semantic/{preferences,capabilities,device-profiles,world-model-metadata,intent,agent-rules,learned-optimizations,hive-state}
mkdir -p configs/enforcement/{validator,schema-checker,compiler,guardrails,snapshot-diff-monitor,policy-engine}
mkdir -p settings/resolved
mkdir -p settings/profiles
mkdir -p settings/overrides
```

**Implementation Steps:**

1. **Layer 1 (Immutable Base):**
   ```bash
   # Migrate baseline configs to configs/base/
   cp config/bootstrap-tools.json configs/base/toolchain-versions/
   cp config/schemas/* configs/base/schemas/

   # Create microkernel layout definition
   cat > configs/base/microkernel-layout/directory-structure.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Immutable directory structure baseline",
     "updated_at": "2026-01-02T00:00:00Z",
     "structure": {
       "sys/core": "Trusted microkernel",
       "gateway/mcp": "Tool bus and routing",
       "providers": "Model providers and pools",
       "sandbox": "Execution containment",
       "data/cas": "Content-addressed storage"
     }
   }
   EOF
   ```

2. **Layer 2 (Mutable Semantic):**
   ```bash
   # Migrate mutable configs
   cp config/features.json configs/semantic/preferences/
   cp config/device-orchestration.json configs/semantic/device-profiles/
   cp config/ai-providers.json configs/semantic/capabilities/
   ```

3. **Layer 3 (Enforcement):**
   ```bash
   # Create schema validator
   cat > configs/enforcement/schema-checker/validator.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "JSON Schema validator configuration",
     "rules": {
       "require_metadata": true,
       "require_version": true,
       "require_updated_at": true,
       "allow_noa_root_var": true
     },
     "schemas_path": "${NOA_ROOT}/configs/base/schemas/"
   }
   EOF
   ```

**Validation:**
```bash
# Verify directory structure
find configs -type d | sort
# Expected: All 3 layers with subdirectories

# Verify no hardcoded paths
grep -r "N:\\\\noa" configs/ && echo "FAIL: Hardcoded paths found" || echo "PASS"
```

---

### 1.2 Migrate Provider Structure

**Goal:** Align with README.md architecture: `/n/noa/providers/`

```bash
# Create new provider structure
mkdir -p providers/local/{llama_cpp,candle,ollama}
mkdir -p providers/remote/{codex_cli,claude_code_cli,copilot_bridge}
mkdir -p providers/shared/{kv-cache,embedding-cache}
mkdir -p providers/pool/{scheduler,router,budget-manager}
```

**Migration Steps:**

1. **Migrate local providers:**
   ```bash
   # Copy and adapt configs
   cp -r ai/providers/local/* providers/local/
   # Rename directories to match README.md naming
   mv providers/local/llama-cpp providers/local/llama_cpp 2>/dev/null || true
   ```

2. **Migrate remote providers:**
   ```bash
   # Cloud providers become remote
   cp -r ai/providers/cloud/claude-code providers/remote/claude_code_cli
   cp -r ai/providers/cloud/codex providers/remote/codex_cli
   # IDE provider copilot becomes remote
   cp -r ai/providers/ide/vscode-copilot providers/remote/copilot_bridge
   ```

3. **Create shared resource configurations:**
   ```bash
   # KV Cache config
   cat > providers/shared/kv-cache/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Shared key-value cache for model inference",
     "metadata": {
       "version": "1.0.0",
       "description": "Shared KV cache across providers",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "cache": {
       "type": "redis",
       "host": "${NOA_REDIS_HOST:-localhost}",
       "port": "${NOA_REDIS_PORT:-6379}",
       "db": 0,
       "ttl_seconds": 3600,
       "max_memory": "2GB"
     },
     "providers": {
       "llama_cpp": { "enabled": true, "namespace": "llama" },
       "claude_code_cli": { "enabled": true, "namespace": "claude" },
       "codex_cli": { "enabled": true, "namespace": "codex" }
     }
   }
   EOF

   # Embedding Cache config
   cat > providers/shared/embedding-cache/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Shared embedding cache for vector operations",
     "metadata": {
       "version": "1.0.0",
       "description": "Shared embedding cache across providers",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "cache": {
       "type": "qdrant",
       "host": "${NOA_QDRANT_HOST:-localhost}",
       "port": "${NOA_QDRANT_PORT:-6333}",
       "collection": "noa_embeddings",
       "vector_size": 768,
       "distance": "Cosine"
     },
     "providers": {
       "llama_cpp": { "enabled": true, "model": "all-MiniLM-L6-v2" },
       "claude_code_cli": { "enabled": true, "model": "voyage-2" }
     }
   }
   EOF
   ```

4. **Create provider pool:**
   ```bash
   # Scheduler config
   cat > providers/pool/scheduler/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Provider task scheduling and queue management",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "scheduling": {
       "algorithm": "priority_capability_match",
       "queue_type": "redis",
       "max_concurrent_per_provider": 10
     },
     "priority_order": [
       "local/llama_cpp",
       "remote/claude_code_cli",
       "remote/codex_cli",
       "remote/copilot_bridge"
     ]
   }
   EOF

   # Router config
   cat > providers/pool/router/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Provider routing and capability matching",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "routing": {
       "locality_preference": ["local", "personal", "regional", "org"],
       "capability_matching": true,
       "load_balancing": "round_robin"
     }
   }
   EOF

   # Budget manager config
   cat > providers/pool/budget-manager/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Provider budget and rate limit management",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "budgets": {
       "local/llama_cpp": { "cost_per_token": 0, "rate_limit": 100 },
       "remote/claude_code_cli": { "cost_per_token": 0.00001, "rate_limit": 50 },
       "remote/codex_cli": { "cost_per_token": 0.00002, "rate_limit": 30 }
     }
   }
   EOF
   ```

**Validation:**
```bash
# Verify structure
ls -la providers/
# Expected: local/, remote/, shared/, pool/

# Validate all configs have metadata
find providers -name "config.json" -exec echo "Checking {}" \; -exec jq '.metadata' {} \;
```

---

## Phase 2: AgentGateway Integration

**Priority:** 🔴 Critical
**Duration:** High
**Dependencies:** Phase 1 complete

### 2.1 Deploy AgentGateway to Gateway/MCP

**Goal:** Integrate the extracted agentgateway as the MCP foundation.

```bash
# Create gateway structure
mkdir -p gateway/mcp/{agentgateway,proxy,registry,routing,authz,connectors}
mkdir -p gateway/api
mkdir -p gateway/ui-bridge
```

**Implementation Steps:**

1. **Move AgentGateway:**
   ```bash
   # Copy agentgateway workspace
   cp -r tmp/agentgateway-extraction/agentgateway-main/* gateway/mcp/agentgateway/

   # Create README
   cat > gateway/mcp/agentgateway/INTEGRATION.md << 'EOF'
   # AgentGateway Integration

   This directory contains the AgentGateway Rust workspace, which provides:
   - MCP (Model Context Protocol) support
   - A2A (Agent-to-Agent) protocol support
   - RBAC and authorization
   - Dynamic routing via xDS
   - Multi-tenant support

   ## Building

   ```bash
   cd gateway/mcp/agentgateway
   cargo build --release
   ```

   ## Crates

   - `a2a-sdk` - Agent-to-Agent SDK
   - `agentgateway` - Core gateway implementation
   - `agentgateway-app` - Application layer
   - `celx` - CEL expression support
   - `core` - Core utilities
   - `hbone` - H-BONE protocol
   - `xds` - Dynamic configuration

   ## Configuration

   See `../../configs/` for gateway configuration.
   EOF
   ```

2. **Create Gateway Proxy Layer:**
   ```bash
   cat > gateway/mcp/proxy/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "MCP proxy configuration - single ingress for tool calls",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "proxy": {
       "listen_address": "${NOA_MCP_PROXY_ADDR:-0.0.0.0:9090}",
       "backend": "agentgateway",
       "agentgateway_path": "${NOA_ROOT}/gateway/mcp/agentgateway",
       "max_connections": 1000
     },
     "protocols": {
       "mcp": { "enabled": true, "version": "1.0" },
       "a2a": { "enabled": true, "version": "1.0" }
     }
   }
   EOF
   ```

3. **Create Registry:**
   ```bash
   cat > gateway/mcp/registry/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Tool/server registry with trust pins",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "registry": {
       "storage": "${NOA_ROOT}/data/registry.db",
       "discovery": {
         "enabled": true,
         "scan_paths": [
           "${NOA_ROOT}/tools",
           "${NOA_ROOT}/gateway/mcp/connectors"
         ],
         "interval_seconds": 60
       },
       "trust": {
         "require_signature": false,
         "allowed_sources": ["local", "noa.dev"]
       }
     }
   }
   EOF
   ```

4. **Create Routing Layer:**
   ```bash
   cat > gateway/mcp/routing/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Locality routing: local → personal → regional → org",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "routing": {
       "locality_order": ["local", "personal", "regional", "org"],
       "prefer_local": true,
       "fallback_enabled": true
     },
     "nodes": {
       "local": { "priority": 1, "latency_target_ms": 10 },
       "personal": { "priority": 2, "latency_target_ms": 50 },
       "regional": { "priority": 3, "latency_target_ms": 200 },
       "org": { "priority": 4, "latency_target_ms": 500 }
     }
   }
   EOF
   ```

5. **Create Authorization Layer:**
   ```bash
   cat > gateway/mcp/authz/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Capability-based authorization for MCP tools",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "authz": {
       "engine": "agentgateway_rbac",
       "policies_path": "${NOA_ROOT}/configs/semantic/capabilities/",
       "default_policy": "deny"
     },
     "capabilities": {
       "reasoning": ["claude_code_cli", "llama_cpp"],
       "code_generation": ["codex_cli", "copilot_bridge"],
       "file_operations": ["claude_code_cli"],
       "git_operations": ["claude_code_cli"]
     }
   }
   EOF
   ```

**Validation:**
```bash
# Build agentgateway
cd gateway/mcp/agentgateway
cargo build --release 2>&1 | tee build.log
# Expected: Successful build

# Verify configs
find gateway/mcp -name "config.json" -exec jq .metadata {} \;
# Expected: All have metadata
```

---

### 2.2 Create MCP Connectors

**Goal:** Build connectors for provider integration.

```bash
mkdir -p gateway/mcp/connectors/{provider-connectors,tool-connectors}
```

**Implementation:**

1. **Provider Connector Template:**
   ```bash
   cat > gateway/mcp/connectors/provider-connectors/template.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Provider connector template for MCP integration",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "connector": {
       "type": "provider",
       "protocol": "mcp",
       "capabilities": [],
       "routing": "gateway/mcp/routing"
     }
   }
   EOF
   ```

2. **Claude Code Connector:**
   ```bash
   cat > gateway/mcp/connectors/provider-connectors/claude-code.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Claude Code CLI connector for MCP",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "connector": {
       "type": "provider",
       "provider_id": "claude_code_cli",
       "protocol": "mcp",
       "capabilities": ["reasoning", "code", "git", "files"],
       "command": "${NOA_ROOT}/providers/remote/claude_code_cli/bin/claude",
       "wrapper": "${NOA_ROOT}/gateway/mcp/connectors/provider-connectors/claude-code-wrapper.sh"
     }
   }
   EOF
   ```

**Validation:**
```bash
# Verify connectors
ls -la gateway/mcp/connectors/
# Expected: provider-connectors/ and tool-connectors/
```

---

## Phase 3: Data Plane and CAS

**Priority:** 🔴 Critical
**Duration:** Medium
**Dependencies:** Phase 1 complete

### 3.1 Implement Content-Addressed Storage (CAS)

**Goal:** Create immutable artifact storage with deduplication.

```bash
# Create CAS structure
mkdir -p data/cas/{blobs,refs,index,gc}
mkdir -p data/db/{postgres,sqlite}
mkdir -p data/vectors
mkdir -p data/object-store
mkdir -p data/logs
```

**Implementation:**

1. **CAS Configuration:**
   ```bash
   cat > data/cas/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Content-Addressed Storage configuration",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "cas": {
       "blobs_path": "${NOA_ROOT}/data/cas/blobs",
       "refs_path": "${NOA_ROOT}/data/cas/refs",
       "index_path": "${NOA_ROOT}/data/cas/index",
       "hash_algorithm": "blake3",
       "compression": "zstd"
     },
     "gc": {
       "enabled": true,
       "interval_hours": 24,
       "min_age_days": 7,
       "keep_refs": true
     },
     "limits": {
       "max_blob_size_mb": 1024,
       "max_total_size_gb": 100
     }
   }
   EOF
   ```

2. **GC Policy:**
   ```bash
   cat > data/cas/gc/policy.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "CAS garbage collection policy",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "gc_policy": {
       "retain_tagged": true,
       "retain_recent_days": 30,
       "retain_pinned": true,
       "scan_interval_hours": 24
     }
   }
   EOF
   ```

**Validation:**
```bash
# Verify CAS structure
ls -la data/cas/
# Expected: blobs/, refs/, index/, gc/, config.json

# Test CAS write (placeholder)
echo "test" | sha256sum
# Expected: hash output
```

---

### 3.2 Implement Bounded Cache Policies

**Goal:** Add GC and limits to prevent cache bloat.

```bash
# Update cache config
cat > cache/gc-policy.json << 'EOF'
{
  "version": "1.0.0",
  "description": "Cache garbage collection and limits policy",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "gc": {
    "enabled": true,
    "interval_hours": 24,
    "targets": {
      "pnpm": { "max_size_gb": 1.0, "max_age_days": 90 },
      "playwright": { "max_size_gb": 0.5, "max_age_days": 60 },
      "npm": { "max_size_gb": 0.5, "max_age_days": 90 },
      "downloads": { "max_size_gb": 0.5, "max_age_days": 180 }
    }
  },
  "isolation": {
    "ide_profiles": "${NOA_ROOT}/ide/*/profiles/cache",
    "app_profiles": "${NOA_ROOT}/apps/*/profiles/cache"
  }
}
EOF
```

**Cleanup Duplicates:**
```bash
# Remove duplicate cmake
rm cache/cmake.zip  # Keep the versioned one

# Clean up old downloads if needed
find cache/downloads -type f -mtime +180 -delete
```

**Validation:**
```bash
# Check cache sizes after cleanup
du -sh cache/*
# Expected: Reduced from 3.8GB
```

---

## Phase 4: System Core Registry

**Priority:** 🟡 High
**Duration:** Medium
**Dependencies:** Phase 2 complete

### 4.1 Implement Core Registry

**Goal:** Central registry for tools, models, servers with trust pins.

```bash
mkdir -p sys/core/registry
mkdir -p sys/core/policy
mkdir -p sys/core/audit
```

**Implementation:**

1. **Registry Database:**
   ```bash
   cat > sys/core/registry/schema.sql << 'EOF'
   -- Tool/Model/Server Registry
   CREATE TABLE IF NOT EXISTS registry_entries (
     id TEXT PRIMARY KEY,
     type TEXT NOT NULL, -- 'tool', 'model', 'server', 'connector'
     name TEXT NOT NULL,
     version TEXT NOT NULL,
     capabilities TEXT, -- JSON array
     trust_level TEXT NOT NULL, -- 'trusted', 'verified', 'unverified'
     signature TEXT,
     metadata TEXT, -- JSON
     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );

   CREATE INDEX idx_type ON registry_entries(type);
   CREATE INDEX idx_trust ON registry_entries(trust_level);
   EOF

   # Initialize database
   sqlite3 sys/core/registry/registry.db < sys/core/registry/schema.sql
   ```

2. **Registry Config:**
   ```bash
   cat > sys/core/registry/config.json << 'EOF'
   {
     "version": "1.0.0",
     "description": "Core registry configuration",
     "metadata": {
       "version": "1.0.0",
       "updated_at": "2026-01-02T00:00:00Z"
     },
     "registry": {
       "database": "${NOA_ROOT}/sys/core/registry/registry.db",
       "auto_discover": true,
       "discovery_paths": [
         "${NOA_ROOT}/tools",
         "${NOA_ROOT}/gateway/mcp/connectors",
         "${NOA_ROOT}/providers"
       ]
     },
     "trust": {
       "require_signature_for_remote": true,
       "trusted_sources": ["noa.dev", "local"]
     }
   }
   EOF
   ```

**Validation:**
```bash
# Verify database created
sqlite3 sys/core/registry/registry.db ".tables"
# Expected: registry_entries
```

---

### 4.2 Policy and Audit

**Goal:** Implement capability-based access control and audit trail.

```bash
cat > sys/core/policy/capabilities.json << 'EOF'
{
  "version": "1.0.0",
  "description": "Capability-based access control policies",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "capabilities": {
    "reasoning": {
      "description": "Complex reasoning and analysis",
      "providers": ["claude_code_cli", "llama_cpp"],
      "budget_tokens_per_day": 100000
    },
    "code_generation": {
      "description": "Code generation and completion",
      "providers": ["codex_cli", "copilot_bridge"],
      "budget_tokens_per_day": 50000
    },
    "file_operations": {
      "description": "File read/write operations",
      "providers": ["claude_code_cli"],
      "sandbox_required": true
    }
  }
}
EOF

cat > sys/core/audit/config.json << 'EOF'
{
  "version": "1.0.0",
  "description": "Audit and provenance configuration",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "audit": {
    "enabled": true,
    "log_path": "${NOA_ROOT}/data/logs/audit.log",
    "rotation": {
      "max_size_mb": 100,
      "max_age_days": 90,
      "compress": true
    },
    "events": ["tool_call", "provider_switch", "capability_check", "budget_update"]
  }
}
EOF
```

---

## Phase 5: Resource Implementation

**Priority:** 🟡 High
**Duration:** Medium
**Dependencies:** Phase 1-3 complete

### 5.1 Implement Resource Registry

**Goal:** Create central resource registry as specified.

```bash
cat > ai/shared/resources/resource-registry.json << 'EOF'
{
  "$schema": "https://noa.local/schemas/resource-registry.json",
  "version": "1.0.0",
  "description": "Central registry of all shared AI resources",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "agents": {},
  "tools": {},
  "workflows": {},
  "prompts": {},
  "commands": {},
  "models": {}
}
EOF
```

### 5.2 Implement Agent and Tool Templates

**Goal:** Convert template mappings to actual implementations.

```bash
# Create reasoning agent
cat > ai/shared/agents/reasoning-agent.json << 'EOF'
{
  "version": "1.0.0",
  "description": "Unified reasoning agent mapping",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "agent": {
    "id": "reasoning-agent",
    "name": "Reasoning Agent",
    "capabilities": ["reasoning", "analysis", "long-context"],
    "providers": {
      "primary": "claude_code_cli",
      "fallback": "llama_cpp"
    },
    "config": {
      "context_length": 100000,
      "temperature": 0.7
    }
  }
}
EOF

# Create code generation agent
cat > ai/shared/agents/code-generation-agent.json << 'EOF'
{
  "version": "1.0.0",
  "description": "Unified code generation agent mapping",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "agent": {
    "id": "code-generation-agent",
    "name": "Code Generation Agent",
    "capabilities": ["code-generation", "completion", "refactoring"],
    "providers": {
      "primary": "codex_cli",
      "fallback": "copilot_bridge"
    },
    "config": {
      "context_length": 8000,
      "temperature": 0.3
    }
  }
}
EOF
```

---

## Phase 6: Third-Party Integration

**Priority:** 📋 Medium
**Duration:** Low-Medium
**Dependencies:** Phase 1-2 complete

### 6.1 Deploy Additional Third-Party Tools

**Goal:** Extract and integrate other useful archives.

**Priorities:**
1. **rust-mcp-sdk.zip** → `lib/noa-mcp/` (MCP SDK)
2. **rust-libp2p.zip** → `lib/noa-p2p/` (P2P networking)
3. **qdrant.zip** → `data/vectors/qdrant/` (Vector DB)
4. **sqlx.zip** → `lib/noa-db/sqlx/` (Database toolkit)

**Example:**
```bash
# Extract MCP SDK
mkdir -p tmp/mcp-sdk-extraction
cd tmp/mcp-sdk-extraction
unzip ../../cache/downloads/thirdparty/rust-mcp-sdk.zip

# Integrate into lib
cp -r rust-mcp-sdk-main/* ../../lib/noa-mcp/
```

**Lower Priority:**
- AGiXT.zip (85MB) - Alternative agent framework (evaluate if needed)
- netdata-observ.zip - Observability (defer to later phase)

---

## Phase 7: Validation and Testing

**Priority:** ✅ Critical
**Duration:** Medium
**Dependencies:** All phases complete

### 7.1 Schema Validation

```bash
# Validate all JSON configs
find configs -name "*.json" -exec echo "Validating {}" \; -exec jq empty {} \;

# Validate metadata presence
find configs providers gateway -name "*.json" -exec jq '.metadata' {} \; | grep -c null
# Expected: 0 (no nulls)
```

### 7.2 Path Validation

```bash
# Check for hardcoded paths
grep -r "N:\\\\noa\|C:\\\\Users" configs/ providers/ gateway/ && echo "FAIL" || echo "PASS"

# Verify ${NOA_ROOT} usage
grep -r "\${NOA_ROOT}" configs/ providers/ gateway/ | wc -l
# Expected: Many matches
```

### 7.3 Build Tests

```bash
# Build agentgateway
cd gateway/mcp/agentgateway
cargo test

# Build Rust workspace
cd sys/core
cargo test
```

### 7.4 Integration Test

```bash
# Test provider routing
# TODO: Create integration test script

# Test CAS write/read
# TODO: Create CAS test script

# Test cache GC
# TODO: Create cache GC test script
```

---

## Phase 8: Cleanup and Documentation

**Priority:** 📋 Medium
**Duration:** Low
**Dependencies:** Phase 7 complete

### 8.1 Cleanup Temporary Files

```bash
# Remove extraction directories
rm -rf tmp/agentgateway-extraction
rm -rf tmp/mcp-sdk-extraction

# Clean duplicate files
rm cache/cmake.zip  # Already removed in Phase 3

# Archive old configs (backup before removal)
mkdir -p .backups/config-migration-2026-01-02
cp -r config .backups/config-migration-2026-01-02/
# After validation, consider removing old config/ if all migrated
```

### 8.2 Update Documentation

```bash
# Update README.md with actual implementation status
# Update architecture diagrams
# Create integration guide
```

### 8.3 Update Resource Registry

```bash
# Register all implemented agents
# Register all tools
# Register all providers
# Register all connectors
```

---

## Rollback Plan

### Rollback Procedure

If any phase fails critically:

1. **Restore from Backups:**
   ```bash
   cp -r .backups/config-migration-2026-01-02/config ./
   ```

2. **Revert Directory Changes:**
   ```bash
   # Remove new directories if they cause issues
   rm -rf configs/  # If Layer 3 causes problems
   rm -rf providers/  # If provider migration fails
   rm -rf gateway/mcp/agentgateway/  # If agentgateway build fails
   ```

3. **Git Reset:**
   ```bash
   git status
   git checkout -- .  # Reset to last commit
   ```

### Rollback Points

- **After Phase 1:** Can rollback to original config/
- **After Phase 2:** Can remove gateway/mcp/ and continue with old structure
- **After Phase 3:** Can remove data/cas/ if CAS causes issues

---

## Success Criteria

### Phase 1 Success
- ✅ `configs/` directory with 3 layers
- ✅ `providers/` directory with local, remote, shared, pool
- ✅ No hardcoded paths
- ✅ All configs have metadata

### Phase 2 Success
- ✅ AgentGateway builds successfully
- ✅ Gateway directory structure complete
- ✅ Connectors configured

### Phase 3 Success
- ✅ CAS structure created
- ✅ Cache GC policy implemented
- ✅ Cache size reduced

### Phase 4 Success
- ✅ Registry database created
- ✅ Policy configs in place
- ✅ Audit logging configured

### Phase 5 Success
- ✅ Resource registry exists
- ✅ Agent templates implemented
- ✅ Tool templates implemented

### Overall Success
- ✅ All schema validations pass
- ✅ All builds succeed
- ✅ Integration tests pass
- ✅ Documentation updated
- ✅ Rollback plan tested

---

## Timeline Estimate

- **Phase 1:** 2-3 hours
- **Phase 2:** 3-4 hours
- **Phase 3:** 2-3 hours
- **Phase 4:** 2 hours
- **Phase 5:** 2 hours
- **Phase 6:** 1-2 hours
- **Phase 7:** 2 hours
- **Phase 8:** 1 hour

**Total:** ~15-19 hours of focused work

---

## Next Steps

1. **Review this plan** - Ensure all stakeholders agree
2. **Create feature branch:**
   ```bash
   git checkout -b feature/provider-shared-resources-fix
   ```
3. **Execute Phase 1** - Foundation and directory structure
4. **Commit after each phase** - Incremental, reviewable changes
5. **Test thoroughly** - Don't skip Phase 7 validation
6. **Document as you go** - Update docs immediately
7. **Create PR** - When all phases complete and tested

---

**End of Fix Implementation Plan**
