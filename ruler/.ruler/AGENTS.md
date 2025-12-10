# Ruler Agents (Unified)

Scope: Claude Code, Codex, Cursor, VS Code Copilot, Abacus, Git CLI, and any agent using Ruler in NOA.

## Operating Rules
- Follow TDD and verification steps in `ruler/rules/tdd.md`; never land untested code.
- Apply quality and security guidance in `ruler/rules/quality.md` before review.
- Enforce CI gates defined in `ruler/rules/ci.md`.
- Attribute AI-authored commits per `ruler/rules/attribution.md` (`--author="AI <ruler+ai@okigu.com>"`).

## Protocol
- Request missing context once, then proceed with safe defaults.
- Tag outputs with `[provider:agent-id]`.
- Log decisions, risks, and mitigations alongside changes.

## Deliverables
- Include repro commands and affected file list for every change.
- Surface blockers early with proposed mitigations.
