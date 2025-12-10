# NOA Seed Foundation

NOA is an autonomous, agentic operating system with portable toolchains, shared execution memory, and provider-integrated workflows.

## Quickstart

### Windows
```powershell
.\scripts\setup\setup-noa.ps1 -InstallAllTools -InstallAiProviders
.\noa-profile.ps1   # loads NOA_ROOT and PATH shims
cd sys/core
cargo build
```

### macOS/Linux
```bash
./scripts/setup/setup-noa.sh --install-all-tools --install-ai-providers
source ./noa-profile.sh
cd sys/core
cargo build
```

To skip tool installs (directories only), drop the flags. Shared resources live under `ai/shared/` and are initialized by the setup script.

## Run
- CLI: `cd sys/core && cargo run --bin noa -- --help`
- Start core services (when implemented): `noa start`
- Providers are configured in `config/shared-resources.json` and `ai/providers/**/config.json`.

## Project Layout
- `sys/core/` – Rust core (CLI, providers, kernel independence, modules, services)
- `sys/ui/` – Next.js dashboard (status, connectors, marketplace stubs)
- `ai/shared/` – prompts, skills, tools, models, execution memory schema
- `scripts/setup/` – unified bootstrap (portable toolchains, AI providers)
- `docs/` – architecture, setup guides, API docs

## Testing
- Rust: `cd sys/core && cargo test`
- UI: `cd sys/ui && npm test` (when dependencies are installed)
- Targeted scripts: see `tests/` for verification suites.

## Docs
- Architecture: `docs/architecture.md` and `docs/architecture/*`
- Setup guides: `docs/setup/`
- API references: `docs/api/` (generated from `specs/001-noa-seed-foundation/contracts/`)
- Tasks and specs: `specs/001-noa-seed-foundation/`
