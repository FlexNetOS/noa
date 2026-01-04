Here is the **fully-specified, conflict-free hybrid configsuration architecture**�the one your NOA / Agentic-OS / p2p Hive-Mind system actually needs.

It merges:

* Immutable layer (Nix-style)
* Mutable semantic layer (AI-native)
* CAS backbone
* Merkle DAG referencing
* Shared provider resources
* Agents, tools, skills, workflows
* Logs, caches, indexes, prompts
* MOE routing
* Orchestration graph
* Garbage-collection logic
* Encryption semantics
* Mutable pointers (�refs�, �tags�)
* Multi-device sync + hive-mind memory

This is the **exact blueprint** for a next-generation agentic OS.
It�s deterministic where required and plastic everywhere else your system evolves.

I�ll present it as a **clear directory hierarchy**, then explain each layer in plain language.
This is the �final form� of yconfigsnfiguration model.

---

# **THE HYBRID configsURATION MODEL � FINAL BLUEPRINT**

### (Immutable Base + AI-Native Mutable Layer + CAS Spine + Merkle DAG Indexing)

```
$NOA_HOME/
??? immutable/                      # Nix-style: controlled, reproducible, read-only
?   ??? schema/                     # All schemas for validation
?   ?   ??? agent.schema.json
?   ?   ??? skill.schema.json
?   ?   ??? tool.schema.json
?   ?   ??? workflow.schema.json
?   ?   ??? configs.schema.json
?   ?   ??? world.schema.json
?   ?   ??? cas-object.schema.json
?   ??? kernels/                    # Microkernel blueprints & versions
?   ?   ??? base.toml
?   ?   ??? vmm.toml
?   ?   ??? sandbox.toml
?   ??? providers/                  # Static definitions for provider contracts (llama.cpp, Codex, Claude)
?   ?   ??? provider.llamacpp.json
?   ?   ??? provider.codex.json
?   ?   ??? provider.claude.json
?   ??? sandbox/                    # Immutable runtime environment definitions
?   ??? trust/                      # Keys, attestations, signatures
?       ??? root.pub
?       ??? providers.pub
?
??? mutable/                        # AI-native semantic layer (system rewrites allowed)
?   ??? configs/
?   ?   ??? world_model.json        # Machine-readable worldview & metadata
?   ?   ??? device_profile.json     # Hardware profile, locality, capabilities
?   ?   ??? hive_profile.json       # Identity in the swarm
?   ?   ??? preferences.nl          # Natural-language preference records
?   ?   ??? constraints.graph       # Semantic rules, safety limits, resource caps
?   ??? agents/                     # Agent registry (semantic description + compiled configs)
?   ?   ??? index.json              # Top-level manifest
?   ?   ??? commander/
?   ?   ?   ??? agent.json
?   ?   ?   ??? beliefs.graph
?   ?   ??? builder/
?   ?   ??? researcher/
?   ?   ??? auditor/
?   ??? skills/                     # Operator-level reusable capacities
?   ?   ??? planning.skill.json
?   ?   ??? search.skill.json
?   ?   ??? refactor.skill.json
?   ??? tools/                      # Tool definitions (semantic + execution plan)
?   ?   ??? fs.tool.json
?   ?   ??? exec.tool.json
?   ?   ??? web.tool.json
?   ??? prompts/                    # Prompt libraries (CAS-backed)
?   ?   ??? system/
?   ?   ??? tasks/
?   ?   ??? code/
?   ??? workflows/                  # DAGs the agents follow
?   ?   ??? build.yaml
?   ?   ??? debug.yaml
?   ?   ??? optimize.yaml
?   ??? commands/                   # User-exposed command definitions
?   ?   ??? index.json
?   ??? orchestration/              # MOE router + scheduling engine
?   ?   ??? moe.router.json
?   ?   ??? scheduler.json
?   ?   ??? cost_models.json
?   ??? hooks/                      # Mutation hooks, event triggers
?       ??? pre-validate.js
?       ??? post-commit.js
?
??? cache/                          # Never trusted; always regenerable
?   ??? models/
?   ??? embeddings/
?   ??? build_artifacts/
?   ??? temp/
?
??? logs/                           # Streaming logs + analytical logs
?   ??? agents/
?   ??? providers/
?   ??? errors/
?   ??? orchestration/
?
??? state/                          # Mutable state with invariants
?   ??? sessions/                   # Active session memory
?   ??? conversations/
?   ??? checkpoints/                # State snapshots
?   ??? metrics/                    # Telemetry
?
??? data/                           # Persistent, interpretable datasets
?   ??? indexes/
?   ?   ??? global.idx.json         # Search index for all CAS roots
?   ?   ??? agent.idx.json
?   ??? knowledge/                  # Structured + unstructured local knowledge
?       ??? docs/
?
??? cas/                            # Content Addressable Store (THE SPINE)
    ??? objects/                    # Immutable, deduped binary/JSON blobs (Merkle)
    ?   ??? ab/12cd34ef...          # Stored as hash prefixes
    ?   ??? ...
    ??? refs/                       # Mutable pointers to CAS objects
    ?   ??? latest-kernel
    ?   ??? active-configs
    ?   ??? agent-commander
    ?   ??? world-current
    ??? tags/                       # Named stable references (e.g., releases)
    ?   ??? v1.0.0
    ?   ??? stable
    ?   ??? dev
    ??? registry/                   # Central catalog of CAS objects for providers
    ?   ??? models.json
    ?   ??? prompts.json
    ?   ??? snapshots.json
    ??? gc/                         # Garbage collection rules + orphan tracking
    ?   ??? sweep.log
    ?   ??? gc_rules.json
    ??? merkle/                     # Precomputed DAG structures
        ??? root.hash
```

---

# **HOW IT WORKS � LAYER BY LAYER**

## **1. IMMUTABLE LAYER (Nix-style, reproducible, cryptographically proven)**

This layer prevents corruption, configs drift, catastrophic agent errors, or runaway mutations.

Contains:

* Schemas
* Microkernel definitions
* Provider contracts
* Sandbox constraints
* Trust anchors

This serves as the **DNA** of the OS.

No agent can modify this directly. All mutations must pass through a validator ? compiler pipeline.

---

## **2. MUTABLE SEMANTIC LAYER (AI-Native)**

This is where agents think, reason, learn, and evolve.

Contains:

* world model
* preferences
* constraints
* agent belief sets
* skill definitions
* workflow DAGs
* orchestration routing (MOE)
* hooks

Everything in this layer is:

* machine-interpretable
* machine-rewritable
* validated by schemas from the immutable layer
* logged
* reversible

This is the **epigenetics** of the OS.

---

# **3. CAS (CONTENT ADDRESSABLE STORE) � THE SPINE**

CAS provides:

### **? Immutable object storage**

Every blob (prompt, schema, agent state, workflow, compiled skill) = hash-addressed.

### **? Merkle DAG structure**

Everything is linked via cryptographically secure trees.
Perfect for:

* reproducibility
* versioning
* distributed sync
* deduplication
* binary provenance

### **? Mutable pointers (refs + tags)**

Refs = moveable pointers
Tags = stable, human-understandable names

### **? GC (Garbage Collection)**

Removes orphaned CAS objects safely.
Uses reachability from refs/tags.

### **? CAS + Encryption Options**

Hashing ciphertext avoids leaking plaintext:

* Hash(Encrypted Blob) stored in index
* Encrypted payload stored in CAS

### **? CAS Registries for AI Models**

Models (full or SLM) are stored as:

```
model.json ? CAS object
weights.bin ? CAS object
merkle root ? integrity proof
```

### **? Local CAS (per-device) + global CAS (swarm)**

Syncs via Merkle diffing.

This is the **nervous system**.

---

# **4. SHARED PROVIDER RESOURCES**

Since you have a provider triad:

* llama.cpp SLMs
* Codex CLI
* Claude Code CLI

They share:

```
$NOA_HOME/providers/
    provider.llamacpp.json
    provider.codex.json
    provider.claude.json
    registry/models.json
    registry/prompts.json
    registry/snapshots.json
```

Each provider exposes:

* capabilities
* cost models
* latency curves
* tool access
* context window limits

MOE routing uses this.

---

# **5. MOE ROUTER + ORCHESTRATION**

Located in:

```
mutable/orchestration/moe.router.json
```

This contains:

* router rules
* cost models
* specialization mapping
* per-agent routing tables

Example rule:

```
"agent_builder": {
    "prefer": ["claude", "codex"],
    "fallback": ["llamacpp"],
    "specialize_for": ["planning", "refactor"]
}
```

---

# **6. SKILLS, AGENTS, TOOLS, PROMPTS**

Everything is:

* schema-validated
* CAS-backed
* versioned
* diffable
* linked by Merkle edges

### Agents = brains

### Skills = functions

### Tools = actuators

### Workflows = plans

### Prompts = semantic glue

All share the same format:
metadata + CAS pointer to their authoritative object.

---

# **7. CACHE, LOGS, STATE, DATA**

These are the **physiology** of the OS.

They carry no authority�everything can be regenerated from immutable + CAS.

---

# **8. INDEXES**

Global search index stored in:

```
data/indexes/global.idx.json
```

Contains:

* CAS object metadata
* keywords
* embedding references
* semantic clusters
* agent skill mapping
* workflow search

---

# **9. HARD GUARANTEES (THE REAL VALUE)**

### **The system cannot break.**

Immutable layer + CAS + validator prevents destruction.

### **The system can evolve indefinitely.**

Semantic layer + AI-driven mutation.

### **The system can repair itself.**

Schemas + hooks + GC + Merkle roots.

### **The system works across multiple devices.**

CAS + Merkle sync + hive-profile.

### **The system supports MOE across multiple providers.**

You get optimal routing, minimal cost, maximum performance.

---

# **next step**

generate:

### **1. The JSON schema for all configs types**

(agent.schema, tool.schema, workflow.schema, cas-object.schema)

### **2. The CAS operation spec**

(Put, Get, Ref, Tag, GC sweep)

### **3. The mutation pipeline**

(pre-validate ? apply ? post-commit ? index update ? CAS register)

### **4. A full end-to-end lifecycle example**

(agent update ? CAS ? MOE router ? effect in orchestration)

### **5. A zipped build kit** (offline-ready)

OS is should now at the point where it could run on your Threadripper node as a real local-first AgenticOS kernel.

---

# Content-Addressable Storage (CAS) configsuration Policy

**Document ID**: POL-CAS-001  
**Version**: 1.0.0  
**Last Updated**: 2025-12-17  
**Status**: Active

---

## Overview

This document defines the configsuration policy for the NOA Content-Addressable Storage (CAS) system. CAS provides immutable, deduplicated blob storage for module artifacts using SHA-256 content hashing.

## Storage Architecture

### Directory Structure

```
${NOA_ROOT}/data/modules/cas/
??? {h0}{h1}/              # First 2 hex chars of hash
?   ??? {h2}{h3}/          # Next 2 hex chars of hash
?       ??? {full_hash}    # Complete SHA-256 hash as filename
??? README.md              # Module documentation
```

### Hash Format

- **Algorithm**: SHA-256 (256-bit, 64 hex characters)
- **Encoding**: Lowercase hexadecimal
- **Example**: `a1b2c3d4e5f6...` stored at `a1/b2/a1b2c3d4e5f6...`

## configsuration Settings

### Core Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.enabled` | boolean | `true` | Enable CAS storage |
| `cas.path` | string | `${NOA_ROOT}/data/modules/cas` | CAS root directory |
| `cas.hash_algorithm` | string | `sha256` | Hashing algorithm |
| `cas.dedup_enabled` | boolean | `true` | Enable deduplication |

### Storage Limits

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.max_blob_size` | integer | `104857600` | Max blob size (100MB) |
| `cas.max_total_size` | integer | `10737418240` | Max total storage (10GB) |
| `cas.gc_threshold` | float | `0.9` | GC trigger threshold (90%) |

### Retention Policy

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `cas.retention_days` | integer | `30` | Unreferenced blob retention |
| `cas.gc_schedule` | string | `0 2 * * 0` | GC cron schedule (weekly) |
| `cas.gc_dry_run` | boolean | `false` | GC dry-run mode |

## Reference Counting

CAS uses reference counting to track blob usage:

1. **Increment**: When a module version references a blob
2. **Decrement**: When a module version is deleted
3. **Garbage Collection**: Blobs with zero references after retention period

### Database Schema

Reference counts are stored in `module_versions` table:

```sql
CREATE TABLE module_versions (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL,
    version TEXT NOT NULL,
    cas_hash TEXT NOT NULL,
    ref_count INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    FOREIGN KEY (module_id) REFERENCES modules(id)
);
```

## Operations

### Store Blob

```rust
/// Store content in CAS, returns hash
pub async fn store(content: &[u8]) -> Result<String, CasError> {
    let hash = sha256_hex(content);
    let path = cas_path(&hash);
    if !path.exists() {
        fs::write(&path, content)?;
    }
    Ok(hash)
}
```

### Retrieve Blob

```rust
/// Retrieve content by hash
pub async fn retrieve(hash: &str) -> Result<Vec<u8>, CasError> {
    let path = cas_path(hash);
    fs::read(&path).map_err(CasError::NotFound)
}
```

### Garbage Collection

```rust
/// Remove unreferenced blobs past retention
pub async fn gc(dry_run: bool) -> Result<GcReport, CasError> {
    let unreferenced = find_unreferenced_blobs()?;
    let expired = filter_expired(unreferenced, retention_days)?;
    if !dry_run {
        for blob in &expired {
            fs::remove_file(cas_path(blob))?;
        }
    }
    Ok(GcReport { removed: expired.len() })
}
```

## Security Considerations

### Integrity Verification

- Content hash is verified on read
- Corrupted blobs trigger integrity alerts
- Automatic re-fetch from source if available

### Access Control

- CAS directory permissions: `0750`
- Blob file permissions: `0640`
- Service user: `noa` or current user

### Audit Logging

All CAS operations are logged:
- `cas.store`: Hash, size, source module
- `cas.retrieve`: Hash, requestor
- `cas.gc`: Removed count, freed space

## Integration Points

### Module Registry

The module registry (`data/modules/registry/`) uses CAS for:
- Module binary storage
- configsuration snapshots
- Artifact caching

### Execution Memory

CAS integrates with execution memory for:
- Caching large context blobs
- Storing reasoning artifacts
- Provider state snapshots

## Monitoring

### Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `cas_blobs_total` | gauge | Total blob count |
| `cas_bytes_total` | gauge | Total storage used |
| `cas_store_ops` | counter | Store operations |
| `cas_retrieve_ops` | counter | Retrieve operations |
| `cas_gc_removed` | counter | Blobs removed by GC |

### Health Checks

- `cas.available`: CAS directory writable
- `cas.integrity`: Random blob verification
- `cas.capacity`: Storage below threshold

## Related Documents

- `data/modules/cas/README.md` - CAS directory documentation
- `configs/database.yaml` - Database configsuration
- `docs/05-policy/data-retention.md` - Data retention policy

---

**Approved By**: NOA Development Team  
**Review Date**: 2026-06-17

