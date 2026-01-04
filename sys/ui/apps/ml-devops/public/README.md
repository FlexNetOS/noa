# ML DevOps Platform - Event-Driven Architecture MVP

## Overview

This is a reference implementation of an event-driven ML DevOps platform built with Next.js, TypeScript, and PostgreSQL. The platform demonstrates event sourcing patterns that will be ported to Tauri v2 + Dioxus with Rust-based ML inference.

### Key Features

- ⚡ **Event Sourcing**: Append-only event stream as single source of truth
- 💬 **Streaming Chat**: Token-by-token AI responses with full event tracking
- 🧩 **Widget System**: Dynamic widget registry supporting 4 widget types
- 💾 **Event Persistence**: Save and load event streams from database
- ⏯️ **Event Replay**: Reconstruct UI state from saved event streams
- 🔧 **Event Simulator**: Testing tool for manual event triggering
- 🧠 **Provider Abstraction**: Pluggable AI provider interface

## Architecture Overview

### Event Flow Architecture

```
┌──────────────────────────────────────────────────┐
│                 User Interactions                      │
│  (Chat, Widget Actions, Simulator, Replay)           │
└───────────────┬──────────────────────────────────┘
                │
                │ emit(event)
                │
                ▼
┌──────────────────────────────────────────────────┐
│            EventStream (Append-Only)                 │
│                                                      │
│  ┌──────────────────────────────────────────┐  │
│  │ Event 1: MESSAGE_SENT                    │  │
│  │ Event 2: TOKEN_STREAMED                  │  │
│  │ Event 3: TOKEN_STREAMED                  │  │
│  │ Event 4: WIDGET_MOUNTED                  │  │
│  │ Event 5: STATUS_CHANGED                  │  │
│  │ ...                                      │  │
│  └──────────────────────────────────────────┘  │
└─────────────────┬────────────────────────────────┘
                │
                │ subscribe(handler)
                │
     ┌──────────┼─────────────────┐
     │           │                │
     ▼           ▼                ▼
┌─────────┐  ┌─────────┐  ┌─────────────┐
│  Chat   │  │ Widgets │  │ Persistence │
│Interface│  │Registry│  │   (DB)     │
└─────────┘  └─────────┘  └─────────────┘
     │           │                │
     ▼           ▼                ▼
┌──────────────────────────────────────────────────┐
│              UI Components                           │
│       (React re-renders on event changes)           │
└──────────────────────────────────────────────────┘
```

### Event Types

All system state changes flow through these event types:

#### Chat Events
- `MESSAGE_SENT` - User or assistant message
- `TOKEN_STREAMED` - Individual token from streaming response
- `MESSAGE_COMPLETED` - Message fully received

#### Widget Events
- `WIDGET_MOUNTED` - Widget added to registry
- `WIDGET_UPDATED` - Widget props updated
- `WIDGET_UNMOUNTED` - Widget removed from registry

#### System Events
- `STATUS_CHANGED` - System status update (idle/processing/success/error)
- `configs_LOADED` - configsuration loaded

#### Replay Events
- `EVENT_STREAM_SAVED` - Event stream persisted to database
- `EVENT_STREAM_LOADED` - Event stream loaded from database
- `REPLAY_STARTED` - Event replay initiated
- `REPLAY_PAUSED` - Event replay paused
- `REPLAY_COMPLETED` - Event replay finished

## Widget Lifecycle

### Mount Phase
```typescript
emit({
  type: 'WIDGET_MOUNTED',
  widgetId: 'widget_123',
  configs: {
    type: 'TextBlock',
    props: { content: '# Hello', markdown: true }
  }
});
```

### Update Phase
```typescript
emit({
  type: 'WIDGET_UPDATED',
  widgetId: 'widget_123',
  updates: {
    props: { content: '# Updated Content' }
  }
});
```

### Unmount Phase
```typescript
emit({
  type: 'WIDGET_UNMOUNTED',
  widgetId: 'widget_123'
});
```

## Supported Widgets

### 1. TextBlock
- **Purpose**: Markdown content rendering
- **Props**: `content: string`, `markdown?: boolean`
- **Use Cases**: Documentation, explanations, formatted text

### 2. CodeBlock
- **Purpose**: Syntax-highlighted code display
- **Props**: `code: string`, `language?: string`, `showLineNumbers?: boolean`
- **Use Cases**: Code snippets, configsuration files

### 3. StatusIndicator
- **Purpose**: System status visualization
- **Props**: `status: 'idle' | 'processing' | 'success' | 'error'`, `message?: string`
- **Use Cases**: Pipeline status, health checks, alerts

### 4. SimpleChart
- **Purpose**: Basic data visualization
- **Props**: `title?: string`, `data: Array<{name, value}>`, `type?: 'bar' | 'line'`
- **Use Cases**: Metrics, analytics, trends

## Provider Abstraction

The `AIProvider` interface enables swappable AI backends:

```typescript
interface AIProvider {
  streamChat(messages: ChatMessage[], configs?: Modelconfigs): Promise<StreamingResponse>;
  generateWidget(prompt: string): Promise<WidgetGeneration>;
  analyzeCode(code: string, language: string): Promise<string>;
  getName(): string;
}
```

### Current Providers

1. **MockAIProvider** - Simulated responses for testing
2. **AbacusAIProvider** - Production LLM API integration

### Future Providers (Phase 2)

3. **RuvllmProvider** - Rust VLLM bindings
4. **RigProvider** - LLM operations framework
5. **CandleProvider** - Pure Rust inference

## Quick Start

See [SETUP.md](./SETUP.md) for detailed setup instructions.

```bash
# Install dependencies
cd nextjs_space
yarn install

# Setup database
yarn prisma generate
yarn prisma db push
yarn prisma db seed

# Run development server
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) to see the platform.

## Project Structure

```
nextjs_space/
├── app/
│   ├── api/              # API routes (chat, events)
│   ├── docs/             # Documentation page
│   ├── layout.tsx        # Root layout
│   └── page.tsx          # Main dashboard
├── components/
│   ├── chat/             # Chat interface components
│   ├── widgets/          # Widget implementations
│   ├── simulator/        # Event simulator
│   └── replay/           # Event replay UI
├── lib/
│   ├── events/           # Event system (types, stream, store)
│   ├── providers/        # AI provider abstractions
│   └── hooks/            # React hooks for event stream
├── prisma/
│   └── schema.prisma     # Database schema
├── scripts/
│   └── seed.ts           # Database seed script
└── configs.json          # Application configsuration
```

## Usage Examples

### Emitting Events

```typescript
import { useEventEmitter } from '@/lib/hooks/use-event-stream';
import { EventFactory } from '@/lib/events/types';

function MyComponent() {
  const { emit } = useEventEmitter();

  const handleAction = () => {
    emit(EventFactory.createEvent('STATUS_CHANGED', {
      status: 'processing',
      message: 'Operation started'
    }));
  };

  return <button onClick={handleAction}>Start</button>;
}
```

### Subscribing to Events

```typescript
import { useEventStream } from '@/lib/hooks/use-event-stream';

function EventListener() {
  const [events] = useEventStream('MESSAGE_SENT');

  return (
    <div>
      {events.map(event => (
        <div key={event.id}>{event.content}</div>
      ))}
    </div>
  );
}
```

## Event Replay

1. Interact with the platform (send messages, mount widgets)
2. Click "Save" in the Event Replay panel
3. Click "Reset" to clear the UI
4. Click "Load" to restore the saved stream
5. Adjust replay speed and click "Replay" to watch the UI reconstruct

## Testing with Event Simulator

The Event Simulator provides one-click testing:

- **Simulate Streaming Message** - Triggers token-by-token streaming
- **Mount Widgets** - Tests widget lifecycle
- **Status Updates** - Changes system status

## Next Steps

See [phase2_tasks.csv](./phase2_tasks.csv) for the roadmap to Tauri + Rust migration.

Key Phase 2 milestones:
1. Port to Tauri v2 with Dioxus UI
2. Integrate ruvllm for Rust-based inference
3. Add Rig framework for LLM ops
4. Implement candle-vllm for local models
5. JSON-patch updates for efficient state sync
6. Dual configs system (runtime + compile-time)

## Architecture Details

For in-depth architecture information, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## License

MIT
