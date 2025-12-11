# End-to-End Tests (US1–US3)

Scope: smoke and flow tests covering initialization (US1), neural runtime basics (US2), and memory sovereignty (US3).

Status: Placeholder suite. Add real cases when services stabilize.

Suggested structure:
- `init/` – bootstrap, config validation, CLI flows (`noa init`, `noa start`).
- `neural/` – model load/unload stubs, inference echo tests.
- `memory/` – create/list/search memory through API and CLI.

Run (when implemented):
```bash
cd sys/core
cargo test --package noa-core --test e2e_us1_us3
```
