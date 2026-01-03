# Generated Documentation

This directory contains auto-generated documentation from the Litho documentation generator.

> ⚠️ **Do not manually edit files in this directory** - they will be overwritten on regeneration.
> Use `<!-- provider:add-manual-edit -->` markers in source files to preserve custom content.

## Structure

- **api/** - Generated API reference documentation
- **crates/** - Rustdoc output for all crates
- **diagrams/** - Auto-generated Mermaid architecture diagrams

## Generation Commands

```bash
# Generate all documentation
noa wiki generate-full

# Generate for changed files only
noa wiki generate-incremental

# Validate generated docs
noa wiki validate
```

## Provider Configuration

Documentation is generated using the multi-pass pipeline:

| Pass | Agent | Output |
|------|-------|--------|
| 1 | RustCrateScannerAgent | Crate structure analysis |
| 2 | RustClippyAgent | Code quality notes |
| 3 | RustDocAgent | API documentation |
| 4 | RustFmtAgent | Validation and cleanup |

## Model

- **Primary**: qwen2.5-coder:1.5b via llama.cpp (local, <3B params)
- **Fallback**: copilot → anthropic → openai → git templates

---

*Last generated: (pending first run)*
