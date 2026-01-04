# configs Audit & Runtime Wiring Plan (Multi-source, 37-file deep audit)

This plan is intended to be copied into the active Visual Studio Copilot plan file.

## Goal
100% verify that configs files are:
1) syntactically valid,
2) schema-consistent where applicable,
3) loaded/merged/consumed by `sys/core` as intended,
4) runtime-updatable via the hot-reload path.

## Inventory (source of truth)

### `N:\noa\configs` (32 items including subdirs)
1. `configs/ai-providers.json`
2. `configs/bootstrap-state.json`
3. `configs/bootstrap-tools.json`
4. `configs/database.yaml`
5. `configs/desktop-apps.json`
6. `configs/device-orchestration.json`
7. `configs/features.json`
8. `configs/git-conflict-ai.json`
9. `configs/git-local-cicd.json`
10. `configs/git-pr-workflow.json`
11. `configs/kernel-selection-policy.json`
12. `configs/minio.yaml`
13. `configs/noa-server.json`
14. `configs/observability.yaml`
15. `configs/providers/default.yaml`
16. `configs/qdrant.yaml`
17. `configs/quickwit.yaml`
18. `configs/shared-resources.json`
19. `configs/tools.json`
20. `configs/README.md` *(doc audit only)*
21. `configs/requirements.txt` *(doc/tooling audit only)*
22. `configs/schemas/configs_schema.json`
23. `configs/schemas/desktop-apps.json`
24. `configs/schemas/mcp-servers.json`
25. `configs/schemas/providers.yaml`
26. `configs/schemas/csv/agent_directory.yaml`
27. `configs/schemas/csv/claims_evidence.yaml`
28. `configs/schemas/csv/metrics_traces.yaml`
29. `configs/schemas/csv/task_tables.yaml`
30. `configs/templates/agent.yaml`
31. `configs/templates/noa-instance.yaml`
32. `configs/templates/provider.yaml`

### `N:\noa\.configs` (5 items)
33. `.configs/.golangci.yml`
34. `.configs/clippy.toml`
35. `.configs/eslint.configs.mjs`
36. `.configs/ruff.toml`
37. `.configs/rustfmt.toml`

---

## 1) Runtime wiring verification (hot reload)
- [ ] Confirm `ApiServer::start()` starts configs polling hot reload
- [ ] Verify reload loop monitors every configsured file path (multi-source)
- [ ] Verify reload swaps the in-memory configs used by live routes/services
- [ ] Add minimal automated verification:
  - [ ] Start server (or `configsAccess`) in test/harness
  - [ ] Modify a watched configs file (temp copy preferred)
  - [ ] Assert updated value is observable through `configsAccess` / API route

---

## 2) Loader coverage audit (what core actually loads vs ignores)
- [ ] Map `configsLoader::load()` inputs ? raw output keys
- [ ] For each configs file, classify:
  - [ ] **Loaded + Consumed**
  - [ ] **Loaded-only**
  - [ ] **Not loaded**
- [ ] For each **Not loaded** file:
  - [ ] Decide if it should be merged into `Noaconfigs.raw`
  - [ ] If yes: define canonical raw key + merge strategy

---

## 3) Format + schema consistency (per file)
For every JSON/YAML: parse ? (if schema exists) validate ? confirm env-var format compatibility.

### A) Core runtime configss (currently merged/expected)
- [ ] `configs/noa-server.json`
  - [ ] Confirm mapping between this file and actual server host/port usage in Rust core
- [ ] `configs/ai-providers.json`
  - [ ] Confirm loader expects `type` but file uses `types` array (fix/compat decision)
- [ ] `configs/shared-resources.json`
- [ ] `configs/device-orchestration.json`
- [ ] `configs/features.json`
  - [ ] Confirm loader�s feature-flag parsing matches file structure

### B) Service configss (validate; decide integration)
- [ ] `configs/database.yaml`
- [ ] `configs/observability.yaml`
- [ ] `configs/minio.yaml`
- [ ] `configs/qdrant.yaml`
- [ ] `configs/quickwit.yaml`

### C) Policy / workflow configss (validate; decide integration)
- [ ] `configs/kernel-selection-policy.json`
- [ ] `configs/desktop-apps.json` (validate vs `configs/schemas/desktop-apps.json`)
- [ ] `configs/git-conflict-ai.json`
- [ ] `configs/git-local-cicd.json`
- [ ] `configs/git-pr-workflow.json`

### D) Bootstrap/tooling configss (validate; decide integration)
- [ ] `configs/tools.json`
- [ ] `configs/bootstrap-tools.json`
- [ ] `configs/bootstrap-state.json`
- [ ] `configs/requirements.txt`

### E) Provider list
- [ ] `configs/providers/default.yaml` (validate vs `configs/schemas/providers.yaml`)

### F) Schemas
- [ ] `configs/schemas/configs_schema.json` (confirm `$schema`/`$id`, env pattern compatibility)
- [ ] `configs/schemas/providers.yaml`
- [ ] `configs/schemas/mcp-servers.json`
- [ ] `configs/schemas/desktop-apps.json`
- [ ] `configs/schemas/csv/*.yaml`

### G) Templates
- [ ] `configs/templates/agent.yaml`
- [ ] `configs/templates/noa-instance.yaml`
- [ ] `configs/templates/provider.yaml`

### H) Repo tooling configss (`N:\noa\.configs`)
- [ ] `.configs/.golangci.yml`
- [ ] `.configs/clippy.toml`
- [ ] `.configs/eslint.configs.mjs`
- [ ] `.configs/ruff.toml`
- [ ] `.configs/rustfmt.toml`

---

## 4) �Properly used� verification (code linkage)
For each file classified **Loaded + Consumed**:
- [ ] Identify every read path in code (configs pointers, `Noaconfigs.get()`, typed fields)
- [ ] Confirm configs keys match what code expects (no silent fallbacks)
- [ ] Add validation failures for required values (avoid masked misconfigs)
- [ ] Add unit tests proving:
  - [ ] load ? access ? correct value
  - [ ] hot reload updates value in runtime

---

## 5) Done criteria
- [ ] Documented map: `file ? merged key ? used by ? required keys`
- [ ] All key-shape mismatches resolved (or explicitly marked raw-only/future)
- [ ] Automated checks added:
  - [ ] parse all json/yaml
  - [ ] schema-validate eligible configss
  - [ ] required-key sanity checks
- [ ] Demonstrated hot reload changes runtime behavior for at least one API-exposed value
