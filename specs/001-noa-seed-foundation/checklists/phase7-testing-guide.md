# Phase 7 Testing Guide & Commands

**Phase**: Phase 7 - Dynamic Context-Aware UI (US5)
**Created**: 2025-12-10
**Purpose**: Complete testing commands and procedures for Phase 7

---

## Quick Start

### Run All Tests (Recommended)
```powershell
# PowerShell
.\scripts\powershell\test-phase7-full.ps1

# Bash (Linux/Mac/Git Bash)
./scripts/bash/test-phase7.sh --all
```

### Run Specific Test Types
```powershell
# PowerShell
.\scripts\powershell\test-phase7.ps1 -Smoke -Unit -Coverage

# Bash
./scripts/bash/test-phase7.sh --smoke --unit --coverage
```

---

## Complete Test Commands

### 1. Type Checking
```bash
cd sys/ui
npm run type-check
```
**Purpose**: Verify TypeScript types are correct
**Expected**: Exit code 0, no type errors

### 2. Linting
```bash
cd sys/ui
npm run lint
```
**Purpose**: Check code style and catch common errors
**Expected**: Exit code 0, no linting errors

### 3. Build Verification
```bash
cd sys/ui
npm run build
```
**Purpose**: Verify production build succeeds
**Expected**: Exit code 0, build artifacts in `.next/`

### 4. Smoke Tests
```bash
cd sys/ui
npm run test:smoke
```
**Purpose**: Basic functionality verification
**Tests**: Component imports, service instantiation, utility functions
**Expected**: All smoke tests pass

### 5. Unit Tests
```bash
cd sys/ui
npm run test:unit
```
**Purpose**: Test isolated components and services
**Tests**:
- API client (`__tests__/unit/lib/api.test.ts`)
- WebSocket client (`__tests__/unit/lib/websocket.test.ts`)
- Context detector (`__tests__/unit/services/contextDetector.test.ts`)
**Expected**: All unit tests pass, >70% coverage

### 6. Integration Tests
```bash
cd sys/ui
npm run test:integration
```
**Purpose**: Test component interactions
**Expected**: All integration tests pass

### 7. E2E Tests
```bash
cd sys/ui
npm run test:e2e
```
**Purpose**: Test complete user workflows
**Expected**: All E2E tests pass

### 8. Coverage Report
```bash
cd sys/ui
npm run test:coverage
```
**Purpose**: Generate code coverage report
**Output**: `coverage/` directory with HTML report
**Threshold**: 70% minimum for branches, functions, lines, statements

### 9. All Tests (CI Mode)
```bash
cd sys/ui
npm run test:ci
```
**Purpose**: Run all tests with coverage in CI-friendly mode
**Expected**: All tests pass, coverage report generated

---

## Test Scripts

### PowerShell Scripts

#### Basic Test Suite
```powershell
.\scripts\powershell\test-phase7.ps1 [Options]
```

**Options**:
- `-Smoke` - Run smoke tests only
- `-Unit` - Run unit tests only
- `-Integration` - Run integration tests only
- `-E2E` - Run E2E tests only
- `-Coverage` - Generate coverage report
- `-All` - Run all test types

**Example**:
```powershell
# Run all tests
.\scripts\powershell\test-phase7.ps1 -All

# Run smoke and unit tests with coverage
.\scripts\powershell\test-phase7.ps1 -Smoke -Unit -Coverage
```

#### Full Test Suite (with Quality Checks)
```powershell
.\scripts\powershell\test-phase7-full.ps1
```

**What it does**:
1. Runs all basic tests
2. Generates file hashes (SHA-256)
3. Verifies all Phase 7 files exist
4. Runs quality checklist verification
5. Generates coverage report
6. Creates test execution summary

**Output Files**:
- `specs/001-noa-seed-foundation/checklists/phase7-hashes.txt` - File hashes
- `specs/001-noa-seed-foundation/checklists/phase7-test-execution-summary.md` - Test summary

### Bash Scripts

#### Basic Test Suite
```bash
./scripts/bash/test-phase7.sh [Options]
```

**Options**:
- `--smoke` - Run smoke tests only
- `--unit` - Run unit tests only
- `--integration` - Run integration tests only
- `--e2e` - Run E2E tests only
- `--coverage` - Generate coverage report
- `--all` - Run all test types

**Example**:
```bash
# Run all tests
./scripts/bash/test-phase7.sh --all

# Run smoke and unit tests with coverage
./scripts/bash/test-phase7.sh --smoke --unit --coverage
```

---

## Manual Testing Procedures

### 1. Component Rendering Test
```bash
cd sys/ui
npm run dev
```
Then manually verify:
- [ ] Home page loads (`http://localhost:3000`)
- [ ] Activity page loads (`http://localhost:3000/activity`)
- [ ] Chat page loads (`http://localhost:3000/chat`)
- [ ] Settings page loads (`http://localhost:3000/settings`)
- [ ] Admin pages load (`http://localhost:3000/admin/*`)

### 2. API Integration Test
```bash
# Start backend API (if available)
# Then test API client
cd sys/ui
npm run dev
```
Verify:
- [ ] API health check works
- [ ] API status endpoint works
- [ ] Error handling works (disconnect backend)

### 3. WebSocket Test
```bash
cd sys/ui
npm run dev
```
Verify:
- [ ] WebSocket connects
- [ ] Messages are received
- [ ] Reconnection works (disconnect/reconnect)

### 4. Settings Sync Test
```bash
cd sys/ui
npm run dev
```
Verify:
- [ ] Settings can be changed
- [ ] Settings persist (refresh page)
- [ ] Settings sync across tabs (if implemented)

### 5. Widget System Test
```bash
cd sys/ui
npm run dev
```
Verify:
- [ ] Widgets can be added
- [ ] Widgets can be rearranged (drag-and-drop)
- [ ] Widget layout persists

---

## Quality Checklist Verification

### Run Quality Checks

The Phase 7 quality checklist is at:
`specs/001-noa-seed-foundation/checklists/phase7-quality.md`

### Checklist Categories

1. **Pre-Commit** (Q7-001 to Q7-025)
   - Evidence & Documentation
   - Run before every commit

2. **Pre-PR** (Q7-026 to Q7-050)
   - Truth Gate & Verification
   - Run before creating pull request

3. **Code Review** (Q7-051 to Q7-075)
   - Code Quality & Consistency
   - Run during code review

4. **Pre-Merge** (Q7-076 to Q7-100)
   - configs & Schema Validation
   - Run before merging

5. **Release Gate** (Q7-101 to Q7-130)
   - Final Verification & Audit
   - Run before release

### Manual Checklist Review

```bash
# Open quality checklist
code specs/001-noa-seed-foundation/checklists/phase7-quality.md

# Review each item and mark as complete [X]
# Document any N/A items with reasons
```

---

## Test File Structure

```
sys/ui/
├── __tests__/
│   ├── smoke/
│   │   └── phase7-smoke.test.ts      # Basic functionality tests
│   ├── unit/
│   │   ├── lib/
│   │   │   ├── api.test.ts            # API client tests
│   │   │   └── websocket.test.ts      # WebSocket client tests
│   │   └── services/
│   │       └── contextDetector.test.ts # Context detector tests
│   ├── integration/                   # Component interaction tests
│   └── e2e/                           # End-to-end workflow tests
├── jest.configs.js                     # Jest configsuration
├── jest.setup.js                      # Test setup and mocks
└── package.json                       # Test scripts
```

---

## Coverage Requirements

### Minimum Thresholds
- **Branches**: 70%
- **Functions**: 70%
- **Lines**: 70%
- **Statements**: 70%

### View Coverage Report
```bash
cd sys/ui
npm run test:coverage
open coverage/lcov-report/index.html
```

### Coverage by Component
- Components: Target 80%+
- Services: Target 80%+
- Utilities: Target 90%+
- Pages: Target 70%+

---

## Troubleshooting

### Issue: SWC Binary Error (Windows)
**Error**: `Failed to load SWC binary for win32/x64`

**Solution**:
1. Reinstall dependencies: `cd sys/ui && npm install`
2. Tests will still run but may be slower
3. This is a known Next.js issue on Windows

### Issue: Module Not Found
**Error**: `Cannot find module '@/components/...'`

**Solution**:
1. Check `jest.configs.js` has correct `moduleNameMapper`
2. Verify `tsconfigs.json` paths match Jest paths
3. Check file extensions in imports

### Issue: Tests Timeout
**Error**: `Timeout - Async callback was not invoked`

**Solution**:
1. Increase timeout: `jest.setTimeout(10000)`
2. Check for infinite loops
3. Verify mocks are properly set up

### Issue: WebSocket Tests Fail
**Error**: `WebSocket is not defined`

**Solution**:
1. Check `jest.setup.js` has WebSocket mock
2. Verify test environment is `jest-environment-jsdom`

---

## Continuous Integration

### GitHub Actions Example

```yaml
name: Phase 7 Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '20'
      - run: cd sys/ui && npm ci
      - run: cd sys/ui && npm run test:ci
      - run: cd sys/ui && npm run test:coverage
      - uses: codecov/codecov-action@v3
        with:
          files: ./sys/ui/coverage/lcov.info
```

---

## Performance Testing

### Lighthouse CI
```bash
# Install
npm install -g @lhci/cli

# Run
cd sys/ui
lhci autorun --configs=.lighthouserc.json
```

### Load Testing
```bash
# Use k6 for API load testing
k6 run load-test.js
```

---

## Security Testing

### Dependency Scanning
```bash
cd sys/ui
npm audit
npm audit fix
```

### OWASP Checklist
See quality checklist items Q7-127 to Q7-130:
- [ ] Q7-127 - API keys not hardcoded
- [ ] Q7-128 - User inputs sanitized
- [ ] Q7-129 - XSS vulnerabilities prevented
- [ ] Q7-130 - CSRF protections in place

---

## Test Execution Summary

After running tests, review:
1. Test results summary
2. Coverage report
3. Quality checklist status
4. File hash verification
5. Test execution summary document

---

## Next Steps

1. ✅ Run smoke tests
2. ✅ Run unit tests
3. ⚠️ Add integration tests
4. ⚠️ Add E2E tests
5. ⚠️ Complete quality checklist
6. ⚠️ Generate file hashes
7. ⚠️ Create test execution report

---

**Last Updated**: 2025-12-10
**Phase**: Phase 7 - Dynamic Context-Aware UI (US5)

