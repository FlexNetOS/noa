# API Documentation

Generated references for the NOA APIs derived from the OpenAPI and protobuf contracts in `specs/001-noa-seed-foundation/contracts/`.

## Sources
- `noa-core.openapi.yaml` – core health, memory, agent, task, model, plane, and promotion endpoints.
- `digest-pipeline.openapi.yaml` – digest pipeline ingestion and knowledge endpoints.
- `p2p-protocol.proto` – P2P discovery, sync, storage, and compute services.

## Generate (local)
Use `redoc-cli` (Node) or `widdershins` for Markdown, and `grpcurl`/`protoc` for protobuf stubs.

```bash
# HTML bundles (Redoc)
npx redoc-cli bundle specs/001-noa-seed-foundation/contracts/noa-core.openapi.yaml -o docs/api/noa-core.html
npx redoc-cli bundle specs/001-noa-seed-foundation/contracts/digest-pipeline.openapi.yaml -o docs/api/digest-pipeline.html

# Markdown (Widdershins)
npx widdershins specs/001-noa-seed-foundation/contracts/noa-core.openapi.yaml -o docs/api/noa-core.md
npx widdershins specs/001-noa-seed-foundation/contracts/digest-pipeline.openapi.yaml -o docs/api/digest-pipeline.md

# Protobuf Markdown summary
python - <<'PY'
from pathlib import Path
spec = Path("specs/001-noa-seed-foundation/contracts/p2p-protocol.proto").read_text()
out = Path("docs/api/p2p-protocol.md")
out.write_text(f"# P2P Protocol\n\n```\n{spec}\n```\n")
print(f"Wrote {out}")
PY
```

Generated artifacts live in this folder and can be published as static assets.
