# Architecture Overview

NOA's system architecture follows a microkernel design with pluggable components.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       User Interface                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │ Desktop App  │  │   Web App    │  │        CLI           │   │
│  │  (Tauri)     │  │   (WASM)     │  │                      │   │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘   │
│         └─────────────────┼─────────────────────┘               │
│                           │                                      │
├───────────────────────────┼──────────────────────────────────────┤
│                    API Gateway                                   │
│                   (Axum + REST)                                  │
├───────────────────────────┼──────────────────────────────────────┤
│                           │                                      │
│  ┌────────────────────────┼────────────────────────────────┐    │
│  │                   Agent System                           │    │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐           │    │
│  │  │ Commander │  │  File-IO  │  │ Terminal  │  ...      │    │
│  │  │  Chief    │  │   Agent   │  │   Agent   │           │    │
│  │  └───────────┘  └───────────┘  └───────────┘           │    │
│  └──────────────────────────────────────────────────────────┘    │
│                           │                                      │
├───────────────────────────┼──────────────────────────────────────┤
│                           │                                      │
│  ┌─────────────┐  ┌───────┴───────┐  ┌─────────────────────┐   │
│  │   Neural    │  │  Governance   │  │    Observability    │   │
│  │   Engine    │  │   (Policy)    │  │   (Metrics/Logs)    │   │
│  └─────────────┘  └───────────────┘  └─────────────────────┘   │
│                           │                                      │
├───────────────────────────┼──────────────────────────────────────┤
│                    Core Services                                 │
│  ┌─────────────┐  ┌───────────────┐  ┌─────────────────────┐   │
│  │  Database   │  │    Config     │  │      Events         │   │
│  │  (SQLite)   │  │   Manager     │  │       Bus           │   │
│  └─────────────┘  └───────────────┘  └─────────────────────┘   │
│                                                                  │
├──────────────────────────────────────────────────────────────────┤
│                     P2P Network Layer                            │
│  ┌─────────────┐  ┌───────────────┐  ┌─────────────────────┐   │
│  │  libp2p     │  │   Gossipsub   │  │     Kademlia        │   │
│  │  Transport  │  │   (Pub/Sub)   │  │      (DHT)          │   │
│  └─────────────┘  └───────────────┘  └─────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

## Design Principles

### 1. Microkernel Architecture

The core is minimal and trusted:
- Essential services in `sys/core`
- Extensions via agents and modules
- Clear isolation boundaries

### 2. Local-First

All data and compute stays local:
- SQLite for persistence
- Local LLM inference
- Optional P2P for distribution

### 3. Constitutional Governance

All actions are policy-controlled:
- CONSTITUTION.md defines rules
- Governor enforces policies
- Audit trail for all actions

### 4. Agent-Based Design

Work is performed by autonomous agents:
- Each agent has specific capabilities
- Commander-Chief coordinates
- Sandboxed execution

## Data Flow

1. **Request** → API Gateway → Agent Router
2. **Agent Selection** → Commander-Chief decides
3. **Execution** → Agent performs task
4. **Governance** → Governor validates actions
5. **Response** → Result returned to user

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| sys-core | `sys/core/` | Microkernel |
| ui-app | `ui/app/` | User interface |
| p2p | `p2p/` | Networking |
| config | `config/` | Configuration |

## See Also

- [Microkernel Design](microkernel.md)
- [Agent System](agent-system.md)
- [P2P Network](p2p-network.md)
