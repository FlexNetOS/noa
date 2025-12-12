# NOA Digest Everything Pipeline

> ⚠️ **Development Status**: This package is currently under active development. CLI commands (`analyze`, `sbom`, `security`) are not yet fully implemented.

The NOA Digest Everything Pipeline provides code analysis and digestion capabilities for the NOA system.

## Planned Features

- Multi-language code parsing using tree-sitter
- Support for Python, JavaScript, TypeScript, Go, Rust, and Java
- Code analysis and extraction
- Database integration for storing analysis results

## Installation

```bash
pip install -e ".[dev]"
```

## Usage

### CLI (In Development)

The CLI provides commands for codebase analysis:

```bash
# Analyze a codebase (not yet implemented)
noa-digest analyze /path/to/repo

# Generate SBOM (not yet implemented)
noa-digest sbom /path/to/repo

# Run security scan (not yet implemented)
noa-digest security /path/to/repo
```

### Python API (In Development)

```python
from noa_digest import DigestPipeline

# Use the digest pipeline
pipeline = DigestPipeline()
```

## Development

Install development dependencies:

```bash
pip install -e ".[dev]"
```

Run tests:

```bash
pytest
```

## License

MIT License
