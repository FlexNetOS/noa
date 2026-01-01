# NOA Pages

Knowledge base and guides for the NOA platform.

## Quick Links

- [Home](../index.md) — Documentation hub
- [Wiki](../wiki/index.md) — Conceptual documentation
- [Runbooks](../runbooks/index.md) — Operational procedures
- [Reference](../reference/index.md) — Technical reference

---

## Design

System architecture and design decisions.

| Document | Description |
|----------|-------------|
| [Architecture Overview](design/architecture.md) | High-level system architecture |
| [Microkernel Design](design/microkernel.md) | Core vs extension design |
| [Agent System](design/agent-system.md) | Autonomous agent architecture |
| [P2P Network](design/p2p-network.md) | libp2p networking layer |

---

## How-To Guides

Step-by-step guides for common tasks.

| Guide | Description |
|-------|-------------|
| [Bootstrap NOA](how-tos/bootstrap.md) | Initialize a new NOA instance |
| [Add Custom Agent](how-tos/add-agent.md) | Create and register custom agents |
| [Configure ML Backend](how-tos/configure-ml.md) | Set up llama.cpp, Ollama, or OpenAI |
| [Setup P2P Network](how-tos/setup-p2p.md) | Connect NOA instances via P2P |

---

## Architecture Decision Records (ADR)

Key architectural decisions and rationale.

| ADR | Decision |
|-----|----------|
| [ADR-001](adr/001-sqlite-primary-store.md) | SQLite as Primary Store |
| [ADR-002](adr/002-libp2p-networking.md) | libp2p for Networking |
| [ADR-003](adr/003-dioxus-ui.md) | Dioxus for UI |
| [ADR-004](adr/004-constitutional-governance.md) | Constitutional Governance |

---

## Integrations

External service integrations.

| Integration | Description |
|-------------|-------------|
| [Ollama](integrations/ollama.md) | Local LLM inference |
| [OpenAI API](integrations/openai.md) | Cloud inference |
| [Hugging Face](integrations/huggingface.md) | Models and datasets |

---

## Project Information

| Item | Link |
|------|------|
| Main README | [README.md](../../README.md) |
| Constitution | [CONSTITUTION.md](../../CONSTITUTION.md) |
| Security | [SECURITY.md](../../SECURITY.md) |
| License | [LICENSE](../../LICENSE) |

---

*This page is part of the NOA documentation system.*
