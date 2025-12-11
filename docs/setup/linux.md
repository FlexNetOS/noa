# NOA Setup – Linux

## Requirements
- Ubuntu/Debian/Fedora or similar
- bash, curl/wget, unzip

## Install
```bash
chmod +x ./scripts/setup/setup-noa.sh
./scripts/setup/setup-noa.sh --install-all-tools --install-ai-providers
source ./noa-profile.sh
```

Flags:
- `--install-all-tools` installs portable Rust, Go, Node, Python, and protoc into `opt/`.
- `--install-ai-providers` installs provider CLIs (Claude Code, Codex, Cursor, Abacus).

## Verify
```bash
cd sys/core
cargo --version
go version
node --version
cargo test --quiet
```

For directory creation only, run without the install flags. Shared resources land in `ai/shared/`.
