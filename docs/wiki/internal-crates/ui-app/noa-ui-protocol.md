# noa-ui-protocol Crate

Backend communication protocol.

**Location**: `ui/app/crates/noa-ui-protocol/`

## Overview

IPC protocol for Tauri commands:

- Command definitions
- Event types
- Message formats

## Commands

Tauri command definitions.

```rust
#[tauri::command]
pub async fn list_agents(state: State<'_, AppState>) -> Result<Vec<Agent>, Error>;

#[tauri::command]
pub async fn spawn_agent(kind: AgentKind, state: State<'_, AppState>) -> Result<AgentId, Error>;

#[tauri::command]
pub async fn execute_task(agent_id: AgentId, input: Value, state: State<'_, AppState>) -> Result<TaskId, Error>;

#[tauri::command]
pub async fn get_task_status(task_id: TaskId, state: State<'_, AppState>) -> Result<TaskStatus, Error>;
```

## Events

Backend event types.

```rust
#[derive(Clone, Serialize)]
pub enum BackendEvent {
    AgentSpawned(AgentId),
    AgentStopped(AgentId),
    TaskStarted(TaskId),
    TaskCompleted { task_id: TaskId, result: Value },
    TaskFailed { task_id: TaskId, error: String },
}
```

## Messages

IPC message formats.

```rust
#[derive(Serialize, Deserialize)]
pub struct IpcRequest<T> {
    pub id: RequestId,
    pub command: String,
    pub payload: T,
}

#[derive(Serialize, Deserialize)]
pub struct IpcResponse<T> {
    pub id: RequestId,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}
```

## See Also

- [noa-ui-core](noa-ui-core.md) — Core types
- [../sys-core/api](../sys-core/api.md) — Backend API
