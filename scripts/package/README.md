# Packaging scripts (T731)

Cross-platform packaging helpers for Phase 13 (US9). Build `sys/core` in release mode first, then:

- Windows PowerShell: `.\scripts\package\package-noa.ps1 -Platform windows`
- Linux/macOS: `bash ./scripts/package/package-noa.sh linux` or `macos`

Artifacts are written to `dist/<platform>/` and include the compiled `noa` binary plus a timestamped README.
