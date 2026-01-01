# Agent System Design

Autonomous agents perform all work in NOA.

## Agent Types

### Built-in Agents

| Agent | Purpose | Capabilities |
|-------|---------|--------------|
| Commander-Chief | Orchestration | `plan`, `delegate`, `verify` |
| File-IO | File operations | `read`, `write`, `search` |
| Terminal | Command execution | `execute`, `background` |
| RAG | Knowledge retrieval | `embed`, `search`, `retrieve` |

### Custom Agents

Users can create custom agents implementing the Agent trait.

## Lifecycle

```
┌─────────┐     spawn      ┌─────────┐
│ Created │ ──────────────>│  Idle   │
└─────────┘                └────┬────┘
                                │ execute
                                ▼
                          ┌─────────┐
                    ┌─────│ Running │─────┐
                    │     └────┬────┘     │
               pause│          │complete  │fail
                    ▼          ▼          ▼
              ┌─────────┐ ┌─────────┐ ┌─────────┐
              │ Paused  │ │  Idle   │ │ Failed  │
              └─────────┘ └─────────┘ └─────────┘
```

## Communication

Agents communicate via:
- **Direct calls**: Request-response
- **Events**: Pub-sub messaging
- **Shared memory**: Via database

## Sandboxing

Each agent runs in isolation:
- File system access restricted
- Network access controlled
- Resource quotas enforced

## See Also

- [Architecture Overview](architecture.md)
- [Governance](../adr/004-constitutional-governance.md)
