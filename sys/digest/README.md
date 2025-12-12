# NOA Digest Everything Pipeline

A comprehensive codebase analysis and digest generation tool for the NOA ecosystem.

## Overview

NOA Digest provides automated analysis of codebases to generate:
- Code structure and dependency analysis
- Software Bill of Materials (SBOM)
- Security vulnerability reports
- Documentation and artifact generation

## Installation

```bash
pip install -e ".[dev]"
```

## Usage

### Analyze a Codebase

```bash
noa-digest analyze /path/to/repository
```

### Generate SBOM

```bash
noa-digest sbom /path/to/repository --format cyclonedx
```

### Security Analysis

```bash
noa-digest security /path/to/repository
```

## Development

### Setup

```bash
pip install -e ".[dev]"
```

### Testing

```bash
pytest
```

### Linting

```bash
ruff check .
```

### Type Checking

```bash
mypy src/
```

## License

MIT
