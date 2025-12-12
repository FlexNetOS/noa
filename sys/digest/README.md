# NOA Digest Everything Pipeline

> **⚠️ Development Status**: This project is currently in early development. The CLI commands (`analyze`, `sbom`, `security`) are not yet fully implemented. See the [Usage](#usage) section for current status.

The NOA Digest Everything Pipeline provides code analysis and digestion capabilities for the NOA system.

## Features (Planned)

- Multi-language code parsing using tree-sitter
- Support for Python, JavaScript, TypeScript, Go, Rust, and Java
- Code analysis and extraction
- Database integration for storing analysis results

## Installation

```bash
pip install -e ".[dev]"
```

## Usage

### Current Status

The digest pipeline is under active development. The CLI interface is available but the core functionality is not yet implemented:

```bash
# CLI commands are available but show "not yet implemented" messages
noa-digest analyze <path>    # Planned: Analyze a codebase and generate digest artifacts
noa-digest sbom <path>        # Planned: Generate Software Bill of Materials
noa-digest security <path>    # Planned: Run security analysis on codebase
```

### API (Planned)

```python
from noa_digest import DigestPipeline

# Use the digest pipeline (not yet implemented)
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
