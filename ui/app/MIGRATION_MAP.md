# rust-lovable → noa-ui migration map

This file is the authoritative mapping from `ui/kimi_test/rust-lovable` sources into the new `ui/app` workspace.

## Rules of the road

- `ui/app` is the **shell**.
- Dioxus target is **0.7.2**.
- All persisted/runtime data must live under `$NOA_DATA/apps/noa-ui`.
- **UI code** moves into `ui/app/crates/noa-ui-shell`.
- **Non-UI core logic** moves into `ui/app/crates/noa-ui-core` initially (later can be split into domain/storage).

## Rust source files

| rust-lovable path | Destination (ui/app) | Notes |
|---|---|---|
| `src/lib.rs` | *absorbed* into workspace crates | Old crate re-exports; replaced by `noa-ui-shell` + `noa-ui-core` crates. |
| `src/main.rs` | `bins/noa-ui-server` (planned) | Legacy entrypoints replaced by platform binaries (`noa-ui-desktop`, `noa-ui-web`, etc.). |
| `src/app.rs` | `crates/noa-ui-shell/src/app.rs` | Port router to Dioxus 0.7 (`dioxus` router feature; no `dioxus-router` dep). |
| `src/components/mod.rs` | `crates/noa-ui-shell/src/components/mod.rs` | Keep module structure; adjust imports to `noa_ui_core::...`. |
| `src/components/canvas.rs` | `crates/noa-ui-shell/src/components/canvas.rs` | Uses `noa_ui_core::{conversational_ai, ui_generator}` types. |
| `src/components/chat.rs` | `crates/noa-ui-shell/src/components/chat.rs` | Remove direct `tokio::time` usage for wasm; use target-specific sleep helper. |
| `src/components/sidebar.rs` | `crates/noa-ui-shell/src/components/sidebar.rs` | Uses `noa_ui_core::ui_generator::ComponentType`. |
| `src/components/toolbar.rs` | `crates/noa-ui-shell/src/components/toolbar.rs` | Remove direct `tokio::time` usage for wasm; use target-specific sleep helper. |
| `src/components/ui_components.rs` | `crates/noa-ui-shell/src/components/ui_components.rs` | Shared UI primitives; stays in shell until we split into `noa-ui-ui`. |
| `src/core/mod.rs` | `crates/noa-ui-core/src/lib.rs` | Converted to a library crate root. |
| `src/core/conversational_ai.rs` | `crates/noa-ui-core/src/conversational_ai.rs` | Types also inform sync schema; later split into `noa-ui-domain`. |
| `src/core/ui_generator.rs` | `crates/noa-ui-core/src/ui_generator.rs` | Update module paths from `crate::core::...` to `crate::...`. |
| `src/core/code_generator.rs` | `crates/noa-ui-core/src/code_generator.rs` | Same path adjustments. |
| `src/core/cross_platform.rs` | `crates/noa-ui-core/src/cross_platform.rs` | Same path adjustments. |
| `src/core/project_manager.rs` | `crates/noa-ui-core/src/project_manager.rs` | Must be refactored to use `noa-ui-paths` (no home dir / no relative `./data`). |
| `src/utils/mod.rs` | `crates/noa-ui-core/src/utils/mod.rs` *(planned)* | For now, `utils/*` are moved into `noa-ui-core` (or later `noa-ui-storage`). |
| `src/utils/file_utils.rs` | `crates/noa-ui-core/src/utils/file_utils.rs` *(planned)* |  |
| `src/utils/serialization.rs` | `crates/noa-ui-core/src/utils/serialization.rs` *(planned)* |  |
| `src/utils/validation.rs` | `crates/noa-ui-core/src/utils/validation.rs` *(planned)* |  |

## Non-Rust assets and templates

| rust-lovable path | Destination (ui/app) | Notes |
|---|---|---|
| `assets/styles.css` | `bins/noa-ui-web/assets/…` and/or `bins/noa-ui-desktop/assets/…` | Adopt lilrep pattern (`asset!("/assets/main.css")`). |
| `templates/*` | `crates/noa-ui-core/templates/*` *(planned)* | UI/codegen templates should be packaged with core. |
| `.env.example` | Merge into root NOA env model | Canonical secrets live in `n:\noa\.env` (untracked). App overrides in `ui/app/.env` (untracked). |
| `Dockerfile`, `docker-compose.yml` | `bins/noa-ui-server` / top-level orchestration | Revisit once server binary exists under ui/app.
