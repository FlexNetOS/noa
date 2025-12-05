---

# NOA: Multi-Platform Autonomous Self-Modifying Agentic OS

---

## This is the root of '$user'

Admin User: <NOA>

Monorepo hosting Monorepo's: (e.g. '/project-mgmt', and more)

Cross-PLatform Adaptive: <Windows10/11, MacOS, Linux>

---

- **NOA is a P2P server designed to connect all user devices for dynamically shared compute and storage.
- **AI-first, conversational UI/OS**: Designed for AI-driven interactions
- **Minimal extensions**: Prefer CLI tools over IDE extensions
- **P2P Architecture**: Connects user devices for shared compute and storage
- **Multi-user**: Any user can login (e.g., deflex) and run the same OS
- **Users**: 'NOA' is the Admin User
- **Paths & Routing: All paths are '$Users, platform, and location agnostic, No external dependencies. **
- **Inspsiration**: GOD's Creation - the universe design. BIOLOGICAL-INSPIRED "STEM CELL"AUTONOMOUS COMPUTING INTEGRATED WITH CAPSULE NETWORKS & CELL-BASED ARCHITECTURE

---

## Overview

- Agentic, full-stack, operating system designed to function as a hive-mind.
- Autonomously plans, acts, learns, and adapts to manage and self-upgrade your entire software and hardware environment.
- Fundamentally replaces the fragility of traditional applications and cloud-based services with a unified neural runtime and a dynamic UI.
- Digests all forms of code and data, composes tools on demand, and continuously optimizes itself across your entire infrastructure-from servers and networks to PCs and mobile devices.
- Autonomous: Progressive Adaptation Evolution System
- Dynamic Harware and OS Host Adaptation
- Peer-to-Peer, NOA is a P2P server designed to connect all user devices for dynamically shared compute and storage.
- Decentralized, Distributed, Flexible, Resilient, Scalable, Intelligent, Interoperability, and Data Secure

---

## Core Principles

• Self-contained & Autonomous: Operates with complete independence with no dependentcies or environment connection to the host or outside the '$user' directory.
• User dynamic compute, storage, & cloud: Users use P2P connections for shared compute and storage within user hardware (e.g. pc, laptop, mobile, xr glasses, and so on) to create a user owned type cloud. Your data, models, and operations stay on your hardware, under your control.
• Online, third-party to Local-First, offline: Mainly Online but needs to function offline and switch third-party apps off with Featured Flag, A/B switching when desired or needed.
• Agentic Orchestration: Moves beyond single-purpose AI tools by deploying a collaborative network of specialized agents that work together to solve complex, multi-step problems.
• Adaptive & Self-Improving: A true learning system that evolves over time, enhancing its own efficiency, capabilities, and understanding of your specific needs.

---

## Key Features Runtime & Model Management

• Agnostic Model Integration: Directly download and run models from open repositories like Hugging Face. Avoids dependency on intermediary services, while retaining optional support for platforms like Ollama for maximum flexibility.

---

## User Interface & Interaction

• Dynamic, Context-Aware UI: The user interface is not a static application but a fluid, agent-driven dashboard that reconfigures itself in real-time to present the most relevant tools, data, and controls for the task at hand.
• "Digest Everything" Engine: Ingests and synthesizes any data source-codebases, databases, documents, network traffic, APIs-to build a holistic, actionable model of your entire digital ecosystem. Operational Capabilities
• On-Demand Tool Composition: Autonomously writes, combines, and deploys software tools and workflows as needed, creating novel solutions without manual intervention.
• Continuous Optimization: Constantly monitors system performance, identifies inefficiencies, and self-upgrades its own processes and components across all managed hardware (Server, Network, PC, Mobile).
• Transparent & Auditable: All agent actions, decisions, and modifications are logged, providing a clear and auditable trail for complete oversight and control.

---

## The Long-Term Vision

• North Star Goal - Serve as the definitive autonomous command center that owns your stack end-to-end. It is a system that is secure by default, auditable by design, and powerful enough to run an entire business autonomously, free from the constraints and costs of the modern SaaS landscape.
• Goal - Complete Offline Capability: Designed to run fully air-gapped with no internet connection required, ensuring maximum security and operational resilience.
• Goal - Full-Stack Ownership: Engineered to eventually give you end-to-end control of your technology stack, eliminating dependencies on external SaaS providers and their vulnerabilities.

---


## Directory Structure

Todo: [update]
**Change 'home/noa' -> '$user'**
**Fix Formatting**

- 'project-mgmt\docs\01-goals - All Goals
- 'project-mgmt\docs\02-policy - All Policy
- 'project-mgmt\docs\03-rules - All Rules
- 'project-mgmt\docs\04-plans - All Plans

- `/noa/repos/` - All repositories (GitHub, local, external)
  - `github/` - Repositories cloned from GitHub
  - `local/` - Local repositories
  - `external/` - Repositories from other sources

- `/noa/containers/` - Container hosting
  - `docker/` - Docker configurations
  - `compose/` - Docker Compose files
  - `volumes/` - Container volumes

- `/noa/workspace/` - Active development workspace
  - `projects/` - Active projects
  - `agents/` - Agent configurations
  - `tools/` - Development tools

- `/noa/p2p/` - P2P server components
  - `nodes/` - P2P node configurations
  - `storage/` - Shared storage management
  - `compute/` - Shared compute resources
  - `network/` - Network configuration

- `noa/ai/` - Multi-provider AI system
  - `shared/` - Shared resources (agents, workflows, prompts, skills, tools, models)
    - All providers use these resources for seamless switching
  - `providers/` - Provider configurations
    - `local/` - Local providers (llama.cpp, ollama) - Highest priority
    - `cloud/` - Cloud providers (OpenAI, Anthropic, etc.)
    - `hybrid/` - Hybrid configurations
  - `devices/` - Multi-device orchestration
    - `registered/` - Registered devices (PC, mobile, MR glasses, laptop)
    - `active/` - Currently active devices
    - `capabilities/` - Device capability definitions
  - `orchestration/` - Parallel task orchestration
    - `tasks/` - Active tasks
    - `queues/` - Task queues
    - `scheduling/` - Scheduling configuration

- `${NOA_ROOT}/git/` - Local Git CI/CD System
  - `repos/` - Git repositories
    - `bare/` - Bare repositories (for local server)
    - `working/` - Working repositories
  - `prs/` - Local Pull Requests
    - `open/` - Open PRs
    - `merged/` - Merged PRs
    - `closed/` - Closed PRs
  - `conflicts/` - Conflict resolution
    - `pending/` - Pending conflicts
    - `resolved/` - Manually resolved
    - `ai-resolved/` - AI/MLOps resolved
  - `ci-cd/` - Local CI/CD pipelines
    - `pipelines/` - Pipeline definitions
    - `jobs/` - Job definitions
    - `artifacts/` - Build artifacts
    - `logs/` - CI/CD logs
  - `merges/` - Merge tracking
  - `mirrors/` - Repository mirrors
    - `github/` - GitHub mirrors (push-only)
    - `local/` - Local mirrors
  - `hooks/` - Git hooks for automation

- `${NOA_ROOT}/config/` - Configuration files
- `${NOA_ROOT}/scripts/` - Utility scripts (CLI tools preferred)
- `${NOA_ROOT}/logs/` - Log files
- `${NOA_ROOT}/tmp/` - Temporary files

## Path Isolation

**CRITICAL**: All paths used within this monorepo MUST be within `${NOA_ROOT}`.

- Use relative paths from `${NOA_ROOT}` when possible
- Use environment variables: `$NOA_ROOT`, `$NOA_REPOS`, etc.
- Never reference paths outside `${NOA_ROOT}`

## Multi-User Access

- **Primary user**: Owns `${NOA_ROOT}`, NOA app user
- **Additional users**: Can access NOA with appropriate permissions
- Users should be in a shared group for collaborative access
- `NOA_ROOT` auto-detects based on repository location (drive-agnostic)

## Quick Start

\`\`\`bash
# Navigate to NOA root
cda  # or: cd $NOA_ROOT

# Navigate to repos
cdr  # or: cd $NOA_ROOT/repos

# Navigate to containers
cdc  # or: cd $NOA_ROOT/containers

# Navigate to workspace
cdw  # or: cd $NOA_ROOT/workspace

# Navigate to P2P
cdp  # or: cd $NOA_ROOT/p2p
\`\`\`

## Adding Repositories

\`\`\`bash
# Clone from GitHub
cd $NOA_ROOT/repos/github
git clone <repository-url>

# Add local repository
cd $NOA_ROOT/repos/local
# Copy or link your repository here
\`\`\`

## Container Management

\`\`\`bash
# Docker containers should be configured in:
$NOA_ROOT/containers/docker/

# Docker Compose files in:
$NOA_ROOT/containers/compose/
\`\`\`

## P2P Server

\`\`\`bash
# P2P server components are in:
$NOA_ROOT/p2p/

# Configure nodes, storage, compute, and network settings
\`\`\`

## Development Philosophy

- **CLI-first**: Prefer CLI tools over IDE extensions
- **AI-driven**: Conversational interface for all operations
- **Minimal dependencies**: Use little to no extensions when possible

## P2P Server Management

\`\`\`bash
# Use NOA CLI tool
noa start      # Start P2P server
noa stop       # Stop P2P server
noa status     # Check server status
noa nodes      # List connected nodes
noa storage    # Check storage
noa compute    # Check compute resources
\`\`\`

## Multi-Provider AI System

NOA uses a multi-provider AI system where all providers share agents, workflows, prompts, skills, and tools. This allows seamless provider switching without recreating configurations.

### Provider Priority

1. **Local providers** (llama.cpp, ollama) - Highest priority, runs on device
2. **Hybrid providers** - Local-first with cloud fallback
3. **Cloud providers** - Used when local unavailable

### Shared Resources

All providers automatically use:
- **Agents**: `$NOA_ROOT/ai/shared/agents/`
- **Workflows**: `$NOA_ROOT/ai/shared/workflows/`
- **Prompts**: `$NOA_ROOT/ai/shared/prompts/`
- **Skills**: `$NOA_ROOT/ai/shared/skills/`
- **Tools**: `$NOA_ROOT/ai/shared/tools/`
- **Models**: `$NOA_ROOT/ai/shared/models/`

### AI Commands

\`\`\`bash
noa ai providers          # List available providers
noa ai devices           # List registered devices
noa ai shared            # Show shared resources
noa ai switch <provider> # Switch AI provider
\`\`\`

## Multi-Device Orchestration

NOA orchestrates tasks across multiple devices in parallel:

- **PC**: Primary compute (GPU, CPU, storage)
- **Mobile**: Input/output (camera, sensors, photos/videos)
- **MR Glasses**: Interface (display, spatial tracking)
- **Laptop**: Secondary compute and storage

### Device Commands

\`\`\`bash
noa device register <name>  # Register a new device
noa device list             # List registered devices
noa device capabilities      # Show device capabilities
\`\`\`

### Parallel Processing

Tasks are automatically distributed:
- **Compute**: PC and laptop handle heavy processing
- **Storage**: Distributed across PC, mobile, laptop
- **Input**: Mobile and MR glasses capture data
- **Output**: Mobile and MR glasses display results

All processing happens in parallel across devices.

## Local Git CI/CD System

NOA runs all Git activities locally with AI/MLOps conflict resolution. The goal is 100% local Git CI/CD, with GitHub sync for PRs until the system is fully ready.

### Goals

- **Local Git**: All git operations run locally
- **Local PRs**: Pull requests created and managed locally
- **Local Merges**: Merge operations handled locally
- **AI Conflict Resolution**: Conflicts resolved using AI/MLOps automations
- **Local CI/CD**: Complete CI/CD pipeline runs locally
- **GitHub Sync**: Push PRs to GitHub until local system is ready

### Local Git Commands

\`\`\`bash
# Pull Request Management
git-pr create <branch>    # Create local PR
git-pr list               # List local PRs
git-pr review <pr-id>     # Review PR locally
git-pr merge <pr-id>      # Merge PR locally (with AI conflict resolution)
git-pr close <pr-id>      # Close PR
git-pr sync <pr-id>       # Sync PR to GitHub (when ready)

# Conflict Resolution
git-conflict detect       # Detect conflicts
git-conflict resolve <id> # Resolve with AI/MLOps
git-conflict list         # List pending conflicts
git-conflict review       # Review AI-resolved conflicts

# Local CI/CD
git-ci run <pipeline>    # Run local CI/CD pipeline
git-ci status            # Check pipeline status
git-ci logs              # View CI/CD logs
git-ci artifacts         # List build artifacts
\`\`\`

### Conflict Resolution

Conflicts are automatically detected and resolved using:
- **AI-assisted resolution**: Uses local AI providers (llama.cpp priority)
- **MLOps automation**: Trained models for conflict resolution
- **Auto-merge**: When confidence is high (>90%)
- **Manual review**: For complex conflicts

All resolved conflicts are stored in `$NOA_ROOT/git/conflicts/ai-resolved/` for review.

### Workflow

1. **Create PR locally**: `git-pr create feature-branch`
2. **Review locally**: Automated + AI review
3. **Resolve conflicts**: AI/MLOps automatic resolution
4. **Merge locally**: `git-pr merge pr-123`
5. **Run CI/CD**: Local pipeline execution
6. **Sync to GitHub**: Push PR when ready (until 100% local)

## Installed Tools

- **Docker & Docker Compose**: Container management
- **Node.js 22.x LTS**: For P2P server development
- **Python 3**: For AI tools and scripts
- **SSH Server**: For P2P connections
- **Git**: Version control (configured for both users)
- **CLI Tools**: tmux, screen, tree, rsync, jq, ripgrep

## Network Configuration

- SSH: Port 22 (configured for noa and deflex users)
- P2P Server: Port 8080 (configurable in $NOA_ROOT/config/noa-server.json)
- Firewall: UFW available for network security

## Glossary [todo]

## Definitions [todo]
