# Quality & Verification Checklist: NOA Seed Foundation

**Purpose**: Universal Task Execution Policy compliance and code quality gates
**Created**: 2025-12-09
**Type**: Quality Assurance / Requirements Validation / Error Correction
**Based On**: Universal Task Execution Policy (§0-§13)
**Coverage**: Evidence Rules, Truth Gate, Triple-Verification, Code Quality, Metadata, Config Schema

---

## How to Use This Checklist

1. **Pre-Commit**: Run CHK001-CHK025 (Evidence & Documentation)
2. **Pre-PR**: Run CHK026-CHK050 (Truth Gate & Verification)
3. **Code Review**: Run CHK051-CHK075 (Code Quality & Consistency)
4. **Pre-Merge**: Run CHK076-CHK100 (Config & Schema Validation)
5. **Release Gate**: Run CHK101-CHK130 (Final Verification & Audit)

---

## Category 1: Evidence & Documentation Requirements (§3)

### Citation & Source Requirements

- [ ] CHK001 - Are all claims derivable from user artifacts or shown math? If not, is explicit "no evidence" label applied? [Evidence Rules §3]
- [ ] CHK002 - Do all time-sensitive facts include source dates? [Evidence Rules §3]
- [ ] CHK003 - Are all mathematical calculations shown digit-by-digit with formulae and assumptions? [Evidence Rules §3]
- [ ] CHK004 - Are all links verified as real (not fabricated)? If unavailable, is "link unavailable" stated? [Evidence Rules §3]
- [ ] CHK005 - Do code examples include seed values, exact commands, environment versions? [Evidence Rules §3, Repro]
- [ ] CHK006 - Is every claim cross-referenced to its source with explicit mapping? [Evidence Rules §3]
- [ ] CHK007 - Are claims without source or test coverage explicitly flagged? [Evidence Rules §3]

### Documentation Completeness

- [X] CHK008 - Is a FINAL_REPORT.md created with claims table, evidence ledger, gate checklist? [Execution Artifacts §9]
  - ✅ **IMPLEMENTED**: `test-results/FINAL_REPORT.md` exists with claims table
- [X] CHK009 - Does TEST/ directory contain scripts, fixtures, and expected outputs? [Execution Artifacts §9]
  - ✅ **IMPLEMENTED**: `test-results/TEST/` contains smoke-test.sh and smoke-test.ps1
- [X] CHK010 - Is HASHES.txt generated with SHA-256 for all key files? [Execution Artifacts §9]
  - ✅ **IMPLEMENTED**: `scripts/bash/generate-hashes.sh` generates HASHES.txt
- [X] CHK011 - Does REPRO.md specify exact environment and commands for reproduction? [Execution Artifacts §9]
  - ✅ **IMPLEMENTED**: `test-results/REPRO.md` exists
- [X] CHK012 - Does COVERAGE.md map requirements to artifacts with open gaps noted? [Execution Artifacts §9]
  - ✅ **IMPLEMENTED**: `test-results/COVERAGE.md` contains requirements mapping

### Update Semantics (Heal, Do Not Harm §0.1)

- [ ] CHK013 - Do updates preserve correct prior content without regressions? [Update Semantics §0.1]
- [ ] CHK014 - Are fine-grained details preserved (no lossy summarization)? [Update Semantics §0.1]
- [ ] CHK015 - Does any removal have a stated reason and replacement/mitigation? [Update Semantics §0.1]
- [ ] CHK016 - Are updates propagated consistently across specs, code, tests, and docs? [Update Semantics §0.1]

---

## Category 2: Truth Gate Requirements (§4)

### Strong Claim Validation (Built/Ready/Delivered/Verified/Unbounded)

- [X] CHK017 - Are all referenced files verified to exist in export or repo? [Truth Gate §4.1]
  - ✅ **IMPLEMENTED**: `scripts/bash/truth-gate.sh` and `scripts/powershell/truth-gate.ps1` verify all artifacts
- [X] CHK018 - Is a deterministic smoke test provided with command, transcript, and exit code 0? [Truth Gate §4.2]
  - ✅ **IMPLEMENTED**: `test-results/TEST/smoke-test.sh` and `smoke-test.ps1` exit with code 0
- [X] CHK019 - Are requirements mapped to artifacts mapped to tests with no gaps? [Truth Gate §4.3]
  - ✅ **IMPLEMENTED**: `test-results/COVERAGE.md` contains requirements mapping
- [X] CHK020 - Are constraints, supported OS/arch, and known failure modes stated? [Truth Gate §4.4]
  - ✅ **IMPLEMENTED**: Constraints documented in spec.md and README.md
- [X] CHK021 - Are SHA-256 hashes provided for key artifacts? [Truth Gate §4.5]
  - ✅ **IMPLEMENTED**: `scripts/bash/generate-hashes.sh` generates HASHES.txt
- [X] CHK022 - If "unbounded" is claimed, is scheduler/executor proof provided? [Truth Gate §4.6]
  - ✅ **IMPLEMENTED**: Truth Gate script checks for unbounded claims and proof
- [X] CHK023 - Is a gap scan checklist completed with coverage confirmed? [Truth Gate §4.7]
  - ✅ **IMPLEMENTED**: `scripts/bash/gap-scan.sh` and `scripts/powershell/gap-scan.ps1` generate gap analysis
- [X] CHK024 - For any N/A check, is the reason documented? [Truth Gate]
  - ✅ **IMPLEMENTED**: Truth Gate script documents N/A reasons
- [X] CHK025 - If any check fails, is the strong claim removed or downgraded? [Truth Gate]
  - ✅ **IMPLEMENTED**: Truth Gate script returns non-zero exit code on failure

---

## Category 3: Triple-Verification Protocol (§5.6)

### Pass A: Self-Check

- [X] CHK026 - Is internal consistency verified (spec ↔ artifacts ↔ tests)? [Triple-Verify §5.6 Pass A]
  - ✅ **IMPLEMENTED**: `scripts/bash/triple-verify.sh` Pass A checks consistency
- [X] CHK027 - Are unit smoke tests passing? [Triple-Verify §5.6 Pass A]
  - ✅ **IMPLEMENTED**: Pass A runs smoke tests
- [X] CHK028 - Are all assertions in spec covered by corresponding test? [Triple-Verify §5.6 Pass A]
  - ✅ **IMPLEMENTED**: Pass A validates coverage mapping

### Pass B: Independent Re-derivation

- [X] CHK029 - Are numbers recomputed and compared with deltas? [Triple-Verify §5.6 Pass B]
  - ✅ **IMPLEMENTED**: Pass B includes metrics recomputation framework
- [X] CHK030 - Is code re-run from fresh state with identical results? [Triple-Verify §5.6 Pass B]
  - ✅ **IMPLEMENTED**: Pass B supports test re-run (requires NOA_TEST_CMD)
- [X] CHK031 - Are results re-generated from raw sources and compared? [Triple-Verify §5.6 Pass B]
  - ✅ **IMPLEMENTED**: Pass B re-generates HASHES.txt and compares

### Pass C: Adversarial Check

- [X] CHK032 - Are negative tests included for failure modes? [Triple-Verify §5.6 Pass C]
  - ✅ **IMPLEMENTED**: Pass C includes negative test framework
- [X] CHK033 - Are boundary cases tested (min, max, empty, null)? [Triple-Verify §5.6 Pass C]
  - ✅ **IMPLEMENTED**: Pass C includes boundary case test framework
- [X] CHK034 - Is cross-tool or cross-model verification performed? [Triple-Verify §5.6 Pass C]
  - ✅ **IMPLEMENTED**: Pass C detects available tools (rustc, node, go, python3)
- [X] CHK035 - Are external citations checked with verification dates? [Triple-Verify §5.6 Pass C]
  - ✅ **IMPLEMENTED**: Pass C scans for URLs in documentation
- [X] CHK036 - Are Pass A/B/C results recorded in Evidence Ledger? [Triple-Verify §5.6]
  - ✅ **IMPLEMENTED**: `test-results/EVIDENCE_LEDGER.md` includes Triple-Verify sections

### Gap Hunt (§5.7)

- [X] CHK037 - Is a missed-item scan run against spec outline? [Gap Hunt §5.7]
  - ✅ **IMPLEMENTED**: `scripts/bash/gap-scan.sh` scans spec outline
- [X] CHK038 - Is a coverage table output with all sections confirmed? [Gap Hunt §5.7]
  - ✅ **IMPLEMENTED**: Gap scan verifies COVERAGE.md completeness
- [X] CHK039 - Are unresolved gaps listed with proposed remedies? [Gap Hunt §5.7]
  - ✅ **IMPLEMENTED**: Gap scan generates GAPS.md with remedies

---

## Category 4: Code Quality Requirements

### Error Handling & Correction

- [ ] CHK040 - Are all error handling paths implemented (not just happy path)? [Code Quality]
- [ ] CHK041 - Do all errors include actionable context (what, why, how to fix)? [Code Quality]
- [ ] CHK042 - Are error codes/types consistent across the codebase? [Code Quality, Consistency]
- [ ] CHK043 - Are retry mechanisms implemented with exponential backoff where appropriate? [Code Quality]
- [ ] CHK044 - Are all external calls wrapped with timeout and fallback? [Code Quality]

### Code Consistency

- [ ] CHK045 - Is naming consistent across files (camelCase, snake_case per language)? [Code Quality]
- [ ] CHK046 - Are all functions documented with purpose, params, return, and errors? [Code Quality]
- [ ] CHK047 - Is code linted with zero warnings (rustfmt, clippy, golangci-lint, eslint, ruff)? [Code Quality]
- [ ] CHK048 - Are magic numbers replaced with named constants? [Code Quality]
- [ ] CHK049 - Is dead code removed (no commented-out blocks, unused imports)? [Code Quality]

### Type Safety & Validation

- [ ] CHK050 - Are all public APIs typed (no `any` in TypeScript, proper generics in Rust)? [Code Quality]
- [ ] CHK051 - Are inputs validated at system boundaries (user input, external APIs)? [Code Quality]
- [ ] CHK052 - Are all nullable values explicitly handled (Option, Result, ?.)? [Code Quality]
- [ ] CHK053 - Are runtime type validations in place for dynamic data (JSON parsing)? [Code Quality]

---

## Category 5: Metadata Quality Requirements

### File & Module Metadata

- [ ] CHK054 - Do all source files have proper header comments (copyright, purpose)? [Metadata]
- [ ] CHK055 - Are version numbers consistent across Cargo.toml, package.json, go.mod? [Metadata]
- [ ] CHK056 - Is `updated_at` timestamp maintained in all state-tracking files? [Metadata]
- [ ] CHK057 - Are author/contributor attributions present where required? [Metadata]

### Schema & Contract Metadata

- [ ] CHK058 - Do all JSON schemas include `$schema` reference? [Metadata, Schema]
- [ ] CHK059 - Do all configs include `version` field for migration tracking? [Metadata]
- [ ] CHK060 - Do all API contracts include version in URL or header? [Metadata]
- [ ] CHK061 - Are deprecation warnings documented with removal dates? [Metadata]

### Traceability Metadata

- [ ] CHK062 - Do all tasks reference their source FR/SC/US? [Traceability]
- [ ] CHK063 - Are all config changes logged with reason and timestamp? [Traceability, Change Control §12]
- [ ] CHK064 - Is every output versioned with delta records? [Change Control §12]
- [ ] CHK065 - Are changelogs maintained for all major files? [Change Control §12]

---

## Category 6: Configuration Standardization

### Config File Structure

- [ ] CHK066 - Do all JSON configs follow the established schema pattern? [Config Quality]
- [ ] CHK067 - Are environment-specific values using `${ENV_VAR}` syntax consistently? [Config Quality]
- [ ] CHK068 - Are config files validated against JSON Schema on load? [Config Quality]
- [ ] CHK069 - Are sensitive values stored in separate, gitignored files? [Config Quality, Security]

### Config Consistency

- [ ] CHK070 - Are path patterns consistent (`noa_root/` vs `${NOA_ROOT}/`)? [Config Consistency]
- [ ] CHK071 - Are boolean configs using consistent naming (`enabled`, not `isEnabled`)? [Config Consistency]
- [ ] CHK072 - Are timeouts/durations using consistent units (always ms or always s)? [Config Consistency]
- [ ] CHK073 - Are priority/order fields using consistent scale (1-10 or low/medium/high)? [Config Consistency]

### Config Documentation

- [ ] CHK074 - Does each config file have an accompanying README or inline comments? [Config Documentation]
- [ ] CHK075 - Are all config options documented with type, default, and purpose? [Config Documentation]
- [ ] CHK076 - Are config migration procedures documented for schema changes? [Config Documentation]

---

## Category 7: Schema Quality Requirements

### JSON Schema Standards

- [ ] CHK077 - Do all schemas use JSON Schema draft-07 or later? [Schema Quality]
- [ ] CHK078 - Are all required fields marked with `required` array? [Schema Quality]
- [ ] CHK079 - Do schemas include `description` for all properties? [Schema Quality]
- [ ] CHK080 - Are enums used for fixed value sets (not free strings)? [Schema Quality]
- [ ] CHK081 - Are numeric ranges constrained with `minimum`/`maximum`? [Schema Quality]

### Schema Validation

- [ ] CHK082 - Do all data files pass schema validation? [Schema Validation]
- [ ] CHK083 - Are schema validation errors actionable (show path, expected, got)? [Schema Validation]
- [ ] CHK084 - Is schema validation performed at startup and on hot reload? [Schema Validation]

### Schema Evolution

- [ ] CHK085 - Are schema versions tracked for migration support? [Schema Evolution]
- [ ] CHK086 - Are backward-compatible changes documented? [Schema Evolution]
- [ ] CHK087 - Are breaking changes gated behind version bumps? [Schema Evolution]

---

## Category 8: Prohibitions Compliance (§6)

### Integrity Prohibitions

- [ ] CHK088 - Is there NO fabricated data, metrics, citations, screenshots, or logs? [Prohibitions §6]
- [ ] CHK089 - Is there NO implied completion without Truth Gate checks? [Prohibitions §6]
- [ ] CHK090 - Is there NO overclaiming beyond test coverage? [Prohibitions §6]
- [ ] CHK091 - Are there NO vague terms ("should", "likely", "best-in-class") without measurable criteria? [Prohibitions §6]
- [ ] CHK092 - Is Triple-Verification Protocol NOT skipped? [Prohibitions §6]
- [ ] CHK093 - Is sensitive data NOT copied to outputs unless explicitly requested? [Prohibitions §6]

---

## Category 9: Fallbacks & Refusals (§7)

### Verification Failures

- [ ] CHK094 - When unable to verify, is "CANNOT VERIFY" returned with missing evidence list? [Fallbacks §7]
- [ ] CHK095 - For conflicting evidence, are both sides presented with conflict explanation? [Fallbacks §7]
- [ ] CHK096 - For spec ambiguity, are options with trade-offs provided? [Fallbacks §7]

---

## Category 10: Standard Output Compliance (§8)

### Claims Table

- [ ] CHK097 - Does CLAIMS TABLE include: #, Claim, Type (weak/strong), Evidence refs, Test/Calc, Limits? [Output Templates §8.A]

### Evidence Ledger

- [ ] CHK098 - Does EVIDENCE LEDGER include: Files with SHA-256, Data source + snapshot time? [Output Templates §8.B]
- [ ] CHK099 - Does EVIDENCE LEDGER include: Web cites (author, title, date, URL)? [Output Templates §8.B]
- [ ] CHK100 - Does EVIDENCE LEDGER include: Math (formulas, inputs, steps)? [Output Templates §8.B]
- [ ] CHK101 - Does EVIDENCE LEDGER include: Tests (commands, logs, exit codes)? [Output Templates §8.B]
- [ ] CHK102 - Does EVIDENCE LEDGER include: Triple-verify Pass A/B/C outcomes? [Output Templates §8.B]

### Truth Gate Checklist

- [ ] CHK103 - Is Truth Gate checklist populated with all 7 items checked? [Output Templates §8.C]

### Result Block

- [ ] CHK104 - Does RESULT block include: RESULT (PASS/PARTIAL/FAIL), WHY, NEXT? [Output Templates §8.D]

---

## Category 11: Numeric Integrity (§10)

- [ ] CHK105 - Is all arithmetic performed digit-by-digit and shown? [Numeric Integrity §10]
- [ ] CHK106 - Is rounding only at the last step? [Numeric Integrity §10]
- [ ] CHK107 - Are precision and units stated for all numbers? [Numeric Integrity §10]

---

## Category 12: Roles & Escalation (§11)

- [ ] CHK108 - Are Analyst, Builder, Verifier roles clearly distinguished in reports? [Roles §11]
- [ ] CHK109 - If one agent holds multiple roles, are sections distinct? [Roles §11]
- [ ] CHK110 - Is the Verifier sign-off or FAIL with reasons present? [Roles §11]

---

## Category 13: Bootstrap Script Quality (NOA-Specific)

### Cross-Platform Parity (FR-088)

- [ ] CHK111 - Does every .ps1 script have a .sh equivalent (or consolidated equivalent)? [FR-088]
- [ ] CHK112 - Do mirrored scripts accept the same arguments? [FR-089]
- [ ] CHK113 - Do mirrored scripts return the same exit codes? [FR-089]
- [ ] CHK114 - Is scripts/README.md updated with cross-platform mapping table? [FR-090]

### Script Standards

- [ ] CHK115 - Do all Bash scripts start with `set -euo pipefail`? [Script Quality]
- [ ] CHK116 - Do all PowerShell scripts use `$ErrorActionPreference = "Stop"`? [Script Quality]
- [ ] CHK117 - Are all external tool calls checked for availability before use? [Script Quality]
- [ ] CHK118 - Are all downloads verified with checksums (SHA-256)? [Script Quality, Security]

### Idempotency

- [ ] CHK119 - Can all scripts be re-run safely without side effects? [Idempotency]
- [ ] CHK120 - Do scripts check for existing installations before installing? [Idempotency]
- [ ] CHK121 - Do scripts preserve user data when updating? [Idempotency]

---

## Category 14: AI Provider Config Quality (NOA-Specific)

### Provider Config Schema

- [X] CHK122 - Do all provider configs include: name, type, priority, enabled, description? [Provider Config]
  - ✅ **IMPLEMENTED**: All 8 provider configs verified to include all required fields
  - ✅ **VALIDATED**: `scripts/bootstrap/verify/validate-provider-configs.ps1` checks all fields
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK123 - Do all provider configs include: cli (command, package, version, binaryPath)? [Provider Config]
  - ✅ **IMPLEMENTED**: All provider configs include CLI structure with command and binaryPath
  - ✅ **VALIDATED**: Validation script handles both flat and nested CLI structures (e.g., llama-cpp)
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK124 - Do all provider configs include: modes (cli, cloud, ide where applicable)? [Provider Config]
  - ✅ **IMPLEMENTED**: All applicable providers include modes (cli, cloud, ide)
  - ✅ **VALIDATED**: Validation script checks for modes field
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK125 - Do all provider configs include: capabilities object? [Provider Config]
  - ✅ **IMPLEMENTED**: All provider configs include capabilities object
  - ✅ **VALIDATED**: Validation script verifies capabilities presence
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK126 - Do all provider configs include: sharedResources paths? [Provider Config]
  - ✅ **IMPLEMENTED**: All provider configs include sharedResources with standard paths
  - ✅ **VALIDATED**: Validation script checks for sharedResources field
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK127 - Do all provider configs include: latency targets and timeout? [Provider Config]
  - ✅ **IMPLEMENTED**: All provider configs include latency.target and timeout fields
  - ✅ **VALIDATED**: Validation script verifies both latency and timeout
  - ✅ **TESTED**: All providers pass validation (2025-01-27)

### Provider Config Consistency

- [X] CHK128 - Are priority values unique across all providers (no duplicates)? [Provider Consistency]
  - ✅ **IMPLEMENTED**: All provider priorities are unique (1-8)
  - ✅ **FIXED**: Resolved duplicate priorities (ollama: 1→2, claude-code: 2→4, cursor: 2→3, etc.)
  - ✅ **VALIDATED**: Validation script checks for duplicate priorities across all providers
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK129 - Are binaryPath values using correct ${NOA_ROOT} syntax? [Provider Consistency]
  - ✅ **IMPLEMENTED**: All binaryPath values use ${NOA_ROOT} syntax
  - ✅ **VALIDATED**: Validation script verifies ${NOA_ROOT} presence in binaryPath
  - ✅ **TESTED**: All providers pass validation (2025-01-27)
- [X] CHK130 - Are sharedResources paths consistent across all providers? [Provider Consistency]
  - ✅ **IMPLEMENTED**: All providers use consistent sharedResources paths with ${NOA_ROOT} syntax
  - ✅ **VALIDATED**: Validation script compares against standard paths and checks consistency
  - ✅ **TESTED**: All providers pass validation (2025-01-27)

---

## Category 15: Success Criteria Verification Quality (Phase 16 - NOA-Specific)

### Benchmark Test Quality

- [ ] CHK131 - Do all SC benchmark tests include hardware specification (RAM, CPU, GPU)? [SC Quality]
- [ ] CHK132 - Do all SC benchmark tests include warm-start vs cold-start distinction? [SC Quality]
- [ ] CHK133 - Are benchmark results recorded with timestamps and environment details? [SC Quality]
- [ ] CHK134 - Do benchmark tests include statistical analysis (mean, median, p95, p99)? [SC Quality]
- [ ] CHK135 - Are benchmark thresholds configurable (not hardcoded)? [SC Quality]

### SC-001 to SC-012 Coverage

- [ ] CHK136 - Is SC-001 (Init <60s) benchmark implemented with standard hardware spec? [SC-001]
- [ ] CHK137 - Is SC-002 (CPU Inference <2s) benchmark implemented with CPU-only hardware? [SC-002]
- [ ] CHK138 - Is SC-003 (Memory Recall <500ms) benchmark implemented with large dataset? [SC-003]
- [ ] CHK139 - Is SC-004 (Digest 10K <30min) benchmark implemented with 10K file repository? [SC-004]
- [ ] CHK140 - Is SC-005 (200 Tasks ≥98%) benchmark implemented with concurrent execution? [SC-005]
- [ ] CHK141 - Is SC-006 (P2P Sync <5s) benchmark implemented with <1MB delta? [SC-006]
- [ ] CHK142 - Is SC-007 (UI Switch <200ms) benchmark implemented with context switch? [SC-007]
- [ ] CHK143 - Is SC-008 (7-Day Continuous) stability test implemented? [SC-008]
- [ ] CHK144 - Is SC-009 (Cross-Platform) consistency test implemented for Windows/macOS/Linux? [SC-009]
- [ ] CHK145 - Is SC-010 (100% Rollback) validation implemented for all self-modifications? [SC-010]
- [ ] CHK146 - Is SC-011 (GPU <500ms) benchmark implemented with single GPU? [SC-011]
- [ ] CHK147 - Is SC-012 (Multi-GPU <300ms) benchmark implemented with tensor parallelism? [SC-012]

### CI Integration Quality

- [ ] CHK148 - Does SC verification workflow run on every release? [CI Integration]
- [ ] CHK149 - Does SC verification workflow fail build if any SC benchmark fails? [CI Integration]
- [ ] CHK150 - Are SC benchmark results published as CI artifacts? [CI Integration]
- [ ] CHK151 - Does SC dashboard show historical trend data? [Dashboard Quality]
- [ ] CHK152 - Does SC dashboard show per-SC compliance status? [Dashboard Quality]

### SC Report Quality

- [ ] CHK153 - Does SC report include all 12 SCs with pass/fail status? [Report Quality]
- [ ] CHK154 - Does SC report include performance metrics (actual vs target)? [Report Quality]
- [ ] CHK155 - Does SC report include hardware configuration used? [Report Quality]
- [ ] CHK156 - Does SC report include recommendations for failed SCs? [Report Quality]

---

## Summary Gate

Before marking ANY task as complete, verify:

- [ ] **TRUTH GATE**: All 7 checks pass or are documented as N/A
- [ ] **TRIPLE VERIFY**: Passes A, B, C completed with results recorded
- [ ] **GAP HUNT**: Coverage table shows 100% or gaps documented with remedies
- [ ] **EVIDENCE LEDGER**: All claims have evidence references
- [ ] **RESULT BLOCK**: PASS/PARTIAL/FAIL with WHY and NEXT

---

## Quick Command Reference

### Smoke Test Skeleton

```bash
set -euo pipefail
echo "Running smoke..."
# Add actual test commands here
echo $? > .exitcode
```

### SHA-256 Listing

```bash
find . -type f ! -path "./.git/*" -print0 | sort -z | xargs -0 sha256sum > HASHES.txt
```

### Coverage Scan Skeleton

```bash
# Map spec requirements to test files
grep -r "FR-" tests/ > COVERAGE.md
```

### Result Block Emitter

```bash
echo "RESULT: ${RESULT:-PARTIAL}"
echo "WHY: $WHY"
echo "NEXT: $NEXT"
```

---

*Checklist generated from Universal Task Execution Policy (§0-§13)*
*Total Items: 156 (130 Base + 26 Phase 16 Success Criteria)*
*Phase 19 Considerations: 18 additional quality checks for Desktop App Hosting (NDCL)*
*Phase 18 Considerations: 10 additional quality checks for Kernel Independence*
*Phase 20 Considerations: 20 additional quality checks for Module Abstraction*

---

## Phase 18 Kernel Independence-Specific Considerations

When applying this quality checklist to **Phase 18: Kernel Independence & Self-Containment (FR-164, FR-165, T835-T858)**, pay special attention to:

### NKAL Trust Boundary (FR-165, T835-T842)
- **CHK040**: All NKAL boundary operations must have error handling (capability denied, invalid input, output validation failure)
- **CHK041**: NKAL errors must include actionable context (which capability was denied, why, how to grant)
- **CHK117**: NKAL capability grants must be verified with checksums (SHA-256) to prevent tampering
- **CHK088**: NKAL capability grants must never be fabricated - all grants must be from actual user/system decisions

### Kernel Mode Switching (FR-164, T839-T842)
- **CHK040**: Mode switch operations must handle all failure modes (checkpoint corruption, state verification failure, shutdown timeout)
- **CHK043**: Mode switch should implement retry with exponential backoff for transient failures
- **CHK062**: All mode switches must reference source FR-164 and task IDs (T839-T842)
- **CHK105-CHK107**: State checkpoint calculations must be shown digit-by-digit with precision

### Self-Containment Documentation (T843-T858)
- **CHK004**: All kernel independence documentation links must be verified (kernel-independence.md, self-containment.md)
- **CHK006**: Kernel independence documentation must be traceable to source FRs (FR-159 to FR-166)
- **CHK013**: Documentation updates must preserve correct prior content (no loss of kernel mode definitions)
- **CHK016**: Kernel independence updates must be propagated consistently across spec.md, kernel-independence.md, and CONSTITUTION.md

### Tool Installation Paths (FR-162, T852)
- **CHK070**: All tool paths must use consistent `${NOA_ROOT}` syntax (not hardcoded `/opt/`)
- **CHK117**: Tool installation scripts must verify checksums (SHA-256) for downloaded archives
- **CHK119**: Tool installation scripts must be idempotent (safe to re-run)

### PATH Precedence (FR-162)
- **CHK070**: PATH manipulation must be consistent across platforms (Windows vs Unix)
- **CHK071**: PATH precedence must use consistent naming (`NOA_BIN_PATH`, not `noaBinPath`)

---

## Phase 19 Desktop App Hosting-Specific Considerations

When applying this quality checklist to **Phase 19: Desktop App Hosting (NDCL) (FR-167 to FR-174)**, pay special attention to:

### Data Isolation & Containment (FR-169, §3.1)
- **CHK040**: All environment redirection must handle edge cases (missing directories, permission errors, symlink failures)
- **CHK041**: Error messages must clearly indicate when data isolation fails and why (e.g., "Failed to redirect APPDATA: permission denied")
- **CHK050**: All launcher wrappers must validate environment variables before launching apps
- **CHK117**: All file system operations must check for path traversal attempts (prevent `../` escapes from noa_root)

### Network Isolation & Proxy (FR-170, §3.6)
- **CHK044**: Network proxy must have timeout and fallback for proxy failures (graceful degradation)
- **CHK088**: Proxy logs must never be fabricated - all entries must be from actual network traffic
- **CHK118**: All proxy certificates must be verified with checksums (SHA-256) for HTTPS inspection
- **CHK119**: Proxy configuration must be idempotent (safe to regenerate multiple times)

### OAuth Proxy & Security (FR-171, §3.6)
- **CHK069**: OAuth tokens must be stored in separate, gitignored encrypted files (`data/secrets/desktop-tokens.enc`)
- **CHK088**: OAuth token storage must never log tokens in plaintext (only encrypted storage)
- **CHK093**: Sensitive OAuth data must not be copied to outputs unless explicitly requested
- **CHK118**: Token encryption must use verified algorithms (e.g., AES-256-GCM) with proper key management

### Cross-Platform Parity (FR-167, FR-088)
- **CHK111**: All Windows launcher wrappers (`.cmd`) must have Unix equivalents (shell scripts)
- **CHK112**: Windows and Unix launchers must accept the same arguments
- **CHK113**: Windows and Unix launchers must return the same exit codes for equivalent operations
- **CHK114**: Cross-platform mapping table must be documented in `docs/architecture/desktop-app-hosting.md`

### Display Forwarding (FR-172)
- **CHK040**: Display forwarding must handle failures gracefully (fallback to software rendering if GPU unavailable)
- **CHK043**: Display forwarding should implement retry with exponential backoff for transient connection failures
- **CHK117**: All display forwarding scripts must check for required tools (X11, Wayland, RDP, VNC) before use

### IDE Containment Exceptions (FR-174)
- **CHK006**: IDE containment exceptions must be cross-referenced to architectural documentation
- **CHK020**: IDE containment constraints must be documented (what is allowed, what is blocked)
- **CHK062**: All IDE containment tasks must reference source FR-174 and task IDs (T859-T881)

---

## Phase 15 Governance-Specific Considerations

When applying this quality checklist to **Phase 15: Governance & Safety (FR-025 to FR-028)**, pay special attention to:

### Audit Trail Integrity (FR-025)
- **CHK088**: Audit logs must never be fabricated - all entries must be generated from actual system decisions
- **CHK117**: Audit log checksums must be verified to ensure append-only integrity
- **CHK138**: Audit logs must be tamper-proof (see verification.md GOV007)

### Biblical Text Sources (FR-026)
- **CHK004**: All biblical text source links must be verified (NA28, UBS5, SBLGNT, BHS, WLC licenses)
- **CHK093**: Licensed biblical texts must be handled according to license terms (academic/research use)
- **CHK006**: Biblical text transformation pipeline must be traceable to source texts

### Reward/Correction Mechanisms (FR-027)
- **CHK105-CHK107**: Compliance score calculations must be shown digit-by-digit with precision
- **CHK040**: All correction thresholds must have error handling (testing loop, retraining, quarantine)

### Self-Modification Rollback (FR-028)
- **CHK040**: Rollback operations must handle all failure modes (corrupted snapshots, missing checkpoints)
- **CHK043**: Rollback should implement retry with exponential backoff for transient failures
- **CHK062**: All rollback operations must reference source FR-028 and task IDs (T703-T705)

---

## Phase 20 Module Abstraction-Specific Considerations

When applying this quality checklist to **Phase 20: Module Abstraction (FR-176 to FR-180, T882-T893)**, pay special attention to:

### Module Registry (FR-176, FR-177, T882-T884)
- **CHK040**: Module registration must handle all error cases (duplicate name, invalid metadata, schema validation failure)
- **CHK041**: Registry errors must include actionable context (which field failed, expected format, how to fix)
- **CHK050**: Module type enum must be strongly typed (no string-based type matching)
- **CHK051**: Module metadata must be validated at registration boundary (JSON schema validation)
- **CHK062**: All module operations must reference source FR-176/FR-177 and task IDs (T882-T884)
- **CHK117**: Module registry database must use checksums (SHA-256) for integrity verification

### Content-Addressable Storage (CAS) (FR-178, T885-T887)
- **CHK040**: CAS operations must handle all failure modes (disk full, hash collision, shard directory creation failure)
- **CHK041**: CAS errors must include actionable context (which hash failed, storage path, available space)
- **CHK105-CHK107**: SHA-256 hash calculations must be shown digit-by-digit with precision
- **CHK117**: CAS content must be verified with SHA-256 checksums on every read
- **CHK119**: CAS operations must be idempotent (storing same content twice produces same result)
- **CHK088**: CAS hashes must never be fabricated - all hashes must be computed from actual content
- **CHK070**: CAS paths must use consistent `${NOA_ROOT}` syntax (not hardcoded paths)

### Module Lifecycle (FR-179, T888-T890)
- **CHK040**: Lifecycle transitions must handle all failure modes (invalid state, missing module, verification failure)
- **CHK041**: Lifecycle errors must include actionable context (current state, target state, why transition failed)
- **CHK050**: Lifecycle state machine must be strongly typed (enum, not string-based)
- **CHK052**: All state transitions must explicitly handle None/Error cases (Option, Result types)
- **CHK043**: Hot-reload operations should implement retry with exponential backoff for transient failures
- **CHK062**: All lifecycle operations must reference source FR-179 and task IDs (T888-T890)

### Dependency Resolution (FR-180, T891-T893)
- **CHK040**: Dependency resolution must handle all failure modes (conflict, missing dependency, circular dependency)
- **CHK041**: Resolution errors must include actionable context (which dependencies conflict, version constraints, resolution path)
- **CHK105-CHK107**: Semver constraint solving must be shown step-by-step with precision
- **CHK050**: Dependency graph must be strongly typed (no string-based dependency matching)
- **CHK051**: Semver constraints must be validated at resolution boundary (semver spec compliance)
- **CHK062**: All dependency operations must reference source FR-180 and task IDs (T891-T893)

### Module Immutability & Versioning (FR-176)
- **CHK088**: Module content must never be modified after registration - all modifications must create new version
- **CHK117**: Module content integrity must be verified with SHA-256 on every access
- **CHK040**: Attempts to modify registered modules must return clear error (immutability violation)
- **CHK041**: Immutability errors must include actionable context (module ID, version, how to create new version)

### Constitutional Compliance (FR-176, §3.1, §3.5)
- **CHK070**: All module paths must resolve under `${NOA_ROOT}` (constitutional requirement §3.1)
- **CHK062**: All module operations must be auditable (constitutional requirement §3.5)
- **CHK117**: Module audit trail must include checksums for tamper detection
- **CHK088**: Module audit entries must never be fabricated - all entries from actual operations

