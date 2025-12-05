# ♫ Signal Flow Documentation: Pull Request Guidelines

## 🔄 Complete Signal Flow for Pull Request Analysis

### **1. Signal Trigger**

```typescript
// Terminal Monitor detects agent creating PR from logs
// Example: Agent runs: gh pr create --title "Add user authentication" --body "JWT auth implemented"

// Terminal Monitor parses the output:
// "Pull request created: https://github.com/dcversus/prp/pull/123"

const prSignal: Signal = {
  id: 'signal-123',
  type: 'Pr', // Pull Request signal
  priority: 2,
  timestamp: TimeUtils.now(),
  data: {
    prNumber: 123,
    prUrl: 'https://github.com/dcversus/prp/pull/123',
    action: 'opened',
    author: 'claude-code-agent',
    command: 'gh pr create --title "Add user authentication feature" --body "Implements JWT-based authentication"',
    sessionId: 'agent-session-456',
    terminalOutput: 'Pull request created: https://github.com/dcversus/prp/pull/123'
  },
  source: 'terminal-monitor',
  metadata: {
    source: 'agent-activity-monitoring',
    sessionId: 'agent-session-456',
    terminalLog: 'gh pr create --title "Add user authentication feature" --body "Implements JWT-based authentication"',
    agentType: 'claude-code-anthropic',
    worktree: '/path/to/project',
    branch: 'feature/auth'
  }
};
```

### **How Terminal Monitor Detects PR Creation**

1. **Agent Command Detection**: Monitor detects `gh pr create` command
2. **Output Parsing**: Parse GitHub CLI output for PR URL and number
3. **Signal Generation**: Create `[Pr]` signal with extracted data
4. **Context Enrichment**: Add agent session, worktree, and branch info

```typescript
// Terminal Monitor implementation:
class TerminalMonitor {
  private parseGHCLOutput(output: string, command: string): Signal | null {
    if (command.includes('gh pr create')) {
      const prUrlMatch = output.match(/https:\/\/github\.com\/[^\/]+\/[^\/]+\/pull\/(\d+)/);
      if (prUrlMatch) {
        return this.createPRSignal(prUrlMatch[1], prUrlMatch[0], command);
      }
    }
    return null;
  }
}
```

### **2. Scanner Layer**
```typescript
// Scanner receives signal, creates 40k payload
scanner.processSignal(prSignal) → {
  // Classifies signal type
  // Creates contextual payload
  // Emits signal event
}
```

### **3. Guidelines Registry**
```typescript
// Registry matches signal to guideline
guidelinesRegistry.processSignal(prSignal) → {
  // Match: 'Pr' signal → 'pull-request-analysis' guideline
  // Check dependencies (GitHub API access)
  // Trigger guideline execution
  guidelinesRegistry.triggerGuideline('pull-request-analysis', prSignal);
}
```

### **4. Guidelines Executor - Step-by-Step**

#### **Step 1: Fetch Pull Request Data**
```typescript
// GitHub API calls
fetch-pull-request-data → {
  gitHubClient.analyzePR(prNumber) → {
    pr: { /* PR metadata */ }
    ci: { /* CI/CD status */ }
    comments: [ /* PR comments */ ]
    reviews: [ /* PR reviews */ ]
    files: [ /* Changed files */ ]
    commits: [ /* Commit history */ ]
  }
}
```

#### **Step 2: Inspector Analysis**
```typescript
// GPT-5 mini analyzes implementation
inspector-analysis → {
  inspector.analyze(payload, prompt) → {
    // Analyzes:
    // - Task completeness (0-100%)
    // - Description vs realization match
    // - Requirements compliance
    // - Code quality assessment
    // - Testing coverage
    // - Priority issues classification

    result: {
      implementation_analysis: { /* Detailed analysis */ },
      overall_assessment: {
        ready_for_review: true,
        recommended_action: 'request_changes',
        confidence_score: 88
      }
    }
  }
}
```

#### **Step 3: Structural Classification**
```typescript
// Classify findings by priority/importance
structural-classification → {
  // Process Inspector analysis
  // Calculate priority scores
  // Generate next actions
  // Determine overall priority

  result: {
    priorityIssues: [ /* Classified issues */ ],
    riskAssessment: { /* Risk analysis */ },
    nextActions: [ /* Action items */ ],
    overallPriority: 'high'
  }
}
```

#### **Step 4: Orchestrator Decision**
```typescript
// GPT-5 makes final decision + actions
orchestrator-decision → {
  orchestrator.makeDecision(context, prompt, tools) → {
    // Evaluate all findings
    // Make decision: approve/request_changes/comment/escalate
    // Execute GitHub actions

    result: {
      action: {
        type: 'request-changes',
        prNumber: 123,
        message: 'Please address missing functionality...',
        issues: [ /* Specific issues with line numbers */ ]
      },
      reasoning: 'Implementation is solid but missing...',
      confidence: 0.88
    }
  }
}
```

### **5. GitHub Actions Execution**
```typescript
// Execute decision on GitHub
executeAction(action) → {
  switch (action.type) {
    case 'request-changes':
      gitHubClient.createReview(prNumber, {
        body: action.message,
        event: 'REQUEST_CHANGES',
        comments: action.issues.map(issue => ({
          path: issue.file,
          line: issue.line_number,
          body: issue.description + '\n\n**Suggested Fix:** ' + issue.suggested_fix
        }))
      });
      break;
    // ... other action types
  }
}
```

### **6. Completion & Storage**
```typescript
// Save execution results
completeExecution(execution) → {
  // Calculate performance metrics
  // Create execution result
  // Save to storage: .prp/executions/{executionId}.json
  // Emit completion event
}
```

## 📊 Signal Types & Guidelines Mapping

| Signal Type | Source | Guideline Triggered | Priority | Use Case |
|-------------|--------|-------------------|----------|-----------|
| `[Pr]` | Terminal Monitor | `pull-request-analysis` | 2 | Agent creates Pull Request (`gh pr create`) |
| `[As]` | Terminal Monitor | `security-review` | 7 | Agent runs security commands (`npm audit`, `snyk test`) |
| `[Or]` | Terminal Monitor | `pull-request-performance-analysis` | 7 | Agent runs performance analysis (`benchmark`, `profile`) |
| `[oa]` | Terminal Monitor | Various guidelines | 2 | Agent status/activity changes |
| `[os]` | Scanner | Various guidelines | 2 | Scanner completes monitoring |
| `[Bb]` | Terminal Monitor | Various guidelines | 7 | Agent encounters blockers (`git push` fails, build errors) |

### **Signal Detection Patterns**

```typescript
// Terminal Monitor detects these agent commands:
'gh pr create' → [Pr] signal
'gh pr merge' → [Pr] signal (merge action)
'gh pr review' → [Pr] signal (review action)
'npm audit' → [As] signal (security)
'snyk test' → [As] signal (security)
'benchmark' → [Or] signal (performance)
'git push --force' → [Bb] signal (blocker)
```

## 🔄 Signal Escalation Flow

```
[Pr] (oo priority, 2 minutes)
  ↓
if not processed → [OP] (OO priority, immediate)
  ↓
if orchestrator fails → [AE] (AA priority, immediate admin)
```

## 🧪 E2E Test Coverage

The E2E test (`pull-request-guidelines.test.ts`) verifies:

1. **✅ Signal Triggering**: `[Pr]` signal triggers correct guideline
2. **✅ GitHub Integration**: API calls work correctly
3. **✅ Inspector Analysis**: GPT-5 mini analyzes implementation
4. **✅ Structural Classification**: Issues classified by priority
5. **✅ Orchestrator Decision**: Final decision made and actions executed
6. **✅ GitHub Actions**: Reviews/comments posted to PR
7. **✅ Storage Persistence**: Execution saved to `.prp/executions/`
8. **✅ Error Handling**: Failures handled gracefully
9. **✅ Signal Escalation**: Priority escalation works
10. **✅ Execution Order**: Steps executed in correct order

## 🎯 Key Success Metrics

- **Signal Processing**: < 5 seconds from trigger to execution start
- **GitHub API Calls**: < 10 seconds for data fetching
- **Inspector Analysis**: < 30 seconds for 40k payload
- **Orchestrator Decision**: < 45 seconds total execution time
- **GitHub Actions**: < 60 seconds total feedback loop
- **Success Rate**: > 95% successful executions
- **Storage**: All executions persisted for audit

## 🚨 Error Handling & Fallbacks

1. **GitHub API Failure**: Retry with exponential backoff
2. **Inspector Failure**: Use cached analysis or escalate
3. **Orchestrator Failure**: Post generic review with error details
4. **Network Issues**: Queue signal for retry
5. **Rate Limiting**: Implement delays and caching

## 📈 Performance Optimization

- **Parallel API Calls**: Fetch PR data concurrently
- **Caching**: Cache PR metadata for repeated signals
- **Batch Processing**: Handle multiple PR signals in batches
- **Token Optimization**: Limit context to essential information
- **Async Processing**: Non-blocking signal processing