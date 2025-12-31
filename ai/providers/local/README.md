# Local Providers

Local-first AI providers that run entirely on the user's machine.

## Providers in This Category

### llama.cpp (Priority 1)
- **Purpose:** Primary inference engine, offline-first
- **Location:** `$NOA_ROOT/ai/providers/local/llama.cpp/`
- **Runtime:** Local CPU/GPU
- **Models:** GGUF format (Llama, Mistral, Phi, etc.)

### Ollama (Future)
- **Purpose:** Local model serving with OpenAI-compatible API
- **Status:** Planned

## Configuration

See **[AGENT.md](../../../AGENT.md)** for canonical provider routing and policies.

### Priority Order
Local providers have **highest priority** (Priority 1) in the routing stack:
```
local → ide → cloud → queue + notify
```

### Constitutional Requirements
- §3.2: Local-First & Offline-Capable
- §3.13: Shared Provider Resource Unification

## Integration Points

All local providers must:
1. Reference AGENT.md for routing logic
2. Use shared resources from `$NOA_AI_SHARED/resources/`
3. Log to `$NOA_ROOT/logs/providers/local/`
4. Store models in `$NOA_ROOT/data/models/`

## Implementation

**Rust Integration:** `sys/core/src/providers/local.rs`
**Priority Logic:** `sys/core/src/providers/mod.rs::default_providers()`
