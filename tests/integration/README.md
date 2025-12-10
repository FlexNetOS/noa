# Integration Tests (US4–US7)

Scope: mid-layer integration for digest pipeline (US4), UI/service touchpoints (US5), P2P (US6), and agent orchestration (US7).

Status: Placeholder. Add real tests as components stabilize.

Suggested structure:
- `digest/` – ingestion API, schema checks, evidence ledger stubs.
- `ui/` – API/UI contract smoke via Playwright or API snapshots.
- `p2p/` – discovery/sync RPC stubs once libp2p is wired.
- `agents/` – orchestrator task/goal flows.

Run (future):
```bash
cd sys/core
cargo test --package noa-core --test integration_us4_us7
```
