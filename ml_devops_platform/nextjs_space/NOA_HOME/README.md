# NOA (Next-generation Organic Architecture) - AgenticOS

This is the **Hybrid Configuration Model** for the ML DevOps Platform, implementing a content-addressable, cryptographically-verifiable agent operating system.

## Architecture Overview

```
NOA_HOME/
├── immutable/          # The DNA - Cannot be modified without validation
│   ├── schema/         # JSON schemas for all object types
│   ├── kernels/        # Microkernel blueprints
│   ├── providers/      # AI provider contracts
│   ├── sandbox/        # Execution environment definitions
│   └── trust/          # Cryptographic trust anchors
│
├── mutable/            # The Epigenetics - AI-rewritable semantic layer
│   ├── config/         # World model, device profile, hive profile
│   ├── agents/         # Agent definitions with beliefs
│   ├── skills/         # Reusable agent capabilities
│   ├── tools/          # Tool definitions with permissions
│   ├── prompts/        # Prompt libraries (CAS-backed)
│   ├── workflows/      # DAG execution plans
│   ├── commands/       # User-exposed commands
│   ├── orchestration/  # MOE router + scheduler
│   └── hooks/          # Mutation event triggers
│
├── cas/                # The Spine - Content Addressable Storage
│   ├── objects/        # Immutable, hash-addressed blobs
│   ├── refs/           # Mutable pointers (e.g., 'latest-kernel')
│   ├── tags/           # Named releases (e.g., 'v1.0.0')
│   ├── registry/       # Central catalog of objects
│   ├── gc/             # Garbage collection rules
│   └── merkle/         # Precomputed DAG structures
│
├── cache/              # Never trusted - Always regenerable
├── logs/               # Streaming + analytical logs
├── state/              # Mutable state with invariants
└── data/               # Persistent, interpretable datasets
    ├── indexes/        # Global search indexes
    └── knowledge/      # Structured + unstructured knowledge
```

## Key Concepts

### 1. Immutable Layer (The DNA)
- **Purpose**: Prevents corruption, config drift, catastrophic errors
- **Contents**: Schemas, kernels, provider contracts, sandbox constraints
- **Mutation**: Only through validator → compiler pipeline
- **Cryptography**: SHA-256 content hashing, Merkle DAG verification

### 2. Mutable Semantic Layer (The Epigenetics)
- **Purpose**: AI agents can think, reason, learn, and evolve
- **Contents**: World model, agent beliefs, skills, workflows, hooks
- **Properties**: Machine-interpretable, machine-rewritable, logged, reversible
- **Validation**: All mutations validated against immutable schemas

### 3. Content Addressable Storage (The Spine)
- **Hash Algorithm**: SHA-256 (64 hex characters)
- **Structure**: Merkle DAG for perfect reproducibility
- **Deduplication**: Automatic - same content = same hash
- **References**: Mutable pointers (refs) + immutable tags
- **Garbage Collection**: Reference counting with retention period

## Operations

### Storing an Object

```typescript
import { getMutationPipeline } from '@/lib/noa';

const pipeline = getMutationPipeline();

const agent = {
  id: 'planner',
  name: 'Strategic Planner',
  role: 'planner',
  version: '1.0.0',
  capabilities: ['planning', 'strategy'],
  skills: [],
  tools: []
};

const result = await pipeline.apply('agent', agent, 'active-planner');
console.log('Agent stored with hash:', result.hash);
```

### Retrieving an Object

```typescript
import { getCASStorage } from '@/lib/cas';

const storage = getCASStorage();
const hash = await storage.getRef('active-planner');
const agent = await storage.get(hash!);
console.log('Retrieved agent:', agent);
```

### Searching Objects

```typescript
import { GlobalIndex } from '@/lib/noa';

const index = new GlobalIndex();
const results = await index.search({
  type: 'agent',
  name: 'planner'
});
console.log('Found agents:', results);
```

### Garbage Collection

```typescript
import { getCASStorage, createGC } from '@/lib/cas';

const storage = getCASStorage();
const gc = createGC(storage);
const report = await gc.run(false); // Set true for dry-run
console.log('GC Report:', report);
```

## MOE Routing

The Mixture of Experts router intelligently selects providers based on:
- Agent role and capabilities
- Provider cost models
- Latency curves
- Context window limits
- Tool access requirements

Configuration: `mutable/orchestration/moe.router.json`

## Validation Pipeline

1. **Pre-validate**: Object validated against JSON schema
2. **Hook Execution**: Optional `pre-validate.js` hook runs
3. **CAS Storage**: Content hashed and stored immutably
4. **Reference Update**: Mutable ref pointer updated (optional)
5. **Index Update**: Global index updated for search
6. **Post-commit Hook**: Optional `post-commit.js` hook runs
7. **Mutable Copy**: Convenience copy written to mutable directory

## Hard Guarantees

✓ **System cannot break** - Immutable + CAS + validator prevents destruction
✓ **System can evolve indefinitely** - Semantic layer + AI-driven mutation
✓ **System can repair itself** - Schemas + hooks + GC + Merkle roots
✓ **System works across devices** - CAS + Merkle sync + hive-profile
✓ **Optimal routing** - MOE across multiple providers

## Environment Variables

```bash
NOA_HOME=./NOA_HOME          # Root directory (default: ./NOA_HOME)
CAS_ENABLED=true             # Enable CAS storage
CAS_MAX_BLOB_SIZE=104857600  # 100MB limit
CAS_RETENTION_DAYS=30        # GC retention period
```

## File Naming Conventions

- **Schemas**: `*.schema.json` (immutable)
- **Agents**: `<agent-id>.json` (mutable copy with casHash)
- **Skills**: `<skill-id>.json` (mutable copy with casHash)
- **Tools**: `<tool-id>.json` (mutable copy with casHash)
- **Workflows**: `<workflow-id>.json` or `*.yaml` (mutable)
- **CAS Objects**: `{h0}{h1}/{h2}{h3}/{full-hash}` (content-addressed)

## Integration with Existing Code

The NOA system integrates seamlessly with existing SONA orchestration:

1. **Agents**: Defined in `mutable/agents/`, backed by CAS
2. **Skills**: Reusable capabilities in `mutable/skills/`
3. **Tools**: Execution primitives in `mutable/tools/`
4. **Workflows**: SONA workflows in `mutable/workflows/`
5. **Providers**: AI provider contracts in `immutable/providers/`

## Next Steps

1. **Migrate Existing Data**: Move `.app/data/` content to NOA structure
2. **Enable CAS**: Update code to use `getMutationPipeline()` for all mutations
3. **Implement Hooks**: Add `pre-validate.js` and `post-commit.js` hooks
4. **Configure MOE**: Tune `moe.router.json` for your provider mix
5. **Test GC**: Run garbage collection to verify reference counting

## Documentation

- JSON Schemas: `immutable/schema/*.schema.json`
- Provider Specs: `immutable/providers/*.json`
- Kernel Configs: `immutable/kernels/*.toml`
- MOE Router: `mutable/orchestration/moe.router.json`

## Rust Migration Path

All TypeScript code includes Rust translation notes:

```typescript
// RUST: Use serde_json for serialization
// RUST: Implement trait Validator for compile-time checks
// RUST: Use tokio::fs for async file operations
```

The CAS layer maps directly to Rust:
- `lib/cas/*.ts` → `src/cas/*.rs` with `std::collections::HashMap`
- Validation → `serde` + custom derive macros
- Hashing → `sha2` crate
- Merkle trees → `merkle` crate

---

**Version**: 1.0.0  
**Status**: Active  
**Last Updated**: 2025-12-18
