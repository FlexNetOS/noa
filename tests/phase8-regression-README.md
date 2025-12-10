# Phase 8: Regression Test Suite

**Purpose**: Comprehensive regression tests for critical paths, provider integration, and data integrity
**Location**: `sys/core/src/regression/tests.rs`
**Coverage**: REG001-REG017 (17 tests)

---

## Test Categories

### Critical Path Tests (REG001-REG006)

These tests verify the core happy paths through the system:

- **REG001**: Init → Load Model → Query → Response
- **REG002**: Create Memory → Persist → Recall
- **REG003**: Submit Goal → Decompose → Execute → Complete
- **REG004**: Digest Repository → Generate Artifacts
- **REG005**: P2P Connect → Sync → Disconnect Gracefully
- **REG006**: Self-Modify → Verify → Rollback

### Provider Integration Tests (REG007-REG010)

These tests verify provider integration and resilience:

- **REG007**: llama.cpp: Load 5 models, run inference
- **REG008**: Claude Code: Connect, execute task, disconnect
- **REG009**: Shared Memory: Create context, multi-provider read/write
- **REG010**: Provider Fallback: Primary unavailable → Secondary used

### Data Integrity Tests (REG011-REG017)

These tests verify data integrity and consistency:

- **REG011**: Memory checksum verification on 1000 entries
- **REG012**: Database foreign key constraint enforcement
- **REG013**: Vector embedding consistency
- **REG014**: Audit log append-only verification
- **REG015**: Metadata validator (id, created_at, updated_at, checksum)
- **REG016**: Config schema validation against config/schemas/
- **REG017**: Index verification for all database tables

---

## Running Tests

### Using Test Scripts

**Bash (Linux/macOS)**:
```bash
bash tests/phase8_regression.sh
```

**PowerShell (Windows)**:
```powershell
pwsh tests/phase8_regression.ps1
```

### Using Cargo Directly

```bash
cd sys/core
cargo test --lib regression::tests::regression_tests
```

### Running Individual Tests

```bash
cd sys/core
cargo test --lib regression::tests::regression_tests::test_reg002_create_memory_persist_recall
```

---

## Test Status

| Test | Status | Notes |
|------|--------|-------|
| REG001 | ✅ Implemented | Infrastructure verified; requires model files for full E2E |
| REG002 | ✅ Implemented | Full memory lifecycle test |
| REG003 | ✅ Implemented | Infrastructure verified; requires agent orchestration |
| REG004 | ✅ Implemented | Infrastructure verified; requires digest service |
| REG005 | ✅ Implemented | Infrastructure verified; requires P2P service |
| REG006 | ✅ Implemented | Infrastructure verified; requires self-improvement service |
| REG007 | ✅ Implemented | Infrastructure verified; requires 5 model files |
| REG008 | ✅ Implemented | Infrastructure verified; requires Claude API credentials |
| REG009 | ✅ Implemented | Infrastructure verified; requires shared memory service |
| REG010 | ✅ Implemented | Infrastructure verified; requires provider management |
| REG011 | ✅ Implemented | Full test with 1000 entries |
| REG012 | ✅ Implemented | Foreign key verification |
| REG013 | ✅ Implemented | Infrastructure verified; requires embedding service |
| REG014 | ✅ Implemented | Infrastructure verified; requires audit service |
| REG015 | ✅ Implemented | Full metadata validation test |
| REG016 | ✅ Implemented | Infrastructure verified; requires config validator |
| REG017 | ✅ Implemented | Database index verification |

---

## Test Requirements

### Prerequisites

- Rust 1.83+ toolchain
- SQLite database (created automatically)
- Temporary directory for test isolation

### Test Data

Tests use `tempfile::TempDir` for isolation. Each test:
1. Creates a temporary NOA root directory
2. Initializes the database
3. Runs the test
4. Cleans up automatically

### Dependencies

Tests require:
- `tempfile` crate (dev dependency)
- `tokio` runtime (for async tests)
- Database initialization modules
- Service modules (MemoryService, NeuralService, etc.)

---

## Implementation Notes

### Infrastructure Tests vs Full E2E Tests

Some tests (REG001, REG003-REG010, REG013-REG014, REG016) are **infrastructure verification tests** that verify:
- Required services exist
- Database is initialized correctly
- Directory structure is created

These tests will be expanded to full E2E tests as the corresponding services are implemented.

### Full Implementation Tests

Tests REG002, REG011, REG012, REG015, and REG017 are **fully implemented** and test actual functionality:
- **REG002**: Complete memory lifecycle (create, persist, recall)
- **REG011**: Creates 1000 memory entries and verifies checksums
- **REG012**: Verifies foreign key constraints are enabled
- **REG015**: Validates all metadata fields (id, timestamps, checksum)
- **REG017**: Verifies database indexes exist

---

## Adding New Regression Tests

When adding new regression tests:

1. Add test function to `sys/core/src/regression/tests.rs`
2. Follow naming convention: `test_reg###_descriptive_name`
3. Use `tempfile::TempDir` for test isolation
4. Update this README with test description
5. Update `verification.md` checklist

---

## Related Documentation

- [Verification Checklist](../specs/001-noa-seed-foundation/checklists/verification.md)
- [Quality Checklist](../specs/001-noa-seed-foundation/checklists/quality.md)
- [Test README](./README.md)

---

**Last Updated**: 2025-01-27
**Status**: All 17 tests implemented (infrastructure + full tests)

