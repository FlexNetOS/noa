# Quality & Verification Checklist: NOA Seed Foundation

**Purpose**: Universal Task Execution Policy compliance and code quality gates
**Created**: 2025-12-09
**Type**: Quality Assurance / Requirements Validation / Error Correction
**Based On**: Universal Task Execution Policy (§0-§13)
**Coverage**: Evidence Rules, Truth Gate, Triple-Verification, Code Quality, Metadata, configs Schema

---

## How to Use This Checklist

1. **Pre-Commit**: Run CHK001-CHK025 (Evidence & Documentation)
2. **Pre-PR**: Run CHK026-CHK050 (Truth Gate & Verification)
3. **Code Review**: Run CHK051-CHK075 (Code Quality & Consistency)
4. **Pre-Merge**: Run CHK076-CHK100 (configs & Schema Validation)
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

- [ ] CHK008 - Is a FINAL_REPORT.md created with claims table, evidence ledger, gate checklist? [Execution Artifacts §9]
- [ ] CHK009 - Does TEST/ directory contain scripts, fixtures, and expected outputs? [Execution Artifacts §9]
- [ ] CHK010 - Is HASHES.txt generated with SHA-256 for all key files? [Execution Artifacts §9]
- [ ] CHK011 - Does REPRO.md specify exact environment and commands for reproduction? [Execution Artifacts §9]
- [ ] CHK012 - Does COVERAGE.md map requirements to artifacts with open gaps noted? [Execution Artifacts §9]

### Update Semantics (Heal, Do Not Harm §0.1)

- [ ] CHK013 - Do updates preserve correct prior content without regressions? [Update Semantics §0.1]
- [ ] CHK014 - Are fine-grained details preserved (no lossy summarization)? [Update Semantics §0.1]
- [ ] CHK015 - Does any removal have a stated reason and replacement/mitigation? [Update Semantics §0.1]
- [ ] CHK016 - Are updates propagated consistently across specs, code, tests, and docs? [Update Semantics §0.1]

---

## Category 2: Truth Gate Requirements (§4)

### Strong Claim Validation (Built/Ready/Delivered/Verified/Unbounded)

- [ ] CHK017 - Are all referenced files verified to exist in export or repo? [Truth Gate §4.1]
- [ ] CHK018 - Is a deterministic smoke test provided with command, transcript, and exit code 0? [Truth Gate §4.2]
- [ ] CHK019 - Are requirements mapped to artifacts mapped to tests with no gaps? [Truth Gate §4.3]
- [ ] CHK020 - Are constraints, supported OS/arch, and known failure modes stated? [Truth Gate §4.4]
- [ ] CHK021 - Are SHA-256 hashes provided for key artifacts? [Truth Gate §4.5]
- [ ] CHK022 - If "unbounded" is claimed, is scheduler/executor proof provided? [Truth Gate §4.6]
- [ ] CHK023 - Is a gap scan checklist completed with coverage confirmed? [Truth Gate §4.7]
- [ ] CHK024 - For any N/A check, is the reason documented? [Truth Gate]
- [ ] CHK025 - If any check fails, is the strong claim removed or downgraded? [Truth Gate]

---

## Category 3: Triple-Verification Protocol (§5.6)

### Pass A: Self-Check

- [ ] CHK026 - Is internal consistency verified (spec ↔ artifacts ↔ tests)? [Triple-Verify §5.6 Pass A]
- [ ] CHK027 - Are unit smoke tests passing? [Triple-Verify §5.6 Pass A]
- [ ] CHK028 - Are all assertions in spec covered by corresponding test? [Triple-Verify §5.6 Pass A]

### Pass B: Independent Re-derivation

- [ ] CHK029 - Are numbers recomputed and compared with deltas? [Triple-Verify §5.6 Pass B]
- [ ] CHK030 - Is code re-run from fresh state with identical results? [Triple-Verify §5.6 Pass B]
- [ ] CHK031 - Are results re-generated from raw sources and compared? [Triple-Verify §5.6 Pass B]

### Pass C: Adversarial Check

- [ ] CHK032 - Are negative tests included for failure modes? [Triple-Verify §5.6 Pass C]
- [ ] CHK033 - Are boundary cases tested (min, max, empty, null)? [Triple-Verify §5.6 Pass C]
- [ ] CHK034 - Is cross-tool or cross-model verification performed? [Triple-Verify §5.6 Pass C]
- [ ] CHK035 - Are external citations checked with verification dates? [Triple-Verify §5.6 Pass C]
- [ ] CHK036 - Are Pass A/B/C results recorded in Evidence Ledger? [Triple-Verify §5.6]

### Gap Hunt (§5.7)

- [ ] CHK037 - Is a missed-item scan run against spec outline? [Gap Hunt §5.7]
- [ ] CHK038 - Is a coverage table output with all sections confirmed? [Gap Hunt §5.7]
- [ ] CHK039 - Are unresolved gaps listed with proposed remedies? [Gap Hunt §5.7]

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
- [ ] CHK059 - Do all configss include `version` field for migration tracking? [Metadata]
- [ ] CHK060 - Do all API contracts include version in URL or header? [Metadata]
- [ ] CHK061 - Are deprecation warnings documented with removal dates? [Metadata]

### Traceability Metadata

- [ ] CHK062 - Do all tasks reference their source FR/SC/US? [Traceability]
- [ ] CHK063 - Are all configs changes logged with reason and timestamp? [Traceability, Change Control §12]
- [ ] CHK064 - Is every output versioned with delta records? [Change Control §12]
- [ ] CHK065 - Are changelogs maintained for all major files? [Change Control §12]

---

## Category 6: configsuration Standardization

### configs File Structure

- [ ] CHK066 - Do all JSON configss follow the established schema pattern? [configs Quality]
- [ ] CHK067 - Are environment-specific values using `${ENV_VAR}` syntax consistently? [configs Quality]
- [ ] CHK068 - Are configs files validated against JSON Schema on load? [configs Quality]
- [ ] CHK069 - Are sensitive values stored in separate, gitignored files? [configs Quality, Security]

### configs Consistency

- [ ] CHK070 - Are path patterns consistent (`noa_root/` vs `${NOA_ROOT}/`)? [configs Consistency]
- [ ] CHK071 - Are boolean configss using consistent naming (`enabled`, not `isEnabled`)? [configs Consistency]
- [ ] CHK072 - Are timeouts/durations using consistent units (always ms or always s)? [configs Consistency]
- [ ] CHK073 - Are priority/order fields using consistent scale (1-10 or low/medium/high)? [configs Consistency]

### configs Documentation

- [ ] CHK074 - Does each configs file have an accompanying README or inline comments? [configs Documentation]
- [ ] CHK075 - Are all configs options documented with type, default, and purpose? [configs Documentation]
- [ ] CHK076 - Are configs migration procedures documented for schema changes? [configs Documentation]

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

## Category 14: AI Provider configs Quality (NOA-Specific)

### Provider configs Schema

- [ ] CHK122 - Do all provider configss include: name, type, priority, enabled, description? [Provider configs]
- [ ] CHK123 - Do all provider configss include: cli (command, package, version, binaryPath)? [Provider configs]
- [ ] CHK124 - Do all provider configss include: modes (cli, cloud, ide where applicable)? [Provider configs]
- [ ] CHK125 - Do all provider configss include: capabilities object? [Provider configs]
- [ ] CHK126 - Do all provider configss include: sharedResources paths? [Provider configs]
- [ ] CHK127 - Do all provider configss include: latency targets and timeout? [Provider configs]

### Provider configs Consistency

- [ ] CHK128 - Are priority values unique across all providers (no duplicates)? [Provider Consistency]
- [ ] CHK129 - Are binaryPath values using correct ${NOA_ROOT} syntax? [Provider Consistency]
- [ ] CHK130 - Are sharedResources paths consistent across all providers? [Provider Consistency]

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
*Total Items: 130*

