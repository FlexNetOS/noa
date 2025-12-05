# PRP CLI Documentation

## Overview

The PRP CLI is a comprehensive command-line interface for project bootstrap, development workflow automation, debugging, and CI/CD pipeline management. It provides complete infrastructure for scaffolding projects, managing development workflows, handling debugging scenarios, and ensuring quality through automated validation and testing.

## Table of Contents

- [Installation](#installation)
- [Getting Started](#getting-started)
- [Commands](#commands)
- [Configuration](#configuration)
- [Debug Mode](#debug-mode)
- [CI/CD Integration](#cicd-integration)
- [Development Workflow](#development-workflow)
- [Troubleshooting](#troubleshooting)

## Installation

```bash
# Install via npm
npm install -g @prp/cli

# Install via yarn
yarn global add @prp/cli

# Install via pnpm
pnpm add -g @prp/cli
```

## Getting Started

### Initialize a New Project

```bash
# Interactive wizard for new project
prp init

# Initialize with specific template
prp init --template node-typescript

# Initialize existing project
prp init --existing

# Non-interactive mode
prp init --template react --name my-project --no-interactive
```

### Basic Commands

```bash
# Show help
prp --help

# Show version
prp --version

# Check system status
prp status

# Enable debug mode
prp --debug
```

## Commands

### Core Commands

| Command | Description | Options |
|---------|-------------|---------|
| `init` | Initialize new or existing project | `--template`, `--name`, `--existing` |
| `status` | Show project and system status | `--verbose`, `--format` |
| `debug` | Enable debug mode | `--level`, `--output`, `--orchestrator` |
| `config` | Manage configuration | `--set`, `--get`, `--list` |
| `build` | Build project | `--mode`, `--target`, `--watch` |
| `test` | Run tests | `--type`, `--coverage`, `--watch` |
| `lint` | Run linting and formatting | `--fix`, `--check` |

### Development Workflow Commands

| Command | Description | Options |
|---------|-------------|---------|
| `workflow` | Manage development workflows | `--init`, `--validate`, `--status` |
| `quality` | Run quality gates | `--strict`, `--report`, `--fix` |
| `commit` | Smart commit with validation | `--message`, `--type`, `--skip-hooks` |
| `release` | Manage releases | `--version`, `--type`, `--skip-tests` |

### CI/CD Commands

| Command | Description | Options |
|---------|-------------|---------|
| `ci` | CI/CD pipeline management | `--validate`, `--run`, `--status` |
| `pipeline` | Manage CI/CD pipelines | `--create`, `--update`, `--test` |
| `deploy` | Deploy applications | `--env`, `--target`, `--dry-run` |

### Debug Commands

| Command | Description | Options |
|---------|-------------|---------|
| `debug node` | Node.js debugging | `--port`, `--inspect`, `--break` |
| `debug python` | Python debugging | `--venv`, `--port`, `--args` |
| `debug browser` | Browser debugging | `--port`, `--headless`, `--devtools` |

## Configuration

### .prprc Configuration File

The `.prprc` file is the main configuration file for PRP CLI. It should be placed in the root of your project.

```json
{
  "name": "my-project",
  "type": "node-typescript",
  "version": "1.0.0",
  "settings": {
    "debug": {
      "enabled": true,
      "level": "info",
      "output": "console"
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

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PRP_DEBUG` | Enable debug mode | `false` |
| `PRP_LOG_LEVEL` | Log level (error, warn, info, debug) | `info` |
| `PRP_CONFIG_PATH` | Custom config file path | `.prprc` |
| `PRP_TOKEN_ACCOUNTING` | Enable token tracking | `true` |

## Debug Mode

Debug mode provides comprehensive logging and CI-like console output for troubleshooting and development.

### Enable Debug Mode

```bash
# Global debug mode
prp --debug

# Specific debug level
prp --debug --level verbose

# Debug with orchestrator integration
prp --debug --orchestrator

# Debug specific command
prp build --debug
```

### Debug Features

- **CI-like Output**: Structured logging with timestamps and levels
- **Orchestrator Integration**: Send debug data to orchestrator (CTRL+D toggle)
- **Performance Monitoring**: Track execution times and resource usage
- **Error Reporting**: Detailed error analysis and suggestions
- **Verbose Logging**: Comprehensive logging for all operations

## CI/CD Integration

PRP CLI provides seamless integration with CI/CD pipelines and automated workflows.

### GitHub Actions Integration

```bash
# Generate GitHub Actions workflow
prp pipeline create --provider github --name test-build-deploy

# Validate existing pipeline
prp ci validate --provider github

# Run pipeline locally
prp ci run --local --workflow test
```

### Quality Gates

```bash
# Run quality gates
prp quality --strict --report json

# Setup quality gates
prp quality init --standards eslint,prettier,jest

# Fix quality issues
prp quality --fix
```

## Development Workflow

### Initialization Workflow

1. **Project Setup**: Run `prp init` to initialize new or existing projects
2. **Configuration**: Configure `.prprc` file with project settings
3. **Dependencies**: Install and configure project dependencies
4. **Quality Setup**: Initialize quality gates and pre-commit hooks
5. **CI/CD Setup**: Configure CI/CD pipelines and workflows

### Daily Development Workflow

1. **Status Check**: Run `prp status` to check project health
2. **Development**: Make changes to code
3. **Quality Check**: Run `prp quality` to validate code quality
4. **Testing**: Run `prp test` to execute tests
5. **Commit**: Use `prp commit` for smart commits with validation
6. **Debug**: Use `prp --debug` for troubleshooting issues

## Troubleshooting

### Common Issues

#### Initialization Problems

```bash
# Check system requirements
prp status --system

# Reinitialize configuration
prp config reset

# Debug initialization
prp init --debug --verbose
```

#### Debug Mode Issues

```bash
# Check debug configuration
prp config get debug

# Test debug output
prp debug test --level verbose

# Reset debug settings
prp config set debug.enabled false
```

#### CI/CD Pipeline Issues

```bash
# Validate pipeline configuration
prp ci validate --verbose

# Test pipeline locally
prp ci test --local

# Check pipeline logs
prp ci logs --workflow test
```

### Getting Help

```bash
# General help
prp --help

# Command-specific help
prp init --help

# Debug help
prp debug --help

# Configuration help
prp config --help
```

### Community Support

- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check comprehensive guides and API docs
- **Discord Community**: Join community discussions and support
- **Examples**: Browse example projects and configurations

## Next Steps

- Read [CLI Reference](./cli-reference.md) for detailed command documentation
- Check [CI/CD Guide](../ci-cd/README.md) for pipeline setup
- Review [Configuration Guide](../config/README.md) for advanced configuration
- Explore [API Documentation](../api/README.md) for programmatic usage