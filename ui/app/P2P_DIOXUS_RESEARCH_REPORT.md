# NOA UI Unification Research Report

**Date:** January 2, 2026  
**Purpose:** Library recommendations for NOA UI unification project  
**Scope:** Dioxus components, P2P libraries, and ML DevOps feature preservation

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Dioxus Component Libraries](#1-dioxus-component-libraries)
3. [P2P Libraries for HIVE_PROTOCOL](#2-p2p-libraries-for-hive_protocol)
4. [ML DevOps Features to Preserve](#3-ml-devops-features-to-preserve)
5. [Integration Approach](#4-integration-approach)
6. [Appendix: Library Matrix](#appendix-library-matrix)

---

## Executive Summary

This report provides research findings for the NOA UI unification project, specifically:
- **Dioxus component ecosystem** has matured significantly with 20+ libraries
- **rust-libp2p** is the recommended P2P foundation with excellent GossipSub/Kademlia support
- **Loro CRDT** or **y-crdt** should be used for state synchronization
- **ML DevOps platform** has 4 core features that must be preserved: streaming chat, widget registry, event replay, and provider selection

---

## 1. Dioxus Component Libraries

### 1.1 Tier 1: Recommended for Immediate Use

| Library | Stars | Description | GitHub URL |
|---------|-------|-------------|------------|
| **DioxusLabs/components** | 231 | Official unstyled, accessible foundational components | https://github.com/DioxusLabs/components |
| **dioxus-free-icons** | 172 | Free SVG icon library (FontAwesome, Heroicons, etc.) | https://github.com/dioxus-community/dioxus-free-icons |
| **daisy-rsx** | 171 | DaisyUI components for Dioxus (styled, production-ready) | https://github.com/bionic-gpt/daisy-rsx |
| **lumen-blocks** | 86 | shadcn-inspired ARIA-accessible styled components | https://github.com/Leaf-Computer/lumen-blocks |
| **dioxus-charts** | 82 | Simple chart components | https://github.com/dioxus-community/dioxus-charts |
| **dioxus-radio** | 73 | Global state management with topic subscriptions | https://github.com/dioxus-community/dioxus-radio |

### 1.2 Tier 2: Complementary Libraries

| Library | Stars | Purpose | GitHub URL |
|---------|-------|---------|------------|
| **dioxus-i18n** | 50 | Internationalization support | https://github.com/dioxus-community/dioxus-i18n |
| **lookbook** | 43 | Component storybook/documentation | https://github.com/dioxus-community/lookbook |
| **dioxus-tw-components** | 38 | TailwindCSS 4 based components | https://github.com/42Angouleme/dioxus-tw-components |
| **dioxus-leaflet** | 37 | Map component (Leaflet) | https://github.com/lheintzmann1/dioxus-leaflet |
| **dioxus-lazy** | 31 | Virtualized list/grid components | https://github.com/dioxus-community/dioxus-lazy |
| **freyr** | 29 | UI component library | https://github.com/cbdefontenay/freyr |

### 1.3 Tier 3: Specialized/Utilities

| Library | Stars | Purpose | GitHub URL |
|---------|-------|---------|------------|
| **dioxus-material** | 24 | Material Design components | https://github.com/dioxus-community/dioxus-material |
| **dioxus-helmet** | 23 | Document head management | https://github.com/dioxus-community/dioxus-helmet |
| **dioxus-spring** | 21 | Animation framework | https://github.com/dioxus-community/dioxus-spring |
| **dioxus-heroicons** | 20 | Heroicons components | https://github.com/houseabsolute/dioxus-heroicons |
| **adui-dioxus** | 18 | Ant Design 6.0 port | https://github.com/feisan/adui-dioxus |
| **table-rs** | 17 | WASM table component | https://github.com/opensass/table-rs |
| **dioxus-markdown** | 15 | Runtime markdown rendering | https://github.com/rambip/dioxus-markdown |

### 1.4 Dashboard/Admin Templates

| Template | Stars | Description | GitHub URL |
|----------|-------|-------------|------------|
| **r-dashboard** | 29 | Dioxus + Tailwind WASM admin | https://github.com/cody-why/r-dashboard |
| **dioxus-daisyui-admin-portal** | 3 | DaisyUI admin portal | https://github.com/zhi-gang/dioxus-daisyui-admin-protal |

### 1.5 Recommended Component Stack

For the NOA UI, I recommend:

```
┌─────────────────────────────────────────────────────────┐
│ UI Layer                                                │
├─────────────────────────────────────────────────────────┤
│ daisy-rsx (styled components) OR lumen-blocks (shadcn)  │
├─────────────────────────────────────────────────────────┤
│ DioxusLabs/components (unstyled primitives)             │
├─────────────────────────────────────────────────────────┤
│ dioxus-free-icons + dioxus-charts + dioxus-lazy         │
├─────────────────────────────────────────────────────────┤
│ dioxus-radio (state management with topic pub/sub)      │
└─────────────────────────────────────────────────────────┘
```

---

## 2. P2P Libraries for HIVE_PROTOCOL

Based on analysis of [HIVE_PROTOCOL.md](HIVE_PROTOCOL.md), here are the recommended libraries:

### 2.1 Core P2P Stack: rust-libp2p

**Repository:** https://github.com/libp2p/rust-libp2p  
**Status:** Production-ready, actively maintained  
**NOA Usage:** Already cloned at `N:\noa\p2p\` (libp2p-core v0.43.1)

#### Key Components for HIVE_PROTOCOL:

| HIVE_PROTOCOL Requirement | libp2p Module | Path |
|--------------------------|---------------|------|
| GossipSub topics (presence, state/op, release, alert) | `libp2p-gossipsub` | `protocols/gossipsub/` |
| Device discovery | `libp2p-kad` (Kademlia DHT) | `protocols/kad/` |
| Request/Response (handshake, state sync) | `libp2p-request-response` | `protocols/request-response/` |
| Peer identity | `libp2p-identity` | `identity/` |
| Encryption/TLS | `libp2p-noise`, `libp2p-tls` | `transports/noise/`, `transports/tls/` |
| NAT traversal | `libp2p-autonat`, `libp2p-dcutr` | `protocols/autonat/`, `protocols/dcutr/` |

#### GossipSub Implementation Details

The `protocols/gossipsub/src/` directory contains:
- `behaviour.rs` (142KB) - Main GossipSub behavior implementation
- `config.rs` (53KB) - Extensive configuration options
- `topic.rs` - Topic hash management
- `peer_score.rs` - Peer scoring for mesh management
- `subscription_filter.rs` - Topic subscription filtering

**HIVE_PROTOCOL Topic Mapping:**
```rust
// Recommended topic configuration
const TOPICS: &[&str] = &[
    "noa-ui/v1/presence",      // Periodic presence announcements
    "noa-ui/v1/state/op",      // Oplog entries broadcast
    "noa-ui/v1/release/manifest", // Release announcements
    "noa-ui/v1/alert",         // Protocol alerts
];
```

### 2.2 State Synchronization: CRDT Libraries

#### Option A: Loro (Recommended)

**Repository:** https://github.com/loro-dev/loro  
**Stars:** 5,251  
**Why:** 
- Collaborative JSON data with built-in versioning
- Designed for local-first applications
- Excellent Rust implementation
- Rich text support via `crdt-richtext`

**Integration:**
```rust
// Loro for StateOp in HIVE_PROTOCOL
use loro::LoroDoc;

struct StateSync {
    doc: LoroDoc,
    oplog: Vec<StateOp>,
}
```

#### Option B: y-crdt (Yjs port)

**Repository:** https://github.com/y-crdt/y-crdt  
**Stars:** 1,923  
**Why:** 
- Port of battle-tested Yjs
- Good WebAssembly support
- Mature ecosystem

#### Option C: OctoBase

**Repository:** https://github.com/toeverything/OctoBase  
**Stars:** 1,850  
**Why:**
- Full database behind AFFiNE
- P2P support built-in
- Serverless/self-contained

#### Option D: rust-crdt

**Repository:** https://github.com/rust-crdt/rust-crdt  
**Stars:** 1,500  
**Why:**
- Pure Rust, well-tested
- Collection of CRDT primitives
- Good for custom implementations

### 2.3 Alternative P2P Stack: Iroh

**Repository:** https://github.com/n0-computer/iroh  
**Stars:** 7,636  
**Why Consider:**
- "P2P that just works" - simpler API than libp2p
- QUIC-native transport
- Built-in NAT traversal (magicsock)
- Excellent for file/blob sync

**Best for:** Release/binary sync in HIVE_PROTOCOL

```rust
// Iroh for artifact distribution
use iroh::node::Node;
use iroh_blobs::store::mem::Store;

async fn distribute_release(artifact: &[u8]) -> Result<Hash> {
    let node = Node::builder().spawn().await?;
    node.blobs().add_bytes(artifact).await
}
```

### 2.4 Reference Implementations

#### hyveos (p2p-industries)

**Repository:** https://github.com/p2p-industries/hyveos  
**Architecture:** Decentralized robot communication  
**Relevant Crates:**
- `crates/p2p-stack/` - libp2p wrapper with unified API
- `crates/core/` - Core abstractions
- `crates/bridge/` - Network bridging
- `crates/config/` - Configuration management

**Key Features:**
- Pub-Sub with GossipSub
- DHT for discovery (Key-Value Store)
- Request-Response patterns
- File transfer capabilities

#### fungi (enbop)

**Repository:** https://github.com/enbop/fungi  
**Architecture:** Swarm-based P2P  
**Relevant Crates:**
- `crates/swarm/` - libp2p swarm management
- `crates/fs/` - Distributed filesystem
- `crates/daemon/` - Background daemon

#### SwarmKit (Docker/Moby)

**Repository:** https://github.com/moby/swarmkit  
**Language:** Go (reference architecture)  
**Relevant Components:**
- `manager/state/raft/` - Raft consensus implementation
- `manager/scheduler/` - Task scheduling
- `manager/orchestrator/` - Service orchestration
- `manager/dispatcher/` - Work distribution

**Key Patterns for HIVE_PROTOCOL:**
- Raft-based strongly consistent state store
- Leader election for managers
- Task reconciliation loop
- Rolling updates with parallelism control

### 2.5 HIVE_PROTOCOL Implementation Matrix

| Protocol Feature | Primary Library | Alternative |
|------------------|-----------------|-------------|
| Device Discovery | libp2p-kad | Iroh discovery |
| Presence Announce | libp2p-gossipsub | libp2p-broadcast |
| State Ops Broadcast | libp2p-gossipsub + Loro | y-crdt |
| Request/Response | libp2p-request-response | Iroh RPC |
| Snapshot Sync | Loro + libp2p | OctoBase |
| Release Manifest | libp2p-gossipsub | Iroh blobs |
| Artifact Transfer | Iroh blobs | libp2p-stream |
| Consensus (optional) | raft-rs | openraft |

---

## 3. ML DevOps Features to Preserve

Analysis of `N:\noa\ml_devops_platform\nextjs_space\`:

### 3.1 Core Features (from config/features.json)

```json
{
  "eventReplay": true,
  "widgetRegistry": true,
  "streamingChat": true,
  "eventPersistence": true
}
```

### 3.2 Application Structure

```
app/
├── admin/           # Admin panel with OAuth setup
├── api/             # API routes
├── deepcode/        # DeepCode integration
├── docs/            # Documentation pages
├── login/           # Authentication
├── signup/          # User registration
├── profile/         # User profile
├── phase2-tasks/    # Task management
└── sona/            # SONA assistant page
```

### 3.3 Components to Port

#### Chat System (`components/chat/`)
- `chat-interface.tsx` - Main chat with streaming
- `message-input.tsx` - User input handling
- `message-list.tsx` - Message display

**Dioxus Equivalent:**
```rust
// Use coroutines for async streaming
use_coroutine(|rx: UnboundedReceiver<Message>| async move {
    while let Some(msg) = rx.next().await {
        // Handle streaming tokens
    }
});
```

#### Widget Registry (`components/widgets/`)

16 widget types must be preserved:

| Widget | Purpose | Dioxus Approach |
|--------|---------|-----------------|
| `text-block.tsx` | Rich text display | dioxus-markdown |
| `code-block.tsx` | Syntax-highlighted code | syntect + custom component |
| `status-indicator.tsx` | Status badges | daisy-rsx Badge |
| `simple-chart.tsx` | Basic charts | dioxus-charts |
| `data-table.tsx` | Data grids | table-rs / custom |
| `graph.tsx` | Graph visualization | Custom + plotters-rs |
| `image-viewer.tsx` | Image display | Native img element |
| `video-player.tsx` | Video playback | web-sys video |
| `file-uploader.tsx` | File upload | web-sys FileReader |
| `markdown-editor.tsx` | MD editing | dioxus-markdown |
| `form-builder.tsx` | Dynamic forms | DioxusLabs/components |
| `tree-view.tsx` | Tree structure | Custom recursive |
| `grid-container.tsx` | Grid layout | CSS Grid |
| `flex-container.tsx` | Flex layout | CSS Flexbox |
| `tabs-container.tsx` | Tab panels | daisy-rsx Tabs |
| `widget-registry.tsx` | Widget orchestrator | Custom HashMap<ID, WidgetConfig> |

**Widget Registry Pattern (Dioxus):**
```rust
#[component]
fn WidgetRegistry(widgets: Signal<HashMap<String, WidgetConfig>>) -> Element {
    rsx! {
        for (id, config) in widgets.read().iter() {
            match config.widget_type.as_str() {
                "text" => TextBlock { config: config.clone() },
                "code" => CodeBlock { config: config.clone() },
                "chart" => SimpleChart { config: config.clone() },
                _ => UnknownWidget { id: id.clone() }
            }
        }
    }
}
```

#### Provider System (`components/providers/`)
- `provider-selector.tsx` - AI provider selection
- Theme provider via `theme-provider.tsx`

#### SONA Workflow (`components/sona/`)
- `workflow-builder.tsx` - Visual workflow editor
- `workflow-monitor.tsx` - Execution monitoring

#### Inference Components (`components/inference/`)
- `local-server-control.tsx` - Local model server management

#### Analytics (`components/analytics/`)
- Umami analytics integration

### 3.4 Event System Architecture

The ML DevOps platform uses an event-sourced architecture:

```typescript
// Event types to preserve
type EventType = 
  | 'MESSAGE_SENT'
  | 'TOKEN_STREAMED'
  | 'MESSAGE_COMPLETED'
  | 'WIDGET_MOUNTED'
  | 'WIDGET_UPDATED'
  | 'WIDGET_PATCHED'  // JSON patches
  | 'WIDGET_UNMOUNTED'
  | 'STATUS_CHANGED';
```

**Dioxus Event System:**
```rust
// Use channels for event streaming
use tokio::sync::broadcast;

struct EventBus {
    tx: broadcast::Sender<Event>,
}

impl EventBus {
    fn emit(&self, event: Event) {
        self.tx.send(event).ok();
    }
    
    fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }
}
```

### 3.5 Authentication Flow

Current: NextAuth.js with OAuth providers
- Tauri integration via `src-tauri/`
- Session management

**Dioxus Equivalent:**
- Use `dioxus-oauth` (if available) or custom OAuth flow
- Store tokens in secure storage (keyring-rs for desktop)

---

## 4. Integration Approach

### 4.1 Phased Migration Strategy

```
Phase 1: Core Infrastructure (Weeks 1-4)
├── Set up Dioxus project with multi-platform targets
├── Implement event bus with broadcast channels
├── Create widget registry skeleton
└── Port basic UI components (daisy-rsx base)

Phase 2: P2P Foundation (Weeks 5-8)
├── Integrate libp2p with GossipSub
├── Implement HIVE_PROTOCOL message types
├── Add Kademlia for device discovery
└── Integrate Loro for state sync

Phase 3: Feature Parity (Weeks 9-12)
├── Port chat interface with streaming
├── Implement all 16 widget types
├── Add provider selector
├── Port SONA workflow components

Phase 4: P2P Enhancement (Weeks 13-16)
├── Add Iroh for binary distribution
├── Implement release sync
├── Add cross-device state sync
└── Performance optimization
```

### 4.2 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        NOA UI (Dioxus)                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Chat      │  │   Widgets   │  │   SONA      │              │
│  │  Interface  │  │   Registry  │  │  Workflow   │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│  ┌──────┴────────────────┴────────────────┴──────┐              │
│  │              Event Bus (broadcast)             │              │
│  └──────────────────────┬────────────────────────┘              │
│                         │                                        │
│  ┌──────────────────────┴────────────────────────┐              │
│  │              State Management                  │              │
│  │         (dioxus-radio + Loro CRDT)            │              │
│  └──────────────────────┬────────────────────────┘              │
├─────────────────────────┴────────────────────────────────────────┤
│                    P2P Layer (HIVE_PROTOCOL)                     │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  GossipSub  │  │  Kademlia   │  │  Request/   │              │
│  │  (libp2p)   │  │    DHT      │  │  Response   │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│  ┌──────┴────────────────┴────────────────┴──────┐              │
│  │              libp2p Swarm                      │              │
│  └──────────────────────┬────────────────────────┘              │
│                         │                                        │
│  ┌──────────────────────┴────────────────────────┐              │
│  │          Iroh (Binary Distribution)           │              │
│  └───────────────────────────────────────────────┘              │
└─────────────────────────────────────────────────────────────────┘
```

### 4.3 Cargo.toml Dependencies

```toml
[dependencies]
# Dioxus Core
dioxus = { version = "0.6", features = ["web", "desktop"] }
dioxus-free-icons = "0.9"

# UI Components (choose one)
daisy-rsx = "0.7"
# OR
# lumen-blocks = "0.1"

# State Management
dioxus-radio = "0.2"

# P2P Stack
libp2p = { version = "0.54", features = [
    "gossipsub",
    "kad",
    "request-response",
    "noise",
    "tcp",
    "quic",
    "dns",
    "identify",
    "autonat",
]}

# CRDT State Sync
loro = "1.0"
# OR
# yrs = "0.20"

# Binary Distribution
iroh = "0.30"
iroh-blobs = "0.30"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ciborium = "0.2"  # CBOR as per HIVE_PROTOCOL

# Async Runtime
tokio = { version = "1.0", features = ["full"] }

# Charts
dioxus-charts = "0.2"

# Markdown
dioxus-markdown = "0.5"
```

---

## Appendix: Library Matrix

### A.1 Complete Dioxus Ecosystem

| Category | Library | Stars | Status | Recommended |
|----------|---------|-------|--------|-------------|
| **Primitives** | DioxusLabs/components | 231 | Active | ✅ Yes |
| **Styled UI** | daisy-rsx | 171 | Active | ✅ Yes |
| **Styled UI** | lumen-blocks | 86 | Active | ✅ Yes |
| **Icons** | dioxus-free-icons | 172 | Active | ✅ Yes |
| **Charts** | dioxus-charts | 82 | Active | ✅ Yes |
| **State** | dioxus-radio | 73 | Active | ✅ Yes |
| **i18n** | dioxus-i18n | 50 | Active | ⚡ If needed |
| **Storybook** | lookbook | 43 | Active | ⚡ For docs |
| **TailwindCSS** | dioxus-tw-components | 38 | Active | ❌ Use daisy-rsx |
| **Maps** | dioxus-leaflet | 37 | Active | ⚡ If needed |
| **Virtualization** | dioxus-lazy | 31 | Active | ✅ Yes |
| **Material** | dioxus-material | 24 | Stale | ❌ Skip |
| **Animation** | dioxus-spring | 21 | Active | ⚡ If needed |
| **Tables** | table-rs | 17 | Active | ✅ Yes |

### A.2 P2P Library Comparison

| Library | Language | Stars | NAT Traversal | CRDT | Use Case |
|---------|----------|-------|---------------|------|----------|
| libp2p | Rust | 4.5k | ✅ | ❌ | Full P2P stack |
| Iroh | Rust | 7.6k | ✅ (magicsock) | ❌ | Simple P2P + blobs |
| Loro | Rust | 5.3k | ❌ | ✅ | State sync |
| y-crdt | Rust | 1.9k | ❌ | ✅ | State sync |
| hyveos | Rust | - | ✅ | ❌ | Robot swarms |
| SwarmKit | Go | 3.6k | ✅ | ❌ | Container orchestration |

### A.3 ML DevOps Feature Mapping

| NextJS Feature | Dioxus Implementation | Complexity |
|----------------|----------------------|------------|
| Streaming Chat | use_coroutine + SSE | Medium |
| Widget Registry | HashMap + match | Low |
| Event Replay | Event log + playback | Medium |
| JSON Patch Updates | fast-json-patch port | Low |
| Provider Selection | Enum + config | Low |
| OAuth Login | oauth2-rs + keyring | High |
| File Upload | web-sys FileReader | Low |
| Local Model Server | Tauri commands | Medium |
| Analytics | Custom or Plausible | Low |

---

## Conclusion

The NOA UI unification project has a solid foundation of libraries to work with:

1. **Use `daisy-rsx` + `DioxusLabs/components`** for UI components
2. **Use `libp2p`** for P2P networking (already in `N:\noa\p2p\`)
3. **Use `Loro`** for CRDT-based state synchronization
4. **Consider `Iroh`** for simplified binary/release distribution
5. **Port the 16 widget types** using the component stack
6. **Preserve the event-sourced architecture** with broadcast channels

The HIVE_PROTOCOL is well-designed and maps cleanly onto libp2p's GossipSub topics and request-response patterns.
