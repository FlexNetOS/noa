# Orchestrator Directory (coordinator-plane)

This directory contains the orchestrator component of the system, which is responsible for coordinating the various agents and managing the overall workflow. The orchestrator handles task planning, resource allocation, and communication between different parts of the system.
It serves as the central hub that ensures all components work together seamlessly to achieve the desired outcomes.

## Key Features

- **Task Planning**: Develops and manages task plans for agents.
- **Resource Management**: Allocates resources efficiently to optimize performance.
- **Communication Hub**: Facilitates communication between agents and other system components.
- **Monitoring and Logging**: Tracks system performance and logs activities for analysis.

## Directory Structure

- `planner/`: Contains modules related to task planning and strategy development.
- `router/`: Manages routing of tasks and messages between agents.
- `executor/`: Executes tasks and manages agent workflows.
- `utils/`: Utility functions and helper modules for the orchestrator.

## Getting Started

To get started with the orchestrator, follow these steps:


## Current directory tree
```plaintext
orchestrator/
├── bin/
├── config/
├── data/
├── logs/
├── src/
├── target/
├── cargo.lock
├── cargo.toml
└── README.md
```
## Target System Graph

```mermaid
─ orchestrator/                     # Brains: plan + route + run
│  ├─ router/                        # provider+tool selection (budget + locality)
│  ├─ planner/                       # decomposes requests into task packages
│  ├─ executor/                      # runs task packages via gateway/mcp
│  ├─ workflows/                     # high-level DAG workflows (build/test/train/etc.)
│  ├─ commands/                      # "command verbs" mapped to packages/workflows
│  └─ packages/                      # microservice/package format
│     ├─ schema/                     # package schema definitions
│     ├─ templates/                  # common task shapes
│     ├─ compiled/                   # resolved DAGs ready to run
│     └─ staging/                    # resolved packages before promotion to “known good”
```

