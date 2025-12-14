# FlexNetOS Automated CI/CD Guide

> A comprehensive guide to using the fully automated GitHub CI/CD system with AI-powered issue resolution.

## Table of Contents

1. [System Overview](#1-system-overview)
2. [Architecture](#2-architecture)
3. [Quick Start](#3-quick-start)
4. [AI Providers](#4-ai-providers)
5. [Authentication Setup](#5-authentication-setup)
6. [Workflow Configuration](#6-workflow-configuration)
7. [Issue Resolution](#7-issue-resolution)
8. [Safety Features](#8-safety-features)
9. [Troubleshooting](#9-troubleshooting)
10. [Reference](#10-reference)

---

## 1) System Overview

### What This System Does

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FlexNetOS Automation Flow                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   PR Created ──► Aggregate Issues ──► Create Tracking Issue                  │
│                        │                      │                              │
│                        ▼                      ▼                              │
│               ┌───────────────────────────────────────┐                     │
│               │     🤖 AI Resolution Engine            │                     │
│               │                                        │                     │
│               │  • Resolve merge conflicts             │                     │
│               │  • Fix linting errors                  │                     │
│               │  • Apply code suggestions              │                     │
│               │  • Fix security vulnerabilities        │                     │
│               │  • Resolve review comments             │                     │
│               │  • Fix type errors                     │                     │
│               └───────────────────────────────────────┘                     │
│                        │                                                     │
│                        ▼                                                     │
│            ┌──── Issues Remaining? ────┐                                    │
│            │                           │                                    │
│           Yes                          No                                   │
│            │                           │                                    │
│            ▼                           ▼                                    │
│     Loop (max 10x)              Auto-Merge to Main                          │
│                                        │                                    │
│                                        ▼                                    │
│                                Close Tracking Issue                         │
│                                        │                                    │
│                                        ▼                                    │
│                                     Done ✅                                 │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Key Features

| Feature | Description |
|---------|-------------|
| **Issue Aggregation** | Collects all PR issues from multiple sources |
| **AI Resolution** | Uses multiple AI providers to generate fixes |
| **Loop Control** | Continues until all issues resolved (max iterations) |
| **Auto-Merge** | Automatically merges when all checks pass |
| **Rollback** | Auto-reverts on deployment/test failures |
| **Audit Logging** | Complete audit trail for compliance |

### Design Principles

- **No Human in the Loop**: Fully automated resolution and merge
- **Fail-Safe**: Rate limiting, circuit breakers, automatic rollback
- **Provider Agnostic**: Works with multiple AI providers
- **Organization-Wide**: Deploys to all FlexNetOS repositories

---

## 2) Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        FlexNetOS Automation Architecture                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        GitHub Actions Workflows                        │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐   │   │
│  │  │ flexnetos-      │  │ flexnetos-      │  │ caller-             │   │   │
│  │  │ auto-resolve    │  │ reusable-       │  │ template            │   │   │
│  │  │ .yml            │  │ resolver.yml    │  │ .yml                │   │   │
│  │  └────────┬────────┘  └────────┬────────┘  └─────────────────────┘   │   │
│  └───────────┼────────────────────┼─────────────────────────────────────┘   │
│              │                    │                                          │
│              ▼                    ▼                                          │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        Custom Actions                                  │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │ actions/ai-resolver/action.yml                                   │ │   │
│  │  │ - Multi-provider AI integration                                  │ │   │
│  │  │ - Code analysis and fix generation                               │ │   │
│  │  │ - Automatic commit and push                                      │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│              │                                                               │
│              ▼                                                               │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        Python Scripts                                  │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                │   │
│  │  │ Providers    │  │ Auth         │  │ Utilities    │                │   │
│  │  │ ────────     │  │ ────         │  │ ─────────    │                │   │
│  │  │ cursor-cli   │  │ github-app   │  │ rate-limiter │                │   │
│  │  │ abacus-ai    │  │ oauth-       │  │ rollback-    │                │   │
│  │  │ web-app-auth │  │ connectors   │  │ manager      │                │   │
│  │  │              │  │ oauth-server │  │ audit-logger │                │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘                │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### File Structure

```
.github/
├── workflows/
│   ├── flexnetos-auto-resolve.yml      # Main automation workflow
│   ├── flexnetos-reusable-resolver.yml # Reusable workflow for org
│   └── caller-template.yml             # Template for repositories
├── actions/
│   └── ai-resolver/
│       └── action.yml                  # AI resolution composite action
├── scripts/
│   ├── providers/
│   │   ├── cursor-cli.py               # Cursor CLI integration
│   │   └── abacus-ai.py                # Abacus AI integration
│   ├── auth/
│   │   ├── github-app.py               # GitHub App management
│   │   ├── oauth-connectors.py         # OAuth providers
│   │   └── oauth-server.py             # Local OAuth server
│   ├── web-app-auth.py                 # Web app session auth
│   ├── rate-limiter.py                 # Rate limiting & circuit breaker
│   ├── rollback-manager.py             # Automatic rollback
│   ├── audit-logger.py                 # Audit trail logging
│   └── deploy-to-org.sh                # Organization deployment
├── org-config/
│   └── flexnetos-automation.yml        # Organization settings
└── README.md                           # Documentation
```

---

## 3) Quick Start

### Step 1: Set Up Organization Secrets

```bash
# Required: Bot token for GitHub operations
gh secret set FLEXNETOS_BOT_TOKEN --org FlexNetOS

# AI Provider API Keys (at least one required)
gh secret set CURSOR_API_KEY --org FlexNetOS      # Cursor CLI
gh secret set ABACUS_API_KEY --org FlexNetOS      # Abacus AI
gh secret set ANTHROPIC_API_KEY --org FlexNetOS   # Claude API

# Optional: GitHub App (recommended for org-wide access)
gh secret set GITHUB_APP_ID --org FlexNetOS
gh secret set GITHUB_APP_PRIVATE_KEY --org FlexNetOS
```

### Step 2: Deploy to Repositories

```bash
# Preview deployment (dry run)
.github/scripts/deploy-to-org.sh --dry-run

# Deploy to all repositories
.github/scripts/deploy-to-org.sh

# Deploy to specific repositories
.github/scripts/deploy-to-org.sh --repos "repo1,repo2,repo3"
```

### Step 3: Enable for Individual Repositories

Add this workflow to any repository:

```yaml
# .github/workflows/auto-resolve.yml
name: FlexNetOS Auto-Resolve

on:
  pull_request:
    types: [opened, synchronize, reopened, review_requested]
  pull_request_review:
    types: [submitted]

jobs:
  auto-resolve:
    uses: FlexNetOS/.github/.github/workflows/flexnetos-reusable-resolver.yml@main
    with:
      pr_number: ${{ github.event.pull_request.number }}
      max_iterations: 10
      ai_provider: cursor
      auto_merge: true
    secrets:
      BOT_TOKEN: ${{ secrets.FLEXNETOS_BOT_TOKEN }}
```

### Step 4: Verify Installation

```bash
# Create a test PR
git checkout -b test-automation
echo "// test" >> test.js
git add test.js
git commit -m "Test automation"
git push -u origin test-automation
gh pr create --title "Test Automation" --body "Testing FlexNetOS automation"

# Watch the workflow
gh run watch
```

---

## 4) AI Providers

### Provider Priority

The system uses providers in priority order, falling back if one fails:

```yaml
ai_providers:
  priority:
    - cursor     # 1. Best for code operations
    - copilot    # 2. Best GitHub integration
    - abacus     # 3. Workflow automation
    - claude     # 4. Complex reasoning
    - chatgpt    # 5. General purpose
    - gemini     # 6. Multi-modal
    - local      # 7. Fallback (Ollama)
```

### Provider Comparison

| Provider | Auth Type | Best For | Rate Limit | Reference |
|----------|-----------|----------|------------|-----------|
| **Cursor CLI** | API Key | Code operations, CI fix | 200/hr | [cursor.com/docs/cli/headless](https://cursor.com/docs/cli/headless) |
| **Abacus AI** | API Key | Workflow automation | 100/hr | [abacus.ai/help/python-sdk/github_cicd](https://abacus.ai/help/python-sdk/github_cicd) |
| **GitHub Copilot** | GitHub Token | Code suggestions | 200/hr | Native integration |
| **Claude** | API Key / Web | Complex reasoning | 50/hr | [anthropic.com](https://anthropic.com) |
| **ChatGPT** | API Key / Web | General fixes | 60/hr | [openai.com](https://openai.com) |
| **Gemini** | OAuth / API | Multi-modal | 100/hr | [ai.google.dev](https://ai.google.dev) |
| **Local LLM** | None | Offline, privacy | Unlimited | Ollama, llama.cpp |

### Using Cursor CLI

[Cursor CLI](https://cursor.com/docs/cli/headless) is the recommended provider for code operations:

```bash
# Install
curl https://cursor.com/install -fsS | bash

# Run analysis (print mode)
cursor-agent -p "What does this code do?"

# Fix code (with file modifications)
cursor-agent -p --force "Fix linting errors in src/"

# Fix CI failures
cursor-agent -p --force "CI failed with: $(cat failure.log)"

# Batch processing
find src/ -name "*.ts" | while read file; do
  cursor-agent -p --force "Add JSDoc comments to $file"
done
```

**GitHub Actions Integration:**

```yaml
- name: Install Cursor CLI
  run: |
    curl https://cursor.com/install -fsS | bash
    echo "$HOME/.cursor/bin" >> $GITHUB_PATH

- name: Fix Issues
  env:
    CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
  run: |
    cursor-agent -p --force --output-format json \
      "Fix all linting errors and type issues"
```

Reference: [cursor.com/docs/cli/github-actions](https://cursor.com/docs/cli/github-actions)

### Using Abacus AI

[Abacus AI](https://abacus.ai/help/python-sdk/github_cicd) provides workflow automation:

```python
import os
from abacusai import ApiClient

# Initialize client
api_key = os.environ.get('ABACUS_API_KEY')
client = ApiClient(api_key=api_key)

# Execute agent
result = client.execute_agent(
    agent_id='your_agent_id',
    input_text='Review this code for security issues'
)

# Update model (deploy workflow)
agent = client.update_model(
    model_id='your_model_id',
    workflow_graph=workflow_graph,
    agent_interface=agent_interface
)
agent.wait_for_publish()
```

**GitHub Actions Integration:**

```yaml
- name: Run Abacus AI
  env:
    ABACUS_API_KEY: ${{ secrets.ABACUS_API_KEY }}
  run: |
    pip install abacusai
    python << 'EOF'
    from abacusai import ApiClient
    import os

    client = ApiClient(api_key=os.environ['ABACUS_API_KEY'])
    result = client.execute_agent(
        agent_id='code-review',
        input_text='Review the changes in this PR'
    )
    print(result)
    EOF
```

Reference: [abacus.ai/help/python-sdk/github_cicd](https://abacus.ai/help/python-sdk/github_cicd)

---

## 5) Authentication Setup

### GitHub App (Recommended)

For organization-wide automation, create a GitHub App:

```bash
# Generate setup instructions
python .github/scripts/auth/github-app.py --setup

# Generate app manifest for creation
python .github/scripts/auth/github-app.py --manifest
```

**Manual Setup:**

1. Go to [github.com/settings/apps/new](https://github.com/settings/apps/new)
2. Configure permissions:

| Permission | Access | Purpose |
|------------|--------|---------|
| Contents | Read & Write | Commit fixes |
| Issues | Read & Write | Create tracking issues |
| Pull Requests | Read & Write | Merge PRs |
| Workflows | Read & Write | Trigger workflows |
| Checks | Read & Write | Report status |
| Actions | Read | View workflow runs |

3. Subscribe to events:
   - `pull_request`
   - `pull_request_review`
   - `issues`
   - `push`
   - `workflow_run`

4. Generate and save private key

5. Install on organization

Reference: [docs.github.com/en/apps/creating-github-apps/about-creating-github-apps](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps)

### OAuth Connectors

Connect user accounts for personalized access:

```bash
# Start OAuth server for testing
python .github/scripts/auth/oauth-server.py --port 8080

# Navigate to http://localhost:8080 to connect accounts
```

**Supported OAuth Providers:**

| Provider | Scopes | Use Case |
|----------|--------|----------|
| **Google** | openid, email, profile | Gemini AI, GCP |
| **GitHub** | repo, workflow, read:org | Repository access |
| **Microsoft** | openid, profile, email | Azure AD, M365 |
| **OpenAI** | openid, profile, email | ChatGPT platform |

Reference: [docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)

### API Key Configuration

For providers using API keys:

```bash
# Cursor CLI
export CURSOR_API_KEY="your_key"

# Abacus AI
export ABACUS_API_KEY="your_key"

# Anthropic
export ANTHROPIC_API_KEY="your_key"

# OpenAI
export OPENAI_API_KEY="your_key"

# Or set as GitHub secrets
gh secret set CURSOR_API_KEY --org FlexNetOS
gh secret set ABACUS_API_KEY --org FlexNetOS
```

---

## 6) Workflow Configuration

### Organization Config

Edit `.github/org-config/flexnetos-automation.yml`:

```yaml
automation:
  enabled: true
  mode: fully_automated  # or semi_automated, manual_review

ai_providers:
  priority:
    - cursor
    - copilot
    - abacus
    - claude

resolution:
  max_iterations: 10
  confidence_threshold: 0.85
  loop:
    iteration_delay_seconds: 30
    backoff_multiplier: 1.5

auto_merge:
  enabled: true
  requirements:
    all_checks_passing: true
    no_critical_issues: true
```

### Per-Repository Override

Create `.github/flexnetos-config.yml` in any repository:

```yaml
# Override organization defaults
automation:
  mode: semi_automated  # Require approval for this repo

resolution:
  max_iterations: 5

auto_merge:
  enabled: false  # Disable auto-merge for this repo
```

### Workflow Inputs

| Input | Type | Default | Description |
|-------|------|---------|-------------|
| `pr_number` | number | required | PR to process |
| `max_iterations` | number | 10 | Max resolution attempts |
| `ai_provider` | string | copilot | Primary AI provider |
| `auto_merge` | boolean | true | Enable auto-merge |
| `security_scan` | boolean | true | Enable security scanning |
| `quality_gates` | boolean | true | Enable quality checks |

---

## 7) Issue Resolution

### Issue Categories

The system collects and resolves these issue types:

| Category | Source | Auto-Fixable | Priority |
|----------|--------|--------------|----------|
| Merge Conflicts | Git | ✅ Yes | Critical |
| Security Issues | npm audit, CodeQL | ✅ Yes | Critical |
| Test Failures | Jest, pytest | ⚠️ Partial | High |
| Type Errors | TypeScript | ⚠️ Partial | High |
| Workflow Failures | GitHub Actions | ⚠️ Partial | High |
| Linting Errors | ESLint, Prettier | ✅ Yes | Medium |
| Review Comments | PR Reviews | ⚠️ Partial | Medium |
| Code Suggestions | Code review | ✅ Yes | Medium |
| Documentation | Missing docs | ⚠️ Partial | Low |

### Resolution Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Issue Resolution Process                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. COLLECT                                                                  │
│     ┌─────────────┐   ┌─────────────┐   ┌─────────────┐                     │
│     │ ESLint      │   │ TypeScript  │   │ npm audit   │                     │
│     │ Errors      │   │ Errors      │   │ Issues      │                     │
│     └──────┬──────┘   └──────┬──────┘   └──────┬──────┘                     │
│            │                 │                 │                             │
│            └────────────────┬┴─────────────────┘                             │
│                             ▼                                                │
│  2. AGGREGATE                                                                │
│     ┌─────────────────────────────────────────────────────────────────┐     │
│     │ aggregated_issues.json                                           │     │
│     │ {                                                                │     │
│     │   "categories": { "lint": [...], "type": [...], "security": [...] }  │
│     │   "summary": { "total": 15, "critical": 2, "high": 5 }           │     │
│     │ }                                                                │     │
│     └─────────────────────────────────────────────────────────────────┘     │
│                             │                                                │
│                             ▼                                                │
│  3. CREATE TRACKING ISSUE                                                    │
│     ┌─────────────────────────────────────────────────────────────────┐     │
│     │ Issue #123: 🤖 Auto-Resolve: PR #42 - Iteration 1                │     │
│     │ - [ ] 🔴 Merge conflict in src/index.ts                          │     │
│     │ - [ ] 🟠 Type error: Property 'x' does not exist                 │     │
│     │ - [ ] 🟡 Missing semicolon (line 42)                             │     │
│     └─────────────────────────────────────────────────────────────────┘     │
│                             │                                                │
│                             ▼                                                │
│  4. RESOLVE                                                                  │
│     ┌─────────────────────────────────────────────────────────────────┐     │
│     │ AI Resolution Engine                                             │     │
│     │                                                                  │     │
│     │ For each issue:                                                  │     │
│     │   1. Check if auto-fixable                                       │     │
│     │   2. Generate fix with AI if needed                              │     │
│     │   3. Evaluate confidence                                         │     │
│     │   4. Apply if confidence >= threshold                            │     │
│     │                                                                  │     │
│     └─────────────────────────────────────────────────────────────────┘     │
│                             │                                                │
│                             ▼                                                │
│  5. COMMIT & LOOP                                                            │
│     ┌─────────────────┐                                                      │
│     │ git commit -m   │ ──► Push ──► Remaining issues? ──► Loop back to 1   │
│     │ "🤖 Auto-fix"   │                     │                                │
│     └─────────────────┘                     ▼ No                             │
│                                      Auto-Merge                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Confidence Thresholds

| Confidence | Action |
|------------|--------|
| ≥ 0.90 | Auto-apply fix |
| 0.70 - 0.89 | Apply with comment |
| < 0.70 | Skip (flag for review) |

---

## 8) Safety Features

### Rate Limiting

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Rate Limiting Architecture                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐        │
│  │ Hourly Limit    │     │ Daily Limit     │     │ Burst Limit     │        │
│  │                 │     │                 │     │                 │        │
│  │ Check requests  │ ──► │ Check requests  │ ──► │ Check burst     │        │
│  │ this hour       │     │ today           │     │ in last minute  │        │
│  └────────┬────────┘     └────────┬────────┘     └────────┬────────┘        │
│           │                       │                       │                  │
│           ▼                       ▼                       ▼                  │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │ Exponential Backoff                                                  │    │
│  │                                                                      │    │
│  │ On rate limit: wait = initial * (multiplier ^ attempt)               │    │
│  │ Example: 1s → 2s → 4s → 8s → 16s → ... → max 300s                    │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Default Limits:**

| Provider | Hourly | Daily | Backoff Max |
|----------|--------|-------|-------------|
| Cursor | 200 | 5000 | 60s |
| Abacus | 100 | 2000 | 60s |
| Claude | 50 | 500 | 300s |
| ChatGPT | 60 | 1000 | 300s |
| GitHub API | 5000 | 50000 | 60s |

### Circuit Breaker

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Circuit Breaker States                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│     ┌───────────┐        5 failures        ┌───────────┐                    │
│     │  CLOSED   │ ──────────────────────►  │   OPEN    │                    │
│     │ (normal)  │                          │ (failing) │                    │
│     └─────┬─────┘                          └─────┬─────┘                    │
│           │                                      │                          │
│           │              success                 │ 5 min timeout            │
│           │         ┌───────────────┐            │                          │
│           ◄─────────│  HALF-OPEN    │◄───────────┘                          │
│                     │  (testing)    │                                       │
│                     └───────────────┘                                       │
│                            │                                                 │
│                            │ failure                                         │
│                            └──────────────────► Back to OPEN                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Automatic Rollback

Rollback triggers automatically on:

| Trigger | Detection | Action |
|---------|-----------|--------|
| Deployment failure | Health check fails | Revert commit |
| Test regression | Tests that passed now fail | Create rollback PR |
| Security regression | New vulnerabilities introduced | Revert + alert |

```bash
# Manual rollback
python .github/scripts/rollback-manager.py \
  --repo FlexNetOS/noa \
  --action rollback \
  --reason manual_trigger

# Health check
python .github/scripts/rollback-manager.py \
  --repo FlexNetOS/noa \
  --action health-check
```

### Audit Logging

All actions are logged:

```bash
# Generate audit report
python .github/scripts/audit-logger.py \
  --action report \
  --repo FlexNetOS/noa \
  --days 30

# View cost summary
python .github/scripts/audit-logger.py \
  --action costs \
  --days 30

# Cleanup old logs
python .github/scripts/audit-logger.py \
  --action cleanup
```

---

## 9) Troubleshooting

### Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| Rate limited | Too many requests | Wait or switch provider |
| Circuit open | Service failing | Check service health, wait for recovery |
| Max iterations | Complex issues | Increase limit or fix manually |
| Merge conflict | Complex conflict | Check conflict resolution logs |
| Auth failed | Token expired | Refresh session tokens |
| No providers available | All providers failing | Check API keys, rate limits |

### Manual Intervention

```bash
# Force close circuit breaker
python .github/scripts/rate-limiter.py \
  --action force-close \
  --resource github_api

# Trigger manual rollback
python .github/scripts/rollback-manager.py \
  --repo FlexNetOS/noa \
  --action rollback \
  --target HEAD~1

# View audit logs
python .github/scripts/audit-logger.py \
  --action report \
  --repo FlexNetOS/noa

# Re-run workflow manually
gh workflow run flexnetos-auto-resolve.yml \
  -f pr_number=42 \
  -f max_iterations=5
```

### Debug Mode

Enable debug logging:

```yaml
env:
  DEBUG: true
  LOG_LEVEL: debug
```

### Checking Workflow Logs

```bash
# List recent runs
gh run list --workflow=flexnetos-auto-resolve.yml

# View specific run
gh run view <run_id>

# View logs
gh run view <run_id> --log
```

---

## 10) Reference

### External Documentation

| Resource | URL |
|----------|-----|
| Cursor CLI Headless | [cursor.com/docs/cli/headless](https://cursor.com/docs/cli/headless) |
| Cursor CLI GitHub Actions | [cursor.com/docs/cli/github-actions](https://cursor.com/docs/cli/github-actions) |
| Cursor CLI Fix CI | [cursor.com/docs/cli/cookbook/fix-ci](https://cursor.com/docs/cli/cookbook/fix-ci) |
| Abacus AI CI/CD | [abacus.ai/help/python-sdk/github_cicd](https://abacus.ai/help/python-sdk/github_cicd) |
| GitHub Apps | [docs.github.com/en/apps/creating-github-apps](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps) |
| GitHub OAuth Apps | [docs.github.com/en/apps/oauth-apps](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps) |

### File Reference

| File | Purpose |
|------|---------|
| `.github/workflows/flexnetos-auto-resolve.yml` | Main automation workflow |
| `.github/workflows/flexnetos-reusable-resolver.yml` | Reusable workflow for org |
| `.github/workflows/caller-template.yml` | Template for repositories |
| `.github/actions/ai-resolver/action.yml` | AI resolution action |
| `.github/scripts/providers/cursor-cli.py` | Cursor CLI integration |
| `.github/scripts/providers/abacus-ai.py` | Abacus AI integration |
| `.github/scripts/auth/github-app.py` | GitHub App management |
| `.github/scripts/auth/oauth-connectors.py` | OAuth providers |
| `.github/scripts/auth/oauth-server.py` | Local OAuth server |
| `.github/scripts/rate-limiter.py` | Rate limiting & circuit breaker |
| `.github/scripts/rollback-manager.py` | Automatic rollback |
| `.github/scripts/audit-logger.py` | Audit trail logging |
| `.github/org-config/flexnetos-automation.yml` | Organization settings |

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `FLEXNETOS_BOT_TOKEN` | Yes | GitHub token for operations |
| `CURSOR_API_KEY` | No* | Cursor CLI API key |
| `ABACUS_API_KEY` | No* | Abacus AI API key |
| `ANTHROPIC_API_KEY` | No* | Claude API key |
| `GITHUB_APP_ID` | No | GitHub App ID |
| `GITHUB_APP_PRIVATE_KEY` | No | GitHub App private key |

*At least one AI provider API key is required.

### Quick Commands

```bash
# Deploy automation to all repos
.github/scripts/deploy-to-org.sh

# Start OAuth server
python .github/scripts/auth/oauth-server.py --port 8080

# Generate GitHub App manifest
python .github/scripts/auth/github-app.py --manifest

# Generate audit report
python .github/scripts/audit-logger.py --action report --repo FlexNetOS/noa

# Manual rollback
python .github/scripts/rollback-manager.py --repo FlexNetOS/noa --action rollback
```

---

**Document Version:** 1.0
**Last Updated:** 2025-12-05
**Maintainer:** FlexNetOS Team

