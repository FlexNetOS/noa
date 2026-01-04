# IDE Providers

IDE-integrated AI assistants for in-editor context and orchestration.

## Providers in This Category

### Cursor (Priority 2)
- **Purpose:** IDE context + orchestration
- **Location:** `$NOA_ROOT/ai/providers/ide/cursor/`
- **Integration:** VS Code / Cursor IDE
- **Context:** Full workspace awareness

### GitHub Copilot (Priority 5)
- **Purpose:** Inline completions
- **Location:** `$NOA_ROOT/ai/providers/ide/copilot/`
- **Integration:** Multi-IDE support
- **Context:** File-level awareness

## configsuration

See **[AGENT.md](../../../AGENT.md)** for canonical provider routing and policies.

### Priority Order
IDE providers are **second tier** (Priority 2-5) in the routing stack:
```
local → ide → cloud → queue + notify
```

### Constitutional Requirements
- §3.2: Graceful degradation when offline
- §3.13: Shared Provider Resource Unification

## Integration Points

All IDE providers must:
1. Reference AGENT.md for routing logic
2. Use shared resources from `$NOA_AI_SHARED/resources/`
3. Log to `$NOA_ROOT/logs/providers/ide/`
4. Support offline fallback to local providers

## Implementation

**Rust Integration:** `sys/core/src/providers/ide.rs`
**Priority Logic:** `sys/core/src/providers/mod.rs::default_providers()`
