# CLI Reference Documentation

## Overview

This comprehensive reference covers all PRP CLI commands, options, and usage patterns. Each command includes detailed descriptions, options, examples, and exit codes.

## Global Options

These options can be used with any PRP CLI command:

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--help` | `-h` | boolean | false | Show help information |
| `--version` | `-v` | boolean | false | Show CLI version |
| `--debug` | `-d` | boolean | false | Enable debug mode |
| `--config` | `-c` | string | `.prprc` | Specify config file path |
| `--quiet` | `-q` | boolean | false | Suppress output except errors |
| `--verbose` | boolean | false | Enable verbose output |
| `--no-color` | boolean | false | Disable colored output |

## Core Commands

### `init` - Initialize Project

Initialize a new project or upgrade an existing project with PRP configuration.

#### Usage

```bash
prp init [options]
prp init <project-name> [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--template` | `-t` | string | - | Project template to use |
| `--existing` | `-e` | boolean | false | Initialize existing project |
| `--name` | `-n` | string | - | Project name |
| `--description` | `-D` | string | - | Project description |
| `--author` | `-a` | string | - | Project author |
| `--license` | `-l` | string | MIT | Project license |
| `--git` | boolean | true | Initialize Git repository |
| `--install` | boolean | true | Install dependencies |
| `--no-interactive` | boolean | false | Non-interactive mode |
| `--skip-git` | boolean | false | Skip Git initialization |
| `--package-manager` | `-p` | string | auto | Package manager (npm, yarn, pnpm) |

#### Templates

Available project templates:

- `node` - Basic Node.js project
- `node-typescript` - Node.js with TypeScript
- `react` - React application
- `react-typescript` - React with TypeScript
- `vue` - Vue.js application
- `angular` - Angular application
- `express` - Express.js server
- `fastify` - Fastify server
- `nextjs` - Next.js application
- `nuxtjs` - Nuxt.js application
- `python` - Python project
- `django` - Django application
- `fastapi` - FastAPI application
- `go` - Go project
- `rust` - Rust project
- `cli` - CLI application
- `library` - Library project
- `monorepo` - Monorepo setup

#### Examples

```bash
# Interactive new project
prp init

# New project with template
prp init --template node-typescript my-app

# Existing project upgrade
prp init --existing

# Non-interactive with all options
prp init --template react --name my-app --author "John Doe" --no-interactive

# Skip Git and dependency installation
prp init --template node --skip-git --no-install
```

#### Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid template
- `3` - Project already exists
- `4` - Git initialization failed
- `5` - Dependency installation failed

---

### `status` - Show Status

Display project and system status information.

#### Usage

```bash
prp status [options]
prp status [component] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--format` | `-f` | string | table | Output format (table, json, yaml) |
| `--verbose` | `-v` | boolean | false | Show detailed information |
| `--system` | `-s` | boolean | false | Show system status only |
| `--project` | `-p` | boolean | false | Show project status only |
| `--watch` | `-w` | boolean | false | Watch for changes |
| `--no-color` | boolean | false | Disable colored output |

#### Components

| Component | Description |
|-----------|-------------|
| `system` | System information and dependencies |
| `project` | Project configuration and status |
| `git` | Git repository status |
| `dependencies` | Dependencies status |
| `quality` | Quality gates status |
| `ci` | CI/CD pipeline status |
| `debug` | Debug configuration |

#### Examples

```bash
# Show all status
prp status

# Show system information
prp status system

# JSON output
prp status --format json

# Watch for changes
prp status --watch

# Detailed project status
prp status project --verbose
```

#### Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid component
- `3` - Configuration error

---

### `config` - Manage Configuration

Manage PRP CLI configuration and settings.

#### Usage

```bash
prp config [action] [key] [value] [options]
```

#### Actions

| Action | Description |
|--------|-------------|
| `get` | Get configuration value |
| `set` | Set configuration value |
| `list` | List all configuration |
| `reset` | Reset configuration to defaults |
| `edit` | Open configuration in editor |
| `validate` | Validate configuration file |

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--global` | `-g` | boolean | false | Use global configuration |
| `--local` | `-l` | boolean | false | Use local configuration |
| `--format` | `-f` | string | table | Output format (table, json, yaml) |
| `--editor` | `-e` | string | auto | Editor to use |

#### Examples

```bash
# List all configuration
prp config list

# Get specific value
prp config get debug.enabled

# Set configuration value
prp config set debug.level verbose

# Set global configuration
prp config set quality.strict --global

# Reset configuration
prp config reset

# Edit configuration
prp config edit

# Validate configuration
prp config validate
```

#### Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid action
- `3` - Configuration key not found
- `4` - Invalid configuration value
- `5` - Configuration validation failed

---

### `build` - Build Project

Build and compile project artifacts.

#### Usage

```bash
prp build [options]
prp build [target] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--mode` | `-m` | string | production | Build mode (development, production) |
| `--target` | `-t` | string | default | Build target |
| `--watch` | `-w` | boolean | false | Watch for changes |
| `--analyze` | boolean | false | Analyze bundle size |
| `--clean` | boolean | false | Clean build directory |
| `--incremental` | boolean | false | Incremental build |
| `--parallel` | boolean | false | Parallel build |
| `--output` | `-o` | string | dist | Output directory |
| `--sourcemap` | boolean | true | Generate source maps |

#### Targets

| Target | Description |
|--------|-------------|
| `default` | Default build target |
| `development` | Development build |
| `production` | Production build |
| `test` | Test build |
| `analyze` | Build with bundle analysis |
| `storybook` | Storybook build |

#### Examples

```bash
# Production build
prp build --mode production

# Development build with watch
prp build --mode development --watch

# Build specific target
prp build test

# Clean build with analysis
prp build --clean --analyze

# Incremental build
prp build --incremental
```

#### Exit Codes

- `0` - Success
- `1` - General error
- `2` - Build failed
- `3` - Invalid target
- `4` - Configuration error

---

### `test` - Run Tests

Execute test suites and generate coverage reports.

#### Usage

```bash
prp test [options]
prp test [test-pattern] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--type` | `-t` | string | all | Test type (unit, integration, e2e, all) |
| `--coverage` | `-c` | boolean | false | Generate coverage report |
| `--watch` | `-w` | boolean | false | Watch for changes |
| `--verbose` | `-v` | boolean | false | Verbose test output |
| `--reporter` | `-r` | string | default | Test reporter |
| `--timeout` | number | 5000 | Test timeout (ms) |
| `--parallel` | boolean | false | Run tests in parallel |
| `--bail` | boolean | false | Stop on first failure |
| `--update-snapshots` | `-u` | boolean | false | Update snapshots |

#### Test Types

| Type | Description |
|------|-------------|
| `unit` | Unit tests |
| `integration` | Integration tests |
| `e2e` | End-to-end tests |
| `all` | All test types |

#### Examples

```bash
# Run all tests
prp test

# Run unit tests with coverage
prp test --type unit --coverage

# Watch mode
prp test --watch

# Run specific test pattern
prp test "**/*.spec.ts"

# Verbose output with custom reporter
prp test --verbose --reporter spec

# Parallel tests with bail
prp test --parallel --bail
```

#### Exit Codes

- `0` - Success
- `1` - Tests failed
- `2` - No tests found
- `3` - Configuration error
- `4` - Timeout error

---

### `lint` - Lint and Format

Run linting and code formatting checks.

#### Usage

```bash
prp lint [options]
prp lint [path-pattern] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--fix` | boolean | false | Auto-fix issues |
| `--check` | boolean | true | Check only, don't fix |
| `--format` | boolean | true | Run formatter |
| `--type` | `-t` | string | all | Lint type (eslint, prettier, all) |
| `--max-warnings` | number | 0 | Maximum warnings allowed |
| `--quiet` | boolean | false | Suppress warnings |
| `--cache` | boolean | true | Use cache |
| `--ignore-pattern` | string | - | Ignore pattern |

#### Lint Types

| Type | Description |
|------|-------------|
| `eslint` | ESLint checks |
| `prettier` | Prettier formatting |
| `all` | All linting and formatting |

#### Examples

```bash
# Run all linting
prp lint

# Fix issues
prp lint --fix

# ESLint only
prp lint --type eslint

# Check specific directory
prp lint src/

# Allow warnings
prp lint --max-warnings 10

# Quiet mode
prp lint --quiet
```

#### Exit Codes

- `0` - Success
- `1` - Linting errors found
- `2` - Configuration error
- `3` - Maximum warnings exceeded

---

## Development Workflow Commands

### `workflow` - Manage Workflows

Manage development workflows and automation.

#### Usage

```bash
prp workflow [action] [workflow-name] [options]
```

#### Actions

| Action | Description |
|--------|-------------|
| `init` | Initialize workflow |
| `list` | List available workflows |
| `run` | Run workflow |
| `validate` | Validate workflow |
| `status` | Show workflow status |

#### Examples

```bash
# Initialize workflow
prp workflow init

# List workflows
prp workflow list

# Run specific workflow
prp workflow run quality-check

# Validate workflow
prp workflow validate
```

---

### `quality` - Quality Gates

Run quality gates and code analysis.

#### Usage

```bash
prp quality [options]
prp quality [gate-name] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--strict` | boolean | false | Strict quality mode |
| `--report` | `-r` | string | console | Report format (console, json, html) |
| `--fix` | boolean | false | Auto-fix issues |
| `--threshold` | number | 80 | Quality threshold |
| `--exclude` | string | - | Exclude patterns |

#### Examples

```bash
# Run quality gates
prp quality

# Strict mode with HTML report
prp quality --strict --report html

# Fix issues
prp quality --fix
```

---

### `commit` - Smart Commit

Smart commit with validation and automation.

#### Usage

```bash
prp commit [options]
prp commit [message] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--message` | `-m` | string | - | Commit message |
| `--type` | `-t` | string | - | Commit type |
| `--scope` | `-s` | string | - | Commit scope |
| `--skip-hooks` | boolean | false | Skip git hooks |
| `--dry-run` | boolean | false | Show what would be committed |

#### Examples

```bash
# Interactive commit
prp commit

# Commit with message
prp commit --message "Add new feature"

# Skip hooks
prp commit --skip-hooks

# Dry run
prp commit --dry-run
```

---

## Debug Commands

### `debug` - Debug Mode

Enable and configure debug mode.

#### Usage

```bash
prp debug [options]
prp debug [component] [options]
```

#### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--level` | `-l` | string | info | Debug level (error, warn, info, debug, verbose) |
| `--output` | `-o` | string | console | Output format (console, file, json) |
| `--orchestrator` | boolean | false | Enable orchestrator integration |
| `--file` | `-f` | string | - | Output file path |

#### Components

| Component | Description |
|-----------|-------------|
| `node` | Node.js debugging |
| `python` | Python debugging |
| `browser` | Browser debugging |

#### Examples

```bash
# Enable debug mode
prp debug

# Specific debug level
prp debug --level verbose

# Debug with orchestrator
prp debug --orchestrator

# Debug to file
prp debug --file debug.log

# Node.js debugging
prp debug node --port 9229
```

---

## CI/CD Commands

### `ci` - CI/CD Management

Manage CI/CD pipelines and validation.

#### Usage

```bash
prp ci [action] [options]
```

#### Actions

| Action | Description |
|--------|-------------|
| `validate` | Validate CI/CD configuration |
| `run` | Run CI/CD pipeline |
| `status` | Show CI/CD status |
| `logs` | Show CI/CD logs |

#### Examples

```bash
# Validate CI/CD
prp ci validate

# Run pipeline
prp ci run

# Show status
prp ci status

# Show logs
prp ci logs
```

---

## Exit Code Reference

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments or options |
| `3` | Configuration error |
| `4` | Network or external service error |
| `5` | Permission error |
| `6` | Timeout error |
| `7` | Validation failed |
| `8` | Build failed |
| `9` | Test failed |
| `10` | Linting failed |
| `127` | Command not found |
| `130` | Interrupted (Ctrl+C) |

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PRP_DEBUG` | Enable debug mode | `false` |
| `PRP_LOG_LEVEL` | Log level | `info` |
| `PRP_CONFIG_PATH` | Config file path | `.prprc` |
| `PRP_NO_COLOR` | Disable colors | `false` |
| `PRP_EDITOR` | Default editor | `auto` |
| `PRP_PACKAGE_MANAGER` | Package manager | `auto` |
| `PRP_TOKEN_ACCOUNTING` | Token tracking | `true` |
| `PRP_ORCHESTRATOR_URL` | Orchestrator URL | - |
| `PRP_GITHUB_TOKEN` | GitHub token | - |
| `PRP_CACHE_DIR` | Cache directory | `~/.prp/cache` |

## Configuration File Reference

The `.prprc` configuration file supports the following structure:

```json
{
  "name": "project-name",
  "version": "1.0.0",
  "type": "project-type",
  "settings": {
    "debug": {
      "enabled": true,
      "level": "info",
      "output": "console"
    },
    "quality": {
      "strict": false,
      "coverage": 80,
      "preCommitHooks": true
    },
    "ci": {
      "provider": "github",
      "workflows": ["test", "build"]
    },
    "build": {
      "mode": "production",
      "output": "dist",
      "sourcemap": true
    },
    "test": {
      "type": "all",
      "coverage": true,
      "parallel": true
    }
  },
  "scripts": {
    "dev": "prp build --mode development --watch",
    "build": "prp build",
    "test": "prp test",
    "lint": "prp lint",
    "quality": "prp quality"
  }
}
```

## Troubleshooting

### Common Issues

1. **Command not found**: Ensure PRP CLI is installed and in PATH
2. **Permission denied**: Check file permissions and run with appropriate rights
3. **Configuration errors**: Run `prp config validate` to check configuration
4. **Network issues**: Check internet connection and proxy settings
5. **Build failures**: Check dependencies and build configuration

### Debug Information

Get detailed debug information:

```bash
# System information
prp status system --verbose

# Configuration debug
prp config list --verbose

# Debug mode
prp --debug status
```