# Phase 1 Changelog

**Phase**: Phase 1 - Setup (Shared Infrastructure)
**Version**: 0.1.0
**Date**: 2025-01-27

---

## [0.1.0] - 2025-01-27

### Added

#### Directory Structure (FR-029 to FR-036)
- Created `sys/` directory with subdirectories: `core/`, `ui/`, `digest/`, `kernel/`
- Created `p2p/` directory with subdirectories: `discovery/`, `sync/`, `compute/`, `storage/`
- Created `opt/` directory for optional packages
- Created `init/` directory with subdirectories: `migrations/`, `seeds/`, `bootstrap/`
- Created `containers/` directory with subdirectories: `oci/`, `compose/`
- Created `config/` directory with configuration files and schemas
- Created `bin/` directory for executables
- Created `ai/` directory with subdirectories: `providers/`, `models/`, `prompts/`, `grammars/`, `shared/`

#### Project Initialization (T010-T013)
- Initialized Rust workspace (`sys/core/Cargo.toml`) with crates: api, embedder, trainer, indexer, agent, common, neural
- Initialized Go module (`p2p/go.mod`) for P2P services
- Initialized TypeScript/Next.js project (`sys/ui/package.json`)
- Initialized Python project (`sys/digest/pyproject.toml`) for digest pipeline

#### Configuration Files (T016)
- Created `config/noa-server.json` - Main server configuration
- Created `config/ai-providers.json` - AI provider configuration
- Created `config/features.json` - Feature flags
- Created `config/database.yaml` - Database configuration
- Created `config/minio.yaml` - MinIO storage configuration
- Created `config/qdrant.yaml` - Qdrant vector store configuration
- Created `config/bootstrap-state.json` - Bootstrap state tracking
- Created `config/shared-resources.json` - Shared resources configuration

#### Scripts (T015, T673-T674)
- Created `scripts/bash/build.sh` - Cross-platform build script (Bash)
- Created `scripts/powershell/build.ps1` - Cross-platform build script (PowerShell)
- Created `scripts/bash/check-prerequisites.sh` - Prerequisite check (Bash shim)
- Created `init/check-prereqs.sh` - Prerequisite check (authoritative Unix)
- Created `scripts/powershell/check-prerequisites.ps1` - Prerequisite check (PowerShell)
- Created `scripts/setup/check-prereqs.ps1` - Prerequisite check (PowerShell, unified)

#### CI Pipeline (T017, T675)
- Created `.github/workflows/ci.yml` - GitHub Actions CI pipeline
- Integrated prerequisite check into CI pipeline
- Configured multi-platform testing (ubuntu-latest, windows-latest, macos-latest)

#### Documentation (T018)
- Created `README.md` with quickstart instructions
- Created `config/README.md` - Configuration documentation
- Created `scripts/README.md` - Scripts documentation with cross-platform mapping
- Created `specs/001-noa-seed-foundation/FINAL_REPORT.md` - Phase 1 final report
- Created `specs/001-noa-seed-foundation/COVERAGE.md` - Coverage mapping
- Created `specs/001-noa-seed-foundation/HASHES.txt` - SHA-256 hashes
- Created `specs/001-noa-seed-foundation/checklists/phase1-quality-verification.md` - Quality verification report

#### Testing
- Created `scripts/test/smoke-test-phase1.sh` - Phase 1 smoke tests (Bash)
- Created `scripts/test/smoke-test-phase1.ps1` - Phase 1 smoke tests (PowerShell)
- Created `scripts/test/negative-tests-phase1.sh` - Negative and boundary tests (Bash)
- Created `scripts/test/negative-tests-phase1.ps1` - Negative and boundary tests (PowerShell)

#### Quality Assurance
- Completed quality checklist verification (130 items)
- Created evidence ledger and claims table
- Documented gaps and remedies

### Changed

#### Provider Configuration
- Added `latency` and `timeout` fields to `ai/providers/local/ollama/config.json` (CHK127)
  - `latency.target`: "<2s"
  - `latency.timeout`: 60000
  - `timeout`: 60000

### Fixed

- N/A (Initial implementation)

### Removed

- N/A (Initial implementation)

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1.0 | 2025-01-27 | Initial Phase 1 implementation |

---

## Migration Notes

### From Pre-Phase 1

This is the initial Phase 1 implementation. No migration needed.

### To Phase 2

When moving to Phase 2:
1. Database schema will be initialized
2. Storage components will be configured
3. Additional configuration files may be added

---

**Maintained By**: NOA Development Team
**Last Updated**: 2025-01-27

