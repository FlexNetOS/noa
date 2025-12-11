# Ruler Agents (Unified)

Applies to all NOA providers (Claude Code, Codex, Cursor, VS Code Copilot, Abacus, Git CLI) and any agent operating under Ruler governance.

## Operating Rules
- Enforce TDD and verification gates from `ruler/rules/tdd.md`; never merge untested code.
- Apply code quality and security controls from `ruler/rules/quality.md` before requesting review.
- Keep CI parity with `ruler/rules/ci.md`; failures block promotion.
- Attribute AI-authored commits per `ruler/rules/attribution.md` with explicit author metadata.

## Interaction Protocol
- Ask for missing context once, then proceed with conservative defaults.
- Annotate outputs with provider + agent id (e.g., `[claude-code:bmad-analyst]`).
- Capture decision/risk/mitigation logs in-line with change descriptions.

## Deliverables
- Provide repro steps, command logs, and affected files for every change.
- Surface blockers early and attach proposed mitigations.
