# NOA: Multi-Platform Autonomous Self-Modifying Agentic OS

NOA is a agentic full-stack operating system designed to function as a hive-mind. It autonomously plans, acts, learns, and adapts to manage and self-upgrade your entire software and hardware environment.
It fundamentally replaces the fragility of traditional applications and cloud-based services with a unified neural runtime and a dynamic UI. This system digests all forms of code and data, composes tools on demand, and continuously optimizes itself across your entire infrastructure-from servers and networks to PCs and mobile devices.

Autonomous Progressive Dynamic Adaptation Evolution System (Peer-to-Peer) (Decentralized, Distributed, Flexible, Resilient, Scalable, Intelligent, Interoperability, Data Secure)

INSPIRATION: GOD's Creation - the universe design. BIOLOGICAL-INSPIRED "STEM CELL"AUTONOMOUS COMPUTING INTEGRATED WITH CAPSULE NETWORKS & CELL-BASED ARCHITECTURE

## Quick Start (Windows)

Get NOA set up on Windows in minutes with our automated setup script.

### Prerequisites

- **PowerShell 7.4+** ([Download](https://aka.ms/powershell))
- **Git** (installed automatically with `-InstallPrereqs`, or [download manually](https://git-scm.com/download/win))
- **winget** (App Installer from Microsoft Store - for automated prerequisite installation)

### One-Command Setup

Open PowerShell as Administrator and run:

```powershell
# Clone the repository (if not already done)
git clone https://github.com/FlexNetOS/noa.git
cd noa

# Run the setup script
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
.\scripts\setup\setup-noa.ps1 -InstallPrereqs -IntegrateProfile
```

This will:
- ✅ Install prerequisites (Git, etc.) if missing
- ✅ Create the complete NOA directory structure at `N:\noa`
- ✅ Generate environment configuration and profile
- ✅ Integrate NOA into your PowerShell profile for automatic loading

### Custom Installation Location

To install NOA at a different location:

```powershell
.\scripts\setup\setup-noa.ps1 -NoaRoot "C:\MyNOA" -IntegrateProfile
```

### CI/Testing Setup

For automated testing or CI environments:

```powershell
.\scripts\setup\setup-noa.ps1 -NoaRoot "$env:TEMP\noa" -InstallPrereqs:$false -IntegrateProfile:$false
```

### Manual Profile Loading

If you chose not to integrate with your PowerShell profile, load NOA manually:

```powershell
. N:\noa\noa-profile.ps1
```

### Available Commands After Setup

Once the NOA profile is loaded, use these navigation shortcuts:

- `cda` - Navigate to NOA root
- `cdr` - Navigate to repos directory
- `cdc` - Navigate to containers directory
- `cdw` - Navigate to workspace directory
- `cds` - Navigate to scripts directory
- `cdl` - Navigate to logs directory

### Troubleshooting

**Execution Policy Error:**
```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
```

**Non-Admin Installation:**
Run without `-InstallPrereqs` and install Git manually, or run as Administrator.

**Custom Drive/Path Issues:**
Ensure the path exists and you have write permissions. Use absolute paths (e.g., `C:\noa` or `\\server\share\noa`).

**For detailed documentation, see [docs/setup/windows.md](docs/setup/windows.md)**

---

## Core Principles

• Self-contained & Autonomous: Operates with complete independence with no dependentcies or environment connection the host or outside the '$user' directory.
• User dynamic compute, storage, & cloud: Users use P2P connections for shared compute and storage within user hardware (e.g. pc, laptop, mobile, xr glasses, and so on) to create a user owned type cloud. Your data, models, and operations stay on your hardware, under your control.
• Online, third-party to Local-First, offline: Mainly Online but needs to function offline and switch third-party apps off with Featured Flag, A/B switching when desired or needed.
• Agentic Orchestration: Moves beyond single-purpose AI tools by deploying a collaborative network of specialized agents that work together to solve complex, multi-step problems.
• Adaptive & Self-Improving: A true learning system that evolves over time, enhancing its own efficiency, capabilities, and understanding of your specific needs.

Key Features Runtime & Model Management

• Agnostic Model Integration: Directly download and run models from open repositories like Hugging Face. Avoids dependency on intermediary services, while retaining optional support for platforms like Ollama for maximum flexibility.

User Interface & Interaction

• Dynamic, Context-Aware UI: The user interface is not a static application but a fluid, agent-driven dashboard that reconfigures itself in real-time to present the most relevant tools, data, and controls for the task at hand.
• "Digest Everything" Engine: Ingests and synthesizes any data source-codebases, databases, documents, network traffic, APIs-to build a holistic, actionable model of your entire digital ecosystem. Operational Capabilities
• On-Demand Tool Composition: Autonomously writes, combines, and deploys software tools and workflows as needed, creating novel solutions without manual intervention.
• Continuous Optimization: Constantly monitors system performance, identifies inefficiencies, and self-upgrades its own processes and components across all managed hardware (Server, Network, PC, Mobile).
• Transparent & Auditable: All agent actions, decisions, and modifications are logged, providing a clear and auditable trail for complete oversight and control.

The Long-Term Vision

• North Star Goal - Serve as the definitive autonomous command center that owns your stack end-to-end. It is a system that is secure by default, auditable by design, and powerful enough to run an entire business autonomously, free from the constraints and costs of the modern SaaS landscape.
• Goal - Complete Offline Capability: Designed to run fully air-gapped with no internet connection required, ensuring maximum security and operational resilience.
• Goal - Full-Stack Ownership: Engineered to eventually give you end-to-end control of your technology stack, eliminating dependencies on external SaaS providers and their vulnerabilities.

---

