# NOA Platform Tests

This directory contains test scripts and verification utilities.

## Test Structure

### Unit Tests (In-Crate)

| Crate | Location | Description |
|-------|----------|-------------|
| noa-api-client | `gateway/api/client/rust/src/tests.rs` | API client unit tests with wiremock |
| noa-ui-shell | `ui/app/crates/noa-ui-shell/src/tests.rs` | UI component tests |
| noa-ui-hived | `ui/app/bins/noa-ui-hived/src/tests.rs` | Daemon unit tests |

### Integration Tests

| Test File | Description |
|-----------|-------------|
| `sandbox_integration_test.rs` | Agent sandbox capsule validation |
| `ui_e2e_test.rs` | End-to-end UI integration tests |

### Running Tests

```bash
# All unit tests
cargo test

# Specific crate tests
cargo test -p noa-api-client
cargo test -p noa-ui-shell
cargo test -p noa-ui-hived

# Integration tests (requires services running)
cargo test --test sandbox_integration_test
cargo test --test ui_e2e_test -- --ignored  # E2E tests
```

---

## Phase 3 Verification Tests

This directory also contains test scripts for verifying Phase 3 (US1 - Initialize NOA Seed Environment) implementation.

## Test Scripts

### Manual Verification Scripts

#### `phase3_verification.sh` / `phase3_verification.ps1`
Comprehensive verification script that tests all VER001-VER007 items.

**Usage**:
```bash
# Unix/Linux
bash tests/phase3_verification.sh

# Windows
pwsh tests/phase3_verification.ps1
```

**Tests**:
- VER001: All 8 directories created
- VER002: Directory permissions (Unix only)
- VER003: Initialization performance (<60s)
- VER004: Database operational
- VER005: Offline operation
- VER006: Idempotency
- VER007: Cleanup mechanism

#### `phase3_benchmark.sh`
Performance benchmark script for VER003.

**Usage**:
```bash
bash tests/phase3_benchmark.sh
```

**Output**: Reports initialization duration and compares against 60-second target.

## Automated Tests

### Rust Integration Tests

Location: `sys/core/src/init/tests.rs`

**Run tests**:
```bash
cd sys/core
cargo test --lib init::tests::integration_tests
```

**Test Coverage**:
- ✅ VER001: Directory creation
- ✅ VER002: Directory permissions (Unix)
- ✅ VER004: Database operational
- ✅ VER005: Offline operation
- ✅ VER006: Idempotency
- ✅ VER007: Cleanup mechanism
- ✅ Full initialization workflow
- ✅ Verification functionality

## Test Results

After running tests, check:
1. All scripts exit with code 0 (success)
2. All automated tests pass
3. Performance benchmark shows <60s duration
4. No errors in test output

## Next Steps

1. Run manual verification scripts
2. Execute automated test suite
3. Review test results
4. Address any failures
5. Update verification report with results

