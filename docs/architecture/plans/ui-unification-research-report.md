# NOA UI Unification Research Report

**Date:** 2026-01-02  
**Status:** Research Complete  
**Author:** AI Agent (per AGENTS.md)

---

## Executive Summary

NOA has **significant UI fragmentation** across 5+ distinct UI implementations. This report documents the current state and provides recommendations for unification aligned with CONSTITUTION.md principles.

---

## 1. UI Locations Inventory

### Primary UI Locations

| Location | Technology | Port | Purpose | Status |
|----------|------------|------|---------|--------|
| `sys/ui/` | Next.js 15 + React 19 | 3000 | Main NOA Dashboard | ✅ Active |
| `sys/ui/apps/ml-devops/` | Next.js 14 + React 18 + Tauri | 3000 | ML DevOps Platform | ✅ Active |
| `ui/rust-lovable/rust-lovable/` | Dioxus 0.7 + Rust | 8080 | Conversational UI (Lovable clone) | ✅ Active |
| `ui/app/` | Dioxus 0.7 Workspace | - | Planned unified Dioxus UI | 🚧 In Progress |
| `ml_devops_platform/nextjs_space/` | Next.js 14 + React 18 | - | Legacy ML DevOps | ⚠️ Duplicate |
| `pkg/sys/ui/` | Next.js 15 + React 19 | - | Package mirror of sys/ui | ⚠️ Duplicate |

### Secondary/Archived UI Locations

| Location | Purpose | Status |
|----------|---------|--------|
| `data/archive/project-mgmt/Taskosaur/frontend/` | Archived task UI | 📦 Archived |
| `data/archive/project-mgmt/tududi/` | Archived project UI | 📦 Archived |
| `ruler/` | CLI tool (no UI) | N/A |

---

## 2. Technology Stack Analysis

### React/Next.js UIs (3 implementations)

#### sys/ui/ (Main Dashboard)
```json
{
  "react": "^19.0.0",
  "next": "^15.5.8",
  "tailwindcss": "^4.1.17",
  "lucide-react": "^0.556.0"
}
```
- **Components:** Chat, ActivityLog, Widgets, Settings
- **Design System:** CSS variables (simple)
- **API Connection:** localhost:3001 (noa-api)

#### sys/ui/apps/ml-devops/ (ML DevOps)
```json
{
  "react": "18.2.0",
  "next": "14.2.28",
  "tailwindcss": "3.3.3",
  "@radix-ui/*": "Full shadcn/ui library",
  "@tauri-apps/api": "^2.0.0"
}
```
- **Components:** 48+ Radix UI components, Chat, Analytics, Providers, MOE Router, SONA Orchestrator
- **Design System:** shadcn/ui with HSL color variables
- **Features:** Prisma ORM, Playwright testing, Tauri desktop
- **API Connection:** Internal /api routes, Provider abstraction layer

#### ml_devops_platform/nextjs_space/ (Duplicate)
```json
{
  "react": "18.2.0",
  "next": "14.2.28",
  "tailwindcss": "3.3.3"
}
```
- **Status:** Nearly identical to sys/ui/apps/ml-devops/
- **Recommendation:** DEPRECATE - consolidate into sys/ui/apps/ml-devops/

### Dioxus/Rust UIs (2 implementations)

#### ui/rust-lovable/rust-lovable/
```toml
dioxus = "0.7"
dioxus-router = "0.7"
tokio = { version = "1.0", optional = true }
```
- **Components:** Chat, Canvas, Sidebar, Toolbar, CodeEditor
- **Features:** AI-powered conversational UI, project generation, sandbox execution
- **API Connection:** localhost:11434 (Ollama), localhost:8080 (self)

#### ui/app/ (New Unified Workspace)
```toml
[workspace.dependencies]
dioxus = "0.7.2"
```
- **Crates:**
  - `noa-ui-core` - State, hooks, types
  - `noa-ui-shell` - Layout components
  - `noa-ui-protocol` - Backend communication
  - `noa-ui-styleguide-api` - Design tokens
  - `noa-ui-styleguide-ui` - Visual components
  - `noa-ui-paths` - Path resolution
- **Binaries:**
  - `noa-ui-desktop` - Desktop app
  - `noa-ui-web` - Web target
  - `noa-ui-hived` - P2P node UI
- **Status:** Migration in progress from rust-lovable

---

## 3. Current API/Backend Connections

### Backend Services

| Service | Port | Technology | Purpose |
|---------|------|------------|---------|
| noa-api | 3001 | Rust/Axum | Core REST API |
| rust-lovable | 8080 | Rust/Axum | Conversational UI API |
| Ollama | 11434 | Go | Local LLM inference |
| Next.js UI | 3000 | Node | UI development server |

### API Endpoints (noa-api)

```
GET  /health              - Health check
GET  /api/v1/status       - Component status
GET  /api/v1/tasks        - List tasks
POST /api/v1/tasks        - Create task
POST /api/v1/chat         - Chat endpoint
```

### API Route Structure (ml-devops)

```
/api/providers            - AI provider management
/api/chat                 - Chat interface
/api/inference            - Model inference
/api/analytics            - Analytics data
```

---

## 4. AI Provider Integration Points

### configsuration (configs/ai-providers.json)

```json
{
  "providerPriority": ["local", "hybrid", "ide", "cloud"],
  "providers": {
    "local": { "types": ["llama.cpp", "ollama"] },
    "cloud": { "types": ["openai", "anthropic", "claude-code", "codex"] },
    "hybrid": { "types": ["cursor"] },
    "ide": { "types": ["vscode-copilot"] }
  }
}
```

### Provider Abstraction (sys/ui/apps/ml-devops/lib/providers/)

```typescript
// ai-provider.ts - Provider interface
interface AIProvider {
  getName(): string;
  streamChat(messages, configs): Promise<StreamingResponse>;
  generateWidget(prompt): Promise<WidgetGeneration>;
}
```

### Rust Integration (ui/rust-lovable/)

```rust
// Direct Ollama integration
let ai = ConversationalAI::new(AIProvider::Local {
    endpoint: "http://localhost:11434".to_string(),
});
```

### Shared Resources (configs/shared-resources.json)

```
${NOA_ROOT}/ai/shared/
├── agents/       - AI agent definitions
├── workflows/    - Orchestration workflows
├── prompts/      - Prompt templates
├── tools/        - MCP tool definitions
├── models/       - Model adapters
└── resources/    - Execution memory DB
```

---

## 5. Spec-Kit UI Requirements

### From ai/shared/resources/spec/ (GitHub Spec-Kit)

The spec-kit provides a **spec-driven development** framework but does **not prescribe specific UI requirements**. Key principles:

1. **Constitution-First**: UI must reflect CONSTITUTION.md principles
2. **Transparent**: Live scrollable log of agent's thought process (§3.5)
3. **Local-First**: Core UI must work offline (§3.2)
4. **Self-Contained**: All UI assets under noa_root (§3.1)

### Planned UI Crates (docs/wiki/internal-crates/ui-app/)

```
noa-ui-core        - AppState, hooks (use_agent, use_task, use_theme)
noa-ui-styleguide  - Design tokens (colors, spacing, typography)
noa-ui-shell       - Layout components
noa-ui-protocol    - Backend communication
```

---

## 6. Identified Fragmentation Issues

### Critical Issues

| Issue | Impact | Locations Affected |
|-------|--------|--------------------|
| **React version mismatch** | Breaking changes | React 18 vs 19 |
| **Tailwind version mismatch** | Styling inconsistencies | v3 vs v4 |
| **Duplicate codebase** | Maintenance burden | ml_devops_platform/ duplicates sys/ui/apps/ml-devops/ |
| **No shared component library** | Code duplication | All UIs have separate components |
| **Inconsistent design tokens** | Visual inconsistency | HSL vs CSS vars vs Rust constants |

### Detailed Fragmentation Map

#### 1. Tailwind configsuration Inconsistencies

| Location | Version | Color System |
|----------|---------|--------------|
| sys/ui/ | 4.1.17 | CSS variables |
| sys/ui/apps/ml-devops/ | 3.3.3 | HSL variables (shadcn) |
| ml_devops_platform/nextjs_space/ | 3.3.3 | HSL variables (shadcn) |
| ui/app/styleguide | N/A | Rust constants |

#### 2. Component Library Duplication

| Component | sys/ui/ | ml-devops/ | rust-lovable/ |
|-----------|---------|------------|---------------|
| Chat | ✅ ChatInterface.tsx | ✅ chat-interface.tsx | ✅ chat.rs |
| Button | ❌ (inline) | ✅ button.tsx | ✅ ui_components.rs |
| Card | ❌ (inline) | ✅ card.tsx | ❌ |
| Dialog | ❌ | ✅ dialog.tsx | ❌ |
| **Total UI Components** | ~10 | ~48 | ~8 |

#### 3. API Client Duplication

| Location | API Pattern |
|----------|-------------|
| sys/ui/ | fetch() direct |
| ml-devops/ | Provider abstraction + fetch |
| rust-lovable/ | reqwest + custom client |

---

## 7. Recommended File Locations (per AGENTS.md)

### Unified UI Architecture

Per AGENTS.md repository structure, the recommended locations are:

```
noa/
├── ui/                           # PRIMARY: All UI components
│   └── app/                      # Unified Dioxus workspace (in progress)
│       ├── crates/
│       │   ├── noa-ui-core/      # Shared state, hooks, types
│       │   ├── noa-ui-shell/     # Layout components
│       │   ├── noa-ui-styleguide-api/  # Design tokens
│       │   └── noa-ui-styleguide-ui/   # Visual components
│       └── bins/
│           ├── noa-ui-desktop/   # Tauri desktop
│           ├── noa-ui-web/       # WASM web
│           └── noa-ui-hived/     # P2P node
│
├── sys/ui/                       # TRANSITIONAL: Next.js dashboard
│   ├── src/components/           # Migrate to ui/app/
│   └── apps/ml-devops/          # Keep for now, extract shared components
│
├── gateway/mcp/                  # API Gateway (MCP)
│   └── ui-bridge/               # Push events to UI
│
├── configs/                       # Centralized configs
│   ├── ai-providers.json        # Provider configsuration
│   └── schemas/                 # JSON schemas
│
└── ai/shared/                    # Shared AI resources
    ├── agents/
    ├── prompts/
    └── resources/
```

---

## 8. Unification Recommendations

### Phase 1: Immediate Actions

1. **Deprecate ml_devops_platform/nextjs_space/** - It duplicates sys/ui/apps/ml-devops/
2. **Consolidate design tokens** - Create unified token system in configs/schemas/design-tokens.json
3. **Extract shared components** - Move shadcn/ui components from ml-devops to shared location

### Phase 2: Short-term (1-2 weeks)

1. **Upgrade React/Next.js versions** - Align on React 19 + Next.js 15
2. **Standardize Tailwind** - Migrate to Tailwind v4 across all UIs
3. **Create @noa/ui-components package** - Shared React component library

### Phase 3: Medium-term (1 month)

1. **Complete ui/app Dioxus migration** - Per MIGRATION_MAP.md
2. **Implement gateway/mcp/ui-bridge** - Unified event push to all UI targets
3. **Create unified AI provider SDK** - Single interface for all UIs

### Phase 4: Long-term

1. **Single Dioxus codebase** - Web + Desktop + Mobile from one source
2. **Remove Next.js dependencies** - Full Rust stack
3. **P2P UI sync** - Hive mind UI state synchronization

---

## 9. Constitutional Compliance

| Principle | Current State | Required Action |
|-----------|---------------|-----------------|
| §3.1 Self-Contained | ⚠️ Multiple entry points | Unify under ui/app |
| §3.2 Local-First | ⚠️ Some UIs require network | Ensure offline mode |
| §3.5 Transparent | ⚠️ No unified agent log | Add live agent log component |
| §3.7 Memory Sovereignty | ⚠️ Separate state stores | Unified state via noa-ui-core |

---

## 10. Next Steps

1. **Create UI unification spec** at `specs/xxx-ui-unification/spec.md`
2. **Define design token schema** at `configs/schemas/design-tokens.json`
3. **Create migration tasks** at `specs/xxx-ui-unification/tasks.md`
4. **Update AGENTS.md** with unified UI guidance

---

## Appendix: File Inventory

### Package.json Files with UI Dependencies

1. `sys/ui/package.json` - @noa/ui
2. `sys/ui/apps/ml-devops/package.json` - app (ml-devops)
3. `ml_devops_platform/nextjs_space/package.json` - app (duplicate)
4. `pkg/sys/ui/package.json` - @noa/ui (duplicate)
5. `ruler/package.json` - @intellectronica/ruler (CLI, no UI)

### Cargo.toml Files with UI Dependencies

1. `ui/rust-lovable/rust-lovable/Cargo.toml` - rust-lovable
2. `ui/app/Cargo.toml` - noa-ui workspace

### Tailwind configs Files

1. `sys/ui/tailwind.configs.ts`
2. `sys/ui/apps/ml-devops/tailwind.configs.ts`
3. `ml_devops_platform/nextjs_space/tailwind.configs.ts`
4. `pkg/sys/ui/tailwind.configs.ts`
