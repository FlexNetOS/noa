# Development Workflow Guide

## Overview

This guide covers comprehensive development workflows using the PRP CLI, from project initialization to deployment and maintenance. It provides best practices, automation strategies, and efficient development patterns for various project types.

## Table of Contents

- [Development Lifecycle](#development-lifecycle)
- [Project Setup](#project-setup)
- [Daily Development](#daily-development)
- [Code Quality](#code-quality)
- [Testing Strategy](#testing-strategy)
- [Debugging and Troubleshooting](#debugging-and-troubleshooting)
- [Collaboration Workflow](#collaboration-workflow)
- [Release Management](#release-management)
- [Maintenance and Monitoring](#maintenance-and-monitoring)

## Development Lifecycle

### Phase Overview

1. **Setup & Configuration** - Project initialization and configuration
2. **Development** - Feature development and iteration
3. **Quality Assurance** - Testing, linting, and validation
4. **Review & Integration** - Code review and integration
5. **Release & Deployment** - Release preparation and deployment
6. **Monitoring & Maintenance** - Post-release monitoring and maintenance

### Workflow Visualization

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Project Setup  │───▶│   Development    │───▶│  Quality Check  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Configuration  │    │   Testing        │    │   Code Review   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CI/CD Setup    │    │   Debugging      │    │   Integration   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Deployment     │    │   Monitoring     │    │   Maintenance   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Project Setup

### New Project Initialization

```bash
# Interactive initialization
prp init

# Template-based initialization
prp init --template node-typescript --name my-project

# Existing project upgrade
prp init --existing

# Configuration-first approach
prp config init --template production
```

### Project Setup Workflow

1. **Choose Template**: Select appropriate project template
2. **Configure Settings**: Set up development and production settings
3. **Initialize Git**: Set up Git repository and initial commit
4. **Install Dependencies**: Install and configure project dependencies
5. **Setup Quality Gates**: Configure linting, testing, and quality gates
6. **Configure CI/CD**: Set up continuous integration and deployment
7. **Initialize Development Environment**: Configure development tools and settings

### Template Selection Guide

| Project Type | Recommended Template | Features |
|--------------|---------------------|----------|
| API/Backend | `node-typescript` | TypeScript, Express/Fastify, testing |
| Frontend SPA | `react-typescript` | React, TypeScript, bundling, testing |
| Full Stack | `nextjs` | Next.js, API routes, SSR, testing |
| CLI Tool | `cli` | Commander.js, testing, packaging |
| Library | `library` | TypeScript, testing, publishing |
| Monorepo | `monorepo` | Lerna/Nx, multiple packages |

### Configuration Setup

```json
{
  "name": "my-project",
  "type": "node-typescript",
  "settings": {
    "development": {
      "port": 3000,
      "hotReload": true,
      "proxy": {
        "/api": "http://localhost:8080"
      }
    },
    "quality": {
      "strict": true,
      "preCommitHooks": true,
      "coverage": 80
    },
    "ci": {
      "provider": "github",
      "workflows": ["test", "build", "deploy"]
    }
  }
}
```

## Daily Development

### Morning Routine

```bash
# Check project status
prp status

# Pull latest changes
git pull origin main

# Install updates
prp deps update

# Run quality check
prp quality
```

### Development Session

1. **Start Development Server**
   ```bash
   prp dev
   # or
   npm run dev
   ```

2. **Make Changes**: Work on features and fixes

3. **Run Quality Checks**
   ```bash
   # Lint and format
   prp lint --fix

   # Run tests
   prp test

   # Quality gates
   prp quality
   ```

4. **Debug if Needed**
   ```bash
   # Enable debug mode
   prp --debug

   # Debug specific issues
   prp debug node --port 9229
   ```

### Feature Development Workflow

```bash
# Create feature branch
git checkout -b feature/new-feature

# Start development
prp dev

# Make changes, test frequently
prp test --watch

# Run quality checks
prp quality

# Commit changes
prp commit

# Push for review
git push origin feature/new-feature
```

### Development Commands Reference

| Command | Description | Usage |
|---------|-------------|-------|
| `prp dev` | Start development server | Daily development |
| `prp build` | Build project | Before testing/deployment |
| `prp test` | Run tests | During development |
| `prp lint` | Lint and format code | Before commits |
| `prp quality` | Run quality gates | Before commits |
| `prp debug` | Enable debug mode | When troubleshooting |

## Code Quality

### Quality Gates Configuration

```json
{
  "settings": {
    "quality": {
      "gates": {
        "lint": {
          "enabled": true,
          "failOnWarnings": false,
          "maxWarnings": 5
        },
        "test": {
          "enabled": true,
          "coverage": {
            "minimum": 80,
            "threshold": 5
          }
        },
        "security": {
          "enabled": true,
          "failOnHigh": true,
          "failOnMedium": false
        }
      },
      "preCommitHooks": true,
      "prePushHooks": true
    }
  }
}
```

### Pre-commit Workflow

1. **Stage Changes**
   ```bash
   git add .
   ```

2. **Pre-commit Hooks Run Automatically**
   - Linting and formatting
   - Basic tests
   - Security checks

3. **Commit**
   ```bash
   prp commit
   ```

### Quality Check Commands

```bash
# Run all quality gates
prp quality

# Run specific gate
prp quality --gate lint

# Fix issues automatically
prp quality --fix

# Strict mode (fails on warnings)
prp quality --strict

# Generate quality report
prp quality --report html
```

### Linting Configuration

```json
{
  "settings": {
    "quality": {
      "gates": {
        "lint": {
          "tools": ["eslint", "prettier"],
          "configFile": ".eslintrc.json",
          "rules": {
            "no-console": "warn",
            "no-unused-vars": "error"
          }
        }
      }
    }
  }
}
```

## Testing Strategy

### Test Types and Coverage

```json
{
  "settings": {
    "test": {
      "type": "all",
      "framework": "jest",
      "coverage": {
        "enabled": true,
        "minimum": 80,
        "reporters": ["text", "lcov", "html"]
      },
      "testMatch": [
        "**/__tests__/**/*.+(ts|tsx|js)",
        "**/*.(test|spec).+(ts|tsx|js)"
      ]
    }
  }
}
```

### Testing Workflow

```bash
# Run all tests
prp test

# Run specific test type
prp test --type unit
prp test --type integration
prp test --type e2e

# Watch mode for development
prp test --watch

# Coverage report
prp test --coverage

# Run tests with debug output
prp test --debug
```

### Test Structure Best Practices

```
src/
├── components/
│   ├── Button.tsx
│   └── __tests__/
│       └── Button.test.tsx
├── services/
│   ├── userService.ts
│   └── __tests__/
│       └── userService.test.ts
└── __tests__/
    ├── setup.ts
    └── helpers.ts
```

### Test Categories

1. **Unit Tests** - Test individual functions/components
2. **Integration Tests** - Test component interactions
3. **E2E Tests** - Test complete user workflows
4. **Performance Tests** - Test performance benchmarks
5. **Security Tests** - Test security vulnerabilities

## Debugging and Troubleshooting

### Debug Mode Setup

```bash
# Enable global debug
prp --debug

# Debug specific command
prp build --debug

# Enable orchestrator integration
prp debug --orchestrator

# Debug to file
prp debug --file debug.log --level verbose
```

### Node.js Debugging

```json
{
  "settings": {
    "debugging": {
      "node": {
        "enabled": true,
        "port": 9229,
        "break": false,
        "sourceMaps": true
      }
    }
  }
}
```

```bash
# Start Node.js with debugging
node --inspect-brk=0.0.0.0:9229 src/index.js

# Debug with PRP
prp debug node --port 9229

# Debug tests
prp test --debug
```

### Python Debugging

```json
{
  "settings": {
    "debugging": {
      "python": {
        "enabled": true,
        "port": 5678,
        "venv": true
      }
    }
  }
}
```

```bash
# Debug Python application
python -m debugpy --listen 5678 --wait-for-client app.py

# Debug with PRP
prp debug python --port 5678
```

### Common Debugging Scenarios

1. **Build Failures**
   ```bash
   prp build --debug --verbose
   prp debug build --analyze
   ```

2. **Test Failures**
   ```bash
   prp test --debug --verbose
   prp debug test --specific test-name
   ```

3. **Performance Issues**
   ```bash
   prp build --analyze
   prp performance --profile
   ```

4. **CI/CD Failures**
   ```bash
   prp ci debug --workflow failed-workflow
   prp ci logs --last --verbose
   ```

### Troubleshooting Commands

```bash
# System health check
prp status --system --verbose

# Configuration validation
prp config validate --verbose

# Dependency issues
prp deps check
prp deps audit

# Cache issues
prp cache clean
prp cache status
```

## Collaboration Workflow

### Branch Strategy

```
main (production)
├── develop (staging)
│   ├── feature/user-auth
│   ├── feature/payment-gateway
│   └── bugfix/login-issue
└── hotfix/critical-security-patch
```

### Pull Request Workflow

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Development and Testing**
   ```bash
   prp dev
   prp test
   prp quality
   ```

3. **Commit Changes**
   ```bash
   prp commit --type feat --scope auth --message "Add user authentication"
   ```

4. **Push and Create PR**
   ```bash
   git push origin feature/new-feature
   # Create PR on GitHub/GitLab
   ```

5. **PR Validation**
   ```bash
   # PR checks run automatically
   prp ci validate --pr
   ```

### Commit Message Standards

```bash
# Conventional commits
prp commit --type feat --scope auth --message "Add JWT authentication"
prp commit --type fix --scope api --message "Fix user creation endpoint"
prp commit --type docs --message "Update API documentation"
prp commit --type test --message "Add unit tests for user service"
prp commit --type refactor --message "Refactor authentication logic"
```

### Code Review Process

1. **Automated Checks**
   - Quality gates pass
   - Tests pass
   - Security scan passes
   - Build succeeds

2. **Manual Review**
   - Code quality and style
   - Architecture and design
   - Performance considerations
   - Security implications

3. **Review Tools**
   ```bash
   # Review checklist
   prp review checklist

   # Generate review report
   prp review report
   ```

## Release Management

### Release Workflow

```bash
# Prepare release
prp release prepare --version 1.2.0

# Run pre-release checks
prp release pre-check

# Create release
prp release create --version 1.2.0 --type minor

# Deploy release
prp deploy --env production --version 1.2.0
```

### Release Types

| Type | Description | Example |
|------|-------------|---------|
| `major` | Breaking changes | 1.0.0 → 2.0.0 |
| `minor` | New features (backward compatible) | 1.0.0 → 1.1.0 |
| `patch` | Bug fixes (backward compatible) | 1.0.0 → 1.0.1 |
| `prerelease` | Pre-release versions | 1.0.0-alpha.1 |

### Release Configuration

```json
{
  "settings": {
    "release": {
      "autoTag": true,
      "changelog": {
        "enabled": true,
        "template": "conventional"
      },
      "git": {
        "commit": true,
        "push": true,
        "tag": true
      },
      "npm": {
        "publish": true,
        "tag": "latest",
        "access": "public"
      }
    }
  }
}
```

### Pre-release Checklist

```bash
# Run all checks
prp release check-all

# Quality gates
prp quality --strict

# Full test suite
prp test --type all

# Security audit
prp security audit

# Performance test
prp performance test

# Documentation build
prp docs build
```

### Deployment Strategies

1. **Continuous Deployment**
   ```bash
   prp deploy --strategy continuous --env production
   ```

2. **Blue-Green Deployment**
   ```bash
   prp deploy --strategy blue-green --env production
   ```

3. **Canary Deployment**
   ```bash
   prp deploy --strategy canary --traffic 10 --env production
   ```

## Maintenance and Monitoring

### Daily Maintenance

```bash
# Check system health
prp status --all

# Update dependencies
prp deps update

# Check security
prp security check

# Monitor performance
prp performance monitor
```

### Weekly Maintenance

```bash
# Full system update
prp update all

# Dependency audit
prp deps audit --fix

# Quality report
prp quality report --week

# Performance analysis
prp performance analyze --period week
```

### Monitoring Setup

```json
{
  "settings": {
    "monitoring": {
      "enabled": true,
      "metrics": {
        "performance": true,
        "errors": true,
        "usage": true
      },
      "alerts": {
        "performance": {
          "threshold": 90,
          "notifications": true
        },
        "errors": {
          "threshold": 5,
          "notifications": true
        }
      },
      "reporting": {
        "frequency": "daily",
        "formats": ["json", "html"]
      }
    }
  }
}
```

### Performance Monitoring

```bash
# Monitor performance
prp performance monitor --live

# Performance report
prp performance report --month

# Benchmark
prp performance benchmark --compare baseline
```

### Security Monitoring

```bash
# Security scan
prp security scan

# Vulnerability check
prp security check --vulnerabilities

# Dependency audit
prp deps audit --security

# Security report
prp security report --month
```

## Automation and Scripts

### Custom Workflow Scripts

```json
{
  "scripts": {
    "daily": "prp status && prp deps update && prp security check",
    "pre-commit": "prp lint && prp test && prp quality",
    "pre-release": "prp test --all && prp quality --strict && prp security audit",
    "deploy-staging": "prp build && prp test && prp deploy --env staging",
    "deploy-prod": "prp release check-all && prp deploy --env production"
  }
}
```

### Workflow Automation

```bash
# Create custom workflow
prp workflow create --name daily-checkup

# Configure workflow
prp workflow config --name daily-checkup --set schedule="0 9 * * 1-5"

# Run workflow
prp workflow run daily-checkup

# Schedule workflow
prp workflow schedule daily-checkup --cron "0 9 * * 1-5"
```

### Git Hooks Integration

```bash
# Install git hooks
prp hooks install

# Configure pre-commit hook
prp hooks set pre-commit "prp lint && prp test"

# Configure pre-push hook
prp hooks set pre-push "prp quality && prp security check"

# Remove hooks
prp hooks uninstall
```

## Team Collaboration

### Team Configuration

```json
{
  "team": {
    "members": [
      {
        "name": "John Doe",
        "email": "john@example.com",
        "role": "developer",
        "permissions": ["code", "review"]
      }
    ],
    "workflows": {
      "codeReview": {
        "requiredReviewers": 2,
        "autoAssign": true
      },
      "deployment": {
        "approvalRequired": true,
        "approvers": ["@dev-team", "@qa-team"]
      }
    }
  }
}
```

### Collaboration Tools

```bash
# Team status
prp team status

# Assign review
prp review assign --reviewer @john --pr #123

# Check team workload
prp team workload

# Generate team report
prp team report --week
```

## Best Practices

### Development Best Practices

1. **Small, Frequent Commits**: Commit often with clear messages
2. **Test-Driven Development**: Write tests before implementation
3. **Code Reviews**: Always review code before merging
4. **Documentation**: Keep documentation updated with code
5. **Quality Gates**: Use automated quality checks

### Workflow Best Practices

1. **Automate Everything**: Automate repetitive tasks
2. **Consistent Environment**: Use consistent development environments
3. **Version Control**: Use proper Git workflow
4. **Security First**: Include security in all stages
5. **Performance Monitoring**: Monitor performance continuously

### Team Best Practices

1. **Clear Communication**: Use clear commit messages and PR descriptions
2. **Defined Processes**: Have clear development and release processes
3. **Knowledge Sharing**: Share knowledge through documentation and reviews
4. **Continuous Improvement**: Regularly review and improve workflows
5. **Tool Standardization**: Use consistent tools and configurations

## Troubleshooting Common Issues

### Build Issues

```bash
# Debug build
prp build --debug --verbose

# Clean build
prp build --clean

# Check dependencies
prp deps check

# Rebuild from scratch
prp clean && prp build
```

### Test Issues

```bash
# Debug tests
prp test --debug

# Run specific test
prp test --test-name "specific test"

# Update snapshots
prp test --update-snapshots

# Run tests without cache
prp test --no-cache
```

### Quality Gate Issues

```bash
# Check quality configuration
prp quality --debug

# Fix issues automatically
prp quality --fix

# Check specific gate
prp quality --gate lint

# Bypass specific gate (not recommended)
prp quality --bypass lint
```

### CI/CD Issues

```bash
# Debug CI/CD
prp ci debug --all

# Validate configuration
prp ci validate --verbose

# Check logs
prp ci logs --last

# Run locally
prp ci run --local --debug
```

## Integration Examples

### VS Code Integration

```json
{
  "tasks": {
    "version": "2.0.0",
    "tasks": [
      {
        "label": "PRP: Development",
        "type": "shell",
        "command": "prp",
        "args": ["dev"],
        "group": "build"
      },
      {
        "label": "PRP: Test",
        "type": "shell",
        "command": "prp",
        "args": ["test"],
        "group": "test"
      },
      {
        "label": "PRP: Quality",
        "type": "shell",
        "command": "prp",
        "args": ["quality"],
        "group": "test"
      }
    ]
  }
}
```

### GitHub Actions Integration

```yaml
name: PRP Workflow
on: [push, pull_request]

jobs:
  prp:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup PRP
        run: npm install -g @prp/cli
      - name: Quality Check
        run: prp quality --strict
      - name: Test
        run: prp test --coverage
      - name: Build
        run: prp build
```

This comprehensive development workflow guide provides detailed instructions for efficient development using PRP CLI, covering all aspects from project setup to maintenance and monitoring.