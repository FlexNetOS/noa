## Summary
- What does this change do?
- Why is it needed?

## Security impact (required)
- [ ] No secrets added or logged (ENV-001 / ENV-010)
- [ ] No new network egress or external calls (or explicitly documented)
- [ ] Changes reviewed for least-privilege and safe defaults

## Configuration impact (required if configs changed)
- [ ] Config schema updated (if applicable) under `config/schemas/`
- [ ] Config files include `version` and `$schema` where applicable
- [ ] Local-only overrides are gitignored and documented

## Environment impact (required if env/scripts changed)
- [ ] Setup scripts remain idempotent (`scripts/setup/`)
- [ ] Generated artifacts are not committed (e.g. `noa-profile.ps1`, `.noa`, machine-local files)
- [ ] Windows + Unix flows still work (PowerShell + bash)

## Testing / evidence
- Commands run (paste output or summarize with links to logs):
  - `...`

## Rollout / risk
- Risk level: low / medium / high
- Rollback plan:


