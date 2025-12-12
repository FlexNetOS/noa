# NOA Digest Everything Pipeline

A comprehensive codebase analysis and digest generation tool for the NOA ecosystem.

## Overview

NOA Digest provides automated analysis of codebases to generate:
- Code structure and dependency analysis
- Software Bill of Materials (SBOM)
- Security vulnerability reports
- Documentation and artifact generation
The NOA Digest Everything Pipeline provides code analysis and digestion capabilities for the NOA system.

## Features

- Multi-language code parsing using tree-sitter
- Support for Python, JavaScript, TypeScript, Go, Rust, and Java
- Code analysis and extraction
- Database integration for storing analysis results

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
```python
from noa_digest import DigestPipeline

# Use the digest pipeline
pipeline = DigestPipeline()
```

## Development

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
Install development dependencies:

```bash
pip install -e ".[dev]"
```

Run tests:

```bash
pytest
```

## License

MIT
