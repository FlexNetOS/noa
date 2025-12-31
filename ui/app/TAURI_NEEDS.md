# Tauri needs (ui/app)

This file is the canonical checklist for what `ui/app` needs in order to ship a Tauri v2 desktop wrapper (and to keep NOA’s offline/portable toolchain complete).

## Rust crates (pin versions)

These are the core crates involved in Tauri v2 builds. Even when consumed transitively (e.g. via `tauri-build` / `tauri-macros`), we track explicit pins here for reproducibility.

- `tauri` (v2)
- `tauri-build` (v2)
- `tauri-macros` (v2)
- `tauri-codegen = 2.5.2`  ← explicitly required

Notes:
- `tauri-codegen` is typically a transitive dependency, but the version needs to be pinned to avoid lockfile drift and to ensure codegen asset generation stays reproducible.

## Node / CLI tooling

- Tauri CLI (v2) (either `cargo tauri` via `tauri-cli` or `@tauri-apps/cli` depending on chosen workflow)

## Windows runtime prerequisites

- Microsoft Edge WebView2 Runtime

