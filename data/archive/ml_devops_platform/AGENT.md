# Agent & Provider System

**ML DevOps Platform v0.3.0**  
**Last Updated**: December 18, 2025

---

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Provider System](#provider-system)
- [MOE (Mixture of Experts) Router](#moe-mixture-of-experts-router)
- [SONA (Sequential Orchestration)](#sona-sequential-orchestration)
- [Agent Types](#agent-types)
- [Configuration](#configuration)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Best Practices](#best-practices)

---

## Overview

The ML DevOps Platform uses a **hybrid agent system** combining:

1. **AI Providers** - Backend LLM services (Abacus AI, Local Inference, Claude, etc.)
2. **MOE Router** - Intelligent routing to optimal provider based on task
3. **SONA Orchestrator** - Multi-agent workflow orchestration
4. **Specialized Agents** - Role-specific agents with defined capabilities

### Key Features

✅ **Intelligent Routing** - Automatic provider selection based on task requirements  
✅ **Local Inference** - Qwen3-1.7B model for offline operation  
✅ **Multi-Agent Workflows** - SONA orchestration with 5 execution strategies  
✅ **Real-time Collaboration** - Shared state with JSON-Patch synchronization  
✅ **Extensible** - Easy to add new providers and agents  

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      User Interface                         │
│  (Unified Chat, DeepCode, SONA Dashboard, Event Replay)    │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                    MOE Router                               │
│  • Task Analysis   • Provider Selection   • Fallback Logic │
└───────────┬─────────────────────────────────────────────────┘
            │
            ├──────────────┬───────────────┬─────────────────┐
            ▼              ▼               ▼                 ▼
   ┌─────────────┐ ┌─────────────┐ ┌──────────────┐ ┌──────────────┐
   │  Abacus AI  │ │Local Qwen3  │ │  Claude CLI  │ │  Mock (MVP)  │
   │  (Default)  │ │ (Optional)  │ │  (Future)    │ │ (Fallback)   │
   └─────────────┘ └─────────────┘ └──────────────┘ └──────────────┘
            │              │               │                 │
            └──────────────┴───────────────┴─────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                  SONA Orchestrator                          │
│  • Sequential    • Parallel    • Conditional              │
│  • Loop         • Map-Reduce   • Agent Coordination       │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                    Specialized Agents                       │
│  • Coder         • Analyst      • Reviewer                │
│  • Architect     • Tester       • DeepCode Agents         │
└─────────────────────────────────────────────────────────────┘
```

---

## Provider System

### Available Providers

#### 1. **Abacus AI Provider** (Default)

**Status**: ✅ Active  
**Type**: `abacus`  
**Endpoint**: `https://apps.abacus.ai/v1/chat/completions`  
**Model**: `gpt-4.1-mini`  

**Strengths**:
- 200K context window
- Low cost
- High reliability (98%)
- Best for: Chat, code generation, analysis, reasoning

**Configuration**:
```typescript
// config/providers.json
{
  "ai": {
    "type": "abacus",
    "useMock": false,
    "model": "gpt-4.1-mini",
    "temperature": 0.7,
    "maxTokens": 2048
  }
}
```

**Environment Variables**:
```bash
ABACUSAI_API_KEY=adf52b318f6c420eb3008a65111a113a
```

#### 2. **Local Inference Provider** (Optional)

**Status**: ✅ Available  
**Type**: `ruvllm` / `candle`  
**Model**: Qwen3-1.7B-Instruct-GGUF (~1GB)  
**Server**: Rust + Candle ML Framework  

**Strengths**:
- Fully offline operation
- 32K context window
- 4x faster than 7B models
- Q4_K_M quantization for efficiency
- Privacy-focused (no data leaves device)

**Configuration**:
```typescript
// config/providers.json
{
  "ai": {
    "type": "ruvllm",
    "useLocal": true,
    "localUrl": "http://127.0.0.1:8080",
    "model": "Qwen3-1.7B-Instruct"
  }
}
```

**Usage**:
1. Start Rust inference server:
   ```bash
   cd rust_backend/inference_server
   cargo run --release
   ```

2. Server auto-starts on `http://127.0.0.1:8080`
3. Platform detects and uses local inference automatically
4. Falls back to Abacus AI if server unavailable

#### 3. **Claude CLI Provider** (Future)

**Status**: 🔄 Planned  
**Type**: `claude_cli`  
**Context**: 200K tokens  

**Best for**: Reasoning, analysis, code review

#### 4. **GitHub Copilot Provider** (Future)

**Status**: 🔄 Planned  
**Type**: `github_copilot`  
**Context**: 8K tokens  

**Best for**: Code completion, git operations

#### 5. **Mock Provider** (Development)

**Status**: ✅ Always Available  
**Type**: `mock`  
**Purpose**: Testing and development  

**Features**:
- Simulates streaming responses
- Realistic delays (30ms per token)
- No API calls
- Always falls back when providers unavailable

---

## MOE (Mixture of Experts) Router

### Overview

The MOE Router **intelligently selects** the optimal AI provider based on:
- Task type (coding, analysis, reasoning, creative, tools)
- Task complexity (simple, medium, complex)
- Required capabilities
- Provider availability
- Cost and performance trade-offs

### Task Analysis

The router analyzes user input using pattern matching:

```typescript
const TASK_PATTERNS = {
  coding: [
    /write|create|implement|code|function|class/i,
    /fix|bug|debug|error/i,
    /typescript|javascript|python|rust/i
  ],
  analysis: [
    /analyze|review|examine|inspect/i,
    /data|csv|json|metrics/i,
    /summarize|overview/i
  ],
  reasoning: [
    /explain|why|how|reason/i,
    /solve|problem|challenge/i,
    /plan|strategy|design/i
  ],
  creative: [
    /generate|create|write|compose/i,
    /story|article|content/i,
    /brainstorm|ideate/i
  ],
  tools: [
    /search|find|lookup/i,
    /execute|run|call/i,
    /file|read|write/i
  ]
};
```

### Provider Selection

**Decision Factors**:
1. **Task Match**: Provider strengths vs task requirements
2. **Context Needs**: Available context window
3. **Speed Requirements**: Real-time vs batch processing
4. **Cost Optimization**: Balance quality and cost
5. **Availability**: Provider health and fallbacks

**Example Routing**:
```typescript
// Coding task → Abacus AI (best for code generation)
// Analysis task with long context → Claude CLI
// Simple completion → GitHub Copilot
// Offline mode → Local Qwen3
```

### Fallback Chain

```
Primary Provider
      ↓ (failure)
Fallback Provider #1
      ↓ (failure)
Fallback Provider #2
      ↓ (failure)
Mock Provider (always succeeds)
```

### Configuration

**MOE Router Config** (`nextjs_space/NOA_HOME/mutable/orchestration/moe.router.json`):

```json
{
  "defaultProvider": "abacus_deepagent",
  "fallbackChain": [
    "abacus_deepagent",
    "ruvllm_local",
    "mock"
  ],
  "routingRules": {
    "code_agent": {
      "prefer": ["abacus_deepagent", "github_copilot"],
      "fallback": ["ruvllm_local"]
    },
    "analysis_agent": {
      "prefer": ["claude_cli", "abacus_deepagent"],
      "fallback": ["abacus_deepagent"]
    },
    "reasoning_agent": {
      "prefer": ["claude_cli", "abacus_deepagent"],
      "fallback": ["abacus_deepagent"]
    }
  },
  "costThresholds": {
    "maxCostPerRequest": 0.5,
    "dailyBudget": 50.0
  },
  "performanceTargets": {
    "maxLatencyMs": 3000,
    "minThroughput": 100
  }
}
```

---

## SONA (Sequential Orchestration)

### Overview

SONA orchestrates **multi-agent workflows** with 5 execution strategies:

1. **Sequential** - Tasks run one after another
2. **Parallel** - Tasks run simultaneously
3. **Conditional** - Branch based on conditions
4. **Loop** - Repeat until condition met
5. **Map-Reduce** - Distribute work, aggregate results

### Agent Coordination

SONA manages:
- **Agent Memory** - Context preservation across tasks
- **State Updates** - JSON-Patch based state synchronization
- **Tool Execution** - Function calling for agents
- **Error Handling** - Retry logic and fallbacks
- **Result Aggregation** - Merge/concat/vote strategies

### Workflow Execution

```typescript
import { getSonaOrchestrator } from '@/lib/sona/orchestrator';

const orchestrator = getSonaOrchestrator();

const result = await orchestrator.executeWorkflow({
  id: 'paper-to-code',
  name: 'Research Paper to Code',
  strategy: 'sequential',
  agents: [
    {
      role: 'document_analyzer',
      task: 'Analyze research paper',
      capabilities: ['document_understanding', 'technical_analysis']
    },
    {
      role: 'code_generator',
      task: 'Generate implementation',
      capabilities: ['code_generation', 'algorithm_implementation']
    },
    {
      role: 'code_reviewer',
      task: 'Review and test code',
      capabilities: ['code_review', 'testing']
    }
  ]
});
```

---

## Agent Types

### General Purpose Agents

#### **Coder Agent**
**Role**: `coder`  
**Capabilities**: Code generation, refactoring, debugging  
**Best Provider**: Abacus AI, GitHub Copilot  
**Tools**: `execute_code`, `file_operations`  

#### **Analyst Agent**
**Role**: `analyst`  
**Capabilities**: Data analysis, metrics, insights  
**Best Provider**: Abacus AI, Claude CLI  
**Tools**: `analyze_data`, `web_search`  

#### **Reviewer Agent**
**Role**: `reviewer`  
**Capabilities**: Code review, quality assurance  
**Best Provider**: Claude CLI, Abacus AI  
**Tools**: `execute_code`, `file_operations`  

#### **Architect Agent**
**Role**: `architect`  
**Capabilities**: System design, architecture decisions  
**Best Provider**: Claude CLI, Abacus AI  
**Tools**: `analyze_data`, `web_search`  

#### **Tester Agent**
**Role**: `tester`  
**Capabilities**: Test generation, QA automation  
**Best Provider**: Abacus AI, GitHub Copilot  
**Tools**: `execute_code`, `file_operations`  

### DeepCode Agents

#### **Document Analyzer**
**Role**: `document_analyzer`  
**Purpose**: Analyze research papers, technical docs  
**Workflow**: Paper2Code, Doc2API  
**Capabilities**: Document understanding, technical analysis  

#### **Code Generator**
**Role**: `code_generator`  
**Purpose**: Generate implementation from specs  
**Workflow**: Text2Web, Paper2Code  
**Capabilities**: Code generation, algorithm implementation  

#### **Web Builder**
**Role**: `web_builder`  
**Purpose**: Build frontend applications  
**Workflow**: Text2Web  
**Capabilities**: React, Next.js, UI/UX  

#### **Backend Builder**
**Role**: `backend_builder`  
**Purpose**: Build backend services  
**Workflow**: Doc2API  
**Capabilities**: API design, database, authentication  

#### **Code Reviewer**
**Role**: `code_reviewer`  
**Purpose**: Review and improve generated code  
**Workflow**: All DeepCode workflows  
**Capabilities**: Code review, testing, optimization  

---

## Configuration

### Provider Configuration

**File**: `nextjs_space/config/providers.json`

```json
{
  "ai": {
    "type": "abacus",
    "useMock": false,
    "model": "gpt-4.1-mini",
    "temperature": 0.7,
    "maxTokens": 2048,
    "useLocal": false,
    "localUrl": "http://127.0.0.1:8080"
  }
}
```

### Environment Variables

**File**: `.env`

```bash
# AI Providers
ABACUSAI_API_KEY=your_api_key_here

# Local Inference (optional)
LOCAL_INFERENCE_PORT=8080
LOCAL_INFERENCE_MODEL=Qwen3-1.7B-Instruct

# MOE Configuration
MOE_DEFAULT_PROVIDER=abacus_deepagent
MOE_ENABLE_FALLBACK=true
MOE_MAX_RETRIES=3

# SONA Configuration
SONA_MAX_AGENTS=10
SONA_TIMEOUT_MS=30000
```

### NOA Configuration Files

**Provider Definitions** (`NOA_HOME/immutable/providers/`):
- `provider.abacus.json` - Abacus AI configuration
- `provider.llamacpp.json` - Local llama.cpp
- `provider.claude.json` - Claude CLI
- `provider.codex.json` - Codex CLI

**MOE Router** (`NOA_HOME/mutable/orchestration/`):
- `moe.router.json` - Routing rules
- `cost_models.json` - Cost tracking
- `scheduler.json` - Scheduling config

---

## Usage Examples

### Example 1: Simple Chat with Auto-Routing

```typescript
import { createProvider } from '@/lib/providers/ai-provider';

const provider = createProvider();

const response = await provider.streamChat([
  { role: 'user', content: 'Write a React component for a todo list' }
]);

// MOE Router automatically selects Abacus AI (best for coding)
for await (const chunk of response.stream) {
  console.log(chunk);
}
```

### Example 2: Local Inference

```typescript
import { createProvider } from '@/lib/providers/ai-provider';

// Force local inference
const provider = createProvider(true);

const response = await provider.streamChat([
  { role: 'user', content: 'Explain async/await in JavaScript' }
], {
  temperature: 0.5,
  maxTokens: 1024
});
```

### Example 3: SONA Workflow

```typescript
import { getSonaOrchestrator } from '@/lib/sona/orchestrator';
import { createPaper2CodeWorkflow } from '@/lib/sona/workflows';

const orchestrator = getSonaOrchestrator();
const workflow = createPaper2CodeWorkflow();

const result = await orchestrator.executeWorkflow({
  ...workflow,
  input: { paperUrl: 'https://arxiv.org/abs/1234.5678' }
});

console.log('Generated code:', result.output.code);
console.log('Test results:', result.output.tests);
```

### Example 4: MOE with Custom Routing

```typescript
import { getMOERouter } from '@/lib/moe/router';

const router = getMOERouter();

const decision = router.route({
  messages: [{ role: 'user', content: 'Analyze this 100K token dataset' }],
  requirements: {
    contextWindow: 100000,
    speed: 'medium',
    cost: 'low'
  }
});

console.log('Selected provider:', decision.provider);
console.log('Reason:', decision.reason);
console.log('Fallbacks:', decision.fallbacks);
```

### Example 5: Multi-Agent Collaboration

```typescript
import { getSonaOrchestrator } from '@/lib/sona/orchestrator';

const orchestrator = getSonaOrchestrator();

const result = await orchestrator.executeWorkflow({
  id: 'full-stack-app',
  strategy: 'parallel',
  agents: [
    {
      role: 'web_builder',
      task: 'Build frontend with Next.js',
      capabilities: ['react', 'nextjs', 'tailwind']
    },
    {
      role: 'backend_builder',
      task: 'Build REST API with Express',
      capabilities: ['nodejs', 'express', 'mongodb']
    }
  ],
  aggregateStrategy: 'merge'
});
```

---

## API Reference

### Provider Interface

```typescript
interface AIProvider {
  getName(): string;
  
  streamChat(
    messages: ChatMessage[],
    config?: ModelConfig
  ): Promise<StreamingResponse>;
  
  generateWidget(
    description: string,
    config?: ModelConfig
  ): Promise<WidgetGeneration>;
  
  analyzeCode(
    code: string,
    language: string
  ): Promise<{ issues: CodeIssue[]; suggestions: string[] }>;
}
```

### MOE Router

```typescript
interface MOERouter {
  route(request: RoutingRequest): RoutingDecision;
  analyzeTask(input: string, context?: ChatMessage[]): TaskAnalysis;
  selectProvider(analysis: TaskAnalysis): ProviderType;
}
```

### SONA Orchestrator

```typescript
interface SonaOrchestrator {
  executeWorkflow(config: WorkflowConfig): Promise<WorkflowResult>;
  cancelExecution(executionId: string): Promise<boolean>;
  getExecutionStatus(executionId: string): Promise<ExecutionStatus>;
}
```

---

## Best Practices

### 1. Provider Selection

✅ **Use Abacus AI by default** - Best balance of cost, quality, and reliability  
✅ **Enable local inference for offline mode** - Great for demos and privacy  
✅ **Let MOE Router handle selection** - Automatic optimization  
❌ **Don't hardcode provider types** - Use configuration  

### 2. Error Handling

✅ **Always implement fallbacks** - MOE Router handles this automatically  
✅ **Use try-catch with provider calls** - Network errors can occur  
✅ **Log provider failures** - For monitoring and debugging  
✅ **Implement retries with exponential backoff** - Transient failures  

### 3. Cost Optimization

✅ **Set cost thresholds in MOE config** - Prevent runaway costs  
✅ **Use local inference for development** - Free and fast  
✅ **Monitor token usage** - Track via analytics  
✅ **Cache responses when possible** - Reduce API calls  

### 4. SONA Workflows

✅ **Break complex tasks into agent steps** - Better results  
✅ **Use appropriate execution strategy** - Sequential vs parallel  
✅ **Provide clear agent roles and capabilities** - Better routing  
✅ **Test workflows with mock provider first** - Faster iteration  

### 5. Security

✅ **Store API keys in environment variables** - Never commit to git  
✅ **Use HTTPS for all provider calls** - Encrypt in transit  
✅ **Validate user input before sending to providers** - Prevent injection  
✅ **Implement rate limiting** - Prevent abuse  

---

## Troubleshooting

### Provider Connection Issues

**Problem**: "Failed to connect to provider"  
**Solutions**:
1. Check `ABACUSAI_API_KEY` in `.env`
2. Verify network connectivity
3. Check provider status (MOE Router will auto-fallback)
4. Try mock provider: `createProvider(false, true)`

### Local Inference Not Working

**Problem**: "Local inference server not responding"  
**Solutions**:
1. Start Rust server: `cd rust_backend && cargo run --release`
2. Check port 8080 is not in use: `lsof -i :8080`
3. Download model: Server auto-downloads on first run
4. Check logs: `rust_backend/inference_server/target/release/logs/`

### MOE Router Not Selecting Expected Provider

**Problem**: "Wrong provider selected for task"  
**Solutions**:
1. Check MOE router config: `NOA_HOME/mutable/orchestration/moe.router.json`
2. Review task patterns in `lib/moe/router.ts`
3. Use explicit provider: `createProvider(false, false, 'abacus')`
4. Check provider availability: `getProviderManager().getStatus()`

### SONA Workflow Timeout

**Problem**: "Workflow execution timed out"  
**Solutions**:
1. Increase timeout: Set `SONA_TIMEOUT_MS=60000` in `.env`
2. Break workflow into smaller steps
3. Use parallel strategy for independent tasks
4. Check individual agent performance

---

## Future Enhancements

### Planned Providers
- 🔄 Claude API integration
- 🔄 GitHub Copilot plugin
- 🔄 Anthropic Claude
- 🔄 Google Gemini
- 🔄 Ollama integration

### Planned Features
- 🔄 Provider health monitoring dashboard
- 🔄 Automatic model fine-tuning
- 🔄 Multi-modal support (vision, audio)
- 🔄 Federation across multiple devices
- 🔄 P2P provider network

---

## Contributing

### Adding a New Provider

1. **Implement `AIProvider` interface** in `lib/providers/ai-provider.ts`
2. **Add provider config** to `config/providers.json`
3. **Update MOE router** with provider strengths
4. **Add to NOA** immutable providers directory
5. **Update this documentation**

### Adding a New Agent

1. **Define agent role** in `lib/sona/types.ts`
2. **Create agent template** in `lib/sona/workflows.ts`
3. **Configure routing** in `moe.router.json`
4. **Add capabilities** to shared resources
5. **Document agent** in this file

---

## Related Documentation

- [README.md](./README.md) - Project overview
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture
- [SETUP.md](./SETUP.md) - Setup instructions
- [NOA_INTEGRATION_FIXED.md](./NOA_INTEGRATION_FIXED.md) - NOA details
- [QUICKSTART.md](./QUICKSTART.md) - Quick start guide

---

**Questions?** Open an issue or consult the [DeepCode documentation](/docs).
