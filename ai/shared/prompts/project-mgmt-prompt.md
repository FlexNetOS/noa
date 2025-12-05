# Project Management Consolidation Prompt

> **Purpose:** Master prompt for merging, consolidating, and refactoring all project-mgmt content and repos into a unified, cross-platform, kernel-independent application.

---

## Mission Statement

Consolidate 18+ disparate project management repositories into a single, unified **project-mgmt** application that:

### Core Platform
- Operates as a **fully automated AgenticAI** project management system
- Connects to `${NOA_ROOT}/ai/shared` for unified AI resources across all providers
- Runs independently of host kernel on any platform
- Maintains **all 95 features** from source repositories (comprehensive 10-loop scan verified)
- Uses a single unified environment at `${PROJECT_ROOT}/.project-mgmt-env`
- Prioritizes local AI providers (llama.cpp, Ollama) over cloud

### AgenticAI Pipeline (UI Chat → Execute)
- **Goal Decomposition**: Break natural language requests into atomic goals
- **Prompt Engineering**: Generate optimized prompts (DSPy, PromptFusion)
- **Goals → Policy → Rules**: Structured governance framework
- **Memory**: Never forget, instant recall (semantic search)
- **Checklist → Plan → Spec → Task**: Progressive refinement
- **XML/JSON Output → Execute**: Structured autonomous execution

### Agentic Capabilities
- **18 Core Capabilities**: Orchestration, Loop Reasoning, Commands, Automation, Verification
- **Self-improving Agents**: DSPy training, PRD-based learning (Python scripts)
- **Self-healing System**: Auto-recovery, error handling
- **Multi-agent Collaboration**: BMAD (10 agents) + PRP (7 agents) = 17 coordinated agents
- **6 Automated Workflows**: Greenfield/Brownfield for Fullstack/Service/UI
- **Signal System**: PRP priority signals (ATTENTION 10 → COMPLETED 1)
- **Feedback Loops**: Continuous evaluation and self-improvement

### Integrations & Deployments
- **8 Issue Providers**: Jira, GitHub, GitLab, Gitea, OpenProject, Redmine, Trello, CalDAV
- **35+ MCP Tools**: From claude-task-master
- **25+ System Prompts**: From industry AI tools
- **15 Productivity Features**: Pomodoro, Focus Mode, etc.
- **Single-File HTML Deployment**: my-todo-app style (GitHub Pages ready)

---

## 1) Source Repositories to Consolidate

### Primary Repos (Must Retain ALL Features)

| # | Repo | Key Capabilities | Priority |
|---|------|------------------|----------|
| 01 | `prp-main` | Product Requirements Planning | Critical |
| 02 | `PRPs-agentic-eng-development` | Agentic engineering workflows | Critical |
| 03 | `spec-kit` | Specification management, CLI tools, provider auto-detection | Critical |
| 04 | `super-productivity` | Time tracking, tasks, Jira/GitHub sync | Critical |
| 05 | `BMAD-METHOD-v5` | Business Model Agentic Development | High |
| 06 | `BMAD-METHOD` | Legacy BMAD (different implementation) | High |
| 07 | `agentic-cursorrules` | AI agent rules engine | High |
| 08 | `agent-rules` | Rule definitions for agents | High |
| 09 | `ruler` | Rule evaluation engine | High |
| 10 | `my-todo-app-main` | Simple todo implementation | Medium |
| 11 | `tududi` | Full-stack todo with i18n | Medium |
| 12 | `Taskosaur` | Task management with SQL backend | Medium |
| 13 | `system-prompts-and-models-of-ai-tools` | AI prompt library | High |
| 14 | `promptfusion` | Prompt engineering tools | High |
| 15 | `dspy` | DSPy ML pipelines | High |
| 16 | `dspy-code` | DSPy code implementations | High |
| 17 | `claude-task-master` | Claude Code task management | Critical |
| 18 | `Backlog.md` | Backlog management MCP server | High |

### Capability Extraction Matrix

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CAPABILITY → UNIFIED FEATURE MAPPING                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  TASK MANAGEMENT                                                            │
│  ├─ super-productivity  → Core task engine, time tracking                   │
│  ├─ Taskosaur           → SQL-based persistence, reporting                  │
│  ├─ tududi              → Multi-language UI, mobile support                 │
│  ├─ my-todo-app-main    → Lightweight offline mode                          │
│  └─ claude-task-master  → AI-assisted task breakdown                        │
│                                                                             │
│  SPECIFICATION & PLANNING                                                   │
│  ├─ spec-kit            → Spec templates, validation, provider detection    │
│  ├─ prp-main            → Requirements documentation                        │
│  ├─ PRPs-agentic-eng    → Agentic workflow specs                            │
│  └─ BMAD-METHOD*        → Business model specifications                     │
│                                                                             │
│  AI & PROMPTS                                                               │
│  ├─ promptfusion        → Prompt composition, testing                       │
│  ├─ system-prompts      → Prompt library, model configs                     │
│  ├─ dspy/dspy-code      → ML pipeline integration                           │
│  └─ claude-task-master  → Claude Code integration                           │
│                                                                             │
│  RULES & POLICY                                                             │
│  ├─ ruler               → Rule evaluation engine                            │
│  ├─ agent-rules         → Rule definitions                                  │
│  └─ agentic-cursorrules → Cursor-specific rules                             │
│                                                                             │
│  BACKLOG & MCP                                                              │
│  └─ Backlog.md          → MCP server, backlog management                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2) Provider Auto-Detection System

> Ported from spec-kit - ensures all AI agents, CLI tools, and IDE extensions are wired in for shared access.

### Supported AI Agent Providers

| Agent Key | Display Name | Config Folder | Requires CLI | Install URL |
|-----------|--------------|---------------|--------------|-------------|
| `claude` | Claude Code | `.claude/` | Yes | https://docs.anthropic.com/en/docs/claude-code/setup |
| `gemini` | Gemini CLI | `.gemini/` | Yes | https://github.com/google-gemini/gemini-cli |
| `copilot` | GitHub Copilot | `.github/` | No (IDE) | N/A |
| `cursor-agent` | Cursor | `.cursor/` | No (IDE) | N/A |
| `qwen` | Qwen Code | `.qwen/` | Yes | https://github.com/QwenLM/qwen-code |
| `opencode` | opencode | `.opencode/` | Yes | https://opencode.ai |
| `codex` | Codex CLI | `.codex/` | Yes | https://github.com/openai/codex |
| `windsurf` | Windsurf | `.windsurf/` | No (IDE) | N/A |
| `kilocode` | Kilo Code | `.kilocode/` | No (IDE) | N/A |
| `auggie` | Auggie CLI | `.augment/` | Yes | https://docs.augmentcode.com/cli |
| `codebuddy` | CodeBuddy | `.codebuddy/` | Yes | https://www.codebuddy.ai/cli |
| `qoder` | Qoder CLI | `.qoder/` | Yes | https://qoder.com/cli |
| `roo` | Roo Code | `.roo/` | No (IDE) | N/A |
| `q` | Amazon Q Developer | `.amazonq/` | Yes | https://aws.amazon.com/q-developer-cli |
| `amp` | Amp | `.agents/` | Yes | https://ampcode.com |
| `shai` | SHAI | `.shai/` | Yes | https://github.com/ovh/shai |
| `bob` | IBM Bob | `.bob/` | No (IDE) | N/A |

### Agent Configuration Registry

```typescript
// core/providers/agent-config.ts
export interface AgentConfig {
  name: string;           // Human-readable display name
  folder: string;         // Directory for agent files (relative to project root)
  installUrl: string | null;  // Installation docs URL (null for IDE-based)
  requiresCli: boolean;   // True if CLI tool required, False for IDE-based
}

export const AGENT_CONFIG: Record<string, AgentConfig> = {
  'claude': {
    name: 'Claude Code',
    folder: '.claude/',
    installUrl: 'https://docs.anthropic.com/en/docs/claude-code/setup',
    requiresCli: true,
  },
  'gemini': {
    name: 'Gemini CLI',
    folder: '.gemini/',
    installUrl: 'https://github.com/google-gemini/gemini-cli',
    requiresCli: true,
  },
  'copilot': {
    name: 'GitHub Copilot',
    folder: '.github/',
    installUrl: null,
    requiresCli: false,
  },
  'cursor-agent': {
    name: 'Cursor',
    folder: '.cursor/',
    installUrl: null,
    requiresCli: false,
  },
  'qwen': {
    name: 'Qwen Code',
    folder: '.qwen/',
    installUrl: 'https://github.com/QwenLM/qwen-code',
    requiresCli: true,
  },
  'opencode': {
    name: 'opencode',
    folder: '.opencode/',
    installUrl: 'https://opencode.ai',
    requiresCli: true,
  },
  'codex': {
    name: 'Codex CLI',
    folder: '.codex/',
    installUrl: 'https://github.com/openai/codex',
    requiresCli: true,
  },
  'windsurf': {
    name: 'Windsurf',
    folder: '.windsurf/',
    installUrl: null,
    requiresCli: false,
  },
  'kilocode': {
    name: 'Kilo Code',
    folder: '.kilocode/',
    installUrl: null,
    requiresCli: false,
  },
  'auggie': {
    name: 'Auggie CLI',
    folder: '.augment/',
    installUrl: 'https://docs.augmentcode.com/cli/setup-auggie/install-auggie-cli',
    requiresCli: true,
  },
  'codebuddy': {
    name: 'CodeBuddy',
    folder: '.codebuddy/',
    installUrl: 'https://www.codebuddy.ai/cli',
    requiresCli: true,
  },
  'qoder': {
    name: 'Qoder CLI',
    folder: '.qoder/',
    installUrl: 'https://qoder.com/cli',
    requiresCli: true,
  },
  'roo': {
    name: 'Roo Code',
    folder: '.roo/',
    installUrl: null,
    requiresCli: false,
  },
  'q': {
    name: 'Amazon Q Developer CLI',
    folder: '.amazonq/',
    installUrl: 'https://aws.amazon.com/developer/learning/q-developer-cli/',
    requiresCli: true,
  },
  'amp': {
    name: 'Amp',
    folder: '.agents/',
    installUrl: 'https://ampcode.com/manual#install',
    requiresCli: true,
  },
  'shai': {
    name: 'SHAI',
    folder: '.shai/',
    installUrl: 'https://github.com/ovh/shai',
    requiresCli: true,
  },
  'bob': {
    name: 'IBM Bob',
    folder: '.bob/',
    installUrl: null,
    requiresCli: false,
  },
};
```

### Provider Auto-Detection Service

```typescript
// core/providers/provider-detection.service.ts
import { existsSync } from 'fs';
import { execSync } from 'child_process';
import { join, resolve } from 'path';
import { AGENT_CONFIG, AgentConfig } from './agent-config';

export interface DetectedProvider {
  key: string;
  config: AgentConfig;
  isAvailable: boolean;
  cliPath?: string;
  configPath?: string;
  version?: string;
}

export interface ProviderDetectionResult {
  detected: DetectedProvider[];
  cliProviders: DetectedProvider[];
  ideProviders: DetectedProvider[];
  primaryProvider: DetectedProvider | null;
}

/**
 * Cross-platform tool detection using which/where
 */
export function checkToolAvailable(tool: string): string | null {
  // Special handling for Claude CLI after migrate-installer
  // The migrate-installer command creates an alias at ~/.claude/local/claude
  if (tool === 'claude') {
    const homeDir = process.env.HOME || process.env.USERPROFILE || '';
    const claudeLocalPath = join(homeDir, '.claude', 'local', 'claude');
    if (existsSync(claudeLocalPath)) {
      return claudeLocalPath;
    }
  }

  try {
    const cmd = process.platform === 'win32' ? 'where' : 'which';
    const result = execSync(`${cmd} ${tool}`, {
      encoding: 'utf-8',
      stdio: ['pipe', 'pipe', 'pipe']
    }).trim();
    return result.split('\n')[0] || null;
  } catch {
    return null;
  }
}

/**
 * Get version of a CLI tool
 */
export function getToolVersion(tool: string): string | null {
  try {
    const result = execSync(`${tool} --version`, {
      encoding: 'utf-8',
      stdio: ['pipe', 'pipe', 'pipe'],
      timeout: 5000
    }).trim();
    return result.split('\n')[0] || null;
  } catch {
    return null;
  }
}

/**
 * Detect all available providers
 */
export function detectProviders(projectRoot?: string): ProviderDetectionResult {
  const root = projectRoot || process.cwd();
  const detected: DetectedProvider[] = [];

  for (const [key, config] of Object.entries(AGENT_CONFIG)) {
    const configPath = resolve(root, config.folder);
    const hasConfigFolder = existsSync(configPath);

    let isAvailable = false;
    let cliPath: string | undefined;
    let version: string | undefined;

    if (config.requiresCli) {
      // CLI-based provider - check if CLI tool is installed
      const toolPath = checkToolAvailable(key);
      if (toolPath) {
        isAvailable = true;
        cliPath = toolPath;
        version = getToolVersion(key) || undefined;
      }
    } else {
      // IDE-based provider - available if config folder exists or IDE is detected
      isAvailable = hasConfigFolder;
    }

    detected.push({
      key,
      config,
      isAvailable,
      cliPath,
      configPath: hasConfigFolder ? configPath : undefined,
      version
    });
  }

  const cliProviders = detected.filter(p => p.config.requiresCli && p.isAvailable);
  const ideProviders = detected.filter(p => !p.config.requiresCli && p.isAvailable);

  // Determine primary provider (prefer Claude, then first available CLI, then first IDE)
  let primaryProvider: DetectedProvider | null = null;
  const claude = detected.find(p => p.key === 'claude' && p.isAvailable);
  if (claude) {
    primaryProvider = claude;
  } else if (cliProviders.length > 0) {
    primaryProvider = cliProviders[0];
  } else if (ideProviders.length > 0) {
    primaryProvider = ideProviders[0];
  }

  return {
    detected,
    cliProviders,
    ideProviders,
    primaryProvider
  };
}

/**
 * Initialize provider directories for a project
 */
export function initializeProviderDirs(
  projectRoot: string,
  providers: string[]
): void {
  const { mkdirSync, writeFileSync } = require('fs');

  for (const providerKey of providers) {
    const config = AGENT_CONFIG[providerKey];
    if (!config) continue;

    const configDir = resolve(projectRoot, config.folder);
    if (!existsSync(configDir)) {
      mkdirSync(configDir, { recursive: true });
    }

    // Create commands subdirectory for CLI-based providers
    if (config.requiresCli) {
      const commandsDir = join(configDir, 'commands');
      if (!existsSync(commandsDir)) {
        mkdirSync(commandsDir, { recursive: true });
      }
    }
  }
}
```

### IDE Extension Detection

```typescript
// core/providers/ide-detection.service.ts
import { existsSync } from 'fs';
import { join } from 'path';

export interface IDEInfo {
  name: string;
  isRunning: boolean;
  extensionsPath?: string;
  workspacePath?: string;
}

/**
 * Detect running IDE from environment variables
 */
export function detectRunningIDE(): IDEInfo | null {
  // VS Code / Cursor detection
  if (process.env.VSCODE_PID || process.env.TERM_PROGRAM === 'vscode') {
    return {
      name: process.env.CURSOR_TRACE ? 'cursor' : 'vscode',
      isRunning: true,
      workspacePath: process.env.VSCODE_CWD
    };
  }

  // Cursor-specific detection
  if (process.env.CURSOR_TRACE || process.env.CURSOR_SESSION) {
    return {
      name: 'cursor',
      isRunning: true,
      workspacePath: process.cwd()
    };
  }

  // Windsurf detection
  if (process.env.WINDSURF_SESSION) {
    return {
      name: 'windsurf',
      isRunning: true,
      workspacePath: process.cwd()
    };
  }

  // JetBrains IDE detection
  if (process.env.JETBRAINS_IDE) {
    return {
      name: 'jetbrains',
      isRunning: true,
      workspacePath: process.cwd()
    };
  }

  return null;
}

/**
 * Check if VS Code extensions are installed
 */
export function checkVSCodeExtensions(): string[] {
  const installed: string[] = [];

  try {
    const { execSync } = require('child_process');
    const result = execSync('code --list-extensions', {
      encoding: 'utf-8',
      stdio: ['pipe', 'pipe', 'pipe']
    });

    const extensions = result.split('\n').filter(Boolean);

    // Check for known AI extensions
    const aiExtensions = {
      'github.copilot': 'copilot',
      'github.copilot-chat': 'copilot',
      'anthropic.claude-code': 'claude',
      'codeium.codeium': 'codeium',
      'tabnine.tabnine-vscode': 'tabnine',
      'amazonwebservices.amazon-q-vscode': 'q'
    };

    for (const [extId, provider] of Object.entries(aiExtensions)) {
      if (extensions.some(e => e.toLowerCase() === extId.toLowerCase())) {
        installed.push(provider);
      }
    }
  } catch {
    // VS Code CLI not available
  }

  return installed;
}
```

### Shared Provider Access Service

```typescript
// core/providers/shared-access.service.ts
import { detectProviders, DetectedProvider, ProviderDetectionResult } from './provider-detection.service';
import { detectRunningIDE, checkVSCodeExtensions } from './ide-detection.service';
import { AGENT_CONFIG } from './agent-config';

export interface SharedProviderContext {
  projectRoot: string;
  detection: ProviderDetectionResult;
  runningIDE: ReturnType<typeof detectRunningIDE>;
  vsCodeExtensions: string[];
  activeProvider: DetectedProvider | null;
}

let _cachedContext: SharedProviderContext | null = null;

/**
 * Initialize shared provider context for the project
 */
export async function initSharedProviderAccess(
  projectRoot?: string,
  forceRefresh = false
): Promise<SharedProviderContext> {
  if (_cachedContext && !forceRefresh) {
    return _cachedContext;
  }

  const root = projectRoot || process.cwd();
  const detection = detectProviders(root);
  const runningIDE = detectRunningIDE();
  const vsCodeExtensions = checkVSCodeExtensions();

  // Determine active provider based on context
  let activeProvider = detection.primaryProvider;

  // If running in an IDE, prefer IDE-based providers
  if (runningIDE) {
    const ideProviderKey = runningIDE.name === 'cursor' ? 'cursor-agent' : runningIDE.name;
    const ideProvider = detection.detected.find(p => p.key === ideProviderKey);
    if (ideProvider?.isAvailable) {
      activeProvider = ideProvider;
    }
  }

  _cachedContext = {
    projectRoot: root,
    detection,
    runningIDE,
    vsCodeExtensions,
    activeProvider
  };

  return _cachedContext;
}

/**
 * Get all available providers for the current context
 */
export function getAvailableProviders(): DetectedProvider[] {
  if (!_cachedContext) {
    throw new Error('Shared provider access not initialized. Call initSharedProviderAccess() first.');
  }
  return _cachedContext.detection.detected.filter(p => p.isAvailable);
}

/**
 * Switch active provider
 */
export function setActiveProvider(providerKey: string): boolean {
  if (!_cachedContext) {
    throw new Error('Shared provider access not initialized.');
  }

  const provider = _cachedContext.detection.detected.find(
    p => p.key === providerKey && p.isAvailable
  );

  if (provider) {
    _cachedContext.activeProvider = provider;
    return true;
  }

  return false;
}

/**
 * Generate agent files for all detected providers
 */
export async function syncAgentFiles(projectRoot: string): Promise<void> {
  const context = await initSharedProviderAccess(projectRoot);
  const { mkdirSync, writeFileSync, existsSync } = require('fs');
  const { resolve, join } = require('path');

  for (const provider of context.detection.detected.filter(p => p.isAvailable)) {
    const configDir = resolve(projectRoot, provider.config.folder);

    if (!existsSync(configDir)) {
      mkdirSync(configDir, { recursive: true });
    }

    // Create/update AGENTS.md in each provider folder
    const agentsPath = join(configDir, 'AGENTS.md');
    const agentsContent = generateAgentsMd(provider);
    writeFileSync(agentsPath, agentsContent, 'utf-8');
  }
}

function generateAgentsMd(provider: DetectedProvider): string {
  return `# ${provider.config.name} Configuration

> Auto-generated by project-mgmt provider detection.

## Provider Status

- **Available:** ${provider.isAvailable ? 'Yes' : 'No'}
- **CLI Path:** ${provider.cliPath || 'N/A'}
- **Version:** ${provider.version || 'Unknown'}
- **Type:** ${provider.config.requiresCli ? 'CLI-based' : 'IDE-based'}

## Commands

Add your ${provider.config.name} commands in this directory.

## Integration

This provider is integrated with the project-mgmt shared access system.
`;
}
```

---

## 3) Target Architecture

### Unified Directory Structure

> **Comprehensive structure for fully automated AgenticAI project management.**

```
${PROJECT_ROOT}/
├── .project-mgmt                 # Project marker file
├── .project-mgmt-env/            # Unified environment (SINGLE ENV)
│   ├── node_modules/             # All Node.js dependencies
│   ├── python-venv/              # Python virtual environment
│   ├── cargo/                    # Rust dependencies
│   ├── .env                      # Environment variables
│   └── .env.example              # Template
│
├── project-mgmt/                 # Main application directory
│   ├── package.json              # Consolidated dependencies
│   ├── pyproject.toml            # Python dependencies
│   ├── Cargo.toml                # Rust dependencies (if any)
│   │
│   ├── apps/                     # Platform applications
│   │   ├── web/                  # Angular PWA (from super-productivity)
│   │   │   ├── features/         # Feature modules
│   │   │   │   ├── tasks/        # Task management UI
│   │   │   │   ├── boards/       # Kanban boards
│   │   │   │   ├── planner/      # Planner & scheduling
│   │   │   │   ├── calendar/     # Calendar views
│   │   │   │   ├── focus-mode/   # Focus mode UI
│   │   │   │   ├── pomodoro/     # Pomodoro timer
│   │   │   │   ├── worklog/      # Worklog & export
│   │   │   │   ├── metric/       # Productivity metrics
│   │   │   │   └── issue/        # Issue provider UIs
│   │   │   ├── pages/            # Application pages
│   │   │   │   ├── config/       # Settings page
│   │   │   │   ├── daily-summary/# Daily summary
│   │   │   │   ├── metric/       # Metrics page
│   │   │   │   └── search/       # Search page
│   │   │   └── ui/               # Shared UI components
│   │   │
│   │   ├── desktop/              # Electron app
│   │   │   ├── ipc-handlers/     # IPC communication
│   │   │   │   ├── app-control/  # App lifecycle
│   │   │   │   ├── global-shortcuts/ # Keyboard shortcuts
│   │   │   │   ├── jira/         # Jira native integration
│   │   │   │   └── system/       # System operations
│   │   │   ├── tray/             # System tray (indicator)
│   │   │   ├── overlay/          # Floating overlay indicator
│   │   │   ├── lockscreen/       # Lockscreen mode
│   │   │   ├── idle-handler/     # Idle time detection
│   │   │   ├── full-screen-blocker/ # Break enforcement
│   │   │   └── plugin-executor/  # Plugin node executor
│   │   │
│   │   ├── mobile/               # Capacitor app (Android/iOS)
│   │   │   └── saf-adapter/      # Android SAF file adapter
│   │   │
│   │   ├── cli/                  # Command-line interface
│   │   │   ├── commands/         # CLI commands
│   │   │   ├── tui/              # Terminal UI (from Backlog.md)
│   │   │   └── completions/      # Shell completions (bash, fish, zsh)
│   │   │
│   │   └── mcp/                  # MCP server
│   │       ├── tools/            # MCP tool definitions
│   │       │   ├── tasks/        # Task CRUD tools
│   │       │   ├── documents/    # Document tools
│   │       │   └── workflow/     # Workflow tools
│   │       ├── resources/        # MCP resources
│   │       └── prompts/          # MCP prompts
│   │
│   ├── core/                     # Shared core logic
│   │   ├── ai-shared/            # ← Connection to ${NOA_ROOT}/ai/shared
│   │   │   ├── index.ts          # Re-exports & path resolution
│   │   │   └── symlinks.ts       # Symlink management
│   │   │
│   │   ├── providers/            # Provider detection & management
│   │   │   ├── agent-config.ts   # Agent configuration registry
│   │   │   ├── provider-detection.service.ts
│   │   │   ├── provider-priority.service.ts  # Local > hybrid > cloud
│   │   │   ├── ide-detection.service.ts
│   │   │   └── shared-access.service.ts
│   │   │
│   │   ├── memory/               # Memory system (never forget)
│   │   │   ├── memory.service.ts # Persistent memory storage
│   │   │   └── recall.ts         # Instant semantic recall
│   │   │
│   │   ├── tasks/                # Task management engine
│   │   │   ├── task.model.ts     # Task data model
│   │   │   ├── task.service.ts   # Task CRUD operations
│   │   │   ├── subtasks/         # Subtask management
│   │   │   ├── dependencies/     # Task dependencies
│   │   │   ├── repeat/           # Recurring tasks
│   │   │   ├── scoping/          # Task scoping (up/down)
│   │   │   ├── expansion/        # Task expansion (AI)
│   │   │   └── complexity/       # Complexity analysis
│   │   │
│   │   ├── goals/                # Goals tracking
│   │   ├── policy/               # Policy engine
│   │   ├── rules/                # Rules engine (from ruler)
│   │   ├── specs/                # Spec management (from spec-kit)
│   │   ├── backlog/              # Backlog management
│   │   │
│   │   ├── time-tracking/        # Time tracking
│   │   │   ├── tracking.service.ts
│   │   │   ├── idle/             # Idle detection
│   │   │   ├── pomodoro/         # Pomodoro timer
│   │   │   ├── focus-mode/       # Focus mode
│   │   │   ├── take-a-break/     # Break reminders
│   │   │   └── worklog/          # Worklog & export
│   │   │
│   │   ├── sync/                 # Multi-provider sync (pfapi)
│   │   │   ├── vector-clock/     # Conflict resolution
│   │   │   ├── encryption/       # AES-256-GCM encryption
│   │   │   ├── compression/      # Gzip compression
│   │   │   └── providers/        # Sync providers
│   │   │       ├── dropbox/      # Dropbox sync
│   │   │       ├── webdav/       # WebDAV sync
│   │   │       ├── local/        # Local file sync
│   │   │       └── git/          # Git-based sync
│   │   │
│   │   ├── state/                # NgRx state management
│   │   │   ├── root-store/       # Root state
│   │   │   ├── meta-reducers/    # Meta reducers (undo, batch)
│   │   │   └── effects/          # Side effects
│   │   │
│   │   ├── config/               # Configuration
│   │   │   ├── global-config.ts  # Global config model
│   │   │   ├── form-cfgs/        # Form configurations
│   │   │   │   ├── pomodoro.ts   # Pomodoro settings
│   │   │   │   ├── focus-mode.ts # Focus mode settings
│   │   │   │   ├── idle.ts       # Idle settings
│   │   │   │   ├── sync.ts       # Sync settings
│   │   │   │   └── keyboard.ts   # Keyboard shortcuts
│   │   │   └── keyboard.model.ts # Keyboard config
│   │   │
│   │   ├── imex/                 # Import/Export
│   │   │   ├── file-imex/        # File import/export
│   │   │   ├── url-import/       # URL import
│   │   │   ├── local-backup/     # Local backups
│   │   │   └── privacy-export/   # Privacy-safe export
│   │   │
│   │   └── util/                 # Utilities
│   │       ├── hash.ts           # Hashing (MD5, SHA)
│   │       ├── speak.ts          # Text-to-speech
│   │       ├── play-sound.ts     # Sound effects
│   │       └── parse-markdown-tasks.ts # Markdown task parsing
│   │
│   ├── ai/                       # AI integration layer
│   │   ├── providers/            # AI provider adapters
│   │   │   ├── claude/           # Claude integration
│   │   │   ├── openai/           # OpenAI integration
│   │   │   ├── ollama/           # Local Ollama
│   │   │   └── llama-cpp/        # llama.cpp integration
│   │   │
│   │   ├── prompts/              # Unified prompt library
│   │   │   ├── system-prompts/   # 25+ AI tool prompts
│   │   │   │   ├── cursor/       # Cursor prompts
│   │   │   │   ├── claude-code/  # Claude Code prompts
│   │   │   │   ├── vscode-agent/ # VS Code Agent prompts
│   │   │   │   ├── windsurf/     # Windsurf prompts
│   │   │   │   ├── devin/        # Devin prompts
│   │   │   │   └── ...           # 20+ more providers
│   │   │   ├── fusion/           # Prompt fusion engine
│   │   │   └── triggers/         # Prompt triggers
│   │   │
│   │   ├── dspy/                 # DSPy pipelines
│   │   │   ├── modules/          # DSPy modules
│   │   │   ├── optimizers/       # Optimizers
│   │   │   └── signatures/       # Signatures
│   │   │
│   │   └── agents/               # Agentic workflows
│   │       ├── bmad/             # BMAD agents
│   │       │   ├── analyst.md
│   │       │   ├── architect.md
│   │       │   ├── bmad-orchestrator.md
│   │       │   ├── dev.md
│   │       │   ├── pm.md
│   │       │   ├── po.md
│   │       │   ├── qa.md
│   │       │   ├── sm.md
│   │       │   └── ux-expert.md
│   │       │
│   │       ├── prp/              # PRP agents
│   │       │   ├── orchestrator.md
│   │       │   ├── robo-developer.md
│   │       │   ├── robo-aqa.md
│   │       │   ├── robo-quality-control.md
│   │       │   ├── robo-system-analyst.md
│   │       │   ├── robo-devops-sre.md
│   │       │   └── robo-ux-ui-designer.md
│   │       │
│   │       ├── commands/         # Agentic commands
│   │       │   ├── code-quality/ # Review, refactor
│   │       │   ├── development/  # Create PR, debug, onboarding
│   │       │   ├── git-ops/      # Conflict resolution
│   │       │   ├── prp/          # PRP create/execute commands
│   │       │   └── rapid-dev/    # Hackathon, parallel PRPs
│   │       │
│   │       ├── workflows/        # BMAD workflows
│   │       │   ├── greenfield-fullstack.yaml
│   │       │   ├── greenfield-service.yaml
│   │       │   ├── greenfield-ui.yaml
│   │       │   ├── brownfield-fullstack.yaml
│   │       │   ├── brownfield-service.yaml
│   │       │   └── brownfield-ui.yaml
│   │       │
│   │       ├── teams/            # Agent teams
│   │       │   ├── team-all.yaml
│   │       │   ├── team-fullstack.yaml
│   │       │   └── team-ide-minimal.yaml
│   │       │
│   │       └── expansion-packs/  # BMAD expansion packs
│   │           ├── game-dev/     # 2D game development
│   │           ├── creative-writing/
│   │           └── infrastructure/
│   │
│   ├── integrations/             # External service integrations
│   │   ├── issue-providers/      # Issue tracker integrations
│   │   │   ├── jira/             # Jira (with worklog)
│   │   │   ├── github/           # GitHub Issues/PRs
│   │   │   ├── gitlab/           # GitLab Issues/MRs
│   │   │   ├── gitea/            # Gitea Issues
│   │   │   ├── open-project/     # OpenProject
│   │   │   ├── redmine/          # Redmine
│   │   │   ├── trello/           # Trello boards
│   │   │   └── caldav/           # CalDAV calendar
│   │   │
│   │   ├── sync-providers/       # Data sync integrations
│   │   │   ├── dropbox/          # Dropbox sync
│   │   │   ├── webdav/           # WebDAV sync
│   │   │   └── local-file/       # Local file sync
│   │   │
│   │   └── chrome-extension/     # Chrome extension interface
│   │
│   ├── docs/                     # Documentation (331+ files)
│   │   ├── 00-guides/            # How-to guides
│   │   │   └── goals-policy-rules-guide.md
│   │   ├── 00-ideas/             # Feature ideas
│   │   │   ├── add-goals.md
│   │   │   ├── add-north-star-goal.md
│   │   │   └── key-additions.md
│   │   ├── 01-templates/         # Document templates
│   │   │   ├── goal-template.md
│   │   │   ├── policy-template.md
│   │   │   └── rules-template.md
│   │   ├── 02-examples/          # Annotated examples
│   │   │   ├── goal-example.md
│   │   │   ├── policy-example.md
│   │   │   └── rules-example.md
│   │   ├── 04-goals/             # Goal documents + images
│   │   │   └── env-goals.md
│   │   ├── 05-policy/            # Policy documents
│   │   │   ├── env-policy.md
│   │   │   └── universal_task_execution_policy.md  # Critical!
│   │   ├── 06-rules/             # Rule documents
│   │   │   └── env-rule.md
│   │   ├── 07-plans/             # Architecture plans
│   │   │   ├── autonomous-system.md   # CECCA Framework
│   │   │   └── autonomous-system.mmd  # 108KB Mermaid graph
│   │   ├── ai/                   # AI implementation docs
│   │   │   └── webdav-non-etag-implementation-plan.md
│   │   ├── sync/                 # Sync documentation
│   │   │   └── vector-clocks.md  # Vector clock sync
│   │   ├── promotion/            # Store banners (SVG)
│   │   ├── screens/              # Screenshots (PNG)
│   │   └── goal-ledger.md        # Central goal registry
│   │
│   ├── templates/                # Code & spec templates
│   │   ├── agent-files/          # AGENTS.md, CLAUDE.md templates
│   │   ├── specs/                # Specification templates
│   │   ├── prps/                 # PRP templates
│   │   │   ├── prp-base.md
│   │   │   ├── prp-planning.md
│   │   │   ├── prp-spec.md
│   │   │   ├── prp-story.md
│   │   │   └── prp-task.md
│   │   ├── bmad/                 # BMAD templates
│   │   │   ├── architecture.yaml
│   │   │   ├── prd.yaml
│   │   │   ├── project-brief.yaml
│   │   │   └── story.yaml
│   │   ├── checklists/           # Checklists
│   │   │   ├── architect-checklist.md
│   │   │   ├── story-dod-checklist.md
│   │   │   └── qa-gate.md
│   │   └── tasks/                # Task templates
│   │
│   ├── scripts/                  # Build & utility scripts
│   │   ├── bash/                 # Unix scripts
│   │   ├── powershell/           # Windows scripts
│   │   └── cross-platform/       # Node.js scripts
│   │
│   ├── plugins/                  # Plugin system
│   │   ├── plugin-api/           # Plugin API types
│   │   ├── plugin-dev/           # Plugin dev kit
│   │   └── vite-plugin/          # Vite plugin
│   │
│   └── config/                   # Configuration files
│       ├── ai-providers.json     # AI provider configs
│       ├── agent-providers.json  # Agent/IDE provider configs
│       ├── issue-providers.json  # Issue provider configs
│       ├── sync-providers.json   # Sync provider configs
│       └── features.json         # Feature flags
│
├── .claude/                      # Claude Code config (auto-detected)
├── .cursor/                      # Cursor config (auto-detected)
├── .github/                      # GitHub/Copilot config (auto-detected)
└── lib/                          # Bundled libraries (self-contained)
```

### AgenticAI Automation Requirements

> **Critical for fully automated project management.** These features enable autonomous operation.

#### Issue Provider Integrations (8 providers)

| Provider | Source | Capabilities |
|----------|--------|--------------|
| **Jira** | super-productivity | Full sync, worklog, transitions, sprint mapping |
| **GitHub** | super-productivity | Issues, PRs, labels, milestones |
| **GitLab** | super-productivity | Issues, MRs, labels, milestones |
| **Gitea** | super-productivity | Issues, labels, self-hosted |
| **OpenProject** | super-productivity | Work packages, time tracking |
| **Redmine** | super-productivity | Issues, time entries |
| **Trello** | super-productivity | Cards, boards, checklists |
| **CalDAV** | super-productivity | Calendar events, tasks |

#### Agentic Workflow Automation

```yaml
# BMAD Agent Team Configuration
bmad-agents:
  orchestrator: "bmad-orchestrator.md"    # Coordinates all agents
  analyst: "analyst.md"                   # Requirements analysis
  architect: "architect.md"               # System architecture
  po: "po.md"                             # Product ownership
  pm: "pm.md"                             # Project management
  dev: "dev.md"                           # Development
  qa: "qa.md"                             # Quality assurance
  sm: "sm.md"                             # Scrum master
  ux: "ux-expert.md"                      # UX/UI design

# PRP Agent Team Configuration
prp-agents:
  orchestrator: "orchestrator.md"         # Workflow coordination
  developer: "robo-developer.md"          # Code generation
  aqa: "robo-aqa.md"                      # Automated QA
  quality: "robo-quality-control.md"      # Quality gates
  analyst: "robo-system-analyst.md"       # System analysis
  devops: "robo-devops-sre.md"            # DevOps/SRE
  designer: "robo-ux-ui-designer.md"      # Design automation
```

#### Autonomous Task Management

```typescript
// Automated task operations (from claude-task-master)
interface AgenticTaskManager {
  // Task Creation & Parsing
  parsePRD(document: string): Promise<Task[]>;      // Parse PRD to tasks
  expandTask(taskId: string): Promise<Task[]>;      // AI task breakdown
  expandAllTasks(): Promise<void>;                  // Expand all pending

  // Intelligent Task Selection
  nextTask(): Promise<Task | null>;                 // AI-suggested next task
  analyzeComplexity(taskId: string): Promise<ComplexityReport>;

  // Dependency Management (Auto)
  validateDependencies(): Promise<ValidationResult>;
  fixDependencies(): Promise<FixResult>;            // Auto-fix circular deps

  // Scoping
  scopeUp(taskId: string): Promise<Task>;           // Increase scope
  scopeDown(taskId: string): Promise<Task[]>;       // Break into subtasks

  // Research & Context
  research(topic: string): Promise<ResearchResult>; // Web research
  getContext(taskId: string): Promise<Context>;     // Gather context
}
```

#### Workflow Orchestration

```yaml
# Greenfield Project Workflow (automated)
greenfield-fullstack:
  phases:
    - name: "Discovery"
      agents: [analyst, po, ux]
      tasks: [requirements, user-stories, wireframes]
      auto-transition: true

    - name: "Architecture"
      agents: [architect, dev]
      tasks: [system-design, tech-stack, api-contracts]
      requires: [Discovery.complete]

    - name: "Implementation"
      agents: [dev, qa]
      tasks: [coding, unit-tests, integration]
      parallel: true

    - name: "Deployment"
      agents: [devops, qa]
      tasks: [ci-cd, monitoring, release]
      auto-rollback: true

# Brownfield Enhancement Workflow
brownfield-fullstack:
  phases:
    - name: "Analysis"
      tasks: [codebase-analysis, impact-assessment]
    - name: "Planning"
      tasks: [migration-plan, risk-assessment]
    - name: "Execution"
      tasks: [incremental-changes, regression-tests]
```

#### Automated Triggers & Actions

```typescript
// Trigger-based automation
interface AutomationTriggers {
  // Time-based
  onSchedule(cron: string, action: Action): void;
  onDailyStart(action: Action): void;
  onDailyEnd(action: Action): void;

  // Event-based
  onTaskComplete(handler: (task: Task) => void): void;
  onBlockerDetected(handler: (blocker: Blocker) => void): void;
  onDeadlineApproaching(days: number, handler: Handler): void;

  // Context-based
  onIdleDetected(minutes: number, action: Action): void;
  onFocusModeEnd(action: Action): void;
  onPomodoroComplete(action: Action): void;

  // Integration-based
  onGitPush(action: Action): void;
  onPRCreated(action: Action): void;
  onJiraTransition(action: Action): void;
}
```

#### System Prompts Library (25+ AI Tools)

The unified prompts library includes templates from:
- **Anthropic**: Claude Code, Claude Code 2.0, Sonnet 4.5
- **Cursor**: Agent Prompt v1.0-v2.0, CLI Prompt
- **VS Code Agent**: GPT-4, GPT-5, Claude Sonnet 4, Gemini 2.5 Pro
- **Windsurf**: Wave 11 Prompt & Tools
- **Devin**: Main Prompt, DeepWiki Prompt
- **Amp**: Claude 4 Sonnet, GPT-5
- **Augment Code**: Agent prompts & tools
- **Lovable, Replit, v0, Same.dev**: Full prompt sets
- **Manus, Trae, Traycer, Emergent**: Agent prompts
- **Open Source**: Bolt, Cline, Codex CLI, Gemini CLI, RooCode

---

## 3.4) CECCA Autonomous System Framework

> **CECCA: Chief-Executive-Commander-Chief-Agent** - Biological-Inspired Stem Cell Computing Architecture
> Source: `docs/07-plans/autonomous-system.md` + `autonomous-system.mmd`

### Stem Cell Computing Framework

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                    CECCA STEM CELL ARCHITECTURE                                       │
├──────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                       │
│  ┌─────────────────────────────────────────────────────────────────────────────┐     │
│  │                    CECCA ROOT STEM CAPSULE                                   │     │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │     │
│  │  │ CC_CHOP     │  │ CC_STEM_    │  │ CC_STEM_    │  │ CC_TRUTH    │         │     │
│  │  │ (Surgery)   │  │ REPL        │  │ DIFF        │  │ (Gate)      │         │     │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘         │     │
│  └─────────────────────────────────────────────────────────────────────────────┘     │
│                                        │                                              │
│                                        ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────────────┐     │
│  │              25+ DYNAMIC INGESTION GRAPHS                                    │     │
│  │  EFG  │ DSG  │ DHG  │ DFG  │ DNG  │ DUBG │ DPG  │ DRG  │ DSeCG │ DCG        │     │
│  │  DPeG │ DEG  │ DLG  │ DWG  │ DIG  │ DCmG │ DQG  │ DInG │ DKG   │ DLeG       │     │
│  │  DCmGraph │ DColG │ DDG │ DVG │ DTHG │ DAIG                                  │     │
│  └─────────────────────────────────────────────────────────────────────────────┘     │
│                                        │                                              │
│                                        ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────────────┐     │
│  │                     STACK ORGAN SYSTEMS                                      │     │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐           │     │
│  │  │ VPP     │  │ NOA     │  │ INS     │  │ EPC     │  │ MFG     │           │     │
│  │  │ Stack   │  │ Runtime │  │ Stack   │  │ Stack   │  │ Stack   │           │     │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘           │     │
│  └─────────────────────────────────────────────────────────────────────────────┘     │
│                                                                                       │
│  PRIMARY LAYER: Strictly Offline | SECONDARY LAYER: External Adapters (toggle)       │
│                                                                                       │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

### Dynamic Ingestion Graphs (25+ Types)

| Graph | Full Name | Purpose |
|-------|-----------|---------|
| EFG | Environment Function Graph | Live host environment mapping |
| DSG | Dynamic Software Graph | Live software analysis & optimization |
| DHG | Dynamic Hardware Graph | Live hardware monitoring & tuning |
| DFG | Dynamic Firmware Graph | Live firmware analysis & enhancement |
| DNG | Dynamic Network Graph | Live network optimization |
| DUBG | Dynamic User Behavior Graph | Live user pattern analysis |
| DPG | Dynamic Process Graph | Live process monitoring |
| DRG | Dynamic Resource Graph | Live resource utilization |
| DSeCG | Dynamic Security Graph | Live security posture |
| DCG | Dynamic Configuration Graph | Live config state management |
| DPeG | Dynamic Performance Graph | Live performance metrics |
| DEG | Dynamic Error Graph | Live error pattern analysis |
| DLG | Dynamic Log Graph | Live log analysis |
| DWG | Dynamic Workflow Graph | Live workflow patterns |
| DIG | Dynamic Integration Graph | Live integration status |
| DCmG | Dynamic Compliance Graph | Live compliance monitoring |
| DQG | Dynamic Quality Graph | Live quality metrics |
| DInG | Dynamic Innovation Graph | Live innovation opportunity ID |
| DKG | Dynamic Knowledge Graph | Live knowledge synthesis |
| DLeG | Dynamic Learning Graph | Live learning patterns |
| DCmGraph | Dynamic Communication Graph | Live communication enhancement |
| DColG | Dynamic Collaboration Graph | Live collaboration effectiveness |
| DDG | Dynamic Decision Graph | Live decision patterns |
| DVG | Dynamic Value Graph | Live value creation |
| DTHG | Dynamic Threat Graph | Live threat intelligence |
| DAIG | Dynamic AI Graph | Live AI model optimization |

### Autonomy Loop (SENSE → DECIDE → UPDATE)

```typescript
// core/autonomy/autonomy-loop.ts
interface AutonomyLoop {
  sense(): Promise<EnvironmentState>;   // Collect from 25+ dynamic graphs
  decide(): DecisionResult;              // Neural runtime decision
  update(): Promise<void>;               // Self-modification via CC_CHOP
}

// Autonomy loop cycle
async function executeAutonomyLoop() {
  while (true) {
    const state = await sense();         // EFG, DSG, DHG, etc.
    const decision = decide(state);      // Local neural inference

    if (decision.action === 'replicate') {
      await CC_STEM_REPL.clone();        // Self-replication
    } else if (decision.action === 'differentiate') {
      await CC_STEM_DIFF.specialize();   // Cell specialization
    } else if (decision.action === 'quiesce') {
      await enterAMPKMode();             // Resource scarcity mode
    }

    await update();                      // Apply changes
    await recordDecision(decision);      // Evidence trail
  }
}
```

### Universal Task Execution Policy

> Source: `docs/05-policy/universal_task_execution_policy.md`

```yaml
# Evidence-Based Execution Protocol
policy:
  scope: "all tasks, all outputs, all agents, all tools"

  truth_sources:
    1: "user-provided files and chat"
    2: "computations with shown work"
    3: "cited external sources"
    4: "model prior"

  rules:
    - hard_stop: "If any required check fails, FAIL + reasons + remedy"
    - deep_analytics: "In-depth content analytics before completion"
    - gap_hunt: "Search for gaps or errors, report findings"
    - triple_verify: "Verify every result 3 times (Pass A/B/C)"

  update_semantics: "Heal, Do Not Harm"
    - preserve_correct_prior: true
    - granular_preservation: true
    - controlled_change: "removal requires reason + replacement"

  truth_gate:
    - artifact_presence: "All referenced files exist"
    - smoke_test: "Deterministic test exits 0"
    - spec_match: "Requirements → artifacts → tests"
    - limits: "State constraints, OS/arch, failure modes"
    - hashes: "SHA-256 for key artifacts"
    - gap_scan: "Checklist confirms coverage"
```

### Documentation & Graphs Index

| Doc Type | Location | Key Content |
|----------|----------|-------------|
| **Guides** | `docs/00-guides/` | Goals-Policy-Rules Guide |
| **Templates** | `docs/01-templates/` | Goal, Policy, Rules templates |
| **Examples** | `docs/02-examples/` | Annotated examples |
| **Goals** | `docs/04-goals/` | Environment goals |
| **Policy** | `docs/05-policy/` | Universal Task Execution Policy |
| **Rules** | `docs/06-rules/` | Environment rules |
| **Plans** | `docs/07-plans/` | CECCA Autonomous System (mermaid) |
| **Sync** | `docs/sync/` | Vector Clocks documentation |
| **AI** | `docs/ai/` | WebDAV implementation plans |
| **Ledger** | `docs/goal-ledger.md` | Central goal registry |

### Mermaid Graphs Available

| File | Purpose | Size |
|------|---------|------|
| `autonomous-system.mmd` | Complete CECCA stem cell architecture | 108KB |

---

## 3.5) AgenticAI Execution Pipeline

> **Complete autonomous workflow from UI conversation to execution.**

### Target Flow & UX (End-to-End)

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                    AGENTICAI EXECUTION PIPELINE (v1.4)                                │
├──────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                       │
│  ┌─────────────────┐                                                                  │
│  │ UI/Chat Request │ ◄── Natural language input from user                            │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │ Goal Decompos.  │ ◄── Break request into atomic goals                             │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │ Prompt Engineer │ ◄── Generate optimized prompts (DSPy, PromptFusion)             │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐                   │
│  │    Goals        │───►│    Policy       │───►│     Rules       │                   │
│  └────────┬────────┘    └────────┬────────┘    └────────┬────────┘                   │
│           │                      │                      │                             │
│           └──────────────────────┴──────────────────────┘                             │
│                                  │                                                    │
│                                  ▼                                                    │
│  ┌─────────────────┐                                                                  │
│  │     Memory      │ ◄── Never forget, instant recall (semantic search)              │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │   Checklist     │ ◄── Validation gates (QA Gate from BMAD)                        │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │      Plan       │ ◄── Story creation (BMAD SM Agent)                              │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │      Spec       │ ◄── Executable specification (spec-kit)                         │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐                                                                  │
│  │      Task       │ ◄── Task breakdown (claude-task-master)                         │
│  └────────┬────────┘                                                                  │
│           │                                                                           │
│           ▼                                                                           │
│  ┌─────────────────┐    ┌─────────────────┐                                          │
│  │   XML Table     │───►│      JSON       │ ◄── Structured output                    │
│  └────────┬────────┘    └────────┬────────┘                                          │
│           │                      │                                                    │
│           └──────────┬───────────┘                                                    │
│                      │                                                                │
│                      ▼                                                                │
│  ┌─────────────────────────────────────────────────────────────────┐                 │
│  │                    ORCHESTRATION LAYER                          │                 │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │                 │
│  │  │ Loop Reason  │  │ Multi-Agent  │  │ Coordination │          │                 │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │                 │
│  └────────────────────────────┬────────────────────────────────────┘                 │
│                               │                                                       │
│                               ▼                                                       │
│  ┌─────────────────────────────────────────────────────────────────┐                 │
│  │                      EXECUTION LAYER                            │                 │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │                 │
│  │  │  Staging     │─►│   Testing    │─►│   Deploy     │          │                 │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │                 │
│  └────────────────────────────┬────────────────────────────────────┘                 │
│                               │                                                       │
│                               ▼                                                       │
│  ┌─────────────────────────────────────────────────────────────────┐                 │
│  │                     FEEDBACK LAYER                              │                 │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │                 │
│  │  │ Self-Reflect │─►│ Self-Improve │─►│ Self-Heal    │          │                 │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │                 │
│  └─────────────────────────────────────────────────────────────────┘                 │
│                                                                                       │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

### Agentic Capabilities Matrix

| Capability | Source | Implementation |
|------------|--------|----------------|
| **Orchestration** | BMAD-METHOD, PRP | `core/orchestration/orchestrator.service.ts` |
| **Loop Reasoning** | PRP LOOP MODE | `core/reasoning/loop-reasoner.ts` |
| **Commands** | spec-kit, claude-task-master | `core/commands/` |
| **Automation** | Triggers, Actions | `core/automation/trigger-engine.ts` |
| **Verification** | QA Gate, Tests | `core/verification/qa-gate.ts` |
| **Unsupervised Learning** | DSPy Optimizers | `ai/dspy/optimizers/` |
| **Reasoning & Problem Solving** | DSPy ChainOfThought | `ai/dspy/modules/cot.ts` |
| **Multi-agent Collaboration** | BMAD Teams | `ai/agents/teams/` |
| **Agent Coordination** | PRP Signals | `core/signals/signal-system.ts` |
| **Self-reflection** | BMAD QA, PRP | `ai/agents/self-reflect.ts` |
| **Error Recovery** | Self-healing | `core/recovery/error-recovery.ts` |
| **Dynamic Tooling** | MCP Server | `apps/mcp/dynamic-tools.ts` |
| **Autonomous Execution** | Taskosaur | `core/execution/autonomous-executor.ts` |
| **Goal Chaining** | Long-term autonomy | `core/goals/goal-chain.ts` |
| **Feedback Loops** | Evaluators | `core/feedback/evaluator.ts` |
| **Self-improving** | DSPy, PRD Training | `ai/training/self-improve.ts` |
| **Self-healing** | Auto-recovery | `core/healing/self-heal.ts` |
| **Staging** | Pre-production | `core/deploy/staging.ts` |
| **Testing** | E2E, Unit, my-todo-app | `e2e/`, `tests/` |

### Autonomous Orchestration Service

```typescript
// core/orchestration/orchestrator.service.ts
interface OrchestratorConfig {
  loopMode: boolean;                    // PRP continuous iteration
  signalSystem: boolean;                // Signal-based priority (10→1)
  autonomousDecisions: boolean;         // No human questions
  multiAgentEnabled: boolean;           // BMAD team collaboration
}

class AgenticOrchestrator {
  private signalPriorities = {
    ATTENTION: 10,   // New work or critical input
    BLOCKED: 9,      // Can't proceed
    OVERWHELMED: 8,  // Too much complexity
    UNCERTAIN: 7,    // Need clarification
    TIRED: 6,        // Agent fatigue
    CURIOUS: 5,      // Exploring options
    FOCUSED: 4,      // Deep work mode
    CONFIDENT: 3,    // Ready for review
    ENCANTADO: 2,    // Exceeding expectations
    COMPLETED: 1,    // DoD met
  };

  /**
   * Main LOOP MODE execution (from PRP methodology)
   * AI Orchestrator makes decisions AUTONOMOUSLY
   */
  async executeLoopMode(): Promise<void> {
    while (true) {
      // 1. Read ALL PRPs - Load context across entire project
      const prps = await this.loadAllPRPs();

      // 2. Analyze ALL Signals - Identify highest priority
      const highestPriority = this.analyzeSignals(prps);

      // 3. React to Strongest Signal
      const targetPRP = this.selectHighestPriorityPRP(prps);

      // 4. Execute Work - Implement changes autonomously
      const result = await this.executeWork(targetPRP);

      // 5. Update Progress - Leave detailed comment
      await this.updateProgressLog(targetPRP, result);

      // 6. Leave Signal - Express current state
      await this.emitSignal(targetPRP, this.determineSignal(result));

      // 7. Check termination conditions
      if (this.isDoDAchieved(targetPRP)) break;
      if (this.isCheckpointReached(result)) break;
    }
  }

  /**
   * Multi-agent collaboration (BMAD methodology)
   */
  async executeWithTeam(task: Task, team: AgentTeam): Promise<Result> {
    // Phase 1: Planning (Analyst, PM, Architect)
    const requirements = await this.agents.analyst.analyze(task);
    const stories = await this.agents.pm.createStories(requirements);
    const architecture = await this.agents.architect.design(stories);

    // Phase 2: Context Engineering (SM)
    const contextualStories = await this.agents.sm.enrichWithContext(stories, architecture);

    // Phase 3: Execution (Dev, QA)
    for (const story of contextualStories) {
      const implementation = await this.agents.dev.implement(story);
      const testResult = await this.agents.qa.validate(implementation);

      // Self-reflection and error recovery
      if (!testResult.passed) {
        const fix = await this.selfReflect(implementation, testResult);
        await this.agents.dev.applyFix(fix);
      }
    }

    return this.compileResults();
  }
}
```

### Goal Decomposition Engine

```typescript
// core/goals/goal-decomposer.ts
interface DecomposedGoal {
  id: string;
  original: string;
  subGoals: SubGoal[];
  policy: PolicyReference[];
  rules: RuleReference[];
  memory: MemoryContext;
  checklist: ChecklistItem[];
}

class GoalDecomposer {
  /**
   * Decompose natural language request into structured goals
   */
  async decompose(request: string): Promise<DecomposedGoal> {
    // 1. Parse intent using NLP
    const intent = await this.parseIntent(request);

    // 2. Generate sub-goals
    const subGoals = await this.generateSubGoals(intent);

    // 3. Link to existing policy
    const policyRefs = await this.linkToPolicy(subGoals);

    // 4. Apply relevant rules
    const ruleRefs = await this.applyRules(subGoals);

    // 5. Retrieve memory context
    const memory = await this.memoryService.recall(request);

    // 6. Generate validation checklist
    const checklist = await this.generateChecklist(subGoals);

    return {
      id: nanoid(),
      original: request,
      subGoals,
      policy: policyRefs,
      rules: ruleRefs,
      memory,
      checklist,
    };
  }
}
```

### Self-Improving Agent System

```typescript
// ai/training/self-improve.ts
class SelfImprovingAgent {
  /**
   * Learn from PRD training data (Python scripts)
   * Uses DSPy for prompt optimization
   */
  async trainFromPRD(prdPath: string): Promise<void> {
    // Load PRD training data
    const trainingData = await this.loadPRDTrainingData(prdPath);

    // Create DSPy training set
    const trainset = this.createDSPyTrainset(trainingData);

    // Optimize prompts using DSPy
    const optimizer = new dspy.MIPROv2({
      metric: this.evaluateOutput,
      auto: 'light',
    });

    this.optimizedModule = await optimizer.compile(
      this.baseModule,
      { trainset }
    );
  }

  /**
   * Self-reflection and course correction
   */
  async selfReflect(output: any, feedback: Feedback): Promise<Correction> {
    // Analyze what went wrong
    const analysis = await this.analyzeFailure(output, feedback);

    // Generate correction strategy
    const correction = await this.generateCorrection(analysis);

    // Apply and verify
    const verified = await this.applyAndVerify(correction);

    // Update memory for future recall
    await this.memoryService.remember({
      type: 'lesson_learned',
      content: { analysis, correction, verified },
      metadata: { importance: 'high' },
    });

    return correction;
  }
}
```

### Self-Healing System

```typescript
// core/healing/self-heal.ts
class SelfHealingSystem {
  /**
   * Automatic error recovery and system healing
   */
  async heal(error: SystemError): Promise<HealingResult> {
    // 1. Diagnose the issue
    const diagnosis = await this.diagnose(error);

    // 2. Check if auto-recoverable
    if (diagnosis.autoRecoverable) {
      return await this.autoRecover(diagnosis);
    }

    // 3. Try staged recovery
    const recoverySteps = this.generateRecoverySteps(diagnosis);
    for (const step of recoverySteps) {
      const result = await this.executeRecoveryStep(step);
      if (result.success) {
        await this.verifyHealing();
        return { success: true, step };
      }
    }

    // 4. Escalate with full context
    return await this.escalateWithContext(diagnosis, recoverySteps);
  }

  /**
   * Continuous health monitoring
   */
  async monitorHealth(): Promise<void> {
    const healthChecks = [
      this.checkMemoryIntegrity(),
      this.checkSyncStatus(),
      this.checkAgentStatus(),
      this.checkDependencies(),
    ];

    const results = await Promise.all(healthChecks);
    for (const result of results) {
      if (!result.healthy) {
        await this.heal(result.error);
      }
    }
  }
}
```

### Single-File HTML Deployment (my-todo-app)

```typescript
// core/deploy/single-file-deploy.ts
/**
 * Generate single-file HTML deployment (like my-todo-app-main)
 * - Zero server dependencies
 * - Offline-first with localStorage
 * - GitHub Pages ready
 */
class SingleFileDeployer {
  async generateDeployment(config: DeployConfig): Promise<string> {
    // Bundle all CSS, JS into single HTML
    const html = await this.bundleToSingleFile({
      entry: config.entry,
      includeStyles: true,
      includeScripts: true,
      minify: true,
    });

    // Add AI features (categorization, priority detection)
    const withAI = this.injectAIFeatures(html);

    // Add localStorage persistence
    const withStorage = this.injectLocalStorage(withAI);

    return withStorage;
  }
}
```

### Feedback Loop & Evaluators

```typescript
// core/feedback/evaluator.ts
interface EvaluationResult {
  score: number;
  passed: boolean;
  feedback: string[];
  improvements: string[];
}

class FeedbackEvaluator {
  /**
   * Evaluate output quality with multiple metrics
   */
  async evaluate(output: any, criteria: EvaluationCriteria): Promise<EvaluationResult> {
    const scores = await Promise.all([
      this.evaluateCompleteness(output, criteria),
      this.evaluateCorrectness(output, criteria),
      this.evaluateEfficiency(output, criteria),
      this.evaluateStyle(output, criteria),
    ]);

    const avgScore = scores.reduce((a, b) => a + b, 0) / scores.length;

    return {
      score: avgScore,
      passed: avgScore >= criteria.threshold,
      feedback: this.generateFeedback(scores),
      improvements: this.suggestImprovements(scores),
    };
  }

  /**
   * Continuous improvement loop
   */
  async continuousImprove(agent: Agent): Promise<void> {
    const recentOutputs = await this.getRecentOutputs(agent);
    const evaluations = await Promise.all(
      recentOutputs.map(o => this.evaluate(o, this.defaultCriteria))
    );

    // Identify patterns for improvement
    const patterns = this.identifyImprovementPatterns(evaluations);

    // Apply improvements
    for (const pattern of patterns) {
      await agent.applyImprovement(pattern);
    }
  }
}
```

### Naming Convention

| Old Reference | New Reference |
|---------------|---------------|
| `super-productivity` | `project-mgmt` |
| `superProductivity` | `projectMgmt` |
| `sp-*` | `pm-*` |
| `.super-productivity` | `.project-mgmt` |
| `@super-productivity/*` | `@project-mgmt/*` |
| `electron/super-productivity` | `electron/project-mgmt` |

---

## 4) Unified Features (All 95 Required)

> **Expanded from 75 to 95 features** after comprehensive 10-loop scan for fully automated AgenticAI.

### A) Core Task & Project Management (Features 01-20)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 01 | Goals Management | spec-kit, docs | `core/goals/` |
| 02 | Policy Engine | ruler, docs | `core/policy/` |
| 03 | Rules Engine | ruler, agent-rules, agentic-cursorrules | `core/rules/` |
| 04 | Current Todo | super-productivity, tududi, Taskosaur | `core/tasks/` |
| 05 | Backlog Management | Backlog.md, super-productivity | `core/backlog/` |
| 06 | Task Repeat/Recurrence | super-productivity, tududi | `core/tasks/repeat/` |
| 07 | Subtask Management | claude-task-master, super-productivity | `core/tasks/subtasks/` |
| 08 | Task Dependencies | claude-task-master | `core/tasks/dependencies/` |
| 09 | Task Complexity Analysis | claude-task-master | `core/tasks/analysis/` |
| 10 | Next Task Suggestion | claude-task-master, tududi | `core/tasks/ai-suggest/` |
| 11 | Task Scoping (Up/Down) | claude-task-master | `core/tasks/scoping/` |
| 12 | Task Expansion | claude-task-master | `core/tasks/expansion/` |
| 13 | Tag System | claude-task-master, super-productivity, tududi | `core/tags/` |
| 14 | Project Management | super-productivity, tududi | `core/projects/` |
| 15 | Area Management | tududi | `core/areas/` |
| 16 | Inbox System | tududi | `core/inbox/` |
| 17 | Notes System | super-productivity, tududi | `core/notes/` |
| 18 | Quick History | super-productivity | `core/history/` |
| 19 | Completed Tasks Archive | super-productivity | `core/archive/` |
| 20 | Custom Views | tududi, Backlog.md | `core/views/` |

### B) Time Tracking & Productivity (Features 21-35)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 21 | Time Tracking | super-productivity | `core/time-tracking/` |
| 22 | Pomodoro Timer | super-productivity | `core/pomodoro/` |
| 23 | Focus Mode | super-productivity | `core/focus-mode/` |
| 24 | Take a Break Reminders | super-productivity | `core/break-reminders/` |
| 25 | Idle Detection | super-productivity | `core/idle/` |
| 26 | Tracking Reminders | super-productivity | `core/tracking-reminders/` |
| 27 | Worklog & Export | super-productivity | `core/worklog/` |
| 28 | Work Context | super-productivity | `core/work-context/` |
| 29 | Simple Counters | super-productivity | `core/counters/` |
| 30 | Metric System (Obstruction/Improvement) | super-productivity | `core/metrics/` |
| 31 | Productivity Dashboard | tududi | `apps/web/productivity/` |
| 32 | Weekly Completion Chart | tududi | `apps/web/charts/` |
| 33 | Project Insights Panel | tududi | `apps/web/insights/` |
| 34 | Task Timeline | tududi | `apps/web/timeline/` |
| 35 | Today Plan | tududi, super-productivity | `core/planning/` |

### C) UI & Views (Features 36-50)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 36 | Kanban Board | super-productivity, Taskosaur, Backlog.md | `apps/web/board/` |
| 37 | Calendar Views (Day/Week/Month) | tududi, super-productivity | `apps/web/calendar/` |
| 38 | Schedule System | super-productivity | `core/schedule/` |
| 39 | Planner | super-productivity | `core/planner/` |
| 40 | Planning Mode | super-productivity | `core/planning-mode/` |
| 41 | Universal Search | tududi, Backlog.md | `core/search/` |
| 42 | Task View Customizer | super-productivity | `core/view-customizer/` |
| 43 | Right Panel | super-productivity | `apps/web/panels/` |
| 44 | Bottom Panel | super-productivity | `apps/web/panels/` |
| 45 | Issue Panel | super-productivity | `apps/web/issue-panel/` |
| 46 | TUI (Terminal UI) | Backlog.md | `apps/cli/tui/` |
| 47 | View Switcher | Backlog.md | `apps/cli/view-switcher/` |
| 48 | Menu Tree | super-productivity | `core/menu/` |
| 49 | Shepherd (Onboarding Tours) | super-productivity | `core/onboarding/` |
| 50 | Theme System (Custom/Global) | super-productivity | `core/theme/` |

### D) Sync & Persistence (Features 51-60)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 51 | Vector Clock Sync | super-productivity (pfapi) | `core/sync/vector-clock/` |
| 52 | Dropbox Sync | super-productivity (pfapi) | `core/sync/providers/dropbox/` |
| 53 | WebDAV Sync | super-productivity (pfapi) | `core/sync/providers/webdav/` |
| 54 | Local File Sync | super-productivity (pfapi) | `core/sync/providers/local/` |
| 55 | SAF File Adapter (Android) | super-productivity (pfapi) | `core/sync/providers/saf/` |
| 56 | GitHub/GitLab Sync | super-productivity | `core/sync/providers/git/` |
| 57 | Jira Integration | super-productivity | `integrations/jira/` |
| 58 | Encryption Service | super-productivity (pfapi) | `core/encryption/` |
| 59 | Compression Handler | super-productivity (pfapi) | `core/compression/` |
| 60 | Data Repair & Validation | super-productivity (pfapi) | `core/repair/` |

### E) AI & Automation (Features 61-75)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 61 | Provider Auto-Detection | spec-kit | `core/providers/detection/` |
| 62 | AI/Shared Integration | ai/shared | `core/ai-shared/` |
| 63 | Memory System (Never Forget) | ai/shared, New | `core/memory/` |
| 64 | PRD Parsing | claude-task-master | `ai/prd-parser/` |
| 65 | Research Function | claude-task-master | `ai/research/` |
| 66 | DSPy Pipelines | dspy | `ai/dspy/` |
| 67 | Prompt Fusion Engine | promptfusion | `ai/prompts/fusion/` |
| 68 | Context Manager | claude-task-master | `ai/context/` |
| 69 | Agent System | prp, BMAD-METHOD | `ai/agents/` |
| 70 | Expansion Packs | BMAD-METHOD-v5 | `ai/expansion-packs/` |
| 71 | Signal System | prp | `core/signals/` |
| 72 | Nudge System | prp | `core/nudge/` |
| 73 | Plugin System | super-productivity | `core/plugins/` |
| 74 | Chrome Extension Interface | super-productivity | `integrations/chrome/` |
| 75 | Shell Completions | Backlog.md | `cli/completions/`

### F) Issue Provider Integrations (Features 76-83)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 76 | Jira Provider (Full) | super-productivity | `integrations/issue-providers/jira/` |
| 77 | GitHub Provider | super-productivity | `integrations/issue-providers/github/` |
| 78 | GitLab Provider | super-productivity | `integrations/issue-providers/gitlab/` |
| 79 | Gitea Provider | super-productivity | `integrations/issue-providers/gitea/` |
| 80 | OpenProject Provider | super-productivity | `integrations/issue-providers/open-project/` |
| 81 | Redmine Provider | super-productivity | `integrations/issue-providers/redmine/` |
| 82 | Trello Provider | super-productivity | `integrations/issue-providers/trello/` |
| 83 | CalDAV Provider | super-productivity | `integrations/issue-providers/caldav/` |

### G) AgenticAI Workflows (Features 84-95)

| # | Feature | Source Repo(s) | Implementation |
|---|---------|----------------|----------------|
| 84 | BMAD Agent Orchestrator | BMAD-METHOD-v5 | `ai/agents/bmad/orchestrator/` |
| 85 | BMAD Agent Teams | BMAD-METHOD-v5 | `ai/agents/bmad/teams/` |
| 86 | BMAD Workflows (6 types) | BMAD-METHOD-v5 | `ai/agents/workflows/` |
| 87 | PRP Agents (7 roles) | prp | `ai/agents/prp/` |
| 88 | PRP Commands (20+ cmds) | PRPs-agentic | `ai/agents/commands/` |
| 89 | Automated Task Expansion | claude-task-master | `core/tasks/expansion/` |
| 90 | Automated Dependency Fix | claude-task-master | `core/tasks/dependencies/` |
| 91 | System Prompts Library | system-prompts | `ai/prompts/system-prompts/` |
| 92 | Workflow Triggers | prp, BMAD | `core/automation/triggers/` |
| 93 | Git Conflict Resolution | PRPs-agentic | `ai/agents/commands/git-ops/` |
| 94 | Automated PR Creation | PRPs-agentic | `ai/agents/commands/development/` |
| 95 | Supabase Integration | claude-task-master | `integrations/supabase/` |

### Real-Time Metrics

| Metric | Source | Target Component |
|--------|--------|------------------|
| Service Health Status | P2P | `core/metrics/health.ts` |
| CPU/Memory/Disk/Network | System | `core/metrics/system.ts` |
| Request Rate & Error Rate | MCP server | `core/metrics/requests.ts` |
| Cache Hit Rate | IndexedDB | `core/metrics/cache.ts` |
| Active Connections | WebSocket | `core/metrics/connections.ts` |
| Provider Availability | Auto-detection | `core/providers/` |
| AI/Shared Resources | ai/shared | `core/ai-shared/` |
| Memory Index Size | Memory system | `core/memory/` |
| Recall Latency | Memory system | `core/memory/` |
| Time Tracked Today | Time tracking | `core/time-tracking/` |
| Pomodoro Sessions | Pomodoro | `core/pomodoro/` |
| Tasks Completed | Tasks | `core/tasks/` |
| Focus Time | Focus mode | `core/focus-mode/` |

### Detailed Capability Matrix (Discovered Features)

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                    COMPREHENSIVE CAPABILITY MATRIX (v1.3)                            │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                      │
│  DESKTOP/ELECTRON FEATURES (electron/)                                               │
│  ├─ Tray Icon with Animation Indicator                                               │
│  ├─ Full Screen Blocker (break enforcement)                                          │
│  ├─ Idle Time Handler (auto-pause tracking)                                          │
│  ├─ Lockscreen Mode                                                                  │
│  ├─ Overlay Indicator (floating timer)                                               │
│  ├─ Global Shortcuts                                                                 │
│  ├─ Protocol Handler (deep linking)                                                  │
│  ├─ Plugin Node Executor                                                             │
│  ├─ Jira Integration (native)                                                        │
│  └─ Simple Store (persistent settings)                                               │
│                                                                                      │
│  SYNC API (pfapi/)                                                                   │
│  ├─ Vector Clock Conflict Resolution                                                 │
│  ├─ Dropbox Provider                                                                 │
│  ├─ WebDAV Provider (with XML parser)                                                │
│  ├─ Local File Sync (Electron + Android SAF)                                         │
│  ├─ Encryption (AES-256-GCM)                                                         │
│  ├─ Compression Handler (gzip)                                                       │
│  ├─ Cross-Model Migrations (v2→v5)                                                   │
│  ├─ Data Repair Service                                                              │
│  ├─ Typia Validation                                                                 │
│  └─ Load Balancer                                                                    │
│                                                                                      │
│  CLAUDE-TASK-MASTER MCP TOOLS                                                        │
│  ├─ add_task, remove_task, update_task                                               │
│  ├─ add_subtask, remove_subtask, clear_subtasks                                      │
│  ├─ add_dependency, remove_dependency, validate_dependencies, fix_dependencies       │
│  ├─ add_tag, copy_tag, delete_tag, rename_tag, list_tags, use_tag                    │
│  ├─ expand_task, expand_all_tasks                                                    │
│  ├─ scope_up, scope_down                                                             │
│  ├─ next_task                                                                        │
│  ├─ analyze_task_complexity, complexity_report                                       │
│  ├─ parse_prd                                                                        │
│  ├─ research                                                                         │
│  ├─ rules                                                                            │
│  ├─ initialize_project                                                               │
│  └─ set_task_status, models                                                          │
│                                                                                      │
│  SPEC-KIT CLI COMMANDS                                                               │
│  ├─ specify init <project>                                                           │
│  ├─ specify check                                                                    │
│  ├─ /speckit.constitution                                                            │
│  ├─ /speckit.specify                                                                 │
│  ├─ /speckit.plan                                                                    │
│  ├─ /speckit.implement                                                               │
│  ├─ /speckit.tasks                                                                   │
│  ├─ /speckit.checklist                                                               │
│  ├─ /speckit.clarify                                                                 │
│  └─ /speckit.analyze                                                                 │
│                                                                                      │
│  BACKLOG.MD MCP TOOLS & TUI                                                          │
│  ├─ Board View (Kanban)                                                              │
│  ├─ Task Viewer with Search                                                          │
│  ├─ View Switcher                                                                    │
│  ├─ Config Watcher                                                                   │
│  ├─ Task Watcher                                                                     │
│  ├─ Agent Instructions                                                               │
│  ├─ Guidelines System                                                                │
│  ├─ Health Indicator                                                                 │
│  ├─ Mermaid Markdown Rendering                                                       │
│  ├─ Statistics Dashboard                                                             │
│  ├─ Shell Completions (bash, fish, zsh)                                              │
│  └─ Web UI (React)                                                                   │
│                                                                                      │
│  BMAD-METHOD EXPANSION PACKS                                                         │
│  ├─ bmad-2d-phaser-game-dev                                                          │
│  ├─ bmad-2d-unity-game-dev                                                           │
│  ├─ bmad-creative-writing                                                            │
│  ├─ bmad-infrastructure-devops                                                       │
│  ├─ Agent Teams (yaml configs)                                                       │
│  ├─ Workflow Templates                                                               │
│  └─ BMAD Installer & Flattener                                                       │
│                                                                                      │
│  DSPY MODULES                                                                        │
│  ├─ dspy.ChainOfThought                                                              │
│  ├─ dspy.ReAct                                                                       │
│  ├─ dspy.Predict                                                                     │
│  ├─ dspy.Assert                                                                      │
│  ├─ dspy.Signature                                                                   │
│  ├─ Optimizers (BootstrapFewShot, COPRO, etc.)                                       │
│  ├─ Adapters (JSON, XML, Chat, Tool)                                                 │
│  └─ MCP Tool Support                                                                 │
│                                                                                      │
│  PROMPTFUSION                                                                        │
│  ├─ Prompt Fusion Engine                                                             │
│  ├─ Weight Determination Patterns                                                    │
│  ├─ Message Modifier Fusion                                                          │
│  ├─ Anthropic/OpenAI/LangChain Examples                                              │
│  └─ Weight Patterns Documentation                                                    │
│                                                                                      │
│  PRP (PRODUCT REQUIREMENT PLANNING)                                                  │
│  ├─ Signal System                                                                    │
│  ├─ Nudge System                                                                     │
│  ├─ CLI Wizard                                                                       │
│  ├─ Agent System (Orchestrator, Robo-AQA, Robo-Developer)                            │
│  ├─ WikiJS Integration                                                               │
│  └─ PRPs (Product Requirement Plans)                                                 │
│                                                                                      │
│  TUDUDI UNIQUE FEATURES                                                              │
│  ├─ Inbox System (quick capture)                                                     │
│  ├─ Auto-Suggest Next Action                                                         │
│  ├─ Universal Search with Filters                                                    │
│  ├─ Saved Custom Views                                                               │
│  ├─ Project Share Modal                                                              │
│  ├─ Profile Settings (AI, API Keys, Security, Telegram)                              │
│  ├─ 25+ Language Locales                                                             │
│  └─ Weekly Completion Chart                                                          │
│                                                                                      │
│  AGENT-RULES                                                                         │
│  ├─ Project Rules (commit, bug-fix, pr-review, etc.)                                 │
│  ├─ Global Rules (mcp-sync, github-issue-creation)                                   │
│  ├─ Swift/SwiftUI/UIKit/AppKit Documentation                                         │
│  └─ MCP Best Practices                                                               │
│                                                                                      │
│  PLUGIN SYSTEM                                                                       │
│  ├─ Plugin API (TypeScript types)                                                    │
│  ├─ Plugin Dev Kit                                                                   │
│  ├─ Vite Plugin                                                                      │
│  └─ Plugin Node Executor (Electron)                                                  │
│                                                                                      │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 5) Cross-Platform Adaptive Architecture

### Platform Detection & Adaptation

```typescript
// core/platform/platform.service.ts
export enum Platform {
  WEB = 'web',
  ELECTRON = 'electron',
  ANDROID = 'android',
  IOS = 'ios',
  CLI = 'cli',
  MCP = 'mcp',
  WSL = 'wsl',
  DOCKER = 'docker'
}

export interface PlatformCapabilities {
  hasFileSystem: boolean;
  hasNotifications: boolean;
  hasBackgroundTasks: boolean;
  hasTray: boolean;
  hasShortcuts: boolean;
  hasIdleDetection: boolean;
  hasNativeMenus: boolean;
  hasGPU: boolean;
  hasP2P: boolean;
  hasProviderDetection: boolean;
}

export const PLATFORM_CAPABILITIES: Record<Platform, PlatformCapabilities> = {
  [Platform.WEB]: {
    hasFileSystem: false,
    hasNotifications: true,
    hasBackgroundTasks: true,
    hasTray: false,
    hasShortcuts: true,
    hasIdleDetection: false,
    hasNativeMenus: false,
    hasGPU: true,
    hasP2P: false,
    hasProviderDetection: false  // Limited in browser
  },
  [Platform.ELECTRON]: {
    hasFileSystem: true,
    hasNotifications: true,
    hasBackgroundTasks: true,
    hasTray: true,
    hasShortcuts: true,
    hasIdleDetection: true,
    hasNativeMenus: true,
    hasGPU: true,
    hasP2P: true,
    hasProviderDetection: true
  },
  [Platform.CLI]: {
    hasFileSystem: true,
    hasNotifications: false,
    hasBackgroundTasks: true,
    hasTray: false,
    hasShortcuts: false,
    hasIdleDetection: false,
    hasNativeMenus: false,
    hasGPU: false,
    hasP2P: true,
    hasProviderDetection: true
  },
  // ... other platforms
};
```

### Kernel Independence Layer

```typescript
// core/kernel/kernel-abstraction.ts
export interface KernelAbstraction {
  // File operations (works on any OS)
  readFile(path: string): Promise<Uint8Array>;
  writeFile(path: string, data: Uint8Array): Promise<void>;
  exists(path: string): Promise<boolean>;
  mkdir(path: string): Promise<void>;

  // Process operations
  spawn(command: string, args: string[]): Promise<ProcessHandle>;
  getEnv(key: string): string | undefined;
  which(tool: string): Promise<string | null>;  // For provider detection

  // Network operations
  fetch(url: string, options?: RequestInit): Promise<Response>;
  listen(port: number): Promise<Server>;

  // Platform info
  getPlatform(): Platform;
  getArch(): 'x64' | 'arm64' | 'arm' | 'ia32';
  getOS(): 'windows' | 'linux' | 'darwin' | 'android' | 'ios';
  getHomeDir(): string;
}
```

---

## 6) Unified Environment Configuration

### Environment File: `${PROJECT_ROOT}/.project-mgmt-env/.env`

```bash
# Project Root Configuration (auto-detected)
PROJECT_ROOT=${PWD}
PROJECT_MGMT_ENV=${PROJECT_ROOT}/.project-mgmt-env

# NOA Root Path (parent directory containing ai/ and project-mgmt/)
NOA_ROOT=${PROJECT_ROOT}/..

# AI Shared Resource Paths (connects to ${NOA_ROOT}/ai/shared)
AI_SHARED_PATH=${NOA_ROOT}/ai/shared
AI_AGENTS_PATH=${AI_SHARED_PATH}/agents
AI_WORKFLOWS_PATH=${AI_SHARED_PATH}/workflows
AI_PROMPTS_PATH=${AI_SHARED_PATH}/prompts
AI_SKILLS_PATH=${AI_SHARED_PATH}/skills
AI_TOOLS_PATH=${AI_SHARED_PATH}/tools
AI_MODELS_PATH=${AI_SHARED_PATH}/models
AI_COMMANDS_PATH=${AI_SHARED_PATH}/commands

# Local AI Provider Paths
AI_LOCAL_PROVIDERS=${NOA_ROOT}/ai/providers/local
LLAMA_CPP_BIN=${AI_LOCAL_PROVIDERS}/llama.cpp/bin
OLLAMA_CONFIG=${AI_LOCAL_PROVIDERS}/ollama

# Platform Detection
PLATFORM=auto  # auto|web|electron|mobile|cli|mcp

# Provider Auto-Detection (respects priority: local > hybrid > cloud)
PROVIDER_AUTO_DETECT=true
PROVIDER_PRIORITY=local-first  # local-first|cloud-first|balanced
PROVIDER_PRIMARY=auto  # auto|claude|cursor-agent|copilot|...
PROVIDER_FALLBACK=ollama

# AI Provider Configuration
AI_LOCAL_ONLY=false

# Claude Configuration
ANTHROPIC_API_KEY=your-key-here
CLAUDE_MODEL=claude-sonnet-4-20250514

# OpenAI Configuration (optional)
OPENAI_API_KEY=your-key-here
OPENAI_MODEL=gpt-4o

# Ollama Configuration (local)
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=llama3.2

# llama.cpp Configuration (local)
LLAMA_CPP_PATH=${PROJECT_ROOT}/opt/llama.cpp
LLAMA_MODEL_PATH=${PROJECT_ROOT}/ai/models

# Sync Provider Configuration
SYNC_PROVIDER=local  # local|dropbox|webdav|github
WEBDAV_URL=
WEBDAV_USERNAME=
WEBDAV_PASSWORD=
DROPBOX_ACCESS_TOKEN=
GITHUB_TOKEN=

# Database Configuration
DB_TYPE=indexeddb  # indexeddb|sqlite|postgresql
DB_PATH=${PROJECT_ROOT}/project-mgmt/data

# Server Configuration
MCP_PORT=3000
API_PORT=3001
WEB_PORT=4200

# Feature Flags
FEATURE_AI_TASKS=true
FEATURE_P2P_SYNC=true
FEATURE_OFFLINE_MODE=true
FEATURE_DARK_MODE=true
FEATURE_PROVIDER_DETECTION=true

# Development
NODE_ENV=development
DEBUG=project-mgmt:*
```

### Unified Package Configuration

```json
{
  "name": "@project-mgmt/monorepo",
  "version": "1.0.0",
  "private": true,
  "description": "Unified Project Management Application",
  "workspaces": [
    "apps/*",
    "core/*",
    "ai/*",
    "integrations/*"
  ],
  "scripts": {
    "env:setup": "node scripts/setup-env.js",
    "env:link": "node scripts/link-env.js",
    "env:validate": "node scripts/validate-noa-root.js",

    "providers:detect": "node scripts/detect-providers.js",
    "providers:sync": "node scripts/sync-agent-files.js",
    "providers:list": "node scripts/list-providers.js",
    "providers:priority": "node scripts/show-priority.js",

    "shared:init": "node scripts/init-ai-shared.js",
    "shared:list": "node scripts/list-shared-resources.js",
    "shared:sync": "node scripts/sync-shared-resources.js",

    "memory:index": "node scripts/index-memory.js",
    "memory:search": "node scripts/search-memory.js",
    "memory:export": "node scripts/export-memory.js",

    "noa:status": "node scripts/noa-status.js",
    "noa:providers": "node scripts/noa-providers.js",

    "dev": "npm run dev:web",
    "dev:web": "cd apps/web && ng serve",
    "dev:desktop": "npm run build:web && cd apps/desktop && electron .",
    "dev:cli": "cd apps/cli && ts-node src/index.ts",
    "dev:mcp": "cd apps/mcp && ts-node src/server.ts",

    "build": "npm run build:all",
    "build:web": "cd apps/web && ng build --configuration production",
    "build:desktop": "npm run build:web && cd apps/desktop && electron-builder",
    "build:mobile": "npm run build:web && npx cap sync",
    "build:cli": "cd apps/cli && npm run build",
    "build:mcp": "cd apps/mcp && npm run build",
    "build:all": "npm run build:web && npm run build:desktop && npm run build:cli && npm run build:mcp",

    "test": "npm run test:unit && npm run test:e2e",
    "test:unit": "ng test --watch=false",
    "test:e2e": "playwright test",

    "lint": "eslint . && stylelint '**/*.scss'",
    "format": "prettier --write .",

    "consolidate": "node scripts/consolidate-repos.js",
    "migrate": "node scripts/migrate-data.js",
    "verify": "node scripts/verify-features.js"
  },
  "dependencies": {
    "@angular/core": "^20.x",
    "@angular/material": "^20.x",
    "@ngrx/store": "^20.x",
    "@ngrx/effects": "^20.x",

    "electron": "^37.x",
    "@capacitor/core": "^7.x",

    "@anthropic-ai/sdk": "latest",
    "openai": "latest",

    "idb": "^8.x",
    "better-sqlite3": "^11.x",

    "rxjs": "^7.x",
    "nanoid": "^5.x"
  }
}
```

---

## 7) Inconsistency Analysis

### Identified Issues to Resolve

| ID | Category | Description | Source Repos | Resolution |
|----|----------|-------------|--------------|------------|
| INC-001 | **State Management** | NgRx vs Redux vs Zustand | super-productivity (NgRx), tududi (Redux), Taskosaur (Zustand) | Standardize on NgRx with signals |
| INC-002 | **API Style** | REST vs GraphQL vs MCP | Various | MCP primary, REST fallback |
| INC-003 | **Database** | IndexedDB vs SQLite vs PostgreSQL | Various | IndexedDB primary, SQLite for CLI |
| INC-004 | **Task Model** | Different task schemas | All task repos | Unified superset schema |
| INC-005 | **Sync Protocol** | Different sync mechanisms | super-productivity, Backlog.md | Unified sync with vector clocks |
| INC-006 | **AI Integration** | Different AI provider patterns | claude-task-master, dspy | Unified AI adapter layer |
| INC-007 | **Rule Format** | Different rule formats | ruler, agent-rules, agentic-cursorrules | Unified rule schema with translations |
| INC-008 | **Prompt Format** | Different prompt templates | promptfusion, system-prompts | Unified prompt registry |
| INC-009 | **Config Format** | JSON vs YAML vs TOML | Various | JSON primary, YAML for docs |
| INC-010 | **Build System** | Different bundlers | Webpack, Vite, esbuild | Standardize on Vite |
| INC-011 | **Provider Detection** | Missing in most repos | spec-kit only | Port spec-kit detection to all |

### Duplication Analysis

| Feature | Duplicate Implementations | Unified Location |
|---------|---------------------------|------------------|
| Task CRUD | super-productivity, tududi, Taskosaur, my-todo-app | `core/tasks/task.service.ts` |
| Time tracking | super-productivity, Taskosaur | `core/time-tracking/time.service.ts` |
| Rule evaluation | ruler, agent-rules | `core/rules/rule-engine.ts` |
| Spec parsing | spec-kit, BMAD-METHOD | `core/specs/spec-parser.ts` |
| Prompt building | promptfusion, claude-task-master | `ai/prompts/prompt-builder.ts` |
| Kanban board | super-productivity, Taskosaur | `apps/web/features/kanban/` |
| Calendar view | super-productivity, tududi | `apps/web/features/calendar/` |
| Sync engine | super-productivity, Backlog.md | `core/sync/sync-engine.ts` |
| Provider detection | spec-kit | `core/providers/` |

---

## 8) Shared Provider Service Architecture

### Service Provider Interface

```typescript
// core/providers/provider.interface.ts
export interface ProjectMgmtProvider {
  // Identity
  providerId: string;
  providerName: string;
  providerVersion: string;

  // Capabilities
  getCapabilities(): ProviderCapabilities;

  // Task Operations
  getTasks(filter?: TaskFilter): Promise<Task[]>;
  createTask(task: TaskCreate): Promise<Task>;
  updateTask(id: string, updates: TaskUpdate): Promise<Task>;
  deleteTask(id: string): Promise<void>;

  // Goal Operations
  getGoals(): Promise<Goal[]>;
  createGoal(goal: GoalCreate): Promise<Goal>;

  // AI Operations
  generateTaskBreakdown(description: string): Promise<Task[]>;
  suggestNextTask(): Promise<Task | null>;
  analyzeProductivity(): Promise<ProductivityReport>;

  // Sync Operations
  sync(): Promise<SyncResult>;
  getLastSyncTime(): Promise<Date | null>;

  // Events
  onTaskChange: Observable<TaskChangeEvent>;
  onSyncComplete: Observable<SyncResult>;
}
```

### MCP Server Implementation

```typescript
// apps/mcp/src/server.ts
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { initSharedProviderAccess, getAvailableProviders } from '../../core/providers/shared-access.service';

const server = new Server({
  name: 'project-mgmt',
  version: '1.0.0'
}, {
  capabilities: {
    resources: {},
    tools: {},
    prompts: {}
  }
});

// Initialize provider detection on startup
await initSharedProviderAccess();

// Tools
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: 'create_task',
      description: 'Create a new task',
      inputSchema: { /* ... */ }
    },
    {
      name: 'list_tasks',
      description: 'List tasks with optional filters',
      inputSchema: { /* ... */ }
    },
    {
      name: 'list_providers',
      description: 'List all detected AI/IDE providers',
      inputSchema: { type: 'object', properties: {} }
    },
    {
      name: 'switch_provider',
      description: 'Switch the active AI provider',
      inputSchema: {
        type: 'object',
        properties: {
          provider: { type: 'string', description: 'Provider key (e.g., claude, cursor-agent)' }
        },
        required: ['provider']
      }
    },
    {
      name: 'generate_breakdown',
      description: 'AI-powered task breakdown',
      inputSchema: { /* ... */ }
    },
    {
      name: 'get_goals',
      description: 'Get goals and their status',
      inputSchema: { /* ... */ }
    },
    {
      name: 'apply_policy',
      description: 'Check task against policy rules',
      inputSchema: { /* ... */ }
    },
    {
      name: 'run_truth_gate',
      description: 'Execute truth gate verification',
      inputSchema: { /* ... */ }
    }
  ]
}));
```

---

## 9) Implementation Phases

### Phase 1: Foundation (Week 1-2)

```
□ Create unified directory structure
□ Set up unified environment at .project-mgmt-env
□ Create ai/shared connection (core/ai-shared/)
□ Initialize ai/shared directories (agents, workflows, skills, tools, models)
□ Create base package.json with all dependencies
□ Set up monorepo workspace configuration
□ Port provider auto-detection from spec-kit
□ Implement provider priority system (local > hybrid > cloud)
□ Create platform abstraction layer
□ Create kernel abstraction layer
□ Set up build pipeline with Vite
```

### Phase 2: Core Migration (Week 3-4)

```
□ Migrate task management from super-productivity
□ Migrate rule engine from ruler
□ Migrate spec parser from spec-kit
□ Migrate provider detection from spec-kit
□ Migrate AI integration from claude-task-master
□ Migrate prompt library from promptfusion → ai/shared/prompts
□ Implement memory system (core/memory/)
□ Create NOA command integrations
□ Create unified state management with NgRx
□ Create unified data models
```

### Phase 3: Integration (Week 5-6)

```
□ Integrate DSPy pipelines → ai/shared/workflows
□ Integrate BMAD methodology
□ Integrate PRP workflows → ai/shared/agents
□ Implement sync engine
□ Implement MCP server with provider tools
□ Create AI provider adapters (priority-aware)
□ Wire all IDE extensions to shared access
□ Populate ai/shared/skills and ai/shared/tools
□ Connect memory system to ai/shared
```

### Phase 4: Platform Apps (Week 7-8)

```
□ Build web application (Angular PWA)
□ Build desktop application (Electron)
□ Build CLI application with provider & NOA commands
□ Build MCP server with ai/shared integration
□ Create mobile app shell (Capacitor)
□ Test local AI priority on all platforms
```

### Phase 5: Finalization (Week 9-10)

```
□ Run full test suite
□ Performance optimization
□ Verify ai/shared resource loading < 100ms
□ Verify memory recall < 100ms
□ Documentation completion
□ Migration scripts for existing data
□ Release preparation
```

---

## 10) Verification Checklist

### Feature Verification

```bash
# Run feature verification script
npm run verify

# Expected output:
✓ Task management: PASS (100% coverage)
✓ Time tracking: PASS (100% coverage)
✓ Goals/Policy/Rules: PASS (100% coverage)
✓ AI integration: PASS (100% coverage)
✓ Sync providers: PASS (100% coverage)
✓ Cross-platform: PASS (5/5 platforms)
✓ Offline mode: PASS
✓ MCP server: PASS
✓ CLI tools: PASS
✓ Provider detection: PASS (17/17 providers)
✓ AI/Shared integration: PASS (7/7 resource types)
✓ Memory system: PASS (recall < 100ms)
✓ NOA commands: PASS (all commands)
```

### AI/Shared Resource Verification

```bash
# Verify ai/shared connection
npm run shared:list

# Expected output:
┌─────────────────────────────────────────────────────────────┐
│            AI/Shared Resources (${NOA_ROOT}/ai/shared)       │
├─────────────────────────────────────────────────────────────┤
│ agents/                                                      │
│   ● task-breakdown.agent.ts                                  │
│   ● code-review.agent.ts                                     │
│   ● productivity.agent.ts                                    │
│                                                              │
│ workflows/                                                   │
│   ● daily-planning.workflow.ts                               │
│   ● sprint-planning.workflow.ts                              │
│                                                              │
│ prompts/                                                     │
│   ● project-mgmt-prompt.md                                   │
│   ● startup-prmt.md                                          │
│                                                              │
│ skills/                                                      │
│   ● task-estimation.skill.ts                                 │
│   ● priority-sorting.skill.ts                                │
│                                                              │
│ tools/                                                       │
│   ● context-builder.ts                                       │
│   ● memory-retrieval.ts                                      │
│                                                              │
│ models/gguf/                                                 │
│   ○ (local models when downloaded)                           │
│                                                              │
│ commands/                                                    │
│   ● README.md (NOA command reference)                        │
└─────────────────────────────────────────────────────────────┘
```

### Provider Detection Verification

```bash
# Run provider detection
npm run providers:detect

# Expected output:
┌─────────────────────────────────────────────────────────────┐
│                    Provider Detection Results               │
├─────────────────────────────────────────────────────────────┤
│ CLI Providers:                                              │
│   ● claude (v1.0.0) - /usr/local/bin/claude                │
│   ○ gemini - not found                                      │
│   ● q (v2.1.0) - /usr/local/bin/q                          │
│                                                             │
│ IDE Providers:                                              │
│   ● cursor-agent - .cursor/ found                           │
│   ● copilot - .github/ found                                │
│   ○ windsurf - not configured                               │
│                                                             │
│ Primary Provider: claude                                    │
│ Active IDE: cursor                                          │
└─────────────────────────────────────────────────────────────┘
```

### Cross-Platform Verification

| Platform | Build | Test | Deploy | Providers |
|----------|-------|------|--------|-----------|
| Web (PWA) | □ | □ | □ | Limited |
| Desktop (Windows) | □ | □ | □ | Full |
| Desktop (macOS) | □ | □ | □ | Full |
| Desktop (Linux) | □ | □ | □ | Full |
| Mobile (Android) | □ | □ | □ | Limited |
| Mobile (iOS) | □ | □ | □ | Limited |
| CLI | □ | □ | □ | Full |
| MCP | □ | □ | □ | Full |
| WSL | □ | □ | □ | Full |
| Docker | □ | □ | □ | Full |

---

## 11) Success Criteria

| Criterion | Metric | Target |
|-----------|--------|--------|
| Feature parity | Features from all repos | 100% |
| Test coverage | Unit + E2E | > 80% |
| Build time | Full build | < 5 min |
| Bundle size (web) | Gzipped | < 500KB |
| Startup time (desktop) | Cold start | < 2s |
| Offline capability | Core features offline | 100% |
| AI response time | Task breakdown | < 3s |
| Sync reliability | No data loss | 100% |
| Cross-platform parity | Feature consistency | > 95% |
| Provider detection | All 17 providers | 100% |
| IDE integration | Cursor, VS Code, Windsurf | 100% |

---

## Quick Reference Commands

```bash
# Initial setup
cd project-mgmt
npm run env:setup
npm install

# Provider management
npm run providers:detect   # Detect available providers
npm run providers:sync     # Sync agent files to all providers
npm run providers:list     # List all provider status

# Development
npm run dev:web      # Web development
npm run dev:desktop  # Desktop development
npm run dev:cli      # CLI development
npm run dev:mcp      # MCP server development

# Building
npm run build:all    # Build all platforms

# Testing
npm run test         # Run all tests
npm run lint         # Lint codebase

# Consolidation utilities
npm run consolidate  # Run repo consolidation
npm run migrate      # Migrate existing data
npm run verify       # Verify all features
```

---

## 12) AI/Shared Resource Integration

> **Critical:** project-mgmt MUST connect to `${NOA_ROOT}/ai/shared` for unified AI resources across all providers.

### Directory Structure Connection

```
${NOA_ROOT}/                              # NOA Root (N:\noa or /noa)
├── ai/
│   ├── providers/
│   │   └── local/
│   │       ├── llama-cpp.json           # llama.cpp config
│   │       ├── llama.cpp/bin/           # Bundled binaries
│   │       └── ollama/config.json       # Ollama config
│   └── shared/                          # ← SHARED RESOURCES (all providers use these)
│       ├── agents/                      # AI agents (cross-provider)
│       │   ├── task-breakdown.agent.ts
│       │   ├── code-review.agent.ts
│       │   └── productivity.agent.ts
│       ├── workflows/                   # Multi-step AI workflows
│       │   ├── daily-planning.workflow.ts
│       │   ├── sprint-planning.workflow.ts
│       │   └── retrospective.workflow.ts
│       ├── prompts/                     # Reusable prompt templates
│       │   ├── project-mgmt-prompt.md   # ← THIS FILE
│       │   └── startup-prmt.md
│       ├── skills/                      # AI capabilities & skills
│       │   ├── task-estimation.skill.ts
│       │   ├── priority-sorting.skill.ts
│       │   └── conflict-resolution.skill.ts
│       ├── tools/                       # AI tools & utilities
│       │   ├── context-builder.ts
│       │   ├── memory-retrieval.ts
│       │   └── intent-parser.ts
│       ├── models/                      # Local model files
│       │   └── gguf/                    # GGUF format models
│       └── commands/                    # Slash & terminal commands
│           └── README.md                # NOA command reference
├── project-mgmt/                        # ← PROJECT-MGMT APP
│   ├── core/
│   │   └── ai-shared/                   # Connection point to ai/shared
│   │       └── index.ts                 # Re-exports shared resources
│   └── ...
└── .noa-env/                            # Unified environment
    └── .env                             # Shared env vars
```

### Connection Service

```typescript
// project-mgmt/core/ai-shared/index.ts
import { existsSync, mkdirSync, symlinkSync, readFileSync, readdirSync } from 'fs';
import { join, resolve } from 'path';

export interface SharedResourcePaths {
  noaRoot: string;
  aiShared: string;
  agents: string;
  workflows: string;
  prompts: string;
  skills: string;
  tools: string;
  models: string;
  commands: string;
  localProviders: string;
}

/**
 * Resolve paths to ai/shared resources
 */
export function resolveSharedPaths(): SharedResourcePaths {
  // Resolve NOA_ROOT from env or detect
  const noaRoot = process.env.NOA_ROOT
    || process.env.PROJECT_ROOT?.replace(/[\\\/]project-mgmt$/, '')
    || detectNoaRoot();

  const aiShared = join(noaRoot, 'ai', 'shared');

  return {
    noaRoot,
    aiShared,
    agents: join(aiShared, 'agents'),
    workflows: join(aiShared, 'workflows'),
    prompts: join(aiShared, 'prompts'),
    skills: join(aiShared, 'skills'),
    tools: join(aiShared, 'tools'),
    models: join(aiShared, 'models'),
    commands: join(aiShared, 'commands'),
    localProviders: join(noaRoot, 'ai', 'providers', 'local'),
  };
}

/**
 * Auto-detect NOA root by walking up from project-mgmt
 */
function detectNoaRoot(): string {
  let current = process.cwd();
  while (current !== '/' && current !== 'C:\\') {
    if (existsSync(join(current, 'ai', 'shared'))) {
      return current;
    }
    current = resolve(current, '..');
  }
  throw new Error('Could not detect NOA_ROOT. Set NOA_ROOT environment variable.');
}

/**
 * Initialize ai/shared directories (create if missing)
 */
export function initSharedDirectories(): void {
  const paths = resolveSharedPaths();
  const dirs = ['agents', 'workflows', 'skills', 'tools', 'models'];

  for (const dir of dirs) {
    const fullPath = paths[dir as keyof SharedResourcePaths] as string;
    if (!existsSync(fullPath)) {
      mkdirSync(fullPath, { recursive: true });
      console.log(`Created: ${fullPath}`);
    }
  }
}

/**
 * List all available shared resources
 */
export function listSharedResources(): Record<string, string[]> {
  const paths = resolveSharedPaths();
  const resources: Record<string, string[]> = {};

  for (const [key, path] of Object.entries(paths)) {
    if (key !== 'noaRoot' && key !== 'aiShared' && existsSync(path)) {
      try {
        resources[key] = readdirSync(path).filter(f => !f.startsWith('.'));
      } catch {
        resources[key] = [];
      }
    }
  }
  return resources;
}
```

### Environment Variables for ai/shared

```bash
# Add to ${PROJECT_ROOT}/.project-mgmt-env/.env

# NOA Root Path (parent of ai/ and project-mgmt/)
NOA_ROOT=${PROJECT_ROOT}/..

# AI Shared Resource Paths
AI_SHARED_PATH=${NOA_ROOT}/ai/shared
AI_AGENTS_PATH=${AI_SHARED_PATH}/agents
AI_WORKFLOWS_PATH=${AI_SHARED_PATH}/workflows
AI_PROMPTS_PATH=${AI_SHARED_PATH}/prompts
AI_SKILLS_PATH=${AI_SHARED_PATH}/skills
AI_TOOLS_PATH=${AI_SHARED_PATH}/tools
AI_MODELS_PATH=${AI_SHARED_PATH}/models
AI_COMMANDS_PATH=${AI_SHARED_PATH}/commands

# Local AI Provider Paths
AI_LOCAL_PROVIDERS=${NOA_ROOT}/ai/providers/local
LLAMA_CPP_PATH=${AI_LOCAL_PROVIDERS}/llama.cpp
OLLAMA_CONFIG_PATH=${AI_LOCAL_PROVIDERS}/ollama
```

---

## 13) Provider Priority System

> Per `ai/shared/README.md`: Local providers have highest priority, cloud providers are fallback only.

### Priority Levels

| Priority | Type | Providers | Use Case |
|----------|------|-----------|----------|
| 1 (Highest) | **Local** | llama.cpp, Ollama | Privacy, offline, no API costs |
| 2 | **Hybrid** | Cursor (local+cloud), Amazon Q | Local-first with cloud fallback |
| 3 (Lowest) | **Cloud** | Claude, OpenAI, Gemini | When local unavailable |

### Priority Resolution Service

```typescript
// core/providers/provider-priority.service.ts
import { resolveSharedPaths } from '../ai-shared';
import { detectProviders, DetectedProvider } from './provider-detection.service';

export enum ProviderPriority {
  LOCAL = 1,
  HYBRID = 2,
  CLOUD = 3,
}

const PROVIDER_PRIORITIES: Record<string, ProviderPriority> = {
  // Local (highest priority)
  'llama.cpp': ProviderPriority.LOCAL,
  'ollama': ProviderPriority.LOCAL,

  // Hybrid (local-first with cloud fallback)
  'cursor-agent': ProviderPriority.HYBRID,
  'q': ProviderPriority.HYBRID,
  'copilot': ProviderPriority.HYBRID,

  // Cloud (used when local unavailable)
  'claude': ProviderPriority.CLOUD,
  'gemini': ProviderPriority.CLOUD,
  'opencode': ProviderPriority.CLOUD,
  'qwen': ProviderPriority.CLOUD,
  'codex': ProviderPriority.CLOUD,
};

export interface PrioritizedProvider extends DetectedProvider {
  priority: ProviderPriority;
}

/**
 * Get providers sorted by priority (local > hybrid > cloud)
 */
export function getPrioritizedProviders(): PrioritizedProvider[] {
  const { detected } = detectProviders();

  return detected
    .filter(p => p.isAvailable)
    .map(p => ({
      ...p,
      priority: PROVIDER_PRIORITIES[p.key] || ProviderPriority.CLOUD,
    }))
    .sort((a, b) => a.priority - b.priority);
}

/**
 * Select the best available provider based on priority
 */
export function selectBestProvider(forceLocal = false): PrioritizedProvider | null {
  const providers = getPrioritizedProviders();

  if (forceLocal) {
    return providers.find(p => p.priority === ProviderPriority.LOCAL) || null;
  }

  return providers[0] || null;
}

/**
 * Check if local AI is available
 */
export function isLocalAIAvailable(): boolean {
  const paths = resolveSharedPaths();
  const providers = getPrioritizedProviders();

  const hasLocalProvider = providers.some(p => p.priority === ProviderPriority.LOCAL);
  const hasModels = existsSync(join(paths.models, 'gguf'));

  return hasLocalProvider && hasModels;
}
```

---

## 14) NOA Commands Integration

> These commands from `ai/shared/commands/README.md` MUST be accessible from project-mgmt.

### NOA P2P Server Commands

```bash
noa start      # Start P2P server
noa stop       # Stop P2P server
noa status     # Check server status
noa nodes      # List connected nodes
noa storage    # Check storage
noa compute    # Check compute resources
```

### NOA Multi-Provider AI Commands

```bash
noa ai providers          # List AI providers (local first)
noa ai devices            # List registered devices
noa ai shared             # Show shared resources from ai/shared
noa ai switch <provider>  # Switch AI provider
```

### NOA Device Orchestration

```bash
noa device register <name>  # Register device
noa device list             # List devices
noa device capabilities     # Show capabilities
```

### NOA Git CI/CD Commands

```bash
git-pr create <branch>     # Create local PR
git-pr list                # List local PRs
git-pr merge <pr-id>       # Merge PR (AI conflict resolution)
git-pr sync <pr-id>        # Sync PR to GitHub
git-conflict detect        # Detect conflicts
git-conflict resolve <id>  # Resolve with AI/MLOps
git-ci run <pipeline>      # Run local CI/CD
git-ci status              # Check CI/CD status
```

### Self-Containment Commands

```bash
bundle-libraries <binary>    # Bundle libraries for a binary
bundle-all-libs              # Bundle libraries for all NOA binaries
download-static-binaries     # Download static binaries (no deps)
noa-kmod check               # Check kernel module availability
```

### CLI Integration in project-mgmt

```typescript
// apps/cli/src/commands/noa.command.ts
import { Command } from 'commander';
import { resolveSharedPaths, listSharedResources } from '../../../core/ai-shared';
import { getPrioritizedProviders, selectBestProvider } from '../../../core/providers/provider-priority.service';

export function registerNoaCommands(program: Command): void {
  const noa = program.command('noa').description('NOA ecosystem commands');

  // noa ai providers
  noa
    .command('ai')
    .command('providers')
    .description('List AI providers (sorted by priority: local > hybrid > cloud)')
    .action(() => {
      const providers = getPrioritizedProviders();
      console.log('\nAI Providers (by priority):');
      for (const p of providers) {
        const priority = ['', 'LOCAL', 'HYBRID', 'CLOUD'][p.priority];
        console.log(`  [${priority}] ${p.config.name}: ${p.isAvailable ? '✓' : '✗'}`);
      }
    });

  // noa ai shared
  noa
    .command('ai')
    .command('shared')
    .description('Show shared resources from ai/shared')
    .action(() => {
      const paths = resolveSharedPaths();
      const resources = listSharedResources();

      console.log(`\nShared Resources (${paths.aiShared}):`);
      for (const [type, files] of Object.entries(resources)) {
        console.log(`\n  ${type}/`);
        for (const file of files) {
          console.log(`    - ${file}`);
        }
      }
    });

  // noa ai switch <provider>
  noa
    .command('ai')
    .command('switch <provider>')
    .description('Switch active AI provider')
    .action((provider: string) => {
      // Implementation...
    });
}
```

---

## 15) Memory System (Never Forget, Instant Recall)

> Feature #17 from README: "Memory - never forget - Instant recall"

### Memory Architecture

```typescript
// core/memory/memory.service.ts
import { resolveSharedPaths } from '../ai-shared';

export interface MemoryEntry {
  id: string;
  type: 'task' | 'decision' | 'context' | 'conversation' | 'artifact';
  content: string;
  embedding?: number[];
  metadata: {
    source: string;          // Where this memory came from
    timestamp: Date;
    project?: string;
    tags: string[];
    importance: 'low' | 'medium' | 'high' | 'critical';
  };
}

export interface MemorySearchResult {
  entry: MemoryEntry;
  score: number;  // Relevance score 0-1
}

export class MemoryService {
  private indexPath: string;
  private storagePath: string;

  constructor() {
    const paths = resolveSharedPaths();
    this.indexPath = join(paths.aiShared, 'memory', 'index.json');
    this.storagePath = join(paths.aiShared, 'memory', 'entries');
  }

  /**
   * Store a memory entry (never forget)
   */
  async remember(entry: Omit<MemoryEntry, 'id'>): Promise<MemoryEntry> {
    const id = nanoid();
    const fullEntry: MemoryEntry = { ...entry, id };

    // Generate embedding for semantic search
    if (process.env.AI_LOCAL_ONLY === 'true') {
      fullEntry.embedding = await this.generateLocalEmbedding(entry.content);
    } else {
      fullEntry.embedding = await this.generateEmbedding(entry.content);
    }

    // Persist to storage
    await this.persistEntry(fullEntry);

    return fullEntry;
  }

  /**
   * Instant recall using semantic search
   */
  async recall(query: string, options?: {
    limit?: number;
    type?: MemoryEntry['type'];
    minImportance?: MemoryEntry['metadata']['importance'];
  }): Promise<MemorySearchResult[]> {
    const queryEmbedding = await this.generateEmbedding(query);
    const allEntries = await this.loadAllEntries();

    // Calculate similarity scores
    const scored = allEntries
      .filter(e => !options?.type || e.type === options.type)
      .map(entry => ({
        entry,
        score: this.cosineSimilarity(queryEmbedding, entry.embedding || []),
      }))
      .sort((a, b) => b.score - a.score);

    return scored.slice(0, options?.limit || 10);
  }

  /**
   * Get all memories for a project (instant recall)
   */
  async getProjectContext(projectId: string): Promise<MemoryEntry[]> {
    const allEntries = await this.loadAllEntries();
    return allEntries.filter(e => e.metadata.project === projectId);
  }

  /**
   * Use local llama.cpp for embeddings when AI_LOCAL_ONLY=true
   */
  private async generateLocalEmbedding(text: string): Promise<number[]> {
    const paths = resolveSharedPaths();
    const embeddingBin = join(paths.localProviders, 'llama.cpp', 'bin', 'llama-embedding');
    // Implementation using local model...
    return [];
  }
}
```

### Memory Integration Points

```typescript
// Add to package.json scripts
{
  "scripts": {
    "memory:index": "node scripts/index-memory.js",
    "memory:search": "node scripts/search-memory.js",
    "memory:export": "node scripts/export-memory.js"
  }
}
```

---

## 16) Updated Unified Features (All 95 Required)

### Feature Categories Summary

| Category | Features | Count |
|----------|----------|-------|
| A) Core Task & Project Management | 01-20 | 20 |
| B) Time Tracking & Productivity | 21-35 | 15 |
| C) UI & Views | 36-50 | 15 |
| D) Sync & Persistence | 51-60 | 10 |
| E) AI & Automation | 61-75 | 15 |
| F) Issue Provider Integrations | 76-83 | 8 |
| G) AgenticAI Workflows | 84-95 | 12 |
| **Total** | | **95** |

### Updated Feature Matrix

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    UNIFIED FEATURE MAPPING (v1.3)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  AI/SHARED INTEGRATION                                                      │
│  ├─ ai/shared/agents      → Shared AI agents across providers               │
│  ├─ ai/shared/workflows   → Multi-step AI workflows                         │
│  ├─ ai/shared/prompts     → Reusable prompt templates                       │
│  ├─ ai/shared/skills      → AI capabilities & skills                        │
│  ├─ ai/shared/tools       → AI tools & utilities                            │
│  ├─ ai/shared/models      → Local model files (GGUF)                        │
│  └─ ai/shared/commands    → NOA command reference                           │
│                                                                             │
│  MEMORY SYSTEM                                                              │
│  ├─ Never forget          → Persistent memory storage                       │
│  ├─ Instant recall        → Semantic search over memories                   │
│  └─ Project context       → Per-project memory retrieval                    │
│                                                                             │
│  PROVIDER PRIORITY                                                          │
│  ├─ Local (llama.cpp, Ollama)    → Highest priority                        │
│  ├─ Hybrid (Cursor, Amazon Q)    → Local-first fallback                    │
│  └─ Cloud (Claude, OpenAI)       → Used when local unavailable             │
│                                                                             │
│  SYNC PROVIDERS                                                             │
│  ├─ Vector Clock          → Conflict-free sync                              │
│  ├─ Dropbox               → Cloud storage                                   │
│  ├─ WebDAV                → Self-hosted storage                             │
│  ├─ Local File            → Electron + Android SAF                          │
│  └─ GitHub/GitLab         → Git-based sync                                  │
│                                                                             │
│  PRODUCTIVITY FEATURES                                                      │
│  ├─ Pomodoro Timer        → Focus sessions                                  │
│  ├─ Focus Mode            → Distraction-free work                           │
│  ├─ Idle Detection        → Auto-pause tracking                             │
│  ├─ Break Reminders       → Health enforcement                              │
│  └─ Worklog Export        → Reporting                                       │
│                                                                             │
│  AI CAPABILITIES                                                            │
│  ├─ PRD Parsing           → Auto task generation from requirements          │
│  ├─ Task Complexity       → AI-powered analysis                             │
│  ├─ Next Task Suggestion  → Smart recommendations                           │
│  ├─ Research Function     → Web-based research                              │
│  ├─ DSPy Pipelines        → ML optimization                                 │
│  └─ Prompt Fusion         → Weighted prompt composition                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 17) Success Criteria (Updated)

| Criterion | Metric | Target |
|-----------|--------|--------|
| Feature parity | All 95 features from all repos | 100% |
| Test coverage | Unit + E2E | > 80% |
| Build time | Full build | < 5 min |
| Bundle size (web) | Gzipped | < 500KB |
| Startup time (desktop) | Cold start | < 2s |
| Offline capability | Core features offline | 100% |
| AI response time | Task breakdown | < 3s |
| Sync reliability | No data loss | 100% |
| Cross-platform parity | Feature consistency | > 95% |
| Provider detection | All 17 providers | 100% |
| IDE integration | Cursor, VS Code, Windsurf | 100% |
| **ai/shared connection** | All 7 resource types | 100% |
| **Memory recall** | < 100ms search | 100% |
| **Local AI priority** | Local used when available | 100% |
| **NOA command access** | All NOA commands | 100% |
| **MCP tools** | All 35+ claude-task-master tools | 100% |
| **Sync providers** | All 5 sync providers | 100% |
| **Productivity features** | All 15 time tracking features | 100% |
| **Issue providers** | All 8 issue providers | 100% |
| **BMAD agents** | All 10 agent roles | 100% |
| **PRP agents** | All 7 agent roles | 100% |
| **Agentic workflows** | All 6 workflow types | 100% |
| **System prompts** | 25+ AI tool prompts | 100% |
| **Documentation** | 331+ docs indexed | 100% |
| **CECCA Framework** | 25+ dynamic graphs | 100% |
| **Mermaid graphs** | All .mmd files rendered | 100% |
| **Truth Gate** | Evidence-based execution | 100% |

---

## Quick Reference Commands (Updated)

```bash
# Initial setup
cd project-mgmt
npm run env:setup
npm install

# Provider management (priority aware)
npm run providers:detect   # Detect providers (shows priority)
npm run providers:sync     # Sync agent files to all providers
npm run providers:list     # List provider status

# AI/Shared resources
noa ai shared              # Show shared resources
noa ai providers           # List providers by priority
noa ai switch <provider>   # Switch active provider

# Memory operations
npm run memory:index       # Index all memories
npm run memory:search      # Search memories

# NOA ecosystem
noa start                  # Start P2P server
noa status                 # Check status
noa device list            # List devices

# Git workflow
git-pr create <branch>     # Create local PR
git-ci run <pipeline>      # Run CI

# Development
npm run dev:web            # Web development
npm run dev:desktop        # Desktop development
npm run dev:cli            # CLI development
npm run dev:mcp            # MCP server development

# Building
npm run build:all          # Build all platforms

# Testing
npm run test               # Run all tests
npm run lint               # Lint codebase

# Consolidation utilities
npm run consolidate        # Run repo consolidation
npm run migrate            # Migrate existing data
npm run verify             # Verify all features
```

---

**Prompt Version:** 1.6
**Created:** 2025-12-04
**Updated:** 2025-12-04
**Maintainer:** Project Team
**Target Completion:** 10 weeks from start

---

## Changelog

### v1.6 (2025-12-04)
- **MAJOR**: Added CECCA Autonomous System Framework (§3.4) with stem cell computing architecture
- Added 25+ Dynamic Ingestion Graphs table (EFG, DSG, DHG, DFG, DNG, DUBG, DPG, etc.)
- Added Autonomy Loop implementation (SENSE → DECIDE → UPDATE)
- Added Universal Task Execution Policy integration:
  - Truth Gate protocol for evidence-based execution
  - Triple-Verification Protocol (Pass A/B/C)
  - "Heal, Do Not Harm" update semantics
- Expanded Unified Directory Structure with 331+ documentation files:
  - `docs/00-guides/` - Goals-Policy-Rules Guide
  - `docs/00-ideas/` - Feature ideas (add-goals, north-star)
  - `docs/01-templates/` - Goal, Policy, Rules templates
  - `docs/02-examples/` - Annotated examples
  - `docs/05-policy/universal_task_execution_policy.md` - Critical execution policy
  - `docs/07-plans/autonomous-system.md` + `.mmd` - CECCA 108KB mermaid graph
  - `docs/sync/vector-clocks.md` - Vector clock documentation
- Added Documentation & Graphs Index table
- Added Mermaid Graphs Available table
- Added PRP Signal Reference integration (14+ signals with priorities)
- Added Nudge System for async agent-human communication
- Added Prompt Fusion Architecture (3-layer semantic weighting)

### v1.5 (2025-12-04)
- **MAJOR**: Added complete AgenticAI Execution Pipeline (§3.5) with end-to-end workflow
- Added Target Flow & UX diagram: UI Chat → Goal Decomposition → Prompt Engineering → Goals → Policy → Rules → Memory → Checklist → Plan → Spec → Task → XML Table → JSON → Execute
- Added Agentic Capabilities Matrix with 18 core capabilities:
  - Orchestration (BMAD-METHOD, PRP)
  - Loop Reasoning (PRP LOOP MODE)
  - Commands (spec-kit, claude-task-master)
  - Automation (Triggers, Actions)
  - Verification (QA Gate, Tests)
  - Unsupervised Learning (DSPy Optimizers)
  - Reasoning & Problem Solving (DSPy ChainOfThought)
  - Multi-agent Collaboration (BMAD Teams)
  - Agent Coordination (PRP Signals)
  - Self-reflection (BMAD QA, PRP)
  - Error Recovery (Self-healing)
  - Dynamic Tooling (MCP Server)
  - Autonomous Execution (Taskosaur)
  - Goal Chaining (Long-term autonomy)
  - Feedback Loops & Evaluators
  - Self-improving Agents (DSPy, PRD Training)
  - Self-healing Agents (Auto-recovery)
  - Staging & Testing
- Added Autonomous Orchestration Service with LOOP MODE
- Added Goal Decomposition Engine with policy/rules linking
- Added Self-Improving Agent System with PRD training (Python scripts)
- Added Self-Healing System with auto-recovery
- Added Single-File HTML Deployment (my-todo-app style)
- Added Feedback Loop & Evaluators for continuous improvement

### v1.4 (2025-12-04)
- **MAJOR**: Expanded Unified Features from 75 to 95 for fully automated AgenticAI
- Added Unified Directory Structure with 200+ specific paths
- Added AgenticAI Automation Requirements section with:
  - 8 Issue Provider Integrations (Jira, GitHub, GitLab, Gitea, OpenProject, Redmine, Trello, CalDAV)
  - BMAD Agent Team Configuration (10 agent roles)
  - PRP Agent Team Configuration (7 agent roles)
  - Autonomous Task Manager interface
  - Workflow Orchestration (greenfield/brownfield)
  - Automated Triggers & Actions (time/event/context/integration based)
  - System Prompts Library (25+ AI tools)
- Added Feature Category F: Issue Provider Integrations (76-83)
- Added Feature Category G: AgenticAI Workflows (84-95)
- Updated Success Criteria with agentic targets:
  - 8 issue providers
  - 10 BMAD agent roles
  - 7 PRP agent roles
  - 6 workflow types
  - 25+ system prompts

### v1.3 (2025-12-04)
- **MAJOR**: Expanded Unified Features from 38 to 75 after comprehensive 10-loop scan
- Added 5 feature categories: Core Task (20), Time Tracking (15), UI/Views (15), Sync (10), AI (15)
- Added Detailed Capability Matrix documenting all discovered features
- Added Desktop/Electron features: tray animation, full screen blocker, idle handler, overlay indicator
- Added Sync API (pfapi) features: vector clock, Dropbox, WebDAV, local file, encryption, compression
- Added claude-task-master MCP tools: 35+ task management functions
- Added spec-kit CLI commands: specify, constitution, plan, implement, tasks, checklist
- Added Backlog.md features: TUI, board view, task viewer, config watcher, guidelines
- Added BMAD-METHOD expansion packs: game dev, creative writing, infrastructure devops
- Added DSPy modules: ChainOfThought, ReAct, Predict, Assert, Optimizers, MCP tools
- Added promptfusion features: fusion engine, weight determination, message modifier
- Added PRP features: signal system, nudge system, agent system
- Added tududi features: inbox, universal search, auto-suggest, project insights
- Added agent-rules: project rules, global rules, mcp best practices
- Added plugin system: plugin API, plugin dev kit, vite plugin
- Updated success criteria with MCP tools, sync providers, productivity features targets

### v1.2 (2025-12-04)
- Added AI/Shared Resource Integration (§12) with full directory structure
- Added Connection Service for resolving ai/shared paths
- Added Provider Priority System (§13): local > hybrid > cloud
- Added NOA Commands Integration (§14): P2P, AI, device, git-ci
- Added Memory System (§15): never forget, instant recall
- Updated Unified Features to 38 (added ai/shared integration & memory)
- Added environment variables for ai/shared paths
- Added success criteria for ai/shared connection, memory, local AI priority
- Added NOA and memory commands to Quick Reference

### v1.1 (2025-12-04)
- Added Provider Auto-Detection System (§2) ported from spec-kit
- Added 17 supported AI agent providers with configuration registry
- Added IDE extension detection (Cursor, VS Code, Windsurf)
- Added shared provider access service
- Removed all absolute path references (N:\noa\*)
- Changed to relative paths using ${PROJECT_ROOT} variables
- Added provider detection verification to success criteria
- Added provider-related npm scripts
