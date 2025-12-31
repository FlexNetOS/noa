# Advanced Automation Guide

## Overview

This guide demonstrates how to use NOA's advanced automation features for real-world scenarios:

1. **Automated Code Reviews**
2. **Deployment Automation**  
3. **Multi-Agent Task Execution**
4. **Knowledge Base Interrogation**
5. **Real-World Workflows**

---

## 1. Automated Code Reviews

### Quick Start

```rust
use noa_core::agents::{WorkflowOrchestrator, workflows};
use noa_core::services::RAGService;

// Create code review workflow
let mut orchestrator = WorkflowOrchestrator::new();
let config = workflows::code_review(
    "PR-456".to_string(),
    "main".to_string()
);

// Execute review
let result = orchestrator.execute_workflow(config)?;
println!("{}", result.summary);
```

### CLI Usage

```bash
# Review a pull request
noa workflow code-review --pr 456 --branch main

# Review specific files
noa agents run file-io '{"op":"list","path":"./src"}'
noa agents run rag "best practices for error handling"

# Generate review report
noa workflow run code-review '{"pr":"456","files":["src/main.rs"]}'
```

### Automated Checks

The code review system automatically checks for:

- **Error Handling**: Detects `.unwrap()`, missing error propagation
- **Security**: Identifies `unsafe` blocks, hardcoded credentials, SQL injection risks
- **Documentation**: Flags missing doc comments on public APIs
- **Best Practices**: TODO/FIXME comments, code complexity
- **Testing**: Missing test coverage indicators

### Example Output

```
Code Review Summary:
- Critical Issues: 0
- High Priority: 2
- Medium Priority: 5
- Security Concerns: 1
- Total Issues: 8

Issues Found:
  [HIGH] src/auth.rs:45 - Unsafe code detected
    Suggestion: Review for memory safety and document necessity
  
  [HIGH] src/db.rs:123 - Hardcoded credentials (CWE-798)
    Suggestion: Use environment variables or secrets manager
  
  [MEDIUM] src/api.rs:67 - Use of unwrap() detected
    Suggestion: Replace with ? operator or proper error handling

Recommendations:
  ✓ Add integration tests for new auth endpoints
  ✓ Document all public API functions
  ✓ Review error handling consistency
```

---

## 2. Deployment Automation

### Quick Start

```rust
use noa_core::agents::{WorkflowOrchestrator, workflows};

let mut orchestrator = WorkflowOrchestrator::new();

// Create deployment workflow
let config = workflows::deployment(
    "production".to_string(),
    "v2.1.0".to_string()
);

// Execute deployment
let result = orchestrator.execute_workflow(config)?;
if result.success {
    println!("Deployment successful!");
}
```

### CLI Usage

```bash
# Deploy to staging
noa workflow deploy --env staging --version v2.1.0

# Deploy with custom parameters
noa workflow run deployment '{
  "environment": "production",
  "version": "v2.1.0",
  "rollback_on_failure": true,
  "health_check_timeout": 300
}'

# Check deployment status
noa workflow run deployment '{"action":"status","environment":"production"}'
```

### Deployment Steps

The system automatically:

1. **Pre-deployment checks**
   - Verify version compatibility
   - Check resource availability
   - Validate configuration

2. **Build & Test**
   - Run build process
   - Execute test suite
   - Generate artifacts

3. **Deploy**
   - Stop services gracefully
   - Deploy new version
   - Start services

4. **Post-deployment**
   - Health checks
   - Smoke tests
   - Rollback if needed

### Example Workflow

```bash
$ noa workflow deploy --env production --version v2.1.0

Starting Deployment Workflow
Environment: production
Version: v2.1.0
--------------------------------------------------------------------------------

[1/6] Pre-flight checks...                          ✓ PASSED (2.3s)
[2/6] Building artifacts...                         ✓ PASSED (45.2s)
[3/6] Running test suite...                         ✓ PASSED (23.1s)
[4/6] Deploying to production...                    ✓ PASSED (12.8s)
[5/6] Health check...                               ✓ PASSED (5.4s)
[6/6] Post-deployment verification...               ✓ PASSED (3.2s)

✓ SUCCESS - Deployment
Total Tasks: 6
Successful: 6
Failed: 0
Execution Time: 92.0s

Deployment URL: https://app.example.com
```

---

## 3. Multi-Agent Task Execution

### Architecture

```
Commander Agent → Plans execution
    ↓
Multi-Agent Executor → Coordinates agents
    ↓
Specialized Agents → Execute tasks
    - FileIOAgent: File operations
    - TerminalAgent: Command execution
    - RAGAgent: Knowledge retrieval
    - CommanderAgent: Task decomposition
```

### Quick Start

```rust
use noa_core::agents::{MultiAgentExecutor, CommanderChiefAgent, CommanderRequest};

// Create executor with all agents
let mut executor = MultiAgentExecutor::new();

// Create a complex task
let commander = CommanderChiefAgent::new();
let request = CommanderRequest {
    goal: "Analyze codebase and generate security report".to_string(),
    context: Some(hashmap!{
        "repository".to_string() => "./src".to_string(),
        "focus".to_string() => "security".to_string(),
    }),
    constraints: None,
};

// Plan and execute
let plan = commander.plan_execution(request)?;
let result = executor.execute_plan(plan)?;

// Review results
for task_result in result.task_results {
    println!("[{}] {}: {}",
        task_result.agent_name,
        task_result.task_description,
        if task_result.status == TaskStatus::Completed { "✓" } else { "✗" }
    );
}
```

### CLI Usage

```bash
# List available agents
noa agents list

# Run specific agent
noa agents run file-io '{"op":"read","path":"Cargo.toml"}'
noa agents run terminal '{"command":"cargo","args":["test"]}'
noa agents run rag "How do I handle errors in Rust?"

# Complex multi-agent task
noa agents run commander-chief "Analyze all TypeScript files for security issues"
```

### Example: Multi-Step Analysis

```bash
$ noa agents run commander-chief "Analyze project for deployment readiness"

Planning execution...
Created plan with 5 tasks

Executing Plan:
  [1/5] [file-io] List all source files...           ✓ (125ms)
  [2/5] [rag] Retrieve deployment best practices...   ✓ (234ms)
  [3/5] [terminal] Run test suite...                  ✓ (12.3s)
  [4/5] [terminal] Check dependencies...              ✓ (1.2s)
  [5/5] [file-io] Verify configuration files...       ✓ (89ms)

✓ Plan executed successfully
Success Rate: 100% (5/5)
Total Time: 14.0s

Summary:
- All tests passing (142/142)
- Dependencies up to date
- Configuration valid
- Ready for deployment ✓
```

---

## 4. Knowledge Base Interrogation

### Quick Start

```rust
use noa_core::services::{RAGService, Document};
use noa_core::agents::rag::RAGQuery;
use std::path::Path;

let service = RAGService::new();
let db_path = Path::new("./data/knowledge.db");

// Add documentation to knowledge base
let doc = Document {
    title: "Authentication Guide".to_string(),
    content: "To implement authentication, use OAuth 2.0 with JWT tokens...".to_string(),
    source: "docs/auth.md".to_string(),
    metadata: serde_json::json!({
        "category": "security",
        "version": "1.0"
    }).as_object().unwrap().clone(),
};
service.add_document(doc, db_path)?;

// Query the knowledge base
let query = RAGQuery {
    query: "How do I implement secure authentication?".to_string(),
    top_k: Some(5),
    filters: None,
    include_sources: true,
};

let results = service.search(&query, db_path)?;
for item in results.items {
    println!("Score: {:.2} - {}", item.score, item.content);
    if let Some(source) = item.source {
        println!("  Source: {}", source);
    }
}
```

### CLI Usage

```bash
# Query the knowledge base
noa agents run rag "What are best practices for error handling?"

# With parameters
noa agents run rag '{
  "query": "security patterns",
  "top_k": 10,
  "filters": {"category": "security"},
  "include_sources": true
}'

# Generate context-aware prompt
noa agents run rag "How to deploy to Kubernetes?" | noa ask
```

### Building Knowledge Base

```rust
// Index project documentation
for doc_file in glob("docs/**/*.md")? {
    let content = fs::read_to_string(&doc_file)?;
    let doc = Document {
        title: doc_file.file_stem().unwrap().to_string_lossy().to_string(),
        content,
        source: doc_file.to_string_lossy().to_string(),
        metadata: Default::default(),
    };
    service.add_document(doc, db_path)?;
}

// Index code comments
for rs_file in glob("src/**/*.rs")? {
    let content = fs::read_to_string(&rs_file)?;
    // Extract doc comments
    let docs = extract_doc_comments(&content);
    // Add to knowledge base
}
```

### Example Query Session

```bash
$ noa agents run rag "error handling patterns"

Searching knowledge base...
Found 5 relevant results

[1] Score: 0.92 - Error Handling in Rust
  Use Result<T, E> for recoverable errors and panic! for unrecoverable errors.
  The ? operator propagates errors up the call stack.
  Source: docs/rust-patterns.md

[2] Score: 0.85 - Custom Error Types
  Define custom error types with thiserror or implement Error trait manually.
  Source: docs/error-types.md

[3] Score: 0.78 - Error Context
  Use anyhow for application errors with context.
  Use .context() to add meaningful error messages.
  Source: docs/error-context.md

Generate prompt with this context? (y/n): y

Context-enhanced prompt generated. Use with 'noa ask'.
```

---

## 5. Real-World Workflows

### Pre-built Workflows

```bash
# 1. Code Review Workflow
noa workflow code-review --pr 456 --branch main

# 2. Deployment Workflow
noa workflow deploy --env staging --version v2.1.0

# 3. Testing Workflow
noa workflow test --component api --type integration

# 4. Security Audit
noa workflow security-audit --target ./src

# 5. Documentation Generation
noa workflow run documentation '{"target":"api","format":"html"}'
```

### Custom Workflows

```rust
use noa_core::agents::workflows::{WorkflowConfig, WorkflowType};

// Define custom workflow
let config = WorkflowConfig {
    workflow_type: WorkflowType::Custom("ci-pipeline".to_string()),
    parameters: serde_json::json!({
        "stages": ["lint", "test", "build", "deploy"],
        "environment": "staging",
        "notify_on_failure": true
    }),
};

let result = orchestrator.execute_workflow(config)?;
```

### Workflow Composition

```bash
# Chain workflows
noa workflow test --component all && \
noa workflow code-review --pr auto && \
noa workflow deploy --env staging

# Conditional execution
if noa workflow test --component auth; then
  noa workflow deploy --env staging --version auto
else
  echo "Tests failed, skipping deployment"
fi
```

### Example: CI/CD Pipeline

```rust
// Complete CI/CD pipeline as workflow
async fn ci_cd_pipeline(commit_hash: &str) -> Result<()> {
    let mut orchestrator = WorkflowOrchestrator::new();
    
    // Stage 1: Test
    let test_config = workflows::testing("all".to_string(), "full".to_string());
    let test_result = orchestrator.execute_workflow(test_config)?;
    
    if !test_result.success {
        return Err(NoaError::Workflow("Tests failed".into()));
    }
    
    // Stage 2: Security Scan
    let security_config = workflows::security_audit("./src".to_string(), "full".to_string());
    let security_result = orchestrator.execute_workflow(security_config)?;
    
    if !security_result.success {
        return Err(NoaError::Workflow("Security issues found".into()));
    }
    
    // Stage 3: Deploy
    let deploy_config = workflows::deployment("staging".to_string(), commit_hash.to_string());
    let deploy_result = orchestrator.execute_workflow(deploy_config)?;
    
    Ok(())
}
```

---

## Integration Examples

### 1. Automated PR Review + Deployment

```bash
#!/bin/bash
# auto-review-and-deploy.sh

PR_NUMBER=$1
BRANCH=$2

# Step 1: Code review
echo "Running automated code review..."
noa workflow code-review --pr $PR_NUMBER --branch $BRANCH > review.txt

# Step 2: Check if review passed
if grep -q "PASSED" review.txt; then
  echo "Code review passed. Proceeding with deployment..."
  
  # Step 3: Deploy to staging
  noa workflow deploy --env staging --version "pr-$PR_NUMBER"
  
  echo "Deployment complete. Review staging at: https://staging.example.com"
else
  echo "Code review failed. Please address issues before deploying."
  cat review.txt
  exit 1
fi
```

### 2. Knowledge-Enhanced Development

```rust
// Use RAG to enhance code generation
async fn generate_with_context(task: &str, db_path: &Path) -> Result<String> {
    let rag = RAGService::new();
    
    // Retrieve relevant context
    let prompt = rag.generate_prompt(task, 5, db_path)?;
    
    // Use with LLM
    let enhanced_response = llm_generate(&prompt)?;
    
    Ok(enhanced_response)
}

// Example usage
let code = generate_with_context(
    "implement authentication middleware",
    Path::new("./kb.db")
)?;
```

### 3. Multi-Agent DevOps Pipeline

```rust
async fn devops_pipeline(project_path: &Path) -> Result<()> {
    let mut executor = MultiAgentExecutor::new();
    let commander = CommanderChiefAgent::new();
    
    // Create comprehensive plan
    let request = CommanderRequest {
        goal: format!("Complete DevOps pipeline for {:?}", project_path),
        context: Some(hashmap!{
            "path".into() => project_path.to_string_lossy().to_string(),
            "stages".into() => "lint,test,build,security,deploy".into(),
        }),
        constraints: Some(vec!["max_time: 30m".into()]),
    };
    
    let plan = commander.plan_execution(request)?;
    let result = executor.execute_plan(plan)?;
    
    println!("Pipeline result: {}/{} tasks successful",
        result.successful_tasks, result.total_tasks);
    
    Ok(())
}
```

---

## Best Practices

### 1. Code Review Automation
- Run on every PR
- Store results in database
- Track improvement over time
- Customize rules per project

### 2. Deployment Safety
- Always test before deploying
- Use staging environments
- Implement rollback capability
- Monitor post-deployment

### 3. Multi-Agent Coordination
- Keep tasks focused and atomic
- Use commander for complex workflows
- Monitor execution history
- Handle failures gracefully

### 4. Knowledge Base
- Index documentation regularly
- Use semantic search for better results
- Tag documents with metadata
- Keep knowledge base up to date

### 5. Workflow Design
- Compose small workflows
- Make workflows reusable
- Add error handling
- Log all executions

---

## Performance Tips

- **Parallel Execution**: Process independent tasks concurrently
- **Caching**: Cache RAG results for common queries
- **Incremental Analysis**: Only review changed files
- **Resource Limits**: Set timeouts and memory limits
- **Monitoring**: Track execution metrics

---

## Troubleshooting

### Code Review Issues
```bash
# Check agent status
noa agents list

# Test file-io agent
noa agents run file-io '{"op":"exists","path":"./src"}'

# Verify RAG service
noa agents run rag "test query"
```

### Deployment Failures
```bash
# Check logs
noa logs --filter "deployment" --last 100

# Verify configuration
noa agents run file-io '{"op":"read","path":"deploy.yaml"}'

# Test deployment dry-run
noa workflow run deployment '{"environment":"staging","dry_run":true}'
```

### Workflow Debugging
```bash
# View workflow history
noa workflow list --history

# Detailed execution log
noa workflow run test-workflow --verbose

# Step-by-step execution
noa workflow run test-workflow --step-by-step
```

---

## Next Steps

1. **Extend Agents**: Create custom agents for your specific needs
2. **Custom Workflows**: Build domain-specific automation
3. **Integration**: Connect to CI/CD systems, Slack, etc.
4. **ML Enhancement**: Add vector embeddings for better RAG
5. **Monitoring**: Implement metrics and alerting

For more information, see:
- [Agent Implementation Guide](./AGENT-IMPLEMENTATION-SUMMARY.md)
- [Workflow System Documentation](./ADVANCED-FEATURES-FINAL-REPORT.md)
- [API Reference](./docs/api)
