# NOA Directory Structure

> Comprehensive file tree of the NOA platform codebase.
> Last updated: 2026-01-01

## Root Structure

```
noa/
├── 📄 AGENTS.md                 # Agent instructions for AI assistants
├── 📄 CONSTITUTION.md           # Core principles and guidelines
├── 📄 LICENSE                   # Project license
├── 📄 Makefile                  # Build automation
├── 📄 README.md                 # Project overview
├── 📄 SECURITY.md               # Security policies
├── 📄 tsconfig.json             # TypeScript configuration
├── 📄 noa-env.ps1               # Environment setup script
│
├── 📁 ai/                       # AI and ML components
├── 📁 apps/                     # Application packages
├── 📁 bin/                      # Executable wrappers
├── 📁 cache/                    # Cache storage
├── 📁 cmd/                      # Command-line tools
├── 📁 config/                   # Configuration files
├── 📁 containers/               # Container definitions
├── 📁 data/                     # Runtime data
├── 📁 docs/                     # Documentation
├── 📁 etc/                      # System configuration
├── 📁 gateway/                  # API gateway
├── 📁 init/                     # Initialization scripts
├── 📁 lib/                      # Shared libraries
├── 📁 logs/                     # Log files
├── 📁 ml_devops_platform/       # ML/DevOps platform
├── 📁 opt/                      # Optional tools
├── 📁 orchestrator/             # Core orchestrator
├── 📁 p2p/                      # P2P networking
├── 📁 pkg/                      # Package definitions
├── 📁 ruler/                    # Task management
├── 📁 sandbox/                  # Isolated environments
├── 📁 scripts/                  # Utility scripts
├── 📁 specs/                    # API specifications
├── 📁 sys/                      # System crates
├── 📁 tests/                    # Test suites
├── 📁 tmp/                      # Temporary files
└── 📁 ui/                       # User interfaces
```

## AI Directory (`ai/`)

```
ai/
├── agents/                      # AI agent definitions
│   ├── orchestrator/            # Agent orchestration
│   └── personas/                # Agent personality configs
├── providers/                   # AI service integrations
│   ├── ollama/                  # Local Ollama integration
│   ├── openai/                  # OpenAI API client
│   ├── anthropic/               # Anthropic Claude client
│   └── azure/                   # Azure OpenAI client
└── shared/                      # Shared AI resources
    ├── models/                  # Model configurations
    ├── prompts/                 # Prompt templates
    └── resources/               # Shared resources
```

## Command Tools (`cmd/`)

```
cmd/
├── apps/                        # Application launchers
├── tasks-cli/                   # Task CLI tool
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   └── config/
│   └── Cargo.toml
├── tasks-extension/             # VS Code extension
│   ├── src/
│   │   ├── extension.ts
│   │   └── providers/
│   └── package.json
└── tasks-mcp/                   # MCP server
    ├── src/
    │   ├── main.rs
    │   └── handlers/
    └── Cargo.toml
```

## Configuration (`config/`)

```
config/
├── 📄 ai-providers.json         # AI provider settings
├── 📄 bootstrap-state.json      # Bootstrap state
├── 📄 bootstrap-tools.json      # Bootstrap tool list
├── 📄 database.yaml             # Database configuration
├── 📄 desktop-apps.json         # Desktop app registry
├── 📄 device-orchestration.json # Device management
├── 📄 features.json             # Feature flags
├── 📄 minio.yaml                # MinIO object storage
├── 📄 noa-server.json           # Server configuration
├── 📄 observability.yaml        # Metrics/logging config
├── 📄 qdrant.yaml               # Qdrant vector DB
├── 📄 quickwit.yaml             # Quickwit search
├── 📄 shared-resources.json     # Shared resource paths
├── 📄 tools.json                # Tool definitions
│
├── providers/                   # Provider-specific configs
├── schemas/                     # JSON schemas
└── templates/                   # Configuration templates
```

## Documentation (`docs/`)

```
docs/
├── 📄 index.md                  # Documentation home
├── 📄 _config.yml               # Jekyll configuration
│
├── adr/                         # Architecture Decision Records
│   ├── 0001-*.md
│   └── template.md
├── agents/                      # Agent documentation
├── api/                         # API reference
├── architecture/                # System architecture
│   ├── diagrams/
│   └── graphs/                  # Generated Mermaid graphs
├── assets/                      # Static assets
├── guides/                      # User guides
├── ml-devops/                   # ML/DevOps docs
├── operations/                  # Operations docs
├── pages/                       # Static reference pages
│   ├── directory-tree.md        # This file
│   ├── design/
│   └── how-tos/
├── reference/                   # Technical reference
├── runbooks/                    # Operational runbooks
│   ├── system-startup.md
│   ├── system-shutdown.md
│   ├── database-backup.md
│   └── agent-failure.md
├── schema/                      # Schema documentation
├── setup/                       # Setup guides
├── status/                      # System status
└── wiki/                        # Internal wiki
    ├── codebase_map.md          # Complete file listing
    ├── crates/                  # Crate documentation
    └── internal-crates/         # Internal crate docs
```

## System Crates (`sys/`)

```
sys/
├── core/                        # Core system library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── cli/
│   │   ├── config/
│   │   ├── db/
│   │   ├── error/
│   │   ├── init/
│   │   └── logging/
│   └── Cargo.toml
├── kernel/                      # Kernel abstractions
├── runtime/                     # Runtime environment
├── scheduler/                   # Task scheduler
└── storage/                     # Storage abstractions
```

## Scripts (`scripts/`)

```
scripts/
├── sweep/                       # Codebase sweep system
│   ├── sweep.ps1                # Main entry point
│   ├── sweep-orchestrator.ps1   # Parallel orchestrator
│   ├── symbol-extractor.ps1     # Symbol extraction
│   ├── doc-xref-checker.ps1     # Doc cross-reference
│   ├── ollama-embeddings.ps1    # Embedding generation
│   ├── graph-generator.ps1      # Mermaid graphs
│   ├── e2e-test-runner.ps1      # E2E test runner
│   └── README.md
├── build/                       # Build scripts
├── deploy/                      # Deployment scripts
└── utils/                       # Utility scripts
```

## Libraries (`lib/`)

```
lib/
├── flexnetos/                   # FlexNetOS dependencies
│   ├── rusqlite/                # SQLite bindings
│   ├── sqlx/                    # Async SQL toolkit
│   ├── rust-postgres/           # PostgreSQL driver
│   └── vector-db/               # RuVector implementation
├── shared/                      # Shared utilities
└── vendor/                      # Vendored dependencies
```

## Gateway (`gateway/`)

```
gateway/
└── mcp/                         # Model Context Protocol
    ├── servers/                 # MCP server implementations
    ├── tools/                   # Tool definitions
    └── schemas/                 # Protocol schemas
```

## File Count by Type

| Extension | Count | Description |
|-----------|-------|-------------|
| `.rs` | ~2,500 | Rust source files |
| `.ts` | ~1,200 | TypeScript files |
| `.py` | ~800 | Python files |
| `.md` | ~600 | Markdown documentation |
| `.json` | ~400 | JSON configuration |
| `.yaml` | ~150 | YAML configuration |
| `.toml` | ~120 | TOML (Cargo) files |
| `.ps1` | ~80 | PowerShell scripts |
| `.sh` | ~60 | Shell scripts |

## Navigation

- [Home](../index.md)
- [Wiki](../wiki/index.md)
- [Architecture](../architecture/)
- [Runbooks](../runbooks/index.md)
