# TDD Enforcement Rules

- Start every change with a failing test or expectation; capture the failure output in the PR/commit notes.
- Keep test scope minimal and focused on the behavior under change; avoid brittle mocks when an integration test is cheaper.
- Add regression tests when fixing bugs; tests must fail before the fix and pass after.
- Do not commit skipped or xfail tests unless explicitly justified in the PR.
- Track coverage deltas; if coverage drops, add tests or document the exception with owner sign-off.
