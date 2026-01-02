---
# CAS Layout Decision

Here is the **fully-specified, conflict-free hybrid configuration architecture**—the one your NOA / Agentic-OS / p2p Hive-Mind system actually needs.

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
* Mutable pointers ("refs", "tags")
* Multi-device sync + hive-mind memory

This is the **exact blueprint** for a next-generation agentic OS.
It’s deterministic where required and plastic everywhere else your system evolves.

I’ll present it as a **clear directory hierarchy**, then explain each layer in plain language.
This is the “final form” of your configuration model.


**Decision**: Standardize on a single Git-like CAS layout using `objects/`, `refs/`, and `tags/`.

**Rationale**:

* `objects/` provides immutable, content-addressed storage.
* `refs/` provides mutable pointers to objects ("current", "active", "latest").
* `tags/` provides stable human-readable references for releases.
* This layout supports deterministic rollbacks, reachability-based GC, and multi-device synchronization.

**Canonical paths**:

* Canonical CAS root (authoritative): `${NOA_ROOT}/cas`
* Canonical objects store: `${NOA_ROOT}/cas/objects/<h0h1>/<h2h3>/<full_hash>`
* Canonical refs: `${NOA_ROOT}/cas/refs/<name>`
* Canonical tags: `${NOA_ROOT}/cas/tags/<name>`

**Compatibility note**:

* Legacy or subsystem-specific CAS directories (example: `${NOA_ROOT}/data/modules/cas`) MUST be treated as `authority=derived|cache` and MUST reference the canonical CAS root via registry `conflicts_with`.

## NOA_ROOT vs NOA_HOME

Per `docs/05-policy/Path_Home-vs-Root_policy.md`, both variables are required:

* `NOA_ROOT` = ecosystem base install anchor (shared, persistent).
* `NOA_HOME` = active instance directory (versioned/runtime-specific; may equal `NOA_ROOT` for single-folder installs).

This document shows **shared canonical storage** (CAS, data, logs, providers) under `${NOA_ROOT}`, and **instance-scoped runtime** under `${NOA_HOME}`.

---

# THE HYBRID CONFIGURATION MODEL – FINAL BLUEPRINT

(Immutable Base + AI-Native Mutable Layer + CAS Spine + Merkle DAG Indexing)

```text
${NOA_ROOT}/
  cas/                            # Content Addressable Store (authoritative, shared)
    objects/                      # Immutable, deduped binary/JSON blobs (Merkle)
      ab/12cd34ef...              # Stored as hash prefixes
    refs/                         # Mutable pointers to CAS objects
      latest-kernel
      active-config
      agent-commander
      world-current
    tags/                         # Named stable references (e.g., releases)
      v1.0.0
      stable
      dev
    registry/                     # Central catalog of CAS objects for providers
      models.json
      prompts.json
      snapshots.json
    gc/                           # Garbage collection rules + orphan tracking
      sweep.log
      gc_rules.json
    merkle/                       # Precomputed DAG structures
      root.hash

  providers/                      # Shared provider contracts (shared-scoped)
    provider.llamacpp.json
    provider.codex.json
    provider.claude.json

  data/                           # Persistent, interpretable datasets (shared)
    indexes/
      global.idx.json             # Search index for all CAS roots
      agent.idx.json
    knowledge/                    # Structured + unstructured local knowledge
      docs/

  logs/                           # Streaming logs + analytical logs (shared)
    agents/
    providers/
    errors/
    orchestration/

${NOA_HOME}/                      # Active instance (may equal NOA_ROOT)
  immutable/                      # Nix-style: controlled, reproducible, read-only
    schema/                       # All schemas for validation
      agent.schema.json
      skill.schema.json
      tool.schema.json
      workflow.schema.json
      config.schema.json
      world.schema.json
      cas-object.schema.json
    kernels/                      # Microkernel blueprints & versions
      base.toml
      vmm.toml
      sandbox.toml
    sandbox/                      # Immutable runtime environment definitions
    trust/                        # Keys, attestations, signatures
      root.pub
      providers.pub

  mutable/                        # AI-native semantic layer (system rewrites allowed)
    config/
      world_model.json            # Machine-readable worldview & metadata
      device_profile.json         # Hardware profile, locality, capabilities
      hive_profile.json           # Identity in the swarm
      preferences.nl              # Natural-language preference records
      constraints.graph           # Semantic rules, safety limits, resource caps
    agents/                       # Agent registry (semantic description + compiled config)
      index.json                  # Top-level manifest
      commander/
        agent.json
        beliefs.graph
      builder/
      researcher/
      auditor/
    skills/                       # Operator-level reusable capacities
      planning.skill.json
      search.skill.json
      refactor.skill.json
    tools/                        # Tool definitions (semantic + execution plan)
      fs.tool.json
      exec.tool.json
      web.tool.json
    prompts/                      # Prompt libraries (CAS-backed)
      system/
      tasks/
      code/
    workflows/                    # DAGs the agents follow
      build.yaml
      debug.yaml
      optimize.yaml
    commands/                     # User-exposed command definitions
      index.json
    orchestration/                # MOE router + scheduling engine
      moe.router.json
      scheduler.json
      cost_models.json
    hooks/                        # Mutation hooks, event triggers
      pre-validate.js
      post-commit.js

  cache/                          # Never trusted; always regenerable (instance)
    models/
    embeddings/
    build_artifacts/
    temp/

  state/                          # Mutable state with invariants (instance)
    sessions/                     # Active session memory
    conversations/
    checkpoints/                  # State snapshots
    metrics/                      # Telemetry
```

---

## Modules CAS Policy

The module artifact CAS policy (sharded-flat layout under `${NOA_ROOT}/data/modules/cas`) has been split into its own document:

* `docs/05-policy/config-cas-modules.md`

---

## NOA_ROOT vs NOA_HOME (Further Clarification)

The following clarification is adapted from `docs/05-policy/Path_Home-vs-Root_policy.md`.

## Conceptual difference

### NOA_ROOT (the ecosystem base)

Think of `NOA_ROOT` as the absolute **anchor point** for the entire installation.

* **Scope:** Global
* **Purpose:** Locate shared resources that do not change regardless of which version is running
* **Contents:** Shared libraries/assets, plugin folders, and potentially multiple versions

### NOA_HOME (the active instance)

`NOA_HOME` refers to the **active directory** where the specific binaries and configuration for the current execution reside.

* **Scope:** Instance-specific
* **Purpose:** Locate the “heart” of the currently running instance
* **Contents:** Executable/binary, version-scoped config, and local `bin/` / `lib/`

## Comparison

| Feature | NOA_ROOT | NOA_HOME |
| --- | --- | --- |
| Logic | "Where is the platform installed?" | "Where is the active app running?" |
| Hierarchy | Usually the parent directory | Usually a child of the root |
| Portability | Hard-coded to drive/mount point | Relative to root |
| Example | `/opt/noa/` | `/opt/noa/v1.2.0/` |

## Why the distinction matters for install-anywhere

* **Version switching:** one `NOA_ROOT` can host multiple `NOA_HOME` instances for safe upgrades/rollbacks.
* **Hardware abstraction:** scripts should derive paths from `NOA_ROOT`/`NOA_HOME` instead of using host absolute paths.
* **Dependency resolution:** shared dependencies live under `NOA_ROOT`; instance-private config lives under `NOA_HOME`.

### Single-folder exception

If you do not plan on multiple versions or shared global assets, `NOA_HOME` and `NOA_ROOT` may be set to the same path. Defining both from the start is still recommended for future-proofing.
