# Local inference (llama.cpp)

NOA can use a local `llama-server` (from the `opt/llama.cpp` submodule) to provide chat/completion responses.

## What you need

- A **GGUF model weights file** (e.g. `*.gguf`).
- The `llama-server` binary built at:
  - `opt/llama.cpp/build/bin/llama-server.exe` (Windows)

> Note: Model weights are intentionally not checked into the repo. The `opt/` directory is ignored by default.

## Quick setup

1. Download a GGUF model and place it somewhere on disk.

   Recommended convention (ignored by git):

   - `opt/models/default.gguf`

2. configsure the model path via environment:

   - `NOA_LLAMA_MODEL_PATH=opt/models/default.gguf`

   You can also override the llama-server binary path:

   - `NOA_LLAMA_SERVER_PATH=N:\noa\opt\llama.cpp\build\bin\llama-server.exe`

3. Start the Rust API (`noa-api`).

   The API will attempt to:

   - start `llama-server` automatically (unless `NOA_LLAMA_AUTO_START=false`)
   - call `POST /apply-template` then `POST /completion`

4. Start the UI (`sys/ui`).

   The UI calls `POST /api/v1/chat` and `POST /api/v1/chat/stream`.

   A `next.configs.mjs` rewrite proxies `/api/*` from the UI dev server to the Rust API server at `http://127.0.0.1:3001`.

## Useful endpoints

- `GET http://127.0.0.1:8080/health` — llama-server readiness
- `POST http://127.0.0.1:8080/apply-template` — converts chat messages to a prompt string
- `POST http://127.0.0.1:8080/completion` — text generation (supports SSE streaming with `"stream": true`)

## Troubleshooting

- **"Model file not found"**
  - Set `NOA_LLAMA_MODEL_PATH` to the full path of a real `.gguf` model.

- **"llama-server not found"**
  - Build llama.cpp, or set `NOA_LLAMA_SERVER_PATH`.

- **UI still 404s on `/api/v1/chat`**
  - Restart `next dev` after adding `next.configs.mjs`.
