# Code Quality Rules

- Run formatters and linters locally before review (`npm run lint` or language equivalent); lint failures block merge.
- Prefer small, composable functions with clear inputs/outputs; document non-obvious decisions inline.
- Keep dependencies lean; justify any new runtime dependency and ensure licenses are compatible.
- Add security scanning (gitleaks/trivy/grype) for affected surfaces when credentials, secrets, or binaries are involved.
- Ensure docs and examples stay in sync with code changes; update READMEs or inline usage snippets when interfaces change.
