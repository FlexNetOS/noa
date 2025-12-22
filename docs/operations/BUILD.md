# NOA Build System

## Release Gate

The `release-gate` script validates the codebase is ready for release by running all quality checks in sequence.

### Usage

```bash
# Unix/Linux/macOS
./scripts/release-gate.sh

# Windows PowerShell
.\scripts\release-gate.ps1
```

### Options

| Option | Description |
|--------|-------------|
| `--quick` / `-Quick` | Skip slow tests, run minimal checks |
| `--skip-ui` / `-SkipUI` | Skip UI build step |
| `--skip-integration` / `-SkipIntegration` | Skip integration tests |
| `--verbose` / `-VerboseOutput` | Enable verbose output |

### What It Checks

1. **Rust Format** - `cargo fmt --check`
2. **Rust Lints** - `cargo clippy` with warnings as errors
3. **Cargo Check** - Type checking without full compilation
4. **Unit Tests** - `cargo test`
5. **Integration Tests** - Tests marked with `#[test]` in `tests/` directory
6. **UI Build** - TypeScript type-check, lint, and build
7. **Go Checks** - Build and test (if available)
8. **Python Checks** - Ruff lint (if available)

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All checks passed |
| 1 | One or more checks failed |
| 2 | Missing prerequisites |

---

## Windows Build Issues

### The "Access is denied (os error 5)" Problem

On Windows, Rust incremental builds frequently fail with:

```
error: could not write to -C incremental=...: Access is denied. (os error 5)
```

This occurs because:
1. Windows file locking is more aggressive than Unix
2. Antivirus software (Windows Defender) can hold locks on build artifacts
3. The incremental compilation cache uses many small files that can conflict

### Solution: Disable Incremental Builds

For reliable Windows builds, set:

```powershell
$env:CARGO_INCREMENTAL = "0"
```

Or in bash (Git Bash, MSYS2):

```bash
export CARGO_INCREMENTAL=0
```

**The release-gate scripts automatically detect Windows and set this.**

### Permanent Configuration

Add to your `.cargo/config.toml` in the workspace:

```toml
[build]
incremental = false
```

Or add to your shell profile:

```powershell
# PowerShell profile (~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1)
$env:CARGO_INCREMENTAL = "0"
```

```bash
# .bashrc / .zshrc
export CARGO_INCREMENTAL=0
```

### Trade-offs

| Incremental ON | Incremental OFF |
|----------------|-----------------|
| Faster rebuilds after small changes | Slightly slower rebuilds |
| Frequent "Access denied" on Windows | Reliable on all platforms |
| Cache can grow large | Smaller cache |
| Can cause spurious rebuild failures | Deterministic builds |

**Recommendation**: Always disable incremental builds in CI and on Windows development machines. The reliability benefit outweighs the compilation speed cost.

---

## CI Configuration

The GitHub Actions CI workflow runs on Windows, Linux, and macOS. Key settings:

```yaml
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  # Windows-specific (set per-job if needed)
  CARGO_INCREMENTAL: 0
```

### Adding the Release Gate to CI

To use the release-gate script in CI:

```yaml
- name: Release Gate
  run: |
    if [ "$RUNNER_OS" = "Windows" ]; then
      pwsh -File ./scripts/release-gate.ps1 -Quick
    else
      ./scripts/release-gate.sh --quick
    fi
  shell: bash
```

---

## Component Build Commands

### Rust Core

```bash
cd sys/core
cargo build --release
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

### Go P2P

```bash
cd p2p
go build ./...
go test ./...
```

### TypeScript UI

```bash
cd sys/ui
npm ci
npm run type-check
npm run lint
npm run build
```

### Python Digest

```bash
cd sys/digest
pip install -e ".[dev]"
ruff check .
mypy src/
pytest
```

---

## Cleaning Build Artifacts

When builds fail mysteriously, try:

```bash
# Rust
cd sys/core
cargo clean

# Node
cd sys/ui
rm -rf node_modules .next
npm ci

# Go
cd p2p
go clean -cache
```

On Windows, if you get permission errors while cleaning:

```powershell
# Wait for antivirus to release locks
Start-Sleep -Seconds 5
Remove-Item -Recurse -Force .\target
```
