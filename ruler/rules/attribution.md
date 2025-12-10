# AI Commit Attribution Rules

- Use `--author="AI <ruler+ai@okigu.com>"` for AI-authored commits and note the operating provider/agent in the commit message footer (e.g., `Provider: claude-code`).
- Summaries must describe the behavior change and the tests executed; avoid vague messages.
- Keep commit boundaries small and reviewable; prefer multiple focused commits over a single large one.
- Record any human edits on top of AI changes in the PR description or commit body for traceability.
