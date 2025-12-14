# Branch Protections (Required Manual GitHub Settings)

Some critical repo-hardening is configured in GitHub settings (not in git).

## Recommended protection for `main`

Enable branch protection rules with:

- Require a pull request before merging
- Require approvals (at least 1; recommend 2 for `config/`, `scripts/`, `containers/`)
- Require review from Code Owners
- Require conversation resolution
- Require status checks to pass, including:
  - `secrets-scan`
  - `sast`
  - `dependency-scan`
  - `config-validate`
  - `codeql`
  - `sbom`
- Require branches to be up to date before merging
- Restrict force pushes
- Restrict deletions

## Notes

- The exact check names must match the workflow/job names shown in GitHub Actions.
- For a staged rollout, you can initially require only `secrets-scan` + `config-validate`, then add more checks over time.


