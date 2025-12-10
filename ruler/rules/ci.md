# CI Integration Rules

- Pipelines must run lint, unit, and integration suites; failures halt promotion until resolved.
- Treat flaky tests as bugs: quarantine with a ticket ID and a remediation due date.
- Surface build artifacts (logs, coverage reports) in the PR to aid reviewers.
- CI must publish status badges or summaries visible to reviewers; undocumented overrides are not allowed.
- Release workflows require a dry-run stage before publishing or tagging production artifacts.
