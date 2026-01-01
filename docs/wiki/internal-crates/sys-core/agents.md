# agents Module

Agent orchestration and lifecycle management.

**Location**: `sys/core/src/agents/`  
**Feature**: `full`

## Overview

The agent system manages autonomous AI agents that perform tasks:

- Agent registration and discovery
- Lifecycle management (spawn, pause, resume, stop)
- Task routing and execution
- Inter-agent communication

## Key Types

### Agent

Core agent entity.

```rust
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub kind: AgentKind,
    pub status: AgentStatus,
    pub config: AgentConfig,
    pub capabilities: Vec<Capability>,
}

pub enum AgentKind {
    CommanderChief,
    FileIO,
    Terminal,
    Rag,
    Custom(String),
}

pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Failed(String),
    Terminated,
}
```

### AgentCommand

Agent control commands.

```rust
pub enum AgentCommand {
    Spawn { kind: AgentKind, config: AgentConfig },
    Execute { task_id: TaskId, input: Value },
    Pause { reason: String },
    Resume,
    Stop,
    Status,
}
```

### AgentRegistry

Central agent management.

```rust
pub struct AgentRegistry {
    agents: HashMap<AgentId, AgentHandle>,
}

impl AgentRegistry {
    pub async fn spawn(&mut self, kind: AgentKind) -> NoaResult<AgentId>;
    pub async fn execute(&self, id: AgentId, task: Task) -> NoaResult<TaskResult>;
    pub async fn stop(&mut self, id: AgentId) -> NoaResult<()>;
    pub fn list(&self) -> Vec<&Agent>;
}
```

## Built-in Agents

| Agent | Purpose | Capabilities |
|-------|---------|--------------|
| `CommanderChief` | Task decomposition, orchestration | `plan`, `delegate`, `verify` |
| `FileIO` | File system operations | `read`, `write`, `search` |
| `Terminal` | Command execution | `execute`, `background` |
| `Rag` | Knowledge retrieval | `embed`, `search`, `retrieve` |

## Agent Lifecycle

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
              └────┬────┘ └─────────┘ └────┬────┘
                   │ resume                │ stop
                   └───────────┬───────────┘
                               ▼
                        ┌─────────────┐
                        │ Terminated  │
                        └─────────────┘
```

## Usage

```rust
use noa_core::agents::{AgentRegistry, AgentKind, Task};

async fn example(registry: &mut AgentRegistry) -> NoaResult<()> {
    // Spawn a file-io agent
    let agent_id = registry.spawn(AgentKind::FileIO).await?;
    
    // Execute a task
    let task = Task::new("read", json!({"path": "README.md"}));
    let result = registry.execute(agent_id, task).await?;
    
    // Stop agent
    registry.stop(agent_id).await?;
    
    Ok(())
}
```

## See Also

- [autonomy module](autonomy.md) — Self-governance policies
- [automation module](automation.md) — Scheduled tasks
- [db module](db.md) — Agent persistence
