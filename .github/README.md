# 🤖 FlexNetOS Automated CI System

> **Fully Automated GitHub - No Human in the Loop**

This is a comprehensive CI/CD automation system that aggregates issues from pull requests, uses AI to resolve them, and automatically merges changes into main. Designed for the FlexNetOS organization to use across all repositories.

## 🎯 What This System Does

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    FlexNetOS Automation Flow                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   PR Created ──► Aggregate Issues ──► Create Tracking Issue              │
│                        │                      │                          │
│                        ▼                      ▼                          │
│               ┌───────────────────────────────────────┐                 │
│               │     🤖 AI Resolution Engine           │                 │
│               │                                       │                 │
│               │  • Resolve merge conflicts            │                 │
│               │  • Fix linting errors                 │                 │
│               │  • Apply code suggestions             │                 │
│               │  • Fix security vulnerabilities       │                 │
│               │  • Resolve review comments            │                 │
│               │  • Fix type errors                    │                 │
│               └───────────────────────────────────────┘                 │
│                        │                                                 │
│                        ▼                                                 │
│            ┌──── Issues Remaining? ────┐                                │
│            │                           │                                │
│           Yes                          No                               │
│            │                           │                                │
│            ▼                           ▼                                │
│     Loop (max 10x)              Auto-Merge to Main                      │
│                                        │                                │
│                                        ▼                                │
│                                Close Tracking Issue                     │
│                                        │                                │
│                                        ▼                                │
│                                     Done ✅                             │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## 📦 Components

### Core Workflows

| File | Purpose |
|------|---------|
| `workflows/flexnetos-auto-resolve.yml` | Main CI workflow for automated resolution |
| `workflow-templates/flexnetos-reusable-resolver.yml` | Template for organization-wide reusable workflows |
| `workflow-templates/caller-template.yml` | Template for calling org-wide reusable workflows |

### Actions

| File | Purpose |
|------|---------|
| `actions/ai-resolver/action.yml` | Composite action for AI-powered resolution |

### Scripts

| File | Purpose |
|------|---------|
| `scripts/web-app-auth.py` | Web app authentication for AI providers |
| `scripts/rate-limiter.py` | Rate limiting and circuit breaker |
| `scripts/rollback-manager.py` | Automatic rollback system |
| `scripts/audit-logger.py` | Comprehensive audit logging |
| `scripts/deploy-to-org.sh` | Deploy to all org repositories |

### AI Providers

| File | Purpose |
|------|---------|
| `scripts/providers/cursor-cli.py` | [Cursor CLI](https://cursor.com/docs/cli/github-actions) integration |
| `scripts/providers/abacus-ai.py` | [Abacus AI](https://abacus.ai/help/python-sdk/github_cicd) integration |

### Auth Connectors

| File | Purpose |
|------|---------|
| `scripts/auth/github-app.py` | [GitHub App](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps) management |
| `scripts/auth/oauth-connectors.py` | OAuth providers (Google, GitHub, Microsoft, OpenAI) |
| `scripts/auth/oauth-server.py` | Local OAuth callback server |

### Configuration

| File | Purpose |
|------|---------|
| `org-config/flexnetos-automation.yml` | Organization-wide settings |

## 🚀 Quick Start

### 1. Set Up Organization Secrets

```bash
# Required: Bot token for GitHub operations
gh secret set FLEXNETOS_BOT_TOKEN --org FlexNetOS

# AI Provider API Keys
gh secret set CURSOR_API_KEY --org FlexNetOS      # Cursor CLI
gh secret set ABACUS_API_KEY --org FlexNetOS      # Abacus AI
gh secret set ANTHROPIC_API_KEY --org FlexNetOS   # Claude API
gh secret set OPENAI_API_KEY --org FlexNetOS      # OpenAI API

# Optional: Web session tokens
gh secret set CLAUDE_SESSION_TOKEN --org FlexNetOS
gh secret set CHATGPT_SESSION_TOKEN --org FlexNetOS

# GitHub App (recommended for org-wide access)
gh secret set GITHUB_APP_ID --org FlexNetOS
gh secret set GITHUB_APP_PRIVATE_KEY --org FlexNetOS

# OAuth Clients (for user account linking)
gh secret set GOOGLE_CLIENT_ID --org FlexNetOS
gh secret set GOOGLE_CLIENT_SECRET --org FlexNetOS
gh secret set GITHUB_CLIENT_ID --org FlexNetOS
gh secret set GITHUB_CLIENT_SECRET --org FlexNetOS
```

### 2. Deploy to All Repositories

```bash
# Deploy to all repos in the organization
.github/scripts/deploy-to-org.sh

# Or do a dry run first
.github/scripts/deploy-to-org.sh --dry-run

# Deploy to specific repos
.github/scripts/deploy-to-org.sh --repos "repo1,repo2,repo3"
```

### 3. Use in Individual Repositories

#### Option A: Self-Contained Workflow (Recommended)
This repository uses the self-contained `flexnetos-auto-resolve.yml` workflow which doesn't require external dependencies.

#### Option B: Organization-Wide Reusable Workflow
If you have deployed the reusable workflow to your organization's `.github` repository, you can use this pattern:

```yaml
# .github/workflows/auto-resolve.yml
name: FlexNetOS Automation

on:
  pull_request:
    types: [opened, synchronize, reopened, review_requested]
  pull_request_review:
    types: [submitted]

jobs:
  auto-resolve:
    uses: FlexNetOS/.github/workflows/flexnetos-reusable-resolver.yml@main
    with:
      pr_number: ${{ github.event.pull_request.number }}
      max_iterations: 10
      ai_provider: copilot
      auto_merge: true
    secrets:
      BOT_TOKEN: ${{ secrets.FLEXNETOS_BOT_TOKEN }}
```

**Note**: The templates in `workflow-templates/` are provided for reference only. They require the corresponding files to be deployed to the organization's `.github` repository first.

## 🧠 AI Provider Support

This system supports multiple AI providers with flexible authentication:

| Provider | Auth Method | Best For | Reference |
|----------|-------------|----------|-----------|
| **Cursor CLI** | API Key | Code operations, CI fix | [Docs](https://cursor.com/docs/cli/github-actions) |
| **Abacus AI** | API Key | Workflow automation | [Docs](https://abacus.ai/help/python-sdk/github_cicd) |
| **GitHub Copilot** | GitHub Token | Code suggestions | Native integration |
| **Claude.ai** | Web Session/API | Complex reasoning | [anthropic.com](https://anthropic.com) |
| **ChatGPT** | Web Session/API | General fixes | [openai.com](https://openai.com) |
| **Gemini** | OAuth/API | Multi-modal | [ai.google.dev](https://ai.google.dev) |
| **Local LLM** | None | Offline, privacy | Ollama, llama.cpp |

### Provider Priority

```yaml
ai_providers:
  priority:
    - cursor     # Best for code operations
    - copilot    # Best GitHub integration
    - abacus     # Workflow automation
    - claude     # Complex reasoning
    - chatgpt    # General purpose
    - gemini     # Multi-modal
    - local      # Fallback
```

### Cursor CLI Integration

[Cursor CLI](https://cursor.com/docs/cli/headless) provides powerful headless code operations:

```bash
# Install Cursor CLI
curl https://cursor.com/install -fsS | bash

# Run analysis
cursor-agent -p --force "Fix linting errors in this file"

# Fix CI failures
cursor-agent -p --force "A CI workflow failed: $(cat failure.log)"
```

### Abacus AI Integration

[Abacus AI](https://abacus.ai/help/python-sdk/github_cicd) enables AI workflow automation:

```python
from abacusai import ApiClient

client = ApiClient(api_key=os.environ['ABACUS_API_KEY'])
agent = client.execute_agent(agent_id='your_agent', input_text='Review this code')
```

### Why Multiple Auth Methods?

- ✅ **API Keys**: Best for CI/CD automation
- ✅ **Web App Auth**: Access latest models without API costs
- ✅ **OAuth**: User-level authentication and account linking
- ✅ **GitHub App**: Organization-wide access with fine-grained permissions

## 📊 Issue Aggregation

The system collects issues from multiple sources:

### Collected Issue Types

| Category | Source | Auto-Fixable |
|----------|--------|--------------|
| Workflow Failures | GitHub Actions | ⚠️ Partial |
| Linting Errors | ESLint, etc. | ✅ Yes |
| Type Errors | TypeScript | ⚠️ Partial |
| Test Failures | Jest, etc. | ❌ No |
| Security Issues | npm audit, CodeQL | ✅ Yes |
| Merge Conflicts | Git | ✅ Yes |
| Review Comments | PR Reviews | ⚠️ Partial |
| Suggestions | Code Suggestions | ✅ Yes |
| Conversations | Review Threads | ⚠️ Partial |

### Consolidated Issue Example

When issues are found, a tracking issue is created:

```markdown
## 🤖 FlexNetOS Auto-Resolution Report

**PR:** #42
**Iteration:** 1

### 📊 Issue Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | 2 |
| 🟠 High | 5 |
| 🟡 Medium | 12 |
| 🟢 Low | 3 |

### 🔧 Issues by Category

#### 🔀 Merge Conflicts
- [ ] 🔴 Conflict in src/index.ts

#### 📏 Linting Errors
- [ ] 🟡 Missing semicolon (line 42)
- [ ] 🟡 Unused variable 'temp'
```

## 🔄 Loop Control

The system runs in a loop until all issues are resolved:

```
Iteration 1: 22 issues found → 15 fixed → 7 remaining
Iteration 2: 7 issues found → 5 fixed → 2 remaining
Iteration 3: 2 issues found → 2 fixed → 0 remaining
✅ All issues resolved → Auto-merge to main
```

### Loop Settings

| Setting | Default | Description |
|---------|---------|-------------|
| Max Iterations | 10 | Maximum loop iterations |
| Iteration Delay | 30s | Delay between iterations |
| Total Timeout | 2 hours | Maximum total runtime |
| Confidence Threshold | 0.85 | Min confidence for auto-apply |

## 🔐 Authentication & OAuth

### GitHub App Setup

For organization-wide automation, create a GitHub App:

```bash
# Generate setup instructions
python .github/scripts/auth/github-app.py --setup

# Generate app manifest
python .github/scripts/auth/github-app.py --manifest
```

Reference: [Creating GitHub Apps](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps)

### OAuth Connectors

Connect user accounts from multiple providers:

| Provider | Type | Use Case |
|----------|------|----------|
| Google | OAuth 2.0 | Gemini AI, GCP services |
| GitHub | OAuth 2.0 | Repository access |
| Microsoft | OAuth 2.0 | Azure AD, M365 |
| OpenAI | OAuth 2.0 | ChatGPT integration |
| Anthropic | API Key | Claude API |
| Abacus | API Key | AI workflows |
| Cursor | API Key | Code operations |

### OAuth Server

Run the local OAuth server for testing:

```bash
# Start OAuth server
python .github/scripts/auth/oauth-server.py --port 8080

# Navigate to http://localhost:8080 to connect accounts
```

Reference: [Authorizing OAuth Apps](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)

### Auth Connector Files

| File | Purpose |
|------|---------|
| `auth/github-app.py` | GitHub App JWT auth |
| `auth/oauth-connectors.py` | Unified OAuth providers |
| `auth/oauth-server.py` | Local OAuth server |

---

## 🛡️ Safety Features

### Rate Limiting

```python
# GitHub API: 5000 requests/hour
# Claude: 50 requests/hour
# ChatGPT: 60 requests/hour
# Copilot: 200 requests/hour
```

### Circuit Breaker

Prevents cascading failures:

```
CLOSED → (5 failures) → OPEN → (5 min timeout) → HALF_OPEN → (success) → CLOSED
```

### Automatic Rollback

Triggers on:
- 🚨 Deployment failure
- 🧪 Test regression
- 🔒 Security regression

### Audit Logging

All actions are logged for compliance:

```
[2024-01-15T10:30:00Z] [resolution_started] FlexNetOS/noa PR#42
[2024-01-15T10:30:05Z] [fix_applied] ESLint auto-fix
[2024-01-15T10:30:10Z] [merge_completed] PR merged to main
```

## ⚙️ Configuration

### Organization Config

Edit `.github/org-config/flexnetos-automation.yml`:

```yaml
automation:
  enabled: true
  mode: fully_automated  # or semi_automated, manual_review

ai_providers:
  priority:
    - copilot
    - claude
    - chatgpt
    - local

auto_merge:
  enabled: true
  requirements:
    all_checks_passing: true
    no_critical_issues: true
```

### Per-Repository Override

Create `.github/flexnetos-config.yml` in any repo:

```yaml
# Override organization defaults
automation:
  mode: semi_automated  # Require approval for this repo

resolution:
  max_iterations: 5

auto_merge:
  enabled: false  # Disable auto-merge for this repo
```

## 🔧 What Was Missing (Gaps Filled)

The original request asked for automated CI. Here's what was added:

### 1. **Rate Limiting & Backoff**
- Per-provider rate limits
- Exponential backoff on failures
- Burst limiting

### 2. **Circuit Breaker Pattern**
- Prevents hammering failing services
- Automatic recovery testing
- Configurable thresholds

### 3. **Security Scanning Integration**
- CodeQL analysis
- Dependency auditing
- Secret scanning

### 4. **Rollback System**
- Automatic rollback on failures
- Health check monitoring
- Rollback PR creation

### 5. **State Management**
- Iteration state persistence
- Cross-run continuity
- Cleanup on completion

### 6. **Audit Trail**
- Structured event logging
- Cost tracking
- Compliance reports

### 7. **Cost Management**
- Token usage tracking
- Per-provider cost estimation
- Monthly limits

### 8. **Error Handling**
- Graceful degradation
- Retry with backoff
- Fallback providers

### 9. **Quality Gates**
- Test coverage checks
- Lint validation
- Security scanning before merge

### 10. **Notification System**
- GitHub Issues for errors
- Workflow run summaries
- Rollback notifications

## 📈 Monitoring

### GitHub Actions Summary

Each run produces a summary:

```
┌────────────────────────────────────────┐
│   FlexNetOS Auto-Resolution Report     │
├────────────────────────────────────────┤
│ PR Number: #42                         │
│ Iterations: 3                          │
│ Fixes Applied: 22                      │
│ Status: ✅ Merged                      │
└────────────────────────────────────────┘
```

### Audit Report

Generate compliance reports:

```bash
python .github/scripts/audit-logger.py --action report --repo FlexNetOS/noa --days 30
```

### Cost Report

Track AI usage costs:

```bash
python .github/scripts/audit-logger.py --action costs --days 30
```

## 🚨 Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Rate limited | Wait or switch provider |
| Circuit open | Check service health |
| Max iterations | Increase limit or fix manually |
| Merge conflict | Check conflict resolution logs |
| Auth failed | Refresh session tokens |

### Manual Intervention

If automation gets stuck:

```bash
# Force close circuit breaker
python .github/scripts/rate-limiter.py --action force-close --resource github_api

# Trigger manual rollback
python .github/scripts/rollback-manager.py --repo FlexNetOS/noa --action rollback

# View audit logs
python .github/scripts/audit-logger.py --action report --repo FlexNetOS/noa
```

## 📄 License

Part of the FlexNetOS project. See LICENSE for details.

---

> 🤖 **Fully Automated GitHub** - Because developers should code, not manage PRs.

