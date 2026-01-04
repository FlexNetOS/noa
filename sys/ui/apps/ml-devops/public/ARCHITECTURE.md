# Architecture Documentation - ML DevOps Platform

## Overview

This document provides an in-depth explanation of the event-sourcing architecture and how it maps to the future Rust implementation.

## Event Sourcing Pattern

### Core Principles

1. **Append-Only Event Log**: All state changes are recorded as immutable events
2. **Single Source of Truth**: The event stream is the canonical data source
3. **State Reconstruction**: Current state is derived by replaying events
4. **Time-Travel Debugging**: Can replay to any point in time
5. **Full Audit Trail**: Every change is recorded with timestamp and metadata

### Why Event Sourcing?

For ML DevOps workflows, event sourcing provides:

- **Reproducibility**: Re-run any pipeline by replaying events
- **Debugging**: Understand exactly what happened and when
- **Compliance**: Complete audit trail for regulated environments
- **Testing**: Replay production events in development
- **Analytics**: Analyze patterns across all historical events

## Event System Architecture

### EventStream Class

```typescript
class EventStream {
  private events: AppEvent[] = [];  // Append-only log
  private handlers: Set<EventHandler> = new Set();  // Subscribers

  append(event: AppEvent): void  // Only way to modify state
  subscribe(handler: EventHandler): () => void  // Register listener
  replay(delayMs: number): Promise<void>  // Replay with visualization
}
```

**Rust Translation:**

```rust
use tokio::sync::broadcast;
use std::sync::{Arc, Mutex};

pub struct EventStream {
    events: Arc<Mutex<Vec<AppEvent>>>,
    tx: broadcast::Sender<AppEvent>,
}

impl EventStream {
    pub fn append(&self, event: AppEvent) {
        let mut events = self.events.lock().unwrap();
        events.push(event.clone());
        let _ = self.tx.send(event);  // Broadcast to subscribers
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.tx.subscribe()
    }

    pub async fn replay(&self, delay_ms: u64) {
        // Implementation with tokio::time::interval
    }
}
```

### Event Types Hierarchy

```
AppEvent (discriminated union)
├── ChatEvents
│   ├── MESSAGE_SENT
│   ├── TOKEN_STREAMED
│   └── MESSAGE_COMPLETED
├── WidgetEvents
│   ├── WIDGET_MOUNTED
│   ├── WIDGET_UPDATED
│   └── WIDGET_UNMOUNTED
├── SystemEvents
│   ├── STATUS_CHANGED
│   └── configs_LOADED
└── ReplayEvents
    ├── REPLAY_STARTED
    ├── REPLAY_PAUSED
    └── REPLAY_COMPLETED
```

**Rust Translation:**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    MessageSent {
        id: String,
        timestamp: i64,
        message_id: String,
        content: String,
        role: Role,
    },
    TokenStreamed {
        id: String,
        timestamp: i64,
        message_id: String,
        token: String,
        is_complete: bool,
    },
    WidgetMounted {
        id: String,
        timestamp: i64,
        widget_id: String,
        configs: Widgetconfigs,
    },
    // ... other variants
}
```

## Provider Abstraction

### Interface Design

```typescript
interface AIProvider {
  streamChat(messages: ChatMessage[], configs?: Modelconfigs): Promise<StreamingResponse>;
  generateWidget(prompt: string): Promise<WidgetGeneration>;
  analyzeCode(code: string, language: string): Promise<string>;
  getName(): string;
}
```

**Purpose**: Abstract away the underlying AI implementation, allowing swappable backends.

### Rust Trait Mapping

```rust
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn stream_chat(
        &self,
        messages: Vec<ChatMessage>,
        configs: Option<Modelconfigs>,
    ) -> Result<StreamingResponse>;

    async fn generate_widget(&self, prompt: String) -> Result<WidgetGeneration>;

    async fn analyze_code(&self, code: String, language: String) -> Result<String>;

    fn get_name(&self) -> &str;
}
```

### Provider Implementations

#### 1. Mock Provider (MVP)

```typescript
class MockAIProvider implements AIProvider {
  async streamChat(...) {
    // Simulated token-by-token streaming
    return this.tokenizeAndStream(mockResponse);
  }
}
```

**Rust:**

```rust
pub struct MockProvider;

#[async_trait]
impl AIProvider for MockProvider {
    async fn stream_chat(&self, ...) -> Result<StreamingResponse> {
        // Simulate streaming with tokio::time::sleep
    }
}
```

#### 2. Abacus AI Provider (Current)

```typescript
class AbacusAIProvider implements AIProvider {
  async streamChat(...) {
    const response = await fetch('/api/chat', {...});
    return this.streamFromAPI(response);
  }
}
```

**Rust:**

```rust
pub struct AbacusProvider {
    client: reqwest::Client,
    api_key: String,
}

#[async_trait]
impl AIProvider for AbacusProvider {
    async fn stream_chat(&self, ...) -> Result<StreamingResponse> {
        let response = self.client.post("https://apps.abacus.ai/v1/chat/completions")
            .json(&request)
            .send()
            .await?;
        
        // Return stream
        Ok(StreamingResponse::new(response.bytes_stream()))
    }
}
```

#### 3. Ruvllm Provider (Phase 2)

**Rust:**

```rust
use ruvllm::{VllmEngine, SamplingParams};

pub struct RuvllmProvider {
    engine: Arc<VllmEngine>,
}

#[async_trait]
impl AIProvider for RuvllmProvider {
    async fn stream_chat(&self, messages: Vec<ChatMessage>, ...) -> Result<StreamingResponse> {
        let prompt = self.format_prompt(messages);
        let params = SamplingParams::default();
        
        // Direct inference with VLLM
        let stream = self.engine.generate_stream(&prompt, params).await?;
        Ok(StreamingResponse::from_vllm_stream(stream))
    }
}
```

#### 4. Rig Provider (Phase 2)

**Rust:**

```rust
use rig::providers::openai::Client;
use rig::completion::CompletionRequest;

pub struct RigProvider {
    client: Client,
}

#[async_trait]
impl AIProvider for RigProvider {
    async fn stream_chat(&self, ...) -> Result<StreamingResponse> {
        let request = CompletionRequest::new(messages);
        let stream = self.client.completion_stream(request).await?;
        Ok(StreamingResponse::from_rig_stream(stream))
    }
}
```

#### 5. Candle Provider (Phase 2)

**Rust:**

```rust
use candle_vllm::{Model, TextGeneration};

pub struct CandleProvider {
    model: Arc<Model>,
}

#[async_trait]
impl AIProvider for CandleProvider {
    async fn stream_chat(&self, ...) -> Result<StreamingResponse> {
        // Pure Rust inference with Candle
        let generation = TextGeneration::new(self.model.clone());
        let stream = generation.generate_stream(prompt).await?;
        Ok(StreamingResponse::from_candle_stream(stream))
    }
}
```

## State Management

### Current (React)

```typescript
const [state, setState] = useState<T>(initialState);

useEffect(() => {
  const unsubscribe = stream.subscribe((event) => {
    if (event.type === 'WIDGET_MOUNTED') {
      setState(prev => ({ ...prev, widgets: [...prev.widgets, event] }));
    }
  });
  return unsubscribe;
}, []);
```

### Future (Dioxus)

```rust
use dioxus::prelude::*;

#[component]
fn WidgetRegistry(cx: Scope) -> Element {
    let widgets = use_signal(cx, HashMap::<String, Widget>::new);
    let stream = use_shared_state::<EventStream>(cx)?;

    use_coroutine(cx, |mut rx: UnboundedReceiver<AppEvent>| {
        to_owned![widgets];
        async move {
            while let Some(event) = rx.next().await {
                match event {
                    AppEvent::WidgetMounted { widget_id, configs, .. } => {
                        widgets.write().insert(widget_id, Widget::new(configs));
                    }
                    _ => {}
                }
            }
        }
    });

    render! {
        for widget in widgets.read().values() {
            WidgetComponent { widget: widget.clone() }
        }
    }
}
```

## Widget System Architecture

### Widget Registry Pattern

**Current (TypeScript):**

```typescript
const widgets = new Map<string, WidgetInstance>();

switch (configs.type) {
  case 'TextBlock':
    return <TextBlock {...configs.props} />;
  case 'CodeBlock':
    return <CodeBlock {...configs.props} />;
  // ...
}
```

**Future (Rust/Dioxus):**

```rust
use std::collections::HashMap;

enum Widget {
    TextBlock(TextBlockProps),
    CodeBlock(CodeBlockProps),
    StatusIndicator(StatusProps),
    SimpleChart(ChartProps),
}

impl Widget {
    fn render<'a>(&self, cx: Scope<'a>) -> Element<'a> {
        match self {
            Widget::TextBlock(props) => render! { TextBlock { props: props.clone() } },
            Widget::CodeBlock(props) => render! { CodeBlock { props: props.clone() } },
            // ...
        }
    }
}

#[component]
fn WidgetRegistry<'a>(cx: Scope<'a>, widgets: &'a HashMap<String, Widget>) -> Element {
    render! {
        for (id, widget) in widgets {
            div { key: "{id}", widget.render(cx) }
        }
    }
}
```

## Persistence Layer

### Current (PostgreSQL + Prisma)

```typescript
const eventLog = await prisma.eventLog.create({
  data: { name, events: JSON.stringify(events) }
});
```

### Future (PostgreSQL + sqlx)

```rust
use sqlx::{PgPool, query_as};

#[derive(sqlx::FromRow)]
struct EventLog {
    id: String,
    name: String,
    events: sqlx::types::JsonValue,  // JSONB column
    created_at: chrono::DateTime<chrono::Utc>,
}

async fn save_event_log(pool: &PgPool, name: String, events: Vec<AppEvent>) -> Result<EventLog> {
    let events_json = serde_json::to_value(&events)?;
    
    query_as!(EventLog,
        "INSERT INTO event_logs (name, events) VALUES ($1, $2) RETURNING *",
        name,
        events_json
    )
    .fetch_one(pool)
    .await
}
```

## Communication Patterns

### NextJS (Current)

```
React Components ↔ API Routes ↔ LLM API
       │               │
       └───────────────┘
         EventStream
```

### Tauri v2 (Future)

```
Dioxus Components ↔ Tauri Commands ↔ Rust Backend
       │                    │
       │                    ├─── ruvllm (VLLM)
       │                    ├─── Rig (LLM Ops)
       │                    └─── Candle (Local)
       │
       └────────── EventStream (tokio channels)
```

**Tauri Command Example:**

```rust
#[tauri::command]
async fn stream_chat(
    state: tauri::State<'_, AppState>,
    messages: Vec<ChatMessage>,
) -> Result<String, String> {
    let provider = state.ai_provider.lock().await;
    let response = provider.stream_chat(messages, None).await
        .map_err(|e| e.to_string())?;
    
    // Emit events to frontend
    for await token in response.tokens {
        state.event_stream.append(AppEvent::TokenStreamed {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            message_id: response.message_id.clone(),
            token: token.clone(),
            is_complete: false,
        });
        
        // Send to frontend via Tauri event
        state.app.emit_all("token-streamed", &token)?;
    }
    
    Ok(response.message_id)
}
```

## Performance Considerations

### Event Stream Size

**Problem**: Unbounded growth of in-memory event log

**Solution**: Implement event log rotation

```rust
const MAX_EVENTS: usize = 10_000;

impl EventStream {
    pub async fn append(&self, event: AppEvent) {
        let mut events = self.events.lock().await;
        
        if events.len() >= MAX_EVENTS {
            // Archive old events to database
            let old_events = events.drain(0..1000).collect();
            self.archive_events(old_events).await?;
        }
        
        events.push(event.clone());
        let _ = self.tx.send(event);
    }
}
```

### Widget Rendering

**Problem**: Re-rendering all widgets on every event

**Solution**: Use memoization and keyed rendering

```rust
// Dioxus uses keyed rendering automatically
render! {
    for (id, widget) in widgets.read().iter() {
        div { key: "{id}",  // Prevents unnecessary re-renders
            widget.render(cx)
        }
    }
}
```

### Database Queries

**Problem**: Loading large event logs

**Solution**: Pagination and lazy loading

```rust
async fn list_event_logs(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<EventLog>> {
    query_as!(EventLog,
        "SELECT id, name, created_at FROM event_logs 
         ORDER BY created_at DESC 
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
```

## Future Enhancements (Phase 2+)

### 1. JSON-Patch Updates

Instead of full state replacement, use RFC 6902 JSON Patch:

```rust
use json_patch::Patch;

pub enum StateUpdate {
    Full(State),
    Patch(Patch),
}
```

### 2. Dual configs System

```rust
// Compile-time configs (TOML)
#[derive(Deserialize)]
struct Compileconfigs {
    app_name: String,
    version: String,
}

// Runtime configs (JSON)
#[derive(Deserialize)]
struct Runtimeconfigs {
    provider_type: String,
    model: String,
}
```

### 3. Event Compression

```rust
use flate2::write::GzEncoder;

fn compress_events(events: &[AppEvent]) -> Result<Vec<u8>> {
    let json = serde_json::to_vec(events)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&json)?;
    Ok(encoder.finish()?)
}
```

### 4. Event Snapshots

```rust
#[derive(Serialize, Deserialize)]
struct Snapshot {
    timestamp: i64,
    state: AppState,
    last_event_id: String,
}

// Replay from snapshot instead of beginning
impl EventStream {
    pub async fn replay_from_snapshot(&self, snapshot: Snapshot) -> Result<()> {
        // Load state from snapshot
        // Replay only events after snapshot
    }
}
```

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_stream_append() {
        let stream = EventStream::new();
        let event = AppEvent::StatusChanged { /*...*/ };
        
        stream.append(event.clone());
        
        let events = stream.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event);
    }

    #[tokio::test]
    async fn test_widget_lifecycle() {
        let stream = EventStream::new();
        let mut rx = stream.subscribe();
        
        // Mount widget
        stream.append(AppEvent::WidgetMounted { /*...*/ });
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, AppEvent::WidgetMounted { .. }));
        
        // Update widget
        stream.append(AppEvent::WidgetUpdated { /*...*/ });
        // ...
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_end_to_end_chat_flow() {
    let provider = MockProvider::new();
    let stream = EventStream::new();
    
    // Send message
    let response = provider.stream_chat(vec![/*...*/], None).await.unwrap();
    
    // Collect tokens
    let mut tokens = Vec::new();
    for await token in response.tokens {
        tokens.push(token);
        stream.append(AppEvent::TokenStreamed { /*...*/ });
    }
    
    // Verify events
    let events = stream.get_events();
    assert!(events.iter().any(|e| matches!(e, AppEvent::TokenStreamed { .. })));
}
```

## Migration Path

### Phase 1: Current (Complete)
- ✅ Event sourcing with TypeScript
- ✅ Widget registry system
- ✅ Streaming chat with LLM API
- ✅ Event persistence with PostgreSQL
- ✅ Event replay UI

### Phase 2: Rust Backend
- ▢ Port event system to Rust
- ▢ Implement ruvllm provider
- ▢ Add Rig framework integration
- ▢ Setup candle-vllm for local inference
- ▢ Migrate database layer to sqlx

### Phase 3: Tauri + Dioxus
- ▢ Port React components to Dioxus
- ▢ Setup Tauri v2 with IPC
- ▢ Implement event bridge (Rust ↔ Frontend)
- ▢ Optimize for desktop performance
- ▢ Add native OS integration

### Phase 4: Advanced Features
- ▢ JSON-patch state updates
- ▢ Event compression and snapshots
- ▢ Dual configs system
- ▢ Multi-provider support
- ▢ Plugin system

## Conclusion

This architecture provides:

1. **Clean Separation**: Events, state, and UI are decoupled
2. **Testability**: Every component can be tested in isolation
3. **Debuggability**: Full event history enables time-travel debugging
4. **Portability**: Patterns map cleanly to Rust/Dioxus
5. **Scalability**: Event stream can be distributed and sharded

The MVP demonstrates these patterns with Next.js, providing a solid reference for the Rust implementation.
