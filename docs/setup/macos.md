# NOA Setup – macOS

## Requirements
- macOS 12+ (Intel or Apple Silicon)
- Git, unzip, bash

## Install
```bash
chmod +x ./scripts/setup/setup-noa.sh
./scripts/setup/setup-noa.sh --install-all-tools --install-ai-providers
source ./noa-profile.sh
```

Flags:
- `--install-all-tools` installs Rust, Go, Node, Python, protoc into `opt/`.
- `--install-ai-providers` installs provider CLIs (Claude Code, Codex, Cursor, Abacus).

## Verify
```bash
cd sys/core
cargo --version
node --version
python --version
cargo test --quiet
```

If you only want directory scaffolding, omit the install flags. Shared resources are created under `ai/shared/`.
