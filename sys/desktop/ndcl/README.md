# NOA Desktop Containment Layer (NDCL)

This directory hosts the Desktop Containment Layer that keeps third‑party desktop applications self‑contained inside `NOA_ROOT`.

- `../proxy/` contains the network proxy that routes and audits desktop app traffic.
- `../auth/` contains the OAuth proxy and credential vault used by desktop apps.
- Launcher wrappers in `bin/` redirect app data into `data/apps/<app>/` and point binaries located in `opt/apps/<app>/`.
- Configuration lives in `config/desktop-apps.json` with its schema in `config/schemas/desktop-apps.json`.

See `docs/architecture/desktop-app-hosting.md` for the full design.
