# Agent CLI Quick Reference

## Commands

### List All Agents
```bash
noa agents list
```

### Show Agent Details
```bash
noa agents info <agent-name>

# Examples:
noa agents info commander-chief
noa agents info file-io
noa agents info terminal
noa agents info rag
```

### Run an Agent
```bash
noa agents run <agent-name> <task>

# Simple string tasks:
noa agents run rag "What is RAG?"
noa agents run commander-chief "Deploy to staging"

# JSON structured tasks:
noa agents run file-io '{"op": "read", "path": "config.yaml"}'
noa agents run terminal '{"command": "cargo", "args": ["build"], "timeout_secs": 300}'
```

## Agent-Specific Task Formats

### File-IO Agent
```json
// Read a file
{"op": "read", "path": "file.txt"}

// Write a file
{"op": "write", "path": "file.txt", "content": "Hello World"}

// Append to file
{"op": "append", "path": "log.txt", "content": "New entry\n"}

// Delete file
{"op": "delete", "path": "temp.txt"}

// List directory
{"op": "list", "path": "./src"}

// Check if exists
{"op": "exists", "path": "file.txt"}

// Create directory
{"op": "mkdir", "path": "new_dir"}

// Copy file
{"op": "copy", "from": "source.txt", "to": "dest.txt"}

// Move file
{"op": "move", "from": "old.txt", "to": "new.txt"}
```

### Terminal Agent
```json
// Basic command
{"command": "cargo", "args": ["build"], "timeout_secs": 60}

// With working directory
{
  "command": "git",
  "args": ["status"],
  "working_dir": "/path/to/repo",
  "timeout_secs": 10
}

// With environment variables
{
  "command": "npm",
  "args": ["test"],
  "env": {"NODE_ENV": "test"},
  "timeout_secs": 300
}
```

### RAG Agent
```json
// Simple query
{"query": "What is authentication?", "top_k": 5}

// With filters
{
  "query": "security best practices",
  "top_k": 10,
  "filters": {"category": "security"},
  "include_sources": true
}
```

### Commander Agent
```json
// Goal with context
{
  "goal": "Analyze codebase for bugs",
  "context": {"repo": "noa"},
  "constraints": ["max_time: 5m"]
}

// Simple goal (plain string also works)
"Deploy application to production"
```

## Example Workflows

### Code Analysis
```bash
# 1. Plan the analysis
noa agents run commander-chief "Analyze TypeScript files for security issues"

# 2. Execute the plan (example)
noa agents run file-io '{"op": "list", "path": "./src"}'
noa agents run rag "security best practices for TypeScript"
noa agents run terminal '{"command": "npm", "args": ["audit"]}'
```

### Automated Deployment
```bash
# 1. Read configuration
noa agents run file-io '{"op": "read", "path": "deploy.yaml"}'

# 2. Build application
noa agents run terminal '{"command": "cargo", "args": ["build", "--release"]}'

# 3. Run tests
noa agents run terminal '{"command": "cargo", "args": ["test"]}'

# 4. Deploy
noa agents run terminal '{"command": "./deploy.sh", "args": ["staging"]}'
```

### Knowledge Base Query
```bash
# Search for documentation
noa agents run rag '{"query": "How to configure database?", "top_k": 5}'

# Get context for a task
noa agents run rag "authentication setup guide"
```

## Tips

1. **Use quotes** for JSON strings with spaces
2. **Escape quotes** in nested JSON: `'{\"op\": \"read\"}'`
3. **Check agent capabilities** with `noa agents info <name>`
4. **Start simple** with plain strings, use JSON for complex operations
5. **Terminal whitelist**: Only safe commands allowed by default

## Troubleshooting

### Command Not Found
```bash
# Check agent name
noa agents list

# Use exact name
noa agents run file-io "task"  # ✓
noa agents run fileio "task"   # ✗
```

### JSON Parse Error
```bash
# Use proper JSON syntax
noa agents run file-io '{"op": "read"}'  # ✓
noa agents run file-io "{op: read}"      # ✗
```

### Permission Denied
```bash
# File-IO: Check file permissions
# Terminal: Command might not be in whitelist
noa agents info terminal  # Check capabilities
```

## Integration with Other Commands

```bash
# Combine with other noa commands
noa agents list && noa modules list

# Use in scripts
#!/bin/bash
result=$(noa agents run file-io '{"op": "read", "path": "config.yaml"}')
echo "Config: $result"
```
