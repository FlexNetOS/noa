# Architecture Documentation - ML DevOps Platform

# Architecture Documentation - ML DevOps Platform v0.3.0

📚 [Quick Start](./QUICKSTART.md) | 🤖 [Agent System](./AGENT.md) | 🏗️ [NOA System](./NOA_INTEGRATION_FIXED.md) | ⚙️ [Setup Guide](./SETUP.md)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Event Sourcing Pattern](#event-sourcing-pattern)
- [Agent & Provider System](#agent--provider-system)
- [NOA (Next-generation Organic Architecture)](#noa-next-generation-organic-architecture)
- [Event System Architecture](#event-system-architecture)
- [Widget System](#widget-system)
- [Provider Abstraction](#provider-abstraction)
- [Configuration System](#configuration-system)
- [Real-time Collaboration](#real-time-collaboration)
- [Migration Path](#migration-path)

---

## Agent & Provider System

**→ See [AGENT.md](./AGENT.md)** for complete documentation on:

- AI Providers (Abacus AI, Local Qwen3, Claude CLI, Mock)
- MOE (Mixture of Experts) Router
- SONA (Sequential Orchestration) System
- Specialized Agents (Coder, Analyst, Reviewer, DeepCode)
- Configuration and environment setup
- Usage examples and API reference

This section focuses on the **architectural patterns**. For provider-specific details, see [AGENT.md](./AGENT.md).

---

## NOA (Next-generation Organic Architecture)

**→ See [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md)** for complete documentation on:

- 3-Layer hybrid configuration system
- Content-Addressable Storage (CAS)
- Mutation pipeline with hooks
- Global indexing and search
- Garbage collection
- Merkle DAG verification

This section focuses on how NOA integrates with the event system. For NOA-specific details, see [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md).

---

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
│   └── CONFIG_LOADED
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
        config: WidgetConfig,
    },
    // ... other variants
}
```

## Provider Abstraction

### Interface Design

```typescript
interface AIProvider {
  streamChat(messages: ChatMessage[], config?: ModelConfig): Promise<StreamingResponse>;
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
        config: Option<ModelConfig>,
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
                    AppEvent::WidgetMounted { widget_id, config, .. } => {
                        widgets.write().insert(widget_id, Widget::new(config));
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

switch (config.type) {
  case 'TextBlock':
    return <TextBlock {...config.props} />;
  case 'CodeBlock':
    return <CodeBlock {...config.props} />;
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

### 2. Dual Config System

```rust
// Compile-time config (TOML)
#[derive(Deserialize)]
struct CompileConfig {
    app_name: String,
    version: String,
}

// Runtime config (JSON)
#[derive(Deserialize)]
struct RuntimeConfig {
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

## JSON-Patch State Updates

### Efficient State Synchronization

The platform implements **RFC 6902 JSON Patch** for efficient widget state updates:

```typescript
// Instead of replacing entire state:
{
  type: 'WIDGET_UPDATED',
  widgetId: 'widget_123',
  updates: { props: { data: [/* large array */] } }  // Sends entire data
}

// Use JSON-patch for granular updates:
{
  type: 'WIDGET_PATCHED',
  widgetId: 'widget_123',
  patch: [
    { op: 'replace', path: '/props/data/0/value', value: 42 },  // Update single value
    { op: 'add', path: '/props/data/-', value: { x: 10, y: 20 } }  // Append item
  ]
}
```

### Benefits

1. **Bandwidth Efficiency**: Only send changed data, not entire state
2. **Performance**: Faster updates for large widget configurations
3. **Granular Control**: Target specific properties with path-based operations
4. **Conflict Resolution**: Easier to detect and handle concurrent updates

### Supported Operations

- `add`: Insert new values at a path
- `remove`: Delete values from a path
- `replace`: Update existing values
- `move`: Relocate values within the document
- `copy`: Duplicate values to a new path
- `test`: Verify values match before applying other operations

### Implementation

```typescript
import { applyPatch } from 'fast-json-patch';

// In WidgetRegistry
else if (event.type === 'WIDGET_PATCHED') {
  const e = event as WidgetPatchedEvent;
  setWidgets(prev => {
    const next = new Map(prev);
    const existing = next.get(e.widgetId);
    if (existing) {
      try {
        const clonedConfig = JSON.parse(JSON.stringify(existing.config));
        const patchedConfig = applyPatch(clonedConfig, e.patch).newDocument;
        next.set(e.widgetId, { ...existing, config: patchedConfig });
      } catch (error) {
        console.error('Failed to apply widget patch:', error);
      }
    }
    return next;
  });
}
```

### Rust Translation

```rust
use jsonpatch::Patch;
use serde_json::Value;

pub fn apply_widget_patch(
    widget: &mut WidgetConfig,
    patch: &Patch,
) -> Result<(), jsonpatch::PatchError> {
    let mut value = serde_json::to_value(&widget)?;
    jsonpatch::patch(&mut value, patch)?;
    *widget = serde_json::from_value(value)?;
    Ok(())
}
```

**Rust Crate**: `jsonpatch` for RFC 6902 JSON Patch operations

---

## Widget System

### 8 Widget Types

The platform supports a comprehensive widget library for ML DevOps workflows:

#### 1. **TextBlock** - Markdown Content Display
```typescript
{
  type: 'TextBlock',
  props: {
    content: '## Analysis Results\n\nModel accuracy: **94.3%**',
    markdown: true
  }
}
```
**Use cases**: Documentation, analysis summaries, formatted logs

#### 2. **CodeBlock** - Syntax-Highlighted Code
```typescript
{
  type: 'CodeBlock',
  props: {
    code: 'def train_model():\n    pass',
    language: 'python',
    showLineNumbers: true
  }
}
```
**Use cases**: Training scripts, generated code, configuration files

#### 3. **StatusIndicator** - System Status Display
```typescript
{
  type: 'StatusIndicator',
  props: {
    status: 'processing',  // idle | processing | success | error
    message: 'Training epoch 5/100...'
  }
}
```
**Use cases**: Pipeline status, job monitoring, health checks

#### 4. **SimpleChart** - Bar/Line Charts
```typescript
{
  type: 'SimpleChart',
  props: {
    title: 'Model Accuracy Over Time',
    data: [{ x: 1, y: 0.85 }, { x: 2, y: 0.91 }],
    type: 'line'
  }
}
```
**Use cases**: Training metrics, performance plots, loss curves

#### 5. **DataTable** - Sortable, Filterable Tables
```typescript
{
  type: 'DataTable',
  props: {
    title: 'Experiment Results',
    columns: [
      { key: 'model', label: 'Model' },
      { key: 'accuracy', label: 'Accuracy' },
      { key: 'latency', label: 'Latency (ms)' }
    ],
    data: [
      { model: 'BERT-base', accuracy: 0.923, latency: 45 },
      { model: 'GPT-2', accuracy: 0.945, latency: 123 }
    ],
    pageSize: 10,
    searchable: true
  }
}
```
**Use cases**: Model comparisons, hyperparameter logs, dataset previews

**Features**:
- Column sorting (ascending/descending)
- Full-text search across all columns
- Pagination with configurable page size
- Responsive design with horizontal scrolling

#### 6. **Graph** - Network Visualization
```typescript
{
  type: 'Graph',
  props: {
    title: 'Model Architecture',
    nodes: [
      { id: '1', label: 'Input', x: 100, y: 200 },
      { id: '2', label: 'Hidden', x: 300, y: 200 },
      { id: '3', label: 'Output', x: 500, y: 200 }
    ],
    edges: [
      { from: '1', to: '2', weight: 2 },
      { from: '2', to: '3', weight: 1 }
    ],
    width: 600,
    height: 400
  }
}
```
**Use cases**: Neural network architectures, dependency graphs, pipeline DAGs

**Features**:
- Interactive zoom and pan
- Automatic or manual node positioning
- Edge labels and weights
- Click interactions for node details

#### 7. **ImageViewer** - Image Display with Controls
```typescript
{
  type: 'ImageViewer',
  props: {
    title: 'Training Sample',
    src: '/data/image_001.png',
    alt: 'Training image',
    width: 600,
    height: 400,
    downloadable: true
  }
}
```
**Use cases**: Dataset samples, generated images, visualization outputs

**Features**:
- Zoom in/out (up to 5x)
- Pan when zoomed
- 90\u00b0 rotation
- Download functionality
- Responsive container

#### 8. **VideoPlayer** - Video Playback
```typescript
{
  type: 'VideoPlayer',
  props: {
    title: 'Training Progress',
    src: '/data/training_progress.mp4',
    poster: '/data/poster.png',
    width: 640,
    height: 360,
    autoPlay: false,
    loop: false
  }
}
```
**Use cases**: Training visualizations, demo videos, animated results

**Features**:
- Standard playback controls (play/pause/seek)
- Volume control and mute
- Skip forward/backward (10 seconds)
- Fullscreen support
- Custom controls overlay

### Widget Lifecycle

```
Event Flow:
1. WIDGET_MOUNTED → Create and mount widget
2. WIDGET_UPDATED → Replace partial state (full object merge)
3. WIDGET_PATCHED → Apply JSON patches (efficient updates)
4. WIDGET_UNMOUNTED → Remove widget from DOM
```

### Rust/Dioxus Widget Translation

Each widget is designed for straightforward Rust translation:

```rust
#[component]
pub fn DataTable(cx: Scope, props: DataTableProps) -> Element {
    let sort_column = use_state(cx, || None);
    let search_query = use_state(cx, || String::new());
    
    let filtered_data = use_memo(cx, (&search_query,), |query| {
        filter_and_sort(&props.data, query, sort_column.get())
    });
    
    cx.render(rsx! {
        div { class: "data-table",
            // Table rendering with Dioxus components
        }
    })
}
```

**Rust Crates for Widgets**:
- `plotters` - Charts and graphs
- `egui_extras::Table` - Advanced tables
- `image` - Image loading/manipulation
- `gstreamer` - Video playback (via FFI)

---

## Configuration System

### Dual Config Architecture

The platform implements a **portable configuration system** with three layers:

```
Priority (highest first):
1. Environment Variables (.env)
2. Installed Config (user-specific)
3. Portable Config (project defaults)
4. Hardcoded Defaults
```

### Cross-Platform Paths

**Portable Config** (git-tracked):
```
./config/
├── app.json          # Application metadata
├── features.json     # Feature flags
├── providers.json    # Provider settings (no secrets)
└── ui.json           # UI preferences
```

**Installed Config** (user-specific):
- Windows: `%APPDATA%/ml-devops/config.json`
- macOS: `~/Library/Application Support/ml-devops/config.json`
- Linux: `~/.config/ml-devops/config.json` (XDG)

**Secrets** (never in git):
- `.env` file (preferred)
- `~/.config/ml-devops/secrets.json` (fallback)

### Config Loader

```typescript
import { getConfig } from './lib/config';

const config = getConfig();
console.log(config.providers.ai.type); // 'abacus', 'ruvllm', 'candle', etc.
```

**Features:**
- Deep merge of all config layers
- Type-safe with TypeScript interfaces
- Validation with helpful error messages
- Secret masking for logs

### Rust Translation

```rust
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub features: Features,
    pub providers: Providers,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            // 1. Defaults
            .add_source(File::from_str(
                include_str!("../config/defaults.toml"),
                FileFormat::Toml,
            ))
            // 2. Portable config
            .add_source(File::with_name("config/app").required(false))
            // 3. Installed config
            .add_source(File::from(ConfigPaths::get_installed_config_path()).required(false))
            // 4. Environment overrides
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
            
        config.try_deserialize()
    }
}
```

**Key Rust crates:**
- `config` - Configuration management
- `dirs` - Cross-platform directories
- `serde` - Serialization/deserialization
- `toml` - TOML parsing (preferred over JSON in Rust)

### Benefits

1. **Portability**: Works on Windows, macOS, Linux
2. **Security**: Secrets never in git-tracked files
3. **Flexibility**: Users can override any setting
4. **Maintainability**: Config split into logical files
5. **Rust-Ready**: Designed to map to Rust TOML config


## Rust Integration Strategy

### Phase B: Documentation Complete ✅

Comprehensive integration guides for three core Rust technologies have been created:

#### 1. **ruvLLM - Self-Optimizing Neural Architecture (SONA)**

A continuous learning system that enhances any LLM through:
- **Temporal Learning**: Instant (<100µs), Background (hourly), Deep (weekly) loops
- **MicroLoRA Adaptation**: Per-request model tuning (rank 1-2)
- **Intelligent Routing**: FastGRNN model selection
- **Anti-Forgetting**: EWC++ with optimal lambda=2000
- **Pattern Extraction**: ReasoningBank with K-means++ clustering

**Performance**: ~0.09ms orchestration latency, 38,000 queries/sec, 50MB memory footprint

#### 2. **Rig Framework - LLM Application Builder**

Modular framework for building production-grade LLM apps:
- **Unified Interface**: Single API for OpenAI, Anthropic, Cohere, etc.
- **RAG Support**: Built-in vector store integration (MongoDB, Qdrant, etc.)
- **Tool System**: Custom tool definitions with type-safe execution
- **Multi-Provider Fallback**: Automatic failover between cloud and local
- **Type Safety**: Compile-time correctness with Rust's type system

**Use Cases**: Documentation search, model benchmarking, multi-turn conversations

#### 3. **Candle-vLLM - Local Inference Engine**

Efficient local LLM serving with:
- **OpenAI API Compatibility**: Drop-in replacement for cloud APIs
- **PagedAttention**: Efficient KV cache management
- **Quantization**: Q4/Q8 GGUF, GPTQ/Marlin (2x speedup)
- **Multi-GPU**: Tensor parallelism with NCCL
- **Streaming**: Real-time token generation via SSE

**Performance**: 553 tok/s (LLaMA3.1-8B batch 16), 800+ tok/s with Q4 quantization

### Integration Patterns

Three primary deployment patterns documented:

1. **Hybrid Cloud/Local**: Intelligent routing based on cost/complexity
2. **RAG + Local Inference**: Vector search + local LLM generation
3. **Full Stack (SONA + Rig + Candle)**: Complete self-optimizing system

### Documentation

See [RUST_INTEGRATION.md](./RUST_INTEGRATION.md) for:
- Complete API documentation
- Code examples for all three libraries
- Docker and Kubernetes deployment configs
- Performance benchmarks
- Troubleshooting guide
- Migration timeline (Phases 2-4)

### Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Next.js Frontend (Current)              │
└───────────────────────┬─────────────────────────────────┘
                        │ WebSocket/HTTP
┌───────────────────────┴─────────────────────────────────┐
│              TypeScript Bridge Layer (Phase 2)           │
│    Event routing, API gateway, WebSocket management     │
└───────────────────────┬─────────────────────────────────┘
                        │ gRPC/HTTP
┌───────────────────────┴─────────────────────────────────┐
│                  Rust Backend (Phase 2-3)                │
├─────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐          │
│  │ ruvLLM   │──│ Rig      │──│ Candle-vLLM  │          │
│  │ SONA     │  │ Agent    │  │ Inference    │          │
│  └──────────┘  └──────────┘  └──────────────┘          │
│                                                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │   Event Sourcing (SQLite/PostgreSQL via SQLx)    │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Next Steps

**Phase C (Weeks 7-8)**: Tauri v2 Wrapper
- Set up Tauri project structure
- Implement HTTP/WebSocket bridge to Rust
- Test desktop builds (Windows, macOS, Linux)

**Phase D (Weeks 9-12)**: Full Rust Migration
- Replace Next.js with Dioxus web
- Migrate widget system to Rust
- Deploy as native desktop application
- Optimize binary size (<20MB)

## Migration Path

### Phase 1: Current (Complete)
- ✅ Event sourcing with TypeScript
- ✅ Widget registry system with 8 widget types
- ✅ JSON-patch efficient state updates
- ✅ Streaming chat with LLM API
- ✅ Event persistence with PostgreSQL
- ✅ Event replay UI
- ✅ Dual config system with cross-platform support

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
- ▢ Dual config system
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