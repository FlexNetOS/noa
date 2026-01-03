# Rust/Cargo Subagents

## RustCrateScannerAgent
- Scope: Discover crates, versions, features; map dependency tree
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## CargoBuildAgent
- Scope: Build/bench/test workflows with caching and EFG-aware parallelism
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## CargoAuditAgent
- Scope: Integrate cargo-audit, triage RUSTSEC advisories
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## CargoLicenseAgent
- Scope: Scan licenses, enforce allow-lists
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustClippyAgent
- Scope: Clippy linting tiers; autofix common lints
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustFmtAgent
- Scope: Format code; enforce style policies
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustDocAgent
- Scope: Generate and publish doc artifacts
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustFFIAgent
- Scope: bindgen/cbindgen pipelines, ABI tests
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustWasmAgent
- Scope: wasm-pack + size/perf budgeting, bindings
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustCrossAgent
- Scope: Cross-compile matrix: musl/aarch64 etc.
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

## RustReleaseAgent
- Scope: Crate publishing workflow (private/public)
- Inputs: repo/crate workspace
- Outputs: artifacts, SBOM, scores, advisories
- Policies: MSRV, semver, export-control

