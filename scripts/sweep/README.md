# NOA Codebase Sweep System

A comprehensive 10-loop sweep system for auditing, documenting, and improving the noa codebase.

## Overview

The sweep system performs parallel sweeps through the entire noa directory tree:

1. **Symbol Extraction** - Traces every symbol (functions, structs, classes, etc.)
2. **Doc Cross-Reference** - Validates symbols against wiki, runbooks, and pages
3. **Embedding Generation** - Creates vector embeddings using Ollama
4. **SQLite Storage** - Persists all data for analysis
5. **Graph Generation** - Produces Mermaid diagrams for visualization
6. **E2E Testing** - Validates with notebook and integration tests

## Quick Start

```powershell
# Run a complete sweep
.\sweep.ps1 -Sweep 1

# Run specific operations only
.\sweep.ps1 -Sweep 1 -Operations extract,docs

# Dry run (no changes)
.\sweep.ps1 -Sweep 1 -DryRun

# Skip tests for faster iteration
.\sweep.ps1 -Sweep 1 -SkipTests
```

## Architecture

```
scripts/sweep/
├── sweep.ps1                 # Main entry point
├── sweep-orchestrator.ps1    # Parallel file processing
├── symbol-extractor.ps1      # Symbol extraction from source
├── doc-xref-checker.ps1      # Documentation validation
├── ollama-embeddings.ps1     # Ollama vector embeddings
├── graph-generator.ps1       # Mermaid diagram generation
├── e2e-test-runner.ps1       # End-to-end test runner
└── README.md                 # This file
```

## Components

### 1. Symbol Extractor

Extracts symbols from Rust, TypeScript, JavaScript, Python, and Shell files:

```powershell
.\symbol-extractor.ps1 -FilePath "src/main.rs" -Language Rust -OutputFormat json
```

**Extracted Information:**
- Symbol name, type, visibility
- Line numbers and signatures
- Doc comments
- Generics and parameters

### 2. Doc Cross-Reference Checker

Validates all symbols have documentation:

```powershell
.\doc-xref-checker.ps1 -GenerateReport
```

**Produces:**
- Gap report (undocumented symbols)
- Coverage statistics
- Orphaned documentation references

### 3. Ollama Embeddings

Generates vector embeddings using local Ollama:

```powershell
# Ensure Ollama is running
ollama serve

# Generate embeddings
.\ollama-embeddings.ps1 -InputFile "symbols.json" -Model nomic-embed-text
```

**Supported Models:**
- `nomic-embed-text` (default, 768 dimensions)
- `mxbai-embed-large` (1024 dimensions)
- `all-minilm` (384 dimensions)
- `snowflake-arctic-embed`

### 4. Graph Generator

Creates Mermaid diagrams:

```powershell
.\graph-generator.ps1 -InputType all
```

**Generated Graphs:**
- `cargo-dependencies.mmd` - Crate dependency graph
- `module-structure.mmd` - Top-level module structure
- `doc-coverage-pie.mmd` - Documentation distribution
- `doc-coverage-heatmap.mmd` - Coverage by directory
- `symbol-relationships.mmd` - Symbol relationships

### 5. E2E Test Runner

Runs all tests including notebooks:

```powershell
.\e2e-test-runner.ps1 -TestType all
```

**Test Types:**
- `notebooks` - Jupyter notebook tests
- `rust` - Cargo tests
- `typescript` - Jest/npm tests
- `python` - pytest tests
- `integration` - Cross-component tests

## Data Storage

### SQLite Database

Located at `data/state/sweep/sweep.db`:

```sql
-- Tables
sweep_state     -- Sweep run metadata
file_state      -- Per-file processing state
symbols         -- Extracted symbols
embeddings      -- Vector embeddings
doc_refs        -- Documentation references
graph_edges     -- Symbol relationships
```

### FlexNetOS Dependencies

The sweep system uses these cloned repositories:

```
lib/flexnetos/
├── rusqlite/       # SQLite bindings for Rust
├── sqlx/           # Async SQL toolkit
├── rust-postgres/  # PostgreSQL driver
└── vector-db/      # RuVector implementation
```

## Configuration

### Ollama Setup

```powershell
# Install Ollama
winget install ollama

# Start server
ollama serve

# Pull embedding model
ollama pull nomic-embed-text
```

### RuVector Integration

The sweep system integrates with RuVector for:
- HNSW vector indexing
- GNN-enhanced search
- Self-learning optimization (SONA)

See `docs/wiki/crates/ruvector_README.md` for details.

## Sweep Loop Strategy

### Loop 1-3: Foundation
- Extract all symbols
- Build initial doc index
- Generate baseline graphs

### Loop 4-6: Enhancement
- Add embeddings to all symbols
- Cross-reference with RuVector
- Update documentation gaps

### Loop 7-9: Optimization
- Refine symbol relationships
- Optimize test coverage
- Clean up orphaned docs

### Loop 10: Finalization
- Generate final reports
- Validate all cross-references
- Produce roadmap

## Output Files

```
data/state/sweep/
├── sweep.db                    # SQLite database
├── sweep-state.json            # Current sweep state
├── symbols.json                # Extracted symbols
├── symbols-with-embeddings.json
├── doc-gap-report.md           # Documentation gaps
├── doc-gaps.json
├── sweep-N-summary.md          # Per-sweep summaries
└── test-results/
    ├── e2e-test-report.md
    └── e2e-test-results.json

docs/architecture/graphs/
├── cargo-dependencies.mmd
├── module-structure.mmd
├── doc-coverage-pie.mmd
├── doc-coverage-heatmap.mmd
└── symbol-relationships.mmd

logs/sweep/
└── sweep-N-YYYYMMDD-HHmmss.log
```

## Troubleshooting

### Ollama Not Available

```powershell
# Check if running
curl http://localhost:11434/api/tags

# Start server
ollama serve
```

### SQLite Not Found

The system falls back to JSON storage if SQLite is unavailable.
SQLite is typically bundled with Git for Windows at:
`C:\Program Files\Git\usr\bin\sqlite3.exe`

### Slow Symbol Extraction

Increase parallelism:
```powershell
.\sweep.ps1 -Sweep 1 -MaxParallel 16
```

## Integration with RuVector

The sweep system uses RuVector capabilities:

- **HNSW Index**: Fast nearest-neighbor search for embeddings
- **GNN Layers**: Self-improving search results
- **SONA**: Runtime learning from query patterns
- **Cypher Queries**: Graph-based symbol navigation

```rust
// Example: Query related symbols
MATCH (a:Symbol)-[:CALLS]->(b:Symbol)
WHERE a.name = 'process_request'
RETURN b.name, b.type, b.file
```

## Contributing

When extending the sweep system:

1. Follow TDD - write tests first
2. Use PowerShell for cross-platform scripts
3. Store results in SQLite/JSON
4. Generate Mermaid for visualizations
5. Update this README

## License

MIT - See LICENSE file in repository root.
