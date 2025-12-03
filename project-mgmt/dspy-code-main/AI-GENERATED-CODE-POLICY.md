# AI-Generated Code Policy

## Summary
We welcome contributions that used AI tools, provided they meet our quality, testing, security and licensing expectations. AI-generated code is treated the same as human-written code — it must be reviewed, tested, and owned by the contributor.

## Disclosure
If you used AI to produce any part of your contribution, include the following in your PR description:
- **Model/Agent** (e.g., "GPT-5 Thinking mini via Copilot")
- **How AI was used** (e.g., "boilerplate, implementation draft, tests, docs")
- **Manual validation** performed (linters, tests, security checks)

Simple autocompletion does not require disclosure. Substantial generation (functions, algorithms, large refactors, tests, docs) requires disclosure.

## Contributor responsibilities
- You remain fully responsible for the code you submit. Understand, test, and be able to explain all changes.
- All code (AI or human) must pass linters and tests and meet project coding standards.
- Provide documentation and tests for non-trivial changes.

## IP, licensing, and security
- By contributing you confirm you have the right to contribute the content (including AI outputs) under this project's license.
- Do not submit content that includes proprietary, copyrighted, or secret data.
- Avoid insecure patterns and secrets in contributions.

## Workflow rules
- For **core**, API, or architectural changes open an **Issue** first and discuss; link the Issue from the PR.
- Small bug fixes and docs may open PRs directly, but still follow disclosure and testing requirements.

## Maintainer rights
Maintainers may reject or modify PRs that diverge from project goals, introduce undue complexity, or violate the above rules.

## Enforcement and updates
This policy may be updated as the ecosystem evolves. If maintainers suspect problematic AI-generated content they may request provenance, tests, or rework.
