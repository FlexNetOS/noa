# Security Policy

This project is committed to **security, privacy, and self-contained operation** (see `CONSTITUTION.md`, esp. §3.1 and §3.6).

## Supported versions

NOA is under active development. Security fixes are delivered on the default branch and tagged releases as they happen.

- **Default branch (`main`)**: supported
- **Latest tagged release**: supported (when releases exist)
- **Older tags**: best-effort; fixes may be backported only for high/critical issues

## Reporting a vulnerability

### Preferred reporting channel

Please **do not** open a public issue for security-sensitive reports.

- If you have access to GitHub Security Advisories for this repository, submit a report there.
- Otherwise, contact the maintainers privately using the channel documented in your organization’s security process.

### What to include

- A clear description of the issue and impact
- Steps to reproduce (proof-of-concept if safe)
- Affected paths/components (e.g. `sys/core`, `sys/ui`, `scripts/setup`, `config/`)
- Any relevant logs (please redact secrets)

### Response timeline (targets)

- **Acknowledgement**: within 72 hours
- **Triage**: within 7 days
- **Fix ETA**: provided after triage (severity-dependent)

### Responsible disclosure expectations

We ask reporters to keep details private until a fix is available. We will coordinate disclosure timing and credit when desired.

## Security invariants (hard rules)

- **No secrets in source control** (see `AGENT.md` ENV-001/ENV-010 guidance)
- **No secret logging**
- **Prefer least privilege + safe defaults**
- **Configs must allow env substitution** (`${ENV_VAR}`), and sensitive values must be externalized

