# Phase 17 Quality Checklist: New Requirements

**Purpose**: Universal Task Execution Policy compliance and code quality gates for Phase 17
**Created**: 2025-01-27
**Type**: Quality Assurance / Requirements Validation / Error Correction
**Based On**: Universal Task Execution Policy (§0-§13)
**Coverage**: FR-095 to FR-158 (64 Functional Requirements)
**Phase**: Phase 17 - New Requirements

---

## How to Use This Checklist

1. **Pre-Commit**: Run P17CHK001-P17CHK025 (Evidence & Documentation)
2. **Pre-PR**: Run P17CHK026-P17CHK050 (Truth Gate & Verification)
3. **Code Review**: Run P17CHK051-P17CHK075 (Code Quality & Consistency)
4. **Pre-Merge**: Run P17CHK076-P17CHK100 (Config & Schema Validation)
5. **Release Gate**: Run P17CHK101-P17CHK130 (Final Verification & Audit)

---

## Category 1: Evidence & Documentation Requirements (§3)

### Citation & Source Requirements

- [ ] P17CHK001 - Are all Phase 17 claims derivable from user artifacts or shown math? If not, is explicit "no evidence" label applied? [Evidence Rules §3]
- [ ] P17CHK002 - Do all time-sensitive facts include source dates? [Evidence Rules §3]
- [ ] P17CHK003 - Are all mathematical calculations shown digit-by-digit with formulae and assumptions? [Evidence Rules §3]
  - **Applies to**: Rate limiting calculations, exponential backoff math, token refresh timing
- [ ] P17CHK004 - Are all links verified as real (not fabricated)? If unavailable, is "link unavailable" stated? [Evidence Rules §3]
- [ ] P17CHK005 - Do code examples include seed values, exact commands, environment versions? [Evidence Rules §3, Repro]
  - **Applies to**: Pairing flows, upgrade procedures, recovery operations
- [ ] P17CHK006 - Is every claim cross-referenced to its source with explicit mapping? [Evidence Rules §3]
  - **Applies to**: All FR-095 to FR-158 requirements
- [ ] P17CHK007 - Are claims without source or test coverage explicitly flagged? [Evidence Rules §3]

### Documentation Completeness

- [ ] P17CHK008 - Is a FINAL_REPORT.md created with claims table, evidence ledger, gate checklist? [Execution Artifacts §9]
- [ ] P17CHK009 - Does TEST/ directory contain scripts, fixtures, and expected outputs? [Execution Artifacts §9]
- [ ] P17CHK010 - Is HASHES.txt generated with SHA-256 for all key files? [Execution Artifacts §9]
- [ ] P17CHK011 - Does REPRO.md specify exact environment and commands for reproduction? [Execution Artifacts §9]
- [ ] P17CHK012 - Does COVERAGE.md map requirements to artifacts with open gaps noted? [Execution Artifacts §9]

### Update Semantics (Heal, Do Not Harm §0.1)

- [ ] P17CHK013 - Do updates preserve correct prior content without regressions? [Update Semantics §0.1]
- [ ] P17CHK014 - Are fine-grained details preserved (no lossy summarization)? [Update Semantics §0.1]
- [ ] P17CHK015 - Does any removal have a stated reason and replacement/mitigation? [Update Semantics §0.1]
- [ ] P17CHK016 - Are updates propagated consistently across specs, code, tests, and docs? [Update Semantics §0.1]

---

## Category 2: Truth Gate Requirements (§4)

### Strong Claim Validation

- [ ] P17CHK017 - Are all referenced files verified to exist in export or repo? [Truth Gate §4.1]
  - **Applies to**: All Phase 17 implementation files (T771-T826)
- [ ] P17CHK018 - Is a deterministic smoke test provided with command, transcript, and exit code 0? [Truth Gate §4.2]
  - **Applies to**: Each Phase 17 feature area (Rate Limiting, Auth, A11y, etc.)
- [ ] P17CHK019 - Are requirements mapped to artifacts mapped to tests with no gaps? [Truth Gate §4.3]
  - **Applies to**: FR-095 to FR-158 → T771-T826 → P17VER001-P17VER062
- [ ] P17CHK020 - Are constraints, supported OS/arch, and known failure modes stated? [Truth Gate §4.4]
  - **Applies to**: Pairing methods (Bluetooth/NFC hardware requirements), multi-modal (camera/mic availability)
- [ ] P17CHK021 - Are SHA-256 hashes provided for key artifacts? [Truth Gate §4.5]
- [ ] P17CHK022 - If "unbounded" is claimed, is scheduler/executor proof provided? [Truth Gate §4.6]
- [ ] P17CHK023 - Is a gap scan checklist completed with coverage confirmed? [Truth Gate §4.7]
- [ ] P17CHK024 - For any N/A check, is the reason documented? [Truth Gate]
- [ ] P17CHK025 - If any check fails, is the strong claim removed or downgraded? [Truth Gate]

---

## Category 3: Triple-Verification Protocol (§5.6)

### Pass A: Self-Check

- [ ] P17CHK026 - Is internal consistency verified (spec ↔ artifacts ↔ tests)? [Triple-Verify §5.6 Pass A]
- [ ] P17CHK027 - Are unit smoke tests passing? [Triple-Verify §5.6 Pass A]
- [ ] P17CHK028 - Are all assertions in spec covered by corresponding test? [Triple-Verify §5.6 Pass A]

### Pass B: Independent Re-derivation

- [ ] P17CHK029 - Are numbers recomputed and compared with deltas? [Triple-Verify §5.6 Pass B]
  - **Applies to**: Rate limits (10/hour), backoff timings (1s-60s), latency targets (<500ms)
- [ ] P17CHK030 - Is code re-run from fresh state with identical results? [Triple-Verify §5.6 Pass B]
- [ ] P17CHK031 - Are results re-generated from raw sources and compared? [Triple-Verify §5.6 Pass B]

### Pass C: Adversarial Check

- [ ] P17CHK032 - Are negative tests included for failure modes? [Triple-Verify §5.6 Pass C]
  - **Applies to**: Pairing failures, token refresh failures, OOM scenarios, corruption recovery
- [ ] P17CHK033 - Are boundary cases tested (min, max, empty, null)? [Triple-Verify §5.6 Pass C]
  - **Applies to**: Rate limits (0, max), backoff (1s min, 60s max), token expiry (0, max)
- [ ] P17CHK034 - Is cross-tool or cross-model verification performed? [Triple-Verify §5.6 Pass C]
- [ ] P17CHK035 - Are external citations checked with verification dates? [Triple-Verify §5.6 Pass C]
  - **Applies to**: WCAG 2.1 AAA, Ed25519, Argon2id, OpenTelemetry specs
- [ ] P17CHK036 - Are Pass A/B/C results recorded in Evidence Ledger? [Triple-Verify §5.6]

### Gap Hunt (§5.7)

- [ ] P17CHK037 - Is a missed-item scan run against spec outline? [Gap Hunt §5.7]
- [ ] P17CHK038 - Is a coverage table output with all sections confirmed? [Gap Hunt §5.7]
- [ ] P17CHK039 - Are unresolved gaps listed with proposed remedies? [Gap Hunt §5.7]

---

## Category 4: Code Quality Requirements

### Error Handling & Correction

- [ ] P17CHK040 - Are all error handling paths implemented (not just happy path)? [Code Quality]
  - **Applies to**: Pairing failures, token refresh failures, OOM, corruption, upgrade failures
- [ ] P17CHK041 - Do all errors include actionable context (what, why, how to fix)? [Code Quality]
- [ ] P17CHK042 - Are error codes/types consistent across the codebase? [Code Quality, Consistency]
- [ ] P17CHK043 - Are retry mechanisms implemented with exponential backoff where appropriate? [Code Quality]
  - **Applies to**: Token refresh (T818), download resume (T815), pairing retries
- [ ] P17CHK044 - Are all external calls wrapped with timeout and fallback? [Code Quality]
  - **Applies to**: OTLP export, OAuth token refresh, P2P connections

### Code Consistency

- [ ] P17CHK045 - Is naming consistent across files (camelCase, snake_case per language)? [Code Quality]
  - **Applies to**: Rust (snake_case), TypeScript (camelCase), Go (camelCase)
- [ ] P17CHK046 - Are all functions documented with purpose, params, return, and errors? [Code Quality]
- [ ] P17CHK047 - Is code linted with zero warnings (rustfmt, clippy, golangci-lint, eslint, ruff)? [Code Quality]
- [ ] P17CHK048 - Are magic numbers replaced with named constants? [Code Quality]
  - **Applies to**: Rate limits (10/hour), backoff (1s, 60s, 2x), expiry (5 min), contrast (7:1)
- [ ] P17CHK049 - Is dead code removed (no commented-out blocks, unused imports)? [Code Quality]

### Type Safety & Validation

- [ ] P17CHK050 - Are all public APIs typed (no `any` in TypeScript, proper generics in Rust)? [Code Quality]
- [ ] P17CHK051 - Are inputs validated at system boundaries (user input, external APIs)? [Code Quality]
  - **Applies to**: PIN input (6 digits), QR code tokens, pairing files, upgrade versions
- [ ] P17CHK052 - Are all nullable values explicitly handled (Option, Result, ?.)? [Code Quality]
- [ ] P17CHK053 - Are runtime type validations in place for dynamic data (JSON parsing)? [Code Quality]
  - **Applies to**: Feature flags config, i18n locale files, pairing tokens

---

## Category 5: Metadata Quality Requirements

### File & Module Metadata

- [ ] P17CHK054 - Do all source files have proper header comments (copyright, purpose)? [Metadata]
- [ ] P17CHK055 - Are version numbers consistent across Cargo.toml, package.json, go.mod? [Metadata]
- [ ] P17CHK056 - Is `updated_at` timestamp maintained in all state-tracking files? [Metadata]
  - **Applies to**: Feature flags state, pairing tokens, upgrade history, metrics store
- [ ] P17CHK057 - Are author/contributor attributions present where required? [Metadata]

### Schema & Contract Metadata

- [ ] P17CHK058 - Do all JSON schemas include `$schema` reference? [Metadata, Schema]
  - **Applies to**: Feature flags schema (T739d), i18n locale files
- [ ] P17CHK059 - Do all configs include `version` field for migration tracking? [Metadata]
  - **Applies to**: Feature flags config, upgrade detector
- [ ] P17CHK060 - Do all API contracts include version in URL or header? [Metadata]
  - **Applies to**: OTLP export, metrics endpoint, pairing APIs
- [ ] P17CHK061 - Are deprecation warnings documented with removal dates? [Metadata]

### Traceability Metadata

- [ ] P17CHK062 - Do all tasks reference their source FR/SC/US? [Traceability]
  - **Applies to**: All T771-T826 tasks must reference FR-095 to FR-158
- [ ] P17CHK063 - Are all config changes logged with reason and timestamp? [Traceability, Change Control §12]
  - **Applies to**: Feature flag changes (T739c), upgrade operations
- [ ] P17CHK064 - Is every output versioned with delta records? [Change Control §12]
- [ ] P17CHK065 - Are changelogs maintained for all major files? [Change Control §12]

---

## Category 6: Configuration Standardization

### Config File Structure

- [ ] P17CHK066 - Do all JSON configs follow the established schema pattern? [Config Quality]
  - **Applies to**: Feature flags config, i18n locale files
- [ ] P17CHK067 - Are environment-specific values using `${ENV_VAR}` syntax consistently? [Config Quality]
- [ ] P17CHK068 - Are config files validated against JSON Schema on load? [Config Quality]
  - **Applies to**: Feature flags (T739d), i18n locales
- [ ] P17CHK069 - Are sensitive values stored in separate, gitignored files? [Config Quality, Security]
  - **Applies to**: Device keys, pairing tokens, OAuth tokens

### Config Consistency

- [ ] P17CHK070 - Are path patterns consistent (`noa_root/` vs `${NOA_ROOT}/`)? [Config Consistency]
- [ ] P17CHK071 - Are boolean configs using consistent naming (`enabled`, not `isEnabled`)? [Config Consistency]
  - **Applies to**: Feature flags, privacy controls
- [ ] P17CHK072 - Are timeouts/durations using consistent units (always ms or always s)? [Config Consistency]
  - **Applies to**: Rate limits, backoff timings, token expiry, pairing expiry
- [ ] P17CHK073 - Are priority/order fields using consistent scale (1-10 or low/medium/high)? [Config Consistency]
  - **Applies to**: Feature flag scopes, pairing method priority

### Config Documentation

- [ ] P17CHK074 - Does each config file have an accompanying README or inline comments? [Config Documentation]
- [ ] P17CHK075 - Are all config options documented with type, default, and purpose? [Config Documentation]
- [ ] P17CHK076 - Are config migration procedures documented for schema changes? [Config Documentation]
  - **Applies to**: Feature flags schema evolution, upgrade migrations

---

## Category 7: Schema Quality Requirements

### JSON Schema Standards

- [ ] P17CHK077 - Do all schemas use JSON Schema draft-07 or later? [Schema Quality]
  - **Applies to**: Feature flags schema (T739d)
- [ ] P17CHK078 - Are all required fields marked with `required` array? [Schema Quality]
- [ ] P17CHK079 - Do schemas include `description` for all properties? [Schema Quality]
- [ ] P17CHK080 - Are enums used for fixed value sets (not free strings)? [Schema Quality]
  - **Applies to**: Feature flag scopes (global, device, user, session), pairing methods
- [ ] P17CHK081 - Are numeric ranges constrained with `minimum`/`maximum`? [Schema Quality]
  - **Applies to**: Rate limits, backoff timings, token expiry

### Schema Validation

- [ ] P17CHK082 - Do all data files pass schema validation? [Schema Validation]
  - **Applies to**: Feature flags config, i18n locale files
- [ ] P17CHK083 - Are schema validation errors actionable (show path, expected, got)? [Schema Validation]
- [ ] P17CHK084 - Is schema validation performed at startup and on hot reload? [Schema Validation]
  - **Applies to**: Feature flags runtime reload (T739a)

### Schema Evolution

- [ ] P17CHK085 - Are schema versions tracked for migration support? [Schema Evolution]
  - **Applies to**: Feature flags schema, upgrade detector
- [ ] P17CHK086 - Are backward-compatible changes documented? [Schema Evolution]
- [ ] P17CHK087 - Are breaking changes gated behind version bumps? [Schema Evolution]

---

## Category 8: Prohibitions Compliance (§6)

### Integrity Prohibitions

- [ ] P17CHK088 - Is there NO fabricated data, metrics, citations, screenshots, or logs? [Prohibitions §6]
  - **Applies to**: Audit logs (flag changes), metrics data, pairing tokens
- [ ] P17CHK089 - Is there NO implied completion without Truth Gate checks? [Prohibitions §6]
- [ ] P17CHK090 - Is there NO overclaiming beyond test coverage? [Prohibitions §6]
- [ ] P17CHK091 - Are there NO vague terms ("should", "likely", "best-in-class") without measurable criteria? [Prohibitions §6]
- [ ] P17CHK092 - Is Triple-Verification Protocol NOT skipped? [Prohibitions §6]
- [ ] P17CHK093 - Is sensitive data NOT copied to outputs unless explicitly requested? [Prohibitions §6]
  - **Applies to**: Device keys, pairing tokens, OAuth tokens

---

## Category 9: Fallbacks & Refusals (§7)

### Verification Failures

- [ ] P17CHK094 - When unable to verify, is "CANNOT VERIFY" returned with missing evidence list? [Fallbacks §7]
- [ ] P17CHK095 - For conflicting evidence, are both sides presented with conflict explanation? [Fallbacks §7]
  - **Applies to**: Executive Agent conflict resolution (T819)
- [ ] P17CHK096 - For spec ambiguity, are options with trade-offs provided? [Fallbacks §7]

---

## Category 10: Standard Output Compliance (§8)

### Claims Table

- [ ] P17CHK097 - Does CLAIMS TABLE include: #, Claim, Type (weak/strong), Evidence refs, Test/Calc, Limits? [Output Templates §8.A]

### Evidence Ledger

- [ ] P17CHK098 - Does EVIDENCE LEDGER include: Files with SHA-256, Data source + snapshot time? [Output Templates §8.B]
- [ ] P17CHK099 - Does EVIDENCE LEDGER include: Web cites (author, title, date, URL)? [Output Templates §8.B]
  - **Applies to**: WCAG 2.1, Ed25519, Argon2id, OpenTelemetry specs
- [ ] P17CHK100 - Does EVIDENCE LEDGER include: Math (formulas, inputs, steps)? [Output Templates §8.B]
  - **Applies to**: Exponential backoff calculation, rate limit math, token refresh timing
- [ ] P17CHK101 - Does EVIDENCE LEDGER include: Tests (commands, logs, exit codes)? [Output Templates §8.B]
- [ ] P17CHK102 - Does EVIDENCE LEDGER include: Triple-verify Pass A/B/C outcomes? [Output Templates §8.B]

### Truth Gate Checklist

- [ ] P17CHK103 - Is Truth Gate checklist populated with all 7 items checked? [Output Templates §8.C]

### Result Block

- [ ] P17CHK104 - Does RESULT block include: RESULT (PASS/PARTIAL/FAIL), WHY, NEXT? [Output Templates §8.D]

---

## Category 11: Numeric Integrity (§10)

- [ ] P17CHK105 - Is all arithmetic performed digit-by-digit and shown? [Numeric Integrity §10]
  - **Applies to**: Rate limit calculations, backoff math, token timing
- [ ] P17CHK106 - Is rounding only at the last step? [Numeric Integrity §10]
- [ ] P17CHK107 - Are precision and units stated for all numbers? [Numeric Integrity §10]
  - **Applies to**: Rate limits (10/hour), backoff (1s-60s), latency (<500ms), expiry (5 min), contrast (7:1)

---

## Category 12: Roles & Escalation (§11)

- [ ] P17CHK108 - Are Analyst, Builder, Verifier roles clearly distinguished in reports? [Roles §11]
- [ ] P17CHK109 - If one agent holds multiple roles, are sections distinct? [Roles §11]
- [ ] P17CHK110 - Is the Verifier sign-off or FAIL with reasons present? [Roles §11]

---

## Category 13: Phase 17-Specific Quality Checks

### Rate Limiting Quality

- [ ] P17CHK111 - Are rate limits configurable per provider type? [Rate Limiting, FR-095]
- [ ] P17CHK112 - Is exponential backoff math verified (1s init, 60s max, 2x factor)? [Rate Limiting, FR-096]
- [ ] P17CHK113 - Is P2P throttling based on actual peer capacity metrics? [Rate Limiting, FR-097]
- [ ] P17CHK114 - Is self-generated goal rate limit enforced (max 10/hour)? [Rate Limiting, FR-098]

### Authentication Quality

- [ ] P17CHK115 - Are Ed25519 keys generated with secure randomness? [Auth, FR-100]
- [ ] P17CHK116 - Are Argon2id parameters secure (memory, iterations, parallelism)? [Auth, FR-101]
- [ ] P17CHK117 - Are pairing tokens cryptographically secure with proper expiry? [Auth, FR-102-105]
- [ ] P17CHK118 - Is mTLS implemented with proper certificate validation? [Auth, FR-106]
- [ ] P17CHK119 - Are device keys stored securely (encrypted at rest)? [Auth, FR-100-101]

### Accessibility Quality

- [ ] P17CHK120 - Does WCAG 2.1 AAA audit cover all UI components? [A11y, FR-110]
- [ ] P17CHK121 - Are focus indicators verified to have ≥7:1 contrast ratio? [A11y, FR-111]
- [ ] P17CHK122 - Are all interactive elements accessible via keyboard? [A11y, FR-111]
- [ ] P17CHK123 - Are ARIA labels tested with screen readers? [A11y, FR-112]

### Internationalization Quality

- [ ] P17CHK124 - Are all UI strings externalized (no hardcoded text)? [i18n, FR-114]
- [ ] P17CHK125 - Are translations complete for all 5 bundled languages? [i18n, FR-119]
- [ ] P17CHK126 - Is RTL layout tested with Arabic and Hebrew? [i18n, FR-116]
- [ ] P17CHK127 - Does locale switching work without page reload? [i18n, FR-117]

### Multi-Modal Quality

- [ ] P17CHK128 - Is STT latency verified <500ms on standard hardware? [Multi-Modal, FR-128]
- [ ] P17CHK129 - Are privacy controls enforced (camera/mic permissions)? [Multi-Modal, FR-135]
- [ ] P17CHK130 - Is graceful degradation tested when hardware unavailable? [Multi-Modal, FR-133]

### Observability Quality

- [ ] P17CHK131 - Are traces exported correctly to OTLP endpoint? [Observability, FR-155]
- [ ] P17CHK132 - Are metrics exposed in Prometheus format? [Observability, FR-154]
- [ ] P17CHK133 - Is metrics store retention policy enforced (7 days)? [Observability, FR-156]
- [ ] P17CHK134 - Does built-in dashboard work without external Grafana? [Observability, FR-158]

---

## Summary Gate

Before marking ANY Phase 17 task as complete, verify:

- [ ] **TRUTH GATE**: All 7 checks pass or are documented as N/A
- [ ] **TRIPLE VERIFY**: Passes A, B, C completed with results recorded
- [ ] **GAP HUNT**: Coverage table shows 100% or gaps documented with remedies
- [ ] **EVIDENCE LEDGER**: All claims have evidence references
- [ ] **RESULT BLOCK**: PASS/PARTIAL/FAIL with WHY and NEXT

---

## Quick Command Reference

### Smoke Test Skeleton for Phase 17

```bash
set -euo pipefail
echo "Running Phase 17 smoke test..."
# Test rate limiting
cargo test --lib rate_limits
# Test authentication
cargo test --lib device_identity
# Test observability
cargo test --lib observability
echo $? > .exitcode
```

### SHA-256 Listing

```bash
find sys/core/src -name "*.rs" -path "*/rate_limits.rs" -o -path "*/auth/*" -o -path "*/observability/*" | xargs sha256sum > phase17-hashes.txt
```

### Coverage Scan Skeleton

```bash
# Map Phase 17 requirements to test files
grep -r "FR-09[5-9]\|FR-1[0-5][0-9]" tests/ > phase17-coverage.md
```

### Result Block Emitter

```bash
echo "RESULT: ${RESULT:-PARTIAL}"
echo "WHY: Phase 17 has 7/63 tasks complete (11%)"
echo "NEXT: Complete Authentication & Identity (FR-100-109)"
```

---

*Checklist generated from Universal Task Execution Policy (§0-§13)*
*Total Items: 134*
*Phase 17 Coverage: FR-095 to FR-158 (64 Functional Requirements)*

---

**Checklist Created**: 2025-01-27
**Related**: [verification.md](./verification.md) (Master Verification Checklist)
**Related**: [phase17-verification.md](./phase17-verification.md) (Phase 17 Verification)
**Related**: [quality.md](./quality.md) (Master Quality Checklist)
**Related**: [tasks.md](../tasks.md) (Phase 17 Tasks T771-T826)

