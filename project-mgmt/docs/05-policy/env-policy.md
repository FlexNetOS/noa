# env-policy.md

> Governing policy for environment configuration, secrets handling, and cross-environment management. Applies to all projects, all environments, all agents and developers.

## 0) Scope and Priority

- **Applies to:** All environment variables, secrets, configuration files, and build-time constants.
- **Hierarchy:** (1) Security constraints; (2) Consistency requirements; (3) Developer convenience.
- **Hard stop rule:** Any violation of §3 (Secrets Protection) requires immediate remediation. Do not proceed until resolved.
- **Alignment:** Operates under the Universal Task Execution Policy. All evidence and verification requirements apply.

## 1) Definitions

- **Environment Variable:** A key-value pair defining runtime or build-time configuration.
- **Secret:** Any credential, token, API key, password, certificate, or private key.
- **Configuration:** Non-secret settings that may vary between environments.
- **Environment:** A deployment target (development, staging, production, test).
- **Generated File:** Any file auto-created from environment variables (e.g., `env.generated.ts`).

## 2) Environment Tiers

| Tier | Environment | Secret Access | Data Type |
|------|-------------|---------------|-----------|
| Tier 1 | Development | Mock/local only | Synthetic/test |
| Tier 2 | Staging | Rotated test creds | Anonymized/test |
| Tier 3 | Production | Production secrets | Real user data |

**Cross-tier rule:** Production secrets NEVER used in Tier 1 or 2. Test data NEVER used in Tier 3.

## 3) Secrets Protection (MANDATORY)

### 3.1 Storage Rules
- Secrets stored ONLY in `.env` files (local) or secure vaults (CI/CD).
- `.env` files MUST be listed in `.gitignore`. Verify before every commit.
- No secrets in source code, comments, logs, error messages, or documentation.

### 3.2 Transmission Rules
- Secrets transmitted only over encrypted channels (HTTPS, SSH, TLS).
- No secrets in URLs, query parameters, or GET requests.
- No secrets in client-side bundles unless encrypted and necessary.

### 3.3 Rotation Rules
- All secrets rotatable without code changes.
- Rotation frequency: Production secrets every 90 days minimum.
- Immediate rotation required on any suspected exposure.

### 3.4 Exposure Response
1. Revoke exposed credential immediately.
2. Audit access logs for unauthorized use.
3. Rotate all potentially affected secrets.
4. Document incident with timeline and remediation.
5. Notify stakeholders per incident response policy.

## 4) Environment Variable Standards

### 4.1 Naming Convention
```
<SCOPE>_<SUBSYSTEM>_<PROPERTY>

Examples:
  API_GOOGLE_DRIVE_TOKEN
  DB_PRIMARY_CONNECTION_STRING
  CACHE_REDIS_HOST
  FEATURE_DARK_MODE_ENABLED
```

- ALL_CAPS with underscores.
- Prefix with scope/subsystem.
- Boolean flags end with `_ENABLED` or `_DISABLED`.
- No abbreviations except industry-standard (API, DB, URL).

### 4.2 Required Documentation
Every environment variable MUST have:
1. Entry in `.env.example` with placeholder or safe default.
2. TypeScript type definition in generated or manual config.
3. Documentation comment explaining purpose and format.

### 4.3 Type Safety Requirements
- All environment variables accessed through typed interfaces.
- No raw `process.env` access in application code.
- Build fails if required variable missing or malformed.

## 5) File Structure Standards

```
project/
├── .env                    # Local secrets (gitignored)
├── .env.example            # Template with placeholders (committed)
├── src/
│   ├── environments/
│   │   ├── environment.ts      # Development config
│   │   ├── environment.prod.ts # Production config
│   │   └── environment.stage.ts# Staging config
│   └── config/
│       └── env.generated.ts    # Auto-generated (gitignored)
└── scripts/
    └── load-env.js         # Environment loader
```

## 6) Build and Deployment Rules

### 6.1 Build Time
- Environment variables injected at build time, not runtime.
- Build scripts MUST regenerate `env.generated.ts` before compilation.
- Build fails if required variables undefined or invalid.

### 6.2 CI/CD Requirements
- Secrets provided via CI/CD secret management (not in repo).
- Environment variables validated before deployment proceeds.
- Deployment artifacts include hash verification (per Universal Policy §4).

### 6.3 Deployment Verification
Before marking deployment complete:
- [ ] All required environment variables present
- [ ] Smoke test passes with current configuration
- [ ] No secrets in logs or error outputs
- [ ] Health check endpoints responding

## 7) Access Control

| Action | Developer | Tech Lead | DevOps | Security |
|--------|-----------|-----------|--------|----------|
| View .env.example | ✓ | ✓ | ✓ | ✓ |
| Modify .env.example | ✓ (PR) | ✓ (approve) | ✓ | ✓ |
| Access dev secrets | ✓ | ✓ | ✓ | ✓ |
| Access staging secrets | — | ✓ | ✓ | ✓ |
| Access production secrets | — | — | ✓ | ✓ |
| Rotate secrets | — | — | ✓ | ✓ |
| Audit access logs | — | — | ✓ | ✓ |

## 8) Validation Protocol

### 8.1 Pre-Commit Checks
```bash
# Must pass before commit allowed
git-secrets --scan
grep -r "API_KEY\|SECRET\|PASSWORD" --include="*.ts" src/
# Exit code must be non-zero (no matches)
```

### 8.2 Build-Time Validation
```typescript
function validateEnv(): void {
  const required = ['API_GOOGLE_DRIVE_TOKEN', 'DB_CONNECTION_STRING'];
  const missing = required.filter(key => !ENV[key]);
  if (missing.length > 0) {
    throw new Error(`Missing required env vars: ${missing.join(', ')}`);
  }
}
```

### 8.3 Runtime Verification
- Health check endpoint validates critical configuration.
- Startup logs confirm environment tier (without exposing secrets).
- Graceful shutdown if critical config becomes unavailable.

## 9) Prohibitions

- NO secrets in version control, ever.
- NO production data in non-production environments.
- NO environment variable access without type wrapper.
- NO deployment without environment validation passing.
- NO secret logging, even in debug mode.
- NO hardcoded fallbacks for secrets (fail instead).

## 10) Exception Process

Exceptions to this policy require:
1. Written justification with risk assessment.
2. Approval from Tech Lead AND Security.
3. Time-bounded scope (max 30 days).
4. Documented remediation plan.
5. Entry in exception register.

## 11) Compliance Verification

| Check | Frequency | Owner | Evidence |
|-------|-----------|-------|----------|
| Secret scan | Every commit | Developer | Pre-commit hook log |
| .gitignore audit | Weekly | Tech Lead | Git history review |
| Access audit | Monthly | Security | Access log analysis |
| Rotation compliance | Quarterly | DevOps | Credential age report |

---

## POLICY SUMMARY

```
STATUS: ACTIVE
VERSION: 1.0
LAST UPDATED: 2025-12-04
OWNER: Security + DevOps
ENFORCEMENT: Automated (pre-commit, CI) + Manual (audits)
VIOLATION CONSEQUENCE: Build block, deployment halt, incident report
```
