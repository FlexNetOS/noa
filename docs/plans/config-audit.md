# Config Audit & Runtime Wiring Plan (Multi-source, 37-file deep audit)

This plan is intended to be copied into the active Visual Studio Copilot plan file.

## Goal
100% verify that config files are:
1) syntactically valid,
2) schema-consistent where applicable,
3) loaded/merged/consumed by `sys/core` as intended,
4) runtime-updatable via the hot-reload path.

## Inventory (source of truth)

### `N:\noa\config` (32 items including subdirs)
1. `config/ai-providers.json`
2. `config/bootstrap-state.json`
3. `config/bootstrap-tools.json`
4. `config/database.yaml`
5. `config/desktop-apps.json`
6. `config/device-orchestration.json`
7. `config/features.json`
8. `config/git-conflict-ai.json`
9. `config/git-local-cicd.json`
10. `config/git-pr-workflow.json`
11. `config/kernel-selection-policy.json`
12. `config/minio.yaml`
13. `config/noa-server.json`
14. `config/observability.yaml`
15. `config/providers/default.yaml`
16. `config/qdrant.yaml`
17. `config/quickwit.yaml`
18. `config/shared-resources.json`
19. `config/tools.json`
20. `config/README.md` *(doc audit only)*
21. `config/requirements.txt` *(doc/tooling audit only)*
22. `config/schemas/config_schema.json`
23. `config/schemas/desktop-apps.json`
24. `config/schemas/mcp-servers.json`
25. `config/schemas/providers.yaml`
26. `config/schemas/csv/agent_directory.yaml`
27. `config/schemas/csv/claims_evidence.yaml`
28. `config/schemas/csv/metrics_traces.yaml`
29. `config/schemas/csv/task_tables.yaml`
30. `config/templates/agent.yaml`
31. `config/templates/noa-instance.yaml`
32. `config/templates/provider.yaml`

### `N:\noa\.config` (5 items)
33. `.config/.golangci.yml`
34. `.config/clippy.toml`
35. `.config/eslint.config.mjs`
36. `.config/ruff.toml`
37. `.config/rustfmt.toml`

---

## 1) Runtime wiring verification (hot reload)
- [ ] Confirm `ApiServer::start()` starts config polling hot reload
- [ ] Verify reload loop monitors every configured file path (multi-source)
- [ ] Verify reload swaps the in-memory config used by live routes/services
- [ ] Add minimal automated verification:
  - [ ] Start server (or `ConfigAccess`) in test/harness
  - [ ] Modify a watched config file (temp copy preferred)
  - [ ] Assert updated value is observable through `ConfigAccess` / API route

---

## 2) Loader coverage audit (what core actually loads vs ignores)
- [ ] Map `ConfigLoader::load()` inputs ? raw output keys
- [ ] For each config file, classify:
  - [ ] **Loaded + Consumed**
  - [ ] **Loaded-only**
  - [ ] **Not loaded**
- [ ] For each **Not loaded** file:
  - [ ] Decide if it should be merged into `NoaConfig.raw`
  - [ ] If yes: define canonical raw key + merge strategy

---

## 3) Format + schema consistency (per file)
For every JSON/YAML: parse ? (if schema exists) validate ? confirm env-var format compatibility.

### A) Core runtime configs (currently merged/expected)
- [ ] `config/noa-server.json`
  - [ ] Confirm mapping between this file and actual server host/port usage in Rust core
- [ ] `config/ai-providers.json`
  - [ ] Confirm loader expects `type` but file uses `types` array (fix/compat decision)
- [ ] `config/shared-resources.json`
- [ ] `config/device-orchestration.json`
- [ ] `config/features.json`
  - [ ] Confirm loader’s feature-flag parsing matches file structure

### B) Service configs (validate; decide integration)
- [ ] `config/database.yaml`
- [ ] `config/observability.yaml`
- [ ] `config/minio.yaml`
- [ ] `config/qdrant.yaml`
- [ ] `config/quickwit.yaml`

### C) Policy / workflow configs (validate; decide integration)
- [ ] `config/kernel-selection-policy.json`
- [ ] `config/desktop-apps.json` (validate vs `config/schemas/desktop-apps.json`)
- [ ] `config/git-conflict-ai.json`
- [ ] `config/git-local-cicd.json`
- [ ] `config/git-pr-workflow.json`

### D) Bootstrap/tooling configs (validate; decide integration)
- [ ] `config/tools.json`
- [ ] `config/bootstrap-tools.json`
- [ ] `config/bootstrap-state.json`
- [ ] `config/requirements.txt`

### E) Provider list
- [ ] `config/providers/default.yaml` (validate vs `config/schemas/providers.yaml`)

### F) Schemas
- [ ] `config/schemas/config_schema.json` (confirm `$schema`/`$id`, env pattern compatibility)
- [ ] `config/schemas/providers.yaml`
- [ ] `config/schemas/mcp-servers.json`
- [ ] `config/schemas/desktop-apps.json`
- [ ] `config/schemas/csv/*.yaml`

### G) Templates
- [ ] `config/templates/agent.yaml`
- [ ] `config/templates/noa-instance.yaml`
- [ ] `config/templates/provider.yaml`

### H) Repo tooling configs (`N:\noa\.config`)
- [ ] `.config/.golangci.yml`
- [ ] `.config/clippy.toml`
- [ ] `.config/eslint.config.mjs`
- [ ] `.config/ruff.toml`
- [ ] `.config/rustfmt.toml`

---

## 4) “Properly used” verification (code linkage)
For each file classified **Loaded + Consumed**:
- [ ] Identify every read path in code (config pointers, `NoaConfig.get()`, typed fields)
- [ ] Confirm config keys match what code expects (no silent fallbacks)
- [ ] Add validation failures for required values (avoid masked misconfig)
- [ ] Add unit tests proving:
  - [ ] load ? access ? correct value
  - [ ] hot reload updates value in runtime

---

## 5) Done criteria
- [ ] Documented map: `file ? merged key ? used by ? required keys`
- [ ] All key-shape mismatches resolved (or explicitly marked raw-only/future)
- [ ] Automated checks added:
  - [ ] parse all json/yaml
  - [ ] schema-validate eligible configs
  - [ ] required-key sanity checks
- [ ] Demonstrated hot reload changes runtime behavior for at least one API-exposed value
