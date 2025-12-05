# env-rule.md

> Executable rules for environment configuration. Each rule is atomic, testable, and enforceable. Violations block builds or deployments.

## Rule Index

| ID | Rule | Enforcement | Severity |
|----|------|-------------|----------|
| ENV-001 | No secrets in source | Pre-commit | CRITICAL |
| ENV-002 | .env in .gitignore | Pre-commit | CRITICAL |
| ENV-003 | Type-safe access only | Build | HIGH |
| ENV-004 | Required vars defined | Build | HIGH |
| ENV-005 | Naming convention | Lint | MEDIUM |
| ENV-006 | .env.example sync | PR review | MEDIUM |
| ENV-007 | Generated files ignored | Pre-commit | HIGH |
| ENV-008 | Environment isolation | Deploy | CRITICAL |
| ENV-009 | Validation at startup | Runtime | HIGH |
| ENV-010 | No secret logging | Build + Runtime | CRITICAL |

---

## ENV-001: No Secrets in Source Code

**Statement:** Source code files SHALL NOT contain hardcoded secrets, credentials, tokens, or API keys.

**Detection:**
```bash
# Pattern scan - must return no matches
rg -i "(api_key|secret|password|token|credential)\s*[:=]\s*['\"][^'\"]+['\"]" \
   --type ts --type js --type json \
   --glob '!*.example*' --glob '!*.test.*' \
   src/
```

**Exceptions:** None. Test mocks must use obviously fake values (e.g., `test-token-12345`).

**Remediation:** Move secret to `.env`, reference via typed accessor.

---

## ENV-002: .env Files in .gitignore

**Statement:** All `.env` files (except `.env.example`) SHALL be listed in `.gitignore`.

**Detection:**
```bash
# Verify .gitignore contains .env
grep -q "^\.env$" .gitignore && echo "PASS" || echo "FAIL"

# Verify no .env files tracked
git ls-files | grep -E "^\.env$|\.env\." | grep -v example
# Must return empty
```

**Remediation:** Add to `.gitignore`:
```
.env
.env.local
.env.*.local
```

---

## ENV-003: Type-Safe Environment Access

**Statement:** Application code SHALL access environment variables only through typed interfaces, never via raw `process.env`.

**Detection:**
```bash
# Must return no matches in application code
rg "process\.env\[" src/app/
rg "process\.env\." src/app/ --glob '!**/env.generated.ts' --glob '!**/load-env.*'
```

**Correct Pattern:**
```typescript
// ✓ Correct - typed access
import { ENV } from './config/env.generated';
const token = ENV.API_GOOGLE_TOKEN;

// ✓ Correct - utility with type checking
import { getEnv } from './util/env';
const token = getEnv('API_GOOGLE_TOKEN');

// ✗ Wrong - untyped access
const token = process.env.API_GOOGLE_TOKEN;
```

**Remediation:** Replace all `process.env` with typed accessor imports.

---

## ENV-004: Required Variables Defined

**Statement:** All environment variables marked as required SHALL be defined and non-empty at build time.

**Detection:**
```typescript
// Build-time validation
const REQUIRED_VARS = [
  'API_GOOGLE_DRIVE_TOKEN',
  'DB_CONNECTION_STRING',
] as const;

REQUIRED_VARS.forEach(key => {
  if (!process.env[key]?.trim()) {
    console.error(`FATAL: Required env var ${key} is not defined`);
    process.exit(1);
  }
});
```

**Remediation:** Define missing variable in `.env` or provide via CI/CD secrets.

---

## ENV-005: Naming Convention

**Statement:** Environment variable names SHALL follow the pattern `SCOPE_SUBSYSTEM_PROPERTY` in SCREAMING_SNAKE_CASE.

**Detection:**
```bash
# Extract all env var references and validate format
rg "ENV\.[A-Z]" src/ -o | sort -u | \
  grep -vE "^ENV\.[A-Z][A-Z0-9]*(_[A-Z][A-Z0-9]*)*$"
# Must return empty
```

**Valid Examples:**
- `API_GOOGLE_DRIVE_TOKEN`
- `DB_PRIMARY_HOST`
- `FEATURE_DARK_MODE_ENABLED`
- `CACHE_REDIS_TTL`

**Invalid Examples:**
- `googleToken` (wrong case)
- `API-KEY` (wrong separator)
- `TOKEN` (no scope)

**Remediation:** Rename variable following convention, update all references.

---

## ENV-006: .env.example Synchronization

**Statement:** Every environment variable used in code SHALL have a corresponding entry in `.env.example` with a placeholder or safe default value.

**Detection:**
```bash
# Extract env vars from code
CODE_VARS=$(rg "ENV\.\w+" -o src/ | sed 's/ENV\.//' | sort -u)

# Extract env vars from .env.example
EXAMPLE_VARS=$(grep -E "^[A-Z_]+=" .env.example | cut -d= -f1 | sort -u)

# Find missing
comm -23 <(echo "$CODE_VARS") <(echo "$EXAMPLE_VARS")
# Must return empty
```

**.env.example Format:**
```bash
# Google Drive Integration
API_GOOGLE_DRIVE_TOKEN=your-token-here

# Database
DB_CONNECTION_STRING=postgresql://user:pass@localhost:5432/dbname

# Feature Flags (safe defaults)
FEATURE_DARK_MODE_ENABLED=false
```

**Remediation:** Add missing variables to `.env.example` with placeholder values.

---

## ENV-007: Generated Files in .gitignore

**Statement:** Auto-generated environment files SHALL be listed in `.gitignore`.

**Detection:**
```bash
# Verify generated files are ignored
grep -q "env\.generated\.ts" .gitignore && echo "PASS" || echo "FAIL"

# Verify not tracked
git ls-files | grep -q "env.generated" && echo "FAIL" || echo "PASS"
```

**Files That Must Be Ignored:**
- `src/app/config/env.generated.ts`
- `src/environments/*.generated.ts`
- Any file matching `*.generated.*` in config directories

**Remediation:** Add to `.gitignore` and remove from tracking:
```bash
git rm --cached src/app/config/env.generated.ts
```

---

## ENV-008: Environment Isolation

**Statement:** Each environment tier SHALL use isolated credentials. Production secrets SHALL NOT exist in development or staging.

**Detection:**
```bash
# In CI/CD: Compare secret names across environments
# Production-only secrets must not appear in dev/staging configs
PROD_ONLY_PATTERNS=(
  "PROD_"
  "_PRODUCTION_"
  "LIVE_"
)
```

**Verification Checklist:**
- [ ] Development uses mock/local credentials only
- [ ] Staging uses rotated test credentials
- [ ] Production secrets accessible only in production pipeline
- [ ] No credential sharing between tiers

**Remediation:** Provision tier-specific credentials. Never copy production secrets to lower tiers.

---

## ENV-009: Startup Validation

**Statement:** Applications SHALL validate critical environment configuration at startup and fail fast if validation fails.

**Implementation:**
```typescript
// src/bootstrap/validate-env.ts
export function validateEnvironment(): void {
  const errors: string[] = [];

  // Required variables
  if (!ENV.API_GOOGLE_DRIVE_TOKEN) {
    errors.push('API_GOOGLE_DRIVE_TOKEN is required');
  }

  // Format validation
  if (ENV.DB_PORT && isNaN(parseInt(ENV.DB_PORT))) {
    errors.push('DB_PORT must be a valid number');
  }

  // URL validation
  if (ENV.API_ENDPOINT && !isValidUrl(ENV.API_ENDPOINT)) {
    errors.push('API_ENDPOINT must be a valid URL');
  }

  if (errors.length > 0) {
    console.error('Environment validation failed:');
    errors.forEach(e => console.error(`  - ${e}`));
    process.exit(1);
  }

  console.log(`Environment validated: ${getEnvironmentTier()}`);
}
```

**Remediation:** Add validation call to application bootstrap before any other initialization.

---

## ENV-010: No Secret Logging

**Statement:** Secrets SHALL NOT appear in logs, error messages, stack traces, or monitoring data.

**Detection:**
```bash
# Scan for potential secret logging
rg "(console\.(log|error|warn)|logger\.\w+).*ENV\." src/ \
  --glob '!*.test.*'
# Review each match - secrets must be redacted
```

**Correct Pattern:**
```typescript
// ✓ Correct - log presence, not value
console.log(`Google API configured: ${!!ENV.API_GOOGLE_TOKEN}`);

// ✓ Correct - redacted output
console.log(`Token: ${ENV.API_TOKEN?.slice(0,4)}****`);

// ✗ Wrong - exposes secret
console.log(`Using token: ${ENV.API_TOKEN}`);
```

**Implementation:**
```typescript
// Utility for safe logging
function redactSecret(value: string | undefined): string {
  if (!value) return '[NOT SET]';
  if (value.length <= 8) return '****';
  return `${value.slice(0, 4)}...${value.slice(-4)}`;
}
```

**Remediation:** Replace secret logging with presence checks or redacted values.

---

## Rule Enforcement Matrix

| Stage | Rules Enforced | Failure Action |
|-------|----------------|----------------|
| Pre-commit | ENV-001, ENV-002, ENV-007 | Commit blocked |
| Build | ENV-003, ENV-004, ENV-010 | Build fails |
| Lint/Review | ENV-005, ENV-006 | Warning / PR blocked |
| Deploy | ENV-008 | Deployment halted |
| Runtime | ENV-009, ENV-010 | App won't start / Audit alert |

---

## Quick Reference Commands

```bash
# Full environment rule check
npm run env:validate

# Check for secrets in staged files
git diff --cached --name-only | xargs rg -l "(secret|token|password|api_key)" -i

# Validate .env.example coverage
npm run env:check-example

# Generate env types
npm run env:generate

# Audit environment configuration
npm run env:audit
```

---

## RULE SET SUMMARY

```
RULESET: Environment Configuration Rules
VERSION: 1.0
TOTAL RULES: 10
CRITICAL: 4 (ENV-001, ENV-002, ENV-008, ENV-010)
HIGH: 4 (ENV-003, ENV-004, ENV-007, ENV-009)
MEDIUM: 2 (ENV-005, ENV-006)
AUTOMATION: Pre-commit hooks, CI pipeline, Build scripts
```
