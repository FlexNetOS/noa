# Hybrid Providers

Hybrid providers that can operate in both local and cloud modes, or combine multiple provider capabilities.

## Providers in This Category

### Git (Priority 6)
- **Purpose:** Version control automation
- **Location:** `$NOA_ROOT/ai/providers/hybrid/git/`
- **Modes:** Local + Remote
- **Capabilities:** Commit generation, PR management, merge conflict resolution

### Future Hybrid Providers
- **Multi-modal routing:** Dynamic selection between local/cloud based on task complexity
- **Edge computing:** Local preprocessing + cloud refinement
- **Federation:** Distributed model inference across multiple nodes

## Configuration

See **[AGENT.md](../../../AGENT.md)** for canonical provider routing and policies.

### Priority Order
Hybrid providers have **flexible priority** (Priority 6+) based on mode:
```
local → ide → cloud → hybrid → queue + notify
```

### Mode Selection
Hybrid providers determine execution mode based on:
- Network availability
- Task complexity
- Resource constraints
- User preferences

### Constitutional Requirements
- §3.2: Local-First & Offline-Capable (local mode)
- §3.6: Security, Privacy & Full-Stack Ownership
- §3.13: Shared Provider Resource Unification

## Integration Points

All hybrid providers must:
1. Reference AGENT.md for routing logic
2. Use shared resources from `$NOA_AI_SHARED/resources/`
3. Log to `$NOA_ROOT/logs/providers/hybrid/`
4. Support both local and remote execution
5. Implement graceful degradation strategies

## Implementation

**Rust Integration:** `sys/core/src/providers/hybrid.rs`
**Priority Logic:** `sys/core/src/providers/mod.rs::default_providers()`
**Mode Selection:** `sys/core/src/providers/hybrid.rs::select_mode()`
