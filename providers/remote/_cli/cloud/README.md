# Cloud Providers

Cloud-based AI services for complex reasoning and large context windows.

## Providers in This Category

### Claude (Priority 3)
- **Purpose:** Complex reasoning, long context (200K tokens)
- **Location:** `$NOA_ROOT/ai/providers/cloud/claude/`
- **Provider:** Anthropic
- **Models:** Claude 3.5 Sonnet, Claude 3 Opus

### Codex (Priority 4)
- **Purpose:** Code generation
- **Location:** `$NOA_ROOT/ai/providers/cloud/codex/`
- **Provider:** OpenAI
- **Models:** GPT-4, GPT-3.5-turbo

### Abacus (Priority 7)
- **Purpose:** Numerical and analytical tasks
- **Location:** `$NOA_ROOT/ai/providers/cloud/abacus/`
- **Provider:** Custom analytical service

## configsuration

See **[AGENT.md](../../../AGENT.md)** for canonical provider routing and policies.

### Priority Order
Cloud providers are **third tier** (Priority 3-7) in the routing stack:
```
local → ide → cloud → queue + notify
```

### Fallback Strategy
After 3 failed retries:
- Task is queued for later
- User notification sent
- Fallback to local provider attempted

### Constitutional Requirements
- §3.2: Graceful degradation when offline
- §3.6: Zero Secret Exposure
- §3.13: Shared Provider Resource Unification

## Integration Points

All cloud providers must:
1. Reference AGENT.md for routing logic
2. Use shared resources from `$NOA_AI_SHARED/resources/`
3. Log to `$NOA_ROOT/logs/providers/cloud/`
4. Store API keys in secure vault (never in source)
5. Support retry logic with exponential backoff

## Implementation

**Rust Integration:** `sys/core/src/providers/cloud.rs`
**Priority Logic:** `sys/core/src/providers/mod.rs::default_providers()`
**Secret Management:** `sys/core/src/vault.rs`
