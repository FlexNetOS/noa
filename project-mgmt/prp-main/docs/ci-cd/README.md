# CI/CD Pipeline Guide

## Overview

The PRP CLI provides comprehensive CI/CD pipeline management, validation, and automation. This guide covers setting up, configuring, and managing CI/CD workflows for various platforms and project types.

## Table of Contents

- [Supported CI/CD Platforms](#supported-cicd-platforms)
- [Quick Start](#quick-start)
- [Pipeline Configuration](#pipeline-configuration)
- [Workflow Templates](#workflow-templates)
- [Quality Gates](#quality-gates)
- [Deployment Strategies](#deployment-strategies)
- [Monitoring and Logging](#monitoring-and-logging)
- [Troubleshooting](#troubleshooting)

## Supported CI/CD Platforms

| Platform | Support Level | Features |
|----------|---------------|----------|
| GitHub Actions | ✅ Full | Complete workflow generation and management |
| GitLab CI | ✅ Full | Pipeline configuration and validation |
| CircleCI | 🚧 Beta | Basic pipeline support |
| Jenkins | 🚧 Beta | Pipeline as Code support |
| Azure DevOps | 🚧 Beta | YAML pipeline support |
| Bitbucket Pipelines | 🚧 Beta | Basic configuration support |

## Quick Start

### Initialize CI/CD for Your Project

```bash
# Initialize with default GitHub Actions
prp pipeline init

# Choose specific platform
prp pipeline init --provider github

# Interactive setup
prp pipeline init --interactive
```

### Validate Existing Pipeline

```bash
# Validate current configuration
prp ci validate

# Validate with detailed output
prp ci validate --verbose

# Check pipeline status
prp ci status
```

### Run Pipeline Locally

```bash
# Run full pipeline
prp ci run --local

# Run specific workflow
prp ci run --workflow test --local

# Run with debug output
prp ci run --local --debug
```

## Pipeline Configuration

### Basic Configuration

Create a `.prprc` file with CI/CD settings:

```json
{
  "name": "my-project",
  "ci": {
    "provider": "github",
    "workflows": {
      "test": {
        "enabled": true,
        "triggers": ["push", "pull_request"],
        "nodeVersions": ["16", "18", "20"],
        "coverage": true
      },
      "build": {
        "enabled": true,
        "triggers": ["push"],
        "artifacts": true
      },
      "deploy": {
        "enabled": true,
        "triggers": ["push", "release"],
        "environments": ["staging", "production"]
      }
    },
    "quality": {
      "gates": ["lint", "test", "security"],
      "thresholds": {
        "coverage": 80,
        "complexity": 10
      }
    }
  }
}
```

### Advanced Configuration

```json
{
  "ci": {
    "provider": "github",
    "settings": {
      "cache": {
        "enabled": true,
        "paths": ["node_modules", ".npm"]
      },
      "secrets": {
        "required": ["GITHUB_TOKEN", "NPM_TOKEN"],
        "optional": ["SLACK_WEBHOOK"]
      },
      "notifications": {
        "slack": {
          "webhook": "${SLACK_WEBHOOK}",
          "channels": ["#ci-cd", "#dev-team"]
        },
        "email": {
          "recipients": ["team@example.com"]
        }
      },
      "parallelism": {
        "testMatrix": {
          "node": [16, 18, 20],
          "os": ["ubuntu-latest", "windows-latest"]
        }
      }
    }
  }
}
```

## Workflow Templates

### Node.js Application

```bash
# Generate Node.js workflow
prp pipeline create --template nodejs --name ci-cd
```

**Generated GitHub Actions Workflow:**

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16, 18, 20]

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run linting
        run: npm run lint

      - name: Run tests
        run: npm run test:coverage

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          file: ./coverage/lcov.info

  build:
    name: Build
    runs-on: ubuntu-latest
    needs: test

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Build application
        run: npm run build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: build-artifacts
          path: dist/

  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'

    steps:
      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          name: build-artifacts

      - name: Deploy to production
        run: echo "Deploying to production..."
```

### React Application

```bash
# Generate React workflow
prp pipeline create --template react --name react-ci-cd
```

### Python Application

```bash
# Generate Python workflow
prp pipeline create --template python --name python-ci-cd
```

### Go Application

```bash
# Generate Go workflow
prp pipeline create --template go --name go-ci-cd
```

## Quality Gates

### Configure Quality Gates

```json
{
  "ci": {
    "quality": {
      "enabled": true,
      "gates": {
        "lint": {
          "enabled": true,
          "threshold": 0,
          "failOnWarnings": false
        },
        "test": {
          "enabled": true,
          "coverage": {
            "minimum": 80,
            "threshold": 5
          },
          "failures": {
            "maximum": 0
          }
        },
        "security": {
          "enabled": true,
          "tools": ["npm-audit", "snyk"],
          "failOnHigh": true,
          "failOnMedium": false
        },
        "performance": {
          "enabled": true,
          "budget": {
            "size": "500KB",
            "loadTime": "3s"
          }
        }
      },
      "reporting": {
        "formats": ["console", "json", "html"],
        "artifacts": true
      }
    }
  }
}
```

### Quality Gate Commands

```bash
# Run quality gates
prp quality --strict

# Generate quality report
prp quality --report html --output quality-report.html

# Fix quality issues
prp quality --fix

# Check specific gate
prp quality --gate lint

# Set quality thresholds
prp config set ci.quality.gates.test.coverage.minimum 85
```

### Custom Quality Gates

Create custom quality gate configurations:

```json
{
  "ci": {
    "quality": {
      "customGates": {
        "complexity": {
          "enabled": true,
          "tool": "complexity-report",
          "threshold": 10,
          "failOnExceed": true
        },
        "duplication": {
          "enabled": true,
          "tool": "jscpd",
          "threshold": 5,
          "failOnExceed": true
        },
        "maintainability": {
          "enabled": true,
          "tool": "sonarqube",
          "threshold": "B",
          "failBelowGrade": true
        }
      }
    }
  }
}
```

## Deployment Strategies

### Continuous Deployment

```bash
# Setup continuous deployment
prp pipeline deploy --strategy continuous

# Configure environments
prp pipeline env add staging --url https://staging.example.com
prp pipeline env add production --url https://example.com
```

### Blue-Green Deployment

```bash
# Setup blue-green deployment
prp pipeline deploy --strategy blue-green

# Configure blue-green settings
prp config set ci.deployment.blueGreen.enabled true
prp config set ci.deployment.blueGreen.healthCheck /health
prp config set ci.deployment.blueGreen.switchDelay 30
```

### Canary Deployment

```bash
# Setup canary deployment
prp pipeline deploy --strategy canary

# Configure canary settings
prp config set ci.deployment.canary.traffic 10
prp config set ci.deployment.canary.duration 300
```

### Environment Configuration

```json
{
  "ci": {
    "deployment": {
      "environments": {
        "staging": {
          "url": "https://staging.example.com",
          "autoDeploy": true,
          "requiresApproval": false
        },
        "production": {
          "url": "https://example.com",
          "autoDeploy": false,
          "requiresApproval": true,
          "approversers": ["@dev-team", "@qa-team"]
        }
      },
      "strategies": {
        "continuous": {
          "enabled": true,
          "environments": ["staging"]
        },
        "blueGreen": {
          "enabled": false,
          "healthCheck": "/health",
          "switchDelay": 30
        },
        "canary": {
          "enabled": false,
          "traffic": 10,
          "duration": 300
        }
      }
    }
  }
}
```

## Monitoring and Logging

### Pipeline Monitoring

```bash
# Monitor pipeline status
prp ci monitor --live

# Check pipeline history
prp ci history --limit 10

# Monitor specific workflow
prp ci monitor --workflow test
```

### Log Management

```bash
# View pipeline logs
prp ci logs --workflow test --run 123

# Stream logs in real-time
prp ci logs --follow --workflow build

# Export logs
prp ci logs --export logs.json --format json
```

### Performance Monitoring

```bash
# Monitor pipeline performance
prp ci performance --report

# Analyze build times
prp ci performance --analyze --workflow build

# Performance alerts
prp ci monitor --alerts performance
```

### Notification Configuration

```json
{
  "ci": {
    "notifications": {
      "slack": {
        "enabled": true,
        "webhook": "${SLACK_WEBHOOK}",
        "channels": {
          "success": "#ci-success",
          "failure": "#ci-failures",
          "deployments": "#deployments"
        },
        "events": {
          "onSuccess": true,
          "onFailure": true,
          "onDeployment": true
        }
      },
      "email": {
        "enabled": true,
        "recipients": ["team@example.com"],
        "events": {
          "onFailure": true,
          "onDeployment": true
        }
      },
      "github": {
        "enabled": true,
        "statusChecks": true,
        "comments": {
          "onFailure": true,
          "onDeployment": true
        }
      }
    }
  }
}
```

## Security and Compliance

### Security Scanning

```bash
# Run security scan
prp security scan

# Configure security tools
prp security config --tools npm-audit,snyk,semgrep

# Generate security report
prp security report --format html
```

### Compliance Checks

```bash
# Run compliance checks
prp compliance check --standard soc2

# Generate compliance report
prp compliance report --standard gdpr
```

### Secret Management

```bash
# Scan for secrets
prp security scan-secrets

# Configure secret scanning
prp security config --secret-scan true
```

## Advanced Features

### Parallel Execution

```json
{
  "ci": {
    "parallel": {
      "enabled": true,
      "maxJobs": 4,
      "strategies": {
        "test": {
          "matrix": {
            "node": [16, 18, 20],
            "os": ["ubuntu-latest", "windows-latest"]
          }
        }
      }
    }
  }
}
```

### Caching Strategy

```json
{
  "ci": {
    "cache": {
      "enabled": true,
      "strategies": {
        "node_modules": {
          "key": "npm-${{ hashFiles('package-lock.json') }}",
          "paths": ["~/.npm"]
        },
        "build": {
          "key": "build-${{ github.sha }}",
          "paths": [".next/cache"]
        }
      }
    }
  }
}
```

### Artifact Management

```bash
# List artifacts
prp ci artifacts list

# Download artifacts
prp ci artifacts download --name build-artifacts

# Upload custom artifacts
prp ci artifacts upload --path ./dist --name dist-files
```

## Troubleshooting

### Common Issues

#### Pipeline Validation Errors

```bash
# Validate configuration
prp ci validate --verbose

# Check syntax
prp ci validate --syntax-only

# Debug configuration
prp ci validate --debug
```

#### Build Failures

```bash
# Debug build failures
prp ci debug --workflow build

# Check logs
prp ci logs --workflow build --last

# Re-run failed workflow
prp ci run --workflow build --rerun
```

#### Permission Issues

```bash
# Check permissions
prp ci check-permissions

# Setup required permissions
prp ci setup-permissions
```

#### Resource Limits

```bash
# Check resource usage
prp ci resources --report

# Optimize resource usage
prp ci optimize --resources
```

### Debug Commands

```bash
# Debug full pipeline
prp ci debug --all

# Debug specific workflow
prp ci debug --workflow test

# Debug with verbose output
prp ci debug --verbose

# Generate debug report
prp ci debug --report debug-report.json
```

### Health Checks

```bash
# Run health checks
prp ci health-check

# Check specific components
prp ci health-check --component github

# Monitor health
prp ci health-check --monitor
```

## Best Practices

### Pipeline Design

1. **Fast Feedback**: Keep test workflows fast and parallel
2. **Clear Naming**: Use descriptive names for workflows and jobs
3. **Fail Fast**: Configure quality gates to fail early
4. **Resource Optimization**: Use caching and parallel execution
5. **Security**: Include security scanning in all pipelines

### Configuration Management

1. **Environment Variables**: Use secrets for sensitive data
2. **Version Control**: Store pipeline configuration in repository
3. **Documentation**: Document pipeline configuration and decisions
4. **Validation**: Validate configuration before deployment

### Monitoring and Alerting

1. **Comprehensive Logging**: Log all pipeline activities
2. **Performance Metrics**: Monitor build times and resource usage
3. **Alerting**: Configure alerts for failures and performance issues
4. **Reporting**: Generate regular reports on pipeline performance

## Integration Examples

### GitHub + Slack Integration

```json
{
  "ci": {
    "provider": "github",
    "integrations": {
      "slack": {
        "webhook": "${SLACK_WEBHOOK}",
        "notifications": {
          "onSuccess": {
            "channel": "#ci-success",
            "message": "✅ Pipeline succeeded for ${PROJECT_NAME}"
          },
          "onFailure": {
            "channel": "#ci-failures",
            "message": "❌ Pipeline failed for ${PROJECT_NAME}"
          },
          "onDeployment": {
            "channel": "#deployments",
            "message": "🚀 Deployed ${PROJECT_NAME} to ${ENVIRONMENT}"
          }
        }
      }
    }
  }
}
```

### GitHub + Email Integration

```json
{
  "ci": {
    "provider": "github",
    "integrations": {
      "email": {
        "enabled": true,
        "smtp": {
          "host": "${SMTP_HOST}",
          "port": 587,
          "user": "${SMTP_USER}",
          "password": "${SMTP_PASSWORD}"
        },
        "recipients": ["team@example.com"],
        "notifications": {
          "onFailure": true,
          "onDeployment": true
        }
      }
    }
  }
}
```

## Migration Guide

### Migrating from Existing CI/CD

```bash
# Import existing configuration
prp pipeline import --provider github --from .github/workflows

# Validate imported configuration
prp ci validate

# Update and optimize
prp pipeline optimize
```

### Upgrading Pipeline Configuration

```bash
# Check for updates
prp pipeline check-updates

# Update configuration
prp pipeline update

# Validate updated configuration
prp ci validate
```

## API Reference

### CLI Commands

- `prp pipeline init` - Initialize CI/CD pipeline
- `prp pipeline create` - Create new workflow
- `prp pipeline validate` - Validate pipeline configuration
- `prp ci run` - Run pipeline locally
- `prp ci status` - Check pipeline status
- `prp ci logs` - View pipeline logs
- `prp ci monitor` - Monitor pipeline execution

### Configuration Options

See [Configuration Reference](../config/README.md) for detailed configuration options.

## Getting Help

- **Documentation**: Full documentation at `/docs/ci-cd`
- **Command Help**: `prp pipeline --help` or `prp ci --help`
- **Community**: Join our Discord community
- **Issues**: Report bugs on GitHub
- **Examples**: Browse example configurations in `/examples/ci-cd`