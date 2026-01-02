# NOA Gateway API

Shared API layer for NOA platform. Both ml_devops (Next.js) and noa-ui (Dioxus) connect to this API.

## Structure

```
gateway/api/
├── openapi.yaml       # OpenAPI 3.1 specification
├── README.md          # This file
└── client/            # Generated/shared client code
    ├── rust/          # Rust client for Dioxus UI
    └── typescript/    # TypeScript client for ml_devops
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/v1/status` | GET | System component status |
| `/api/v1/chat` | POST | Chat completion |
| `/api/v1/chat/stream` | POST | Streaming chat (SSE) |
| `/api/v1/providers` | GET | List AI providers |
| `/api/v1/providers/{id}/models` | GET | List models for provider |
| `/api/v1/tasks` | GET/POST | Task management |
| `/api/v1/tasks/{id}` | GET | Task details |

## Server Configuration

The API is served by `sys/core/crates/api` on port 3001.

### Provider Priority (from config/ai-providers.json)

1. **Local** (llama.cpp @ localhost:8080) - Always available offline
2. **Hybrid** (Cursor) - IDE context awareness
3. **Cloud** (Claude, OpenAI, Azure) - Complex reasoning

## Client Usage

### Dioxus (Rust)

```rust
use noa_api_client::{Client, ChatRequest};

let client = Client::new("http://localhost:3001");

// Chat
let response = client.chat(ChatRequest {
    message: "Hello".to_string(),
    provider: None,
    history: None,
}).await?;

// Stream
let mut stream = client.chat_stream(request).await?;
while let Some(event) = stream.next().await {
    match event.event.as_str() {
        "token" => print!("{}", event.data),
        "done" => break,
        _ => {}
    }
}
```

### Next.js (TypeScript)

```typescript
import { noaApi } from '@/lib/api';

// Chat
const response = await noaApi.chat({
  message: 'Hello',
  provider: 'llama.cpp',
});

// Stream
const stream = await noaApi.chatStream({ message: 'Hello' });
for await (const event of stream) {
  if (event.event === 'token') {
    process.stdout.write(event.data);
  }
}
```

## Development

### Generate Clients

```bash
# Rust client
cd gateway/api/client/rust
cargo build

# TypeScript client
cd gateway/api/client/typescript
pnpm build
```

### Validate OpenAPI Spec

```bash
npx @redocly/cli lint gateway/api/openapi.yaml
```
