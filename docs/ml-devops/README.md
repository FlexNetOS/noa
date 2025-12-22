# ML DevOps Platform v0.3.0

**Event-Driven ML DevOps Platform with Intelligent AI Orchestration**

🚀 [Quick Start](./QUICKSTART.md) | 📚 [Documentation](./ARCHITECTURE.md) | 🤖 [Agent System](./AGENT.md) | ⚙️ [Setup Guide](./SETUP.md)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Recent Updates](#recent-updates-v030)
- [Quick Start](#-quick-start)
- [Documentation](#-documentation)
- [Agent & Provider System](#-agent--provider-system)
- [NOA System](#-noa-system)
- [Key Features](#key-features)
- [Architecture](#architecture-overview)

---

## 🚀 Quick Start

**Get running in 5 minutes:**

```bash
cd nextjs_space
yarn install
cp .env.example .env
# Add ABACUSAI_API_KEY to .env
yarn prisma generate && yarn prisma db push
yarn dev
```

📖 **Full guide**: See [QUICKSTART.md](./QUICKSTART.md)

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[QUICKSTART.md](./QUICKSTART.md)** | 5-minute setup guide |
| **[AGENT.md](./AGENT.md)** | **AI Provider & Agent System** ⭐ |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | System architecture & design |
| [SETUP.md](./SETUP.md) | Detailed setup instructions |
| [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md) | NOA configuration system |
| [RUST_INTEGRATION.md](./RUST_INTEGRATION.md) | Rust backend & local inference |
| [E2E_TESTING.md](./E2E_TESTING.md) | End-to-end testing guide |

---

## 🤖 Agent & Provider System

**📖 [AGENT.md](./AGENT.md) is the single source of truth** for all provider and agent documentation.

### What's in AGENT.md:

✅ **AI Providers** - Abacus AI (default), Local Qwen3, Claude CLI, Mock  
✅ **MOE Router** - Intelligent task-based routing with fallback chains  
✅ **SONA Orchestrator** - Multi-agent workflows (5 execution strategies)  
✅ **Specialized Agents** - Coder, Analyst, Reviewer, DeepCode agents  
✅ **Configuration** - Complete setup guide with environment variables  
✅ **Usage Examples** - Real code examples for all scenarios  
✅ **API Reference** - Complete TypeScript interfaces  
✅ **Best Practices** - Production-ready patterns and troubleshooting  

### Quick Provider Reference:

| Provider | Type | Status | Best For |
|----------|------|--------|----------|
| **Abacus AI** | `abacus` | ✅ Active | Chat, code, analysis (default) |
| **Local Qwen3** | `ruvllm` | ✅ Available | Offline, privacy, 32K context |
| **Claude CLI** | `claude_cli` | 🔄 Planned | Reasoning, long context |
| **GitHub Copilot** | `github_copilot` | 🔄 Planned | Code completion |
| **Mock** | `mock` | ✅ Always | Development, testing |

**→ See [AGENT.md](./AGENT.md) for complete documentation**

---

## 🏗️ NOA System

**Next-generation Organic Architecture** - A hybrid configuration system with three layers:

### 1. Immutable Layer (DNA) 🧬
- JSON schemas for validation
- Kernel configurations (base, VMM, sandbox)
- Provider definitions (Abacus, LlamaCPP, Claude, Codex)
- Trust anchors and security policies
- **Read-only**, version-controlled

### 2. Mutable Layer (Epigenetics) 🧠
- World model (entities, relationships, context)
- Agent definitions and capabilities
- Skills, tools, workflows
- Configuration (device/hive profiles, preferences)
- Orchestration rules (MOE router, scheduler)
- **AI-rewritable**, semantic layer

### 3. CAS Layer (The Spine) 📦
- SHA-256 content addressing
- Merkle DAG for nested objects
- Mutable refs (`latest-kernel`) + immutable tags (`v1.0.0`)
- Reference counting for garbage collection
- Automatic deduplication
- **Cryptographic verification**

### Hard Guarantees:

1. ✅ **System Cannot Break** - Immutable DNA + CAS + Schema validation
2. ✅ **System Can Evolve** - Mutable semantic layer + AI mutations + Hooks
3. ✅ **System Can Self-Repair** - GC + Merkle verification + Recovery hooks

**→ See [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md) for complete documentation**

---

## Overview

This is a reference implementation of an event-driven ML DevOps platform built with Next.js, TypeScript, and PostgreSQL. The platform demonstrates event sourcing patterns that will be ported to Tauri v2 + Dioxus with Rust-based ML inference.

## Recent Updates (v0.3.0)

### 🔐 Dynamic Google SSO
- **Admin OAuth Configuration**: Set up Google SSO through web interface at `/admin/oauth-setup`
- **Encrypted Storage**: OAuth credentials stored securely in PostgreSQL with AES-256 encryption
- **Real-time Updates**: No server restart required when updating OAuth configuration
- **Dual Mode**: Supports both database-stored and environment variable configurations

### 🧪 End-to-End Testing
- **Playwright Integration**: Complete E2E test suite with 50+ automated tests
- **Test Coverage**: Authentication, navigation, chat, SONA workflows, and profile management
- **CI/CD Ready**: Configured for continuous integration pipelines
- **Mobile Testing**: Includes Pixel 5 and iPhone 12 viewport configurations

### 🔨 Automated Build System
- **Cross-Platform Scripts**: Build scripts for macOS, Windows, Linux, Android, and iOS
- **One-Command Builds**: `./scripts/build-desktop.sh` and `./scripts/build-mobile.sh`
- **Environment Setup**: Automated `setup-env.sh` for dependency installation
- **Platform Detection**: Auto-detects OS and builds appropriate binaries

### 📦 Enhanced Build Configuration
- **Tauri v2**: Latest framework with improved performance and security
- **Multi-Target Support**: Single codebase for web, desktop, and mobile platforms
- **Optimized Bundling**: Platform-specific build configurations
- **Distribution Ready**: Code signing and installer generation included

### Key Features

- ⚡ **Event Sourcing**: Append-only event stream as single source of truth
- 💬 **Streaming Chat**: Token-by-token AI responses with full event tracking
- 🧩 **Widget System**: Dynamic widget registry with 15 widget types + 3 containers (nested support)
- 🔄 **JSON-Patch Updates**: Efficient state synchronization with RFC 6902
- 💾 **Event Persistence**: Save and load event streams from PostgreSQL
- ⏯️ **Event Replay**: Reconstruct UI state from saved event streams
- 🔧 **Event Simulator**: Testing tool for manual event triggering
- 🧠 **Provider Abstraction**: Pluggable AI provider interface (Abacus/ruvllm/candle)
- ⚙️ **Dual Config System**: Portable project config + user-specific overrides
- 🤝 **Real-time Collaboration**: WebSocket-based collaborative editing with OT
- 🔧 **Advanced Patch Utils**: Undo/redo, patch optimization, conflict resolution
- 📦 **Widget Composition**: Container widgets for nested layouts (Grid, Flex, Tabs)
- 📝 **Rich Widgets**: FileUploader, MarkdownEditor, FormBuilder, TreeView
- 🤖 **SONA Orchestration**: Multi-agent LLM workflows with advanced orchestration patterns
- 🖥️ **Desktop Mode**: Native desktop app with Tauri v2 (cross-platform: Windows, macOS, Linux)
- 📊 **Privacy Analytics**: Umami analytics integration (GDPR-compliant, cookieless)

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
- `CONFIG_LOADED` - Configuration loaded

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
  config: {
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
- **Use Cases**: Code snippets, configuration files

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
  streamChat(messages: ChatMessage[], config?: ModelConfig): Promise<StreamingResponse>;
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
└── config.json          # Application configuration
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

## Widget Reference

### Display Widgets

1. **TextBlock** - Markdown text rendering
   - Supports GitHub-flavored markdown
   - Syntax highlighting for code blocks

2. **CodeBlock** - Syntax-highlighted code display
   - 100+ language support
   - Line numbers and copy functionality

3. **StatusIndicator** - System status visualization
   - Animated status icons
   - Color-coded states (idle, processing, success, error)

4. **ImageViewer** - Interactive image display
   - Zoom (up to 5x), pan, rotate
   - Download functionality

5. **VideoPlayer** - Video playback with controls
   - Play/pause, seek, volume control
   - Skip forward/backward, fullscreen

### Data Widgets

6. **DataTable** - Sortable, filterable table
   - Column sorting (asc/desc)
   - Search/filter functionality
   - Pagination with configurable page size

7. **SimpleChart** - Bar and line charts
   - Recharts-based visualization
   - Responsive design
   - Customizable colors and labels

8. **Graph** - Network/graph visualization
   - Interactive nodes and edges
   - Zoom and pan controls
   - Auto-layout with force simulation

9. **TreeView** - Hierarchical data browser
   - Expand/collapse nodes
   - Search and filter
   - Custom icons per node type

### Input Widgets

10. **FileUploader** - Drag-and-drop file upload
    - Multiple file selection
    - Progress tracking
    - File type filtering and size limits

11. **MarkdownEditor** - Live markdown editor
    - Split view (edit + preview)
    - Toolbar shortcuts
    - Auto-save support

12. **FormBuilder** - Dynamic form generator
    - Multiple field types (text, select, checkbox, etc.)
    - Validation with error messages
    - Conditional field visibility

### Container Widgets (Composition)

13. **GridContainer** - CSS Grid layout
    - Configurable columns and gaps
    - Responsive grid
    - Supports nested widgets with col/row span

14. **FlexContainer** - Flexbox layout
    - Horizontal or vertical direction
    - Alignment and justification controls
    - Supports nested widgets with flex properties

15. **TabsContainer** - Tabbed interface
    - Multiple tabs with nested content
    - Lazy loading support
    - Supports nested widgets in each tab

## Collaboration Features

### Real-time Collaboration

- **User Presence**: See who's online in your session
- **Cursor Sharing**: Track collaborator cursor positions
- **Patch Synchronization**: Real-time updates with operational transform
- **Conflict Resolution**: Automatic handling of concurrent edits

### Advanced Patch Utilities

- **Undo/Redo Stack**: History management with 50-entry buffer
- **Patch Optimization**: Remove redundant operations
- **Patch Validation**: RFC 6902 compliance checking
- **Patch Merging**: Combine multiple patch arrays
- **Operational Transform**: Conflict-free concurrent editing

Usage:
```typescript
import { PatchHistory, optimizePatches } from '@/lib/patch-utils';

const history = new PatchHistory();
history.push(patches, inversePatches);

// Undo/Redo
const undoPatches = history.undo();
const redoPatches = history.redo();

// Optimize patches
const optimized = optimizePatches(patches);
```

## Next Steps

See [phase2_tasks.csv](./phase2_tasks.csv) for the roadmap to Tauri + Rust migration.

Key Phase 2 milestones:
1. Port to Tauri v2 with Dioxus UI
2. Integrate ruvllm for Rust-based inference
3. Add Rig framework for LLM ops
4. Implement candle-vllm for local models
5. JSON-patch updates for efficient state sync
6. Dual config system (runtime + compile-time)

## Architecture Details

For in-depth architecture information, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## License

MIT

## SONA Orchestration System

### Overview

SONA (Sequential Orchestration for Neural Agents) is an advanced LLM orchestration system for creating and executing complex multi-agent workflows. It provides a robust framework for coordinating multiple specialized AI agents to solve complex tasks through structured collaboration.

### Key Features

- **Multi-Agent Orchestration**: Coordinate multiple specialized agents (planner, executor, reviewer, specialist, aggregator)
- **Flexible Execution Strategies**: Sequential, parallel, conditional, loop, and map-reduce patterns
- **Real-Time Monitoring**: Stream events and monitor workflow execution in real-time
- **Workflow Templates**: Pre-built templates for common patterns (plan-execute-review, consensus, iterative refinement)
- **Context Management**: Maintain context and memory across workflow steps
- **Tool Integration**: Built-in tools for code execution, web search, data analysis, and more
- **Error Handling**: Automatic retries and graceful error recovery

### Agent Roles

```typescript
- **Planner**: Task decomposition and workflow planning
- **Executor**: Task implementation and execution
- **Reviewer**: Quality control and validation
- **Specialist**: Domain-specific expertise
- **Aggregator**: Result synthesis and aggregation
```

### Workflow Strategies

1. **Sequential**: Execute steps one after another
   - Use case: Tasks with clear dependencies
   - Example: Plan → Execute → Review

2. **Parallel**: Execute steps concurrently
   - Use case: Independent tasks that can run simultaneously
   - Example: Parallel data processing

3. **Conditional**: Execute based on runtime conditions
   - Use case: Dynamic branching logic
   - Example: If-then-else workflows

4. **Loop**: Repeat steps with iteration
   - Use case: Iterative refinement
   - Example: Generate → Review → Refine (repeat)

5. **Map-Reduce**: Parallel map with result reduction
   - Use case: Large-scale data processing
   - Example: Process chunks → Aggregate results

### Workflow Templates

#### 1. Plan-Execute-Review
Three-phase workflow for production-quality results:
```typescript
Planner → Executor → Reviewer
```

#### 2. Multi-Expert Consensus
Leverage multiple specialists for complex decisions:
```typescript
[Specialist 1, Specialist 2, Specialist 3, ...] → Aggregator
```

#### 3. Iterative Refinement
Continuously improve through multiple iterations:
```typescript
Loop: Executor → Reviewer (with feedback)
```

#### 4. Map-Reduce Processing
Parallel processing with result aggregation:
```typescript
[Mapper 1, Mapper 2, Mapper 3, ...] → Reducer
```

### API Usage

#### Execute a Workflow Template

```typescript
const response = await fetch('/api/sona', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    action: 'execute_template',
    workflowType: 'plan_execute_review',
    workflowConfig: {
      name: 'Code Generation Workflow',
      task: 'Build a REST API for user management'
    },
    input: {
      requirements: 'Express.js, TypeScript, PostgreSQL'
    }
  })
});

const result = await response.json();
console.log(result.executionId); // Track execution
console.log(result.result);      // Final output
```

#### Execute with Real-Time Streaming

```typescript
const response = await fetch('/api/sona/stream', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ workflow, input })
});

const reader = response.body.getReader();
const decoder = new TextDecoder();

while (true) {
  const { done, value } = await reader.read();
  if (done) break;
  
  const chunk = decoder.decode(value);
  const lines = chunk.split('\n\n');
  
  for (const line of lines) {
    if (line.startsWith('data: ')) {
      const event = JSON.parse(line.slice(6));
      console.log('Event:', event.type, event.data);
    }
  }
}
```

#### React Hooks

```typescript
import { useSonaWorkflow, useSonaStream } from '@/lib/hooks/use-sona';

// Simple execution
const { executeTemplate, isExecuting, result, error } = useSonaWorkflow();

await executeTemplate('plan_execute_review', {
  name: 'My Workflow',
  task: 'Generate a report'
});

// With streaming
const { executeWithStream, isStreaming, events } = useSonaStream();

await executeWithStream(workflow, input);
```

### Built-in Tools

SONA agents can leverage these built-in tools:

- **execute_code**: Execute code in sandboxed environment
- **web_search**: Search the web for information
- **analyze_data**: Analyze datasets and generate insights
- **file_operations**: Read/write files
- **database_query**: Query databases
- **api_call**: Make HTTP requests to external APIs
- **calculator**: Perform mathematical calculations

### Event Types

SONA emits the following events during workflow execution:

- `WORKFLOW_STARTED`: Workflow execution initiated
- `WORKFLOW_COMPLETED`: Workflow finished successfully
- `WORKFLOW_FAILED`: Workflow encountered an error
- `STEP_STARTED`: Individual step started
- `STEP_COMPLETED`: Step finished successfully
- `STEP_FAILED`: Step encountered an error
- `AGENT_THINKING`: Agent is processing
- `AGENT_RESPONSE`: Agent produced a response
- `TOOL_CALLED`: Tool was invoked
- `TOOL_RESULT`: Tool execution completed

### UI Components

#### WorkflowBuilder
Visual interface for creating and configuring workflows:

```tsx
import { WorkflowBuilder } from '@/components/sona/workflow-builder';

<WorkflowBuilder
  onWorkflowCreated={(workflow) => {
    console.log('Workflow created:', workflow);
  }}
/>
```

#### WorkflowMonitor
Real-time monitoring of workflow execution:

```tsx
import { WorkflowMonitor } from '@/components/sona/workflow-monitor';

<WorkflowMonitor
  workflow={myWorkflow}
  input={{ task: 'Process data' }}
  autoExecute={true}
/>
```

### Example: Custom Workflow

```typescript
import { WorkflowBuilder } from '@/lib/sona/workflows';

const workflow = new WorkflowBuilder('custom-workflow', 'Custom Analysis')
  .setDescription('Analyze code and provide recommendations')
  .setStrategy('sequential')
  .addAgentFromTemplate('planner', 'planner')
  .addAgentFromTemplate('executor', 'executor')
  .addAgentFromTemplate('reviewer', 'reviewer')
  .addStep({
    id: 'plan',
    name: 'Create Analysis Plan',
    agentId: 'planner',
    input: { task: 'Analyze codebase structure' }
  })
  .addStep({
    id: 'execute',
    name: 'Perform Analysis',
    agentId: 'executor',
    input: { task: 'Execute analysis plan' },
    dependencies: ['plan']
  })
  .addStep({
    id: 'review',
    name: 'Review Findings',
    agentId: 'reviewer',
    input: { task: 'Review analysis results' },
    dependencies: ['execute']
  })
  .build();

// Execute the workflow
const result = await orchestrator.executeWorkflow(workflow, {
  codebase: '/path/to/code'
});
```

### Access SONA Dashboard

Visit `/sona` in your application to access the interactive SONA dashboard with:
- Workflow builder interface
- Example workflows
- Real-time execution monitoring
- Comprehensive documentation


## Desktop Mode (Tauri v2)

The platform can run as a native desktop application on Windows, macOS, and Linux using Tauri v2.

### Quick Start

```bash
# Install dependencies (first time only)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Run in development mode
cd nextjs_space
yarn tauri dev

# Build desktop installers
yarn tauri build
```

### Features

- 🖥️ **Native Window**: Cross-platform native window with system integration
- 📁 **File System**: Direct access to local files and directories
- 🔌 **System Integration**: Native dialogs, notifications, and tray icons
- 🚀 **Performance**: Faster startup and lower memory usage vs browser
- 🔒 **Privacy**: All data stays local, no cloud dependencies required
- 📦 **Installers**: DMG (macOS), MSI/NSIS (Windows), DEB/AppImage (Linux)

### Available Commands

The Tauri backend exposes TypeScript-callable Rust commands:

```typescript
import { 
  getSystemInfo, 
  isDesktopMode, 
  getAppVersion,
  saveLocalFile,
  readLocalFile,
  showFilePicker,
  openExternalUrl
} from '@/lib/tauri/commands';

// Check if running in desktop mode
const isDesktop = await isDesktopMode(); // true in Tauri

// Get system information
const sysInfo = await getSystemInfo();
console.log(`Running on ${sysInfo.platform} ${sysInfo.arch}`);

// File operations
await saveLocalFile('/path/to/file.txt', 'content');
const content = await readLocalFile('/path/to/file.txt');

// Native dialogs
const filePath = await showFilePicker();
```

### Future: Local ML Inference

Phase E will add local ML inference capabilities to the desktop app:

- 🧠 **Candle-vLLM Integration**: Run models locally (Qwen2.5-7B, etc.)
- ⚡ **Offline Mode**: Full functionality without internet connection
- 🔒 **Data Privacy**: All inference happens on-device
- 📊 **Benchmarking**: Compare local vs cloud inference performance
- 🎯 **Optimized Models**: Quantized models for faster inference

### Documentation

For complete setup instructions and troubleshooting, see [TAURI_SETUP.md](../TAURI_SETUP.md).

## Privacy & Analytics

The platform integrates **Umami Analytics** - a privacy-focused, GDPR-compliant alternative to Google Analytics:

- ✅ **No Cookies**: Cookieless tracking, no consent banners needed
- ✅ **No PII**: All visitor data anonymized by default
- ✅ **Self-Hosted**: Full control over your analytics data
- ✅ **GDPR Compliant**: Designed for privacy regulations
- ✅ **Lightweight**: <2KB script, minimal performance impact

### Setup

```bash
# Add your Umami website ID to .env
NEXT_PUBLIC_UMAMI_WEBSITE_ID="your_website_id"
NEXT_PUBLIC_UMAMI_HOST="https://cloud.umami.is"

# Or self-host with Docker
docker-compose -f umami-docker-compose.yml up -d
```

The analytics system automatically tracks:
- Page views
- Workflow executions (strategy, duration, success/failure)
- Widget interactions (mount, update, unmount)
- Chat events (messages sent/received)
- Collaboration events (users joined/left)
- Event replay usage

See `.env.example` for full configuration options.

