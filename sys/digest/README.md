# NOA Digest Everything Pipeline

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
