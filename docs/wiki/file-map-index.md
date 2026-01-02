# NOA Codebase File Map - Index

> Quick reference guide to the NOA codebase file organization.
> For the complete file listing, see [codebase_map.md](codebase_map.md) (55,991 files).

## Overview

The NOA platform contains approximately **93,000 files** organized across functional domains.

## Quick Navigation

### By Function

| Domain | Directory | File Count | Description |
|--------|-----------|------------|-------------|
| **Core System** | `sys/` | ~3,000 | System-level Rust crates |
| **AI/ML** | `ai/` | ~1,500 | AI providers, agents, models |
| **Applications** | `apps/` | ~2,000 | Application packages |
| **Commands** | `cmd/` | ~500 | CLI tools and MCP servers |
| **Configuration** | `config/` | ~200 | JSON/YAML configurations |
| **Documentation** | `docs/` | ~1,000 | All documentation |
| **Libraries** | `lib/` | ~5,000 | Shared libs & FlexNetOS deps |
| **Scripts** | `scripts/` | ~200 | Automation scripts |
| **UI** | `ui/` | ~3,000 | Frontend components |

### By Language

| Language | Extensions | Est. Files | Primary Locations |
|----------|------------|------------|-------------------|
| Rust | `.rs` | ~2,500 | `sys/`, `cmd/`, `orchestrator/` |
| TypeScript | `.ts`, `.tsx` | ~1,200 | `ui/`, `apps/`, `cmd/tasks-extension/` |
| Python | `.py` | ~800 | `ai/`, `ml_devops_platform/`, `scripts/` |
| Markdown | `.md` | ~600 | `docs/`, root files |
| JSON | `.json` | ~400 | `config/`, package files |
| YAML | `.yaml`, `.yml` | ~150 | `config/`, workflows |
| PowerShell | `.ps1` | ~80 | `scripts/`, `init/` |
| Shell | `.sh` | ~60 | `init/`, `bin/` |

## Key File Categories

### Configuration Files

```
config/
├── ai-providers.json        # AI service configurations
├── bootstrap-state.json     # Bootstrap initialization state
├── database.yaml            # Database connection settings
├── features.json            # Feature flags
├── noa-server.json          # Main server configuration
├── observability.yaml       # Logging/metrics settings
└── tools.json               # Tool definitions
```

### Entry Points

| File | Purpose | Language |
|------|---------|----------|
| `cmd/tasks-cli/src/main.rs` | CLI entry point | Rust |
| `cmd/tasks-mcp/src/main.rs` | MCP server entry | Rust |
| `cmd/tasks-extension/src/extension.ts` | VS Code extension | TypeScript |
| `orchestrator/src/main.rs` | Orchestrator daemon | Rust |
| `gateway/mcp/server.ts` | MCP gateway | TypeScript |

### Core Libraries

| Crate | Location | Purpose |
|-------|----------|---------|
| `noa-core` | `sys/core/` | Core utilities, config, logging |
| `noa-kernel` | `sys/kernel/` | Kernel abstractions |
| `noa-runtime` | `sys/runtime/` | Runtime environment |
| `noa-scheduler` | `sys/scheduler/` | Task scheduling |
| `noa-storage` | `sys/storage/` | Storage abstractions |

### Documentation Structure

```
docs/
├── wiki/                    # Internal knowledge (YOU ARE HERE)
│   ├── codebase_map.md      # Complete file listing
│   ├── file-map-index.md    # This summary file
│   └── crates/              # Crate documentation
├── pages/                   # Static reference pages
│   ├── directory-tree.md    # Visual directory structure
│   └── how-tos/             # How-to guides
├── architecture/            # System design
│   ├── data-flow.md         # Data flow documentation
│   └── graphs/              # Generated Mermaid graphs
└── runbooks/                # Operational procedures
```

## Search Patterns

### Find Files by Purpose

| Looking for... | Search Pattern | Location |
|----------------|----------------|----------|
| Rust crate roots | `*/Cargo.toml` | Anywhere |
| TypeScript entry | `*/index.ts` or `*/main.ts` | `apps/`, `ui/` |
| Python modules | `*/__init__.py` | `ai/`, `ml_devops_platform/` |
| Config schemas | `*.schema.json` | `config/schemas/` |
| Test files | `*_test.rs`, `*.test.ts` | `tests/`, inline |
| CI workflows | `*.yml` | `.github/workflows/` |

### Find Symbols

Use the sweep system to search symbols:

```powershell
# Extract symbols from a file
.\scripts\sweep\symbol-extractor.ps1 -FilePath "path/to/file.rs"

# Search symbols in database
# (After running a sweep)
sqlite3 data/state/sweep/sweep.db "SELECT * FROM symbols WHERE name LIKE '%search%'"
```

## File Statistics

### By Directory (Top 10)

| Directory | Files | % of Total |
|-----------|-------|------------|
| `lib/flexnetos/` | ~15,000 | 16% |
| `data/` | ~12,000 | 13% |
| `cache/` | ~10,000 | 11% |
| `sys/` | ~8,000 | 9% |
| `ui/` | ~6,000 | 6% |
| `ai/` | ~5,000 | 5% |
| `docs/` | ~4,000 | 4% |
| `apps/` | ~3,500 | 4% |
| `opt/` | ~3,000 | 3% |
| `tests/` | ~2,500 | 3% |

### Recent Activity

Files are tracked by sweep with modification timestamps. Run:

```powershell
.\scripts\sweep\sweep.ps1 -Sweep 1 -Operations extract
```

Then query: `SELECT file_path, last_modified FROM file_state ORDER BY last_modified DESC LIMIT 20`

## Related Documentation

- [Complete File Listing](codebase_map.md) - All 55,991 files
- [Directory Tree](../pages/directory-tree.md) - Visual structure
- [Data Flow](../architecture/data-flow.md) - How data moves
- [AGENTS.md](../../AGENTS.md) - Agent instructions
