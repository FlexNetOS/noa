# Provider Shared Resources Research and Fix Prompt

## Context

This prompt is for researching and fixing provider shared resources in the NOA codebase. The NOA system follows a unified architecture with providers, shared resources, spec-kit tools, and a centralized configuration system.

## Task Overview

Research and fix all issues related to provider shared resources, including cache management, third-party integrations, and resource configuration across the codebase.

## Reference Documentation

Before starting, thoroughly review these key documents:

1. **`/n/noa/README.md`** - Full system architecture, directory structure, and unified architecture documentation
2. **`/n/noa/AGENTS.md`** (if exists) or `/n/noa/AGENTS.md` - Agent system documentation
3. **`/n/noa/config/README.md`** - Configuration system and schema policy

## Critical Analysis Areas

### 1. Third-Party Resources in Cache

**Location:** `/n/noa/cache/downloads/thirdparty/agentgateway.zip`

**Required Actions:**

1. Create a temporary extraction directory:
   ```bash
   mkdir -p /n/noa/tmp/agentgateway-extraction
   ```

2. Extract the archive to the temp folder:
   ```bash
   cd /n/noa/tmp/agentgateway-extraction
   unzip ../../cache/downloads/thirdparty/agentgateway.zip
   ```

3. Analyze the extracted contents:
   - Identify the purpose and structure of the agent gateway
   - Document all configuration files
   - Note any dependencies or requirements
   - Check for any security considerations

4. Determine the proper location for deployment:
   - Based on the NOA architecture (see README.md), the likely target is:
     - `/n/noa/gateway/mcp/` for MCP-related components
     - `/n/noa/gateway/api/` for API gateway components
     - `/n/noa/providers/` for provider-specific code
   - Consult the unified architecture diagram in README.md

5. Move/integrate components to proper locations:
   - Follow the microkernel architecture principles
   - Ensure proper separation of concerns
   - Update any configuration references

6. Clean up temporary extraction directory after deployment

### 2. Provider Shared Resources Configuration

**Locations to investigate:**

- `/n/noa/config/shared-resources.json` (if exists)
- `/n/noa/config/ai-providers.json`
- `/n/noa/config/providers/` directory
- `/n/noa/providers/shared/` directory (per README.md architecture)

**Tasks:**

1. **Inventory all provider configurations:**
   ```bash
   # Search for all provider-related configs
   find /n/noa/config -name "*provider*" -o -name "*shared*"
   find /n/noa/providers -type f -name "*.json" -o -name "*.yaml" -o -name "*.toml"
   ```

2. **Validate against schema:**
   - Check `/n/noa/config/schemas/` for provider schemas
   - Ensure all configs have proper metadata structure (version, description, updated_at)
   - Validate JSON/YAML syntax

3. **Identify shared resources:**
   - KV cache configurations
   - Embedding cache settings
   - Model registries
   - Resource pools
   - Budget allocations

4. **Check for inconsistencies:**
   - Duplicate resource definitions
   - Conflicting cache locations
   - Missing environment variable references (`${NOA_ROOT}`)
   - Hardcoded paths that should be configurable

### 3. Spec-Kit Tools and Resources

**Areas to examine:**

1. **Spec-Kit Tool Integration:**
   ```bash
   # Find spec-kit related files
   find /n/noa/tools -name "*spec*"
   find /n/noa/specs -type f -name "*.md"
   ```

2. **Resource Registry:**
   - Check `/n/noa/ai/shared/resources/resource-registry.json`
   - Verify all spec-kit tools are registered
   - Ensure resource mappings are correct

3. **Spec Distribution:**
   - Review `/n/noa/ai/shared/resources/spec-distribution.json`
   - Validate spec references and paths

### 4. Cache Management

**Directories to audit:**

- `/n/noa/cache/` - Main cache directory
- `/n/noa/cache/downloads/` - Downloaded resources
- `/n/noa/cache/downloads/thirdparty/` - Third-party archives
- `/n/noa/data/cache/` - Runtime cache (per architecture)

**Actions:**

1. **Review cache configuration:**
   ```bash
   cat /n/noa/cache/cache-config.json
   ```

2. **Check cache policies:**
   - Bounded cache limits
   - Rotation policies
   - GC policies
   - CAS integration for immutable artifacts

3. **Identify cache bloat:**
   ```bash
   # Find large files in cache
   find /n/noa/cache -type f -size +100M
   # Check total cache size
   du -sh /n/noa/cache/*
   ```

4. **Verify cache isolation:**
   - Provider caches should be isolated
   - IDE caches should be in `/n/noa/ide/` profiles
   - App caches in `/n/noa/apps/*/profiles/`

### 5. Provider Pool and Routing

**Architecture alignment:**

Per README.md, providers should have:

- `/n/noa/providers/local/` - Local inference (llama.cpp, candle)
- `/n/noa/providers/remote/` - Remote providers (codex_cli, claude_code_cli, copilot)
- `/n/noa/providers/shared/` - Shared resources (kv cache, embedding cache)
- `/n/noa/providers/pool/` - Scheduling, routing, budgets

**Verification tasks:**

1. **Check directory structure:**
   ```bash
   ls -la /n/noa/providers/
   ```

2. **Validate provider configs:**
   - Each provider should have proper config in `/n/noa/config/providers/`
   - Shared resources should be properly referenced
   - No duplicate resource allocations

3. **Review routing logic:**
   - Gateway MCP routing configuration
   - Locality routing (local → personal → regional → org)
   - Budget enforcement

### 6. Configuration Layer Compliance

Per the 3-layer config system in README.md:

**Layer 1 (Immutable Base):**
- `/n/noa/configs/base/` - Should contain baseline schemas, safety rails, sandbox definitions

**Layer 2 (Mutable Semantic):**
- `/n/noa/configs/semantic/` - Preferences, capabilities, device profiles, world model metadata

**Layer 3 (Enforcement):**
- `/n/noa/configs/enforcement/` - Validators, schema checkers, guardrails

**Verification:**

1. **Check if config directories follow 3-layer architecture:**
   ```bash
   ls -la /n/noa/configs/
   ls -la /n/noa/config/  # Current location
   ```

2. **Migrate if necessary:**
   - If configs are in `/n/noa/config/`, plan migration to `/n/noa/configs/` with proper layering
   - Preserve all metadata
   - Update all references

3. **Validate enforcement hooks:**
   - Check for validators in enforcement layer
   - Ensure schema validation is active
   - Verify guardrails are in place

## Deliverables

### 1. Analysis Report

Create `/n/noa/ai/shared/resources/11-research/provider-shared-resources-analysis.md` with:

- Current state assessment
- Issues discovered
- Architecture compliance gaps
- Resource conflicts
- Cache bloat analysis

### 2. Fix Implementation Plan

Create `/n/noa/ai/shared/resources/07-plans/provider-shared-resources-fix-plan.md` with:

- Step-by-step fixes
- File migrations needed
- Configuration updates
- Testing strategy
- Rollback plan

### 3. Updated Configurations

- Fixed/updated config files with proper metadata
- Schema validation passing
- Proper `${NOA_ROOT}` variable usage
- 3-layer config compliance

### 4. AgentGateway Integration

- Properly extracted and deployed agentgateway components
- Configuration integrated into NOA system
- Documentation of integration points

### 5. Resource Cleanup

- Removed duplicate resources
- Cleaned cache bloat
- Proper isolation of provider resources
- Updated resource registry

## Execution Checklist

- [ ] Read and understand `/n/noa/README.md` architecture
- [ ] Review agent system documentation (`AGENTS.md`)
- [ ] Extract and analyze `agentgateway.zip` in temp folder
- [ ] Deploy agentgateway to proper location(s)
- [ ] Audit all provider configurations
- [ ] Validate against schemas
- [ ] Check shared resource definitions
- [ ] Review cache management and policies
- [ ] Verify provider pool structure
- [ ] Assess 3-layer config compliance
- [ ] Identify and document all issues
- [ ] Create detailed fix plan
- [ ] Implement fixes
- [ ] Validate fixes with schema checker
- [ ] Update resource registry
- [ ] Clean up cache and temp files
- [ ] Document changes and integration points

## Tools to Use

### Spec-Kit Tools

Leverage the spec-kit tools available in the NOA system:

1. **Schema Validators:** `/n/noa/config/schemas/`
2. **Resource Registry:** `/n/noa/ai/shared/resources/resource-registry.json`
3. **Spec Distribution:** `/n/noa/ai/shared/resources/spec-distribution.json`
4. **Task Distribution:** `/n/noa/ai/shared/resources/task-distribution.yaml`

### Analysis Commands

```bash
# Find all provider references
grep -r "provider" /n/noa/config --include="*.json" --include="*.yaml"

# Check resource mappings
cat /n/noa/ai/shared/resources/resource-mapping.json

# Validate JSON files
find /n/noa/config -name "*.json" -exec echo "Checking {}" \; -exec jq empty {} \;

# Check for hardcoded paths
grep -r "/n/noa" /n/noa/config --include="*.json" --include="*.yaml" | grep -v "NOA_ROOT"

# Find large cache files
find /n/noa/cache -type f -size +50M -ls
```

## Success Criteria

1. All provider configurations are valid and schema-compliant
2. Shared resources are properly defined without conflicts
3. AgentGateway is properly integrated and documented
4. Cache is cleaned and properly bounded
5. Configuration follows 3-layer architecture
6. All temporary extraction artifacts are cleaned up
7. Resource registry is updated and accurate
8. No hardcoded paths (all use `${NOA_ROOT}`)
9. Provider pool structure matches README.md architecture
10. Documentation is complete and up-to-date

## Notes

- **Do not delete** any existing configs without backing them up first
- **Preserve metadata** in all configuration files (version, description, updated_at)
- **Follow the microkernel principle** - keep sys/core minimal
- **Use CAS references** for immutable artifacts
- **Respect sandbox boundaries** for execution
- **Document all changes** in the analysis and fix plan documents

## References

- Main Architecture: `/n/noa/README.md`
- Agent Documentation: `/n/noa/AGENTS.md`
- Config README: `/n/noa/config/README.md`
- Config Schemas: `/n/noa/config/schemas/`
- Resource Registry: `/n/noa/ai/shared/resources/resource-registry.json`
