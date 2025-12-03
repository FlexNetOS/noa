# PRP CLI Documentation

Welcome to the comprehensive documentation for the PRP (Project Requirements & Planning) CLI bootstrap system. This powerful tool transforms project initialization, debugging, and CI/CD management into an efficient, automated experience.

## 🚀 Quick Start

```bash
# Install the CLI
npm install -g @prp/cli

# Initialize a new project
prp init my-project

# Initialize in existing directory
prp init

# Start debug mode
prp debug

# Show help
prp --help
```

## 📚 Documentation Sections

### Core Documentation
- [CLI Reference](./CLI_REFERENCE.md) - Complete command reference and usage guide
- [Configuration Guide](./CONFIGURATION_GUIDE.md) - .prprc configuration and customization
- [Development Workflow](./DEVELOPMENT_WORKFLOW.md) - Complete development lifecycle management
- [CI/CD Integration](./CICD_GUIDE.md) - Continuous integration and deployment setup

### Advanced Topics
- [Debugging Guide](./DEBUGGING_GUIDE.md) - Debug mode and troubleshooting
- [API Reference](./API_REFERENCE.md) - CLI API and programmatic usage
- [Token Accounting](./TOKEN_ACCOUNTING.md) - Cost tracking and management
- [GitHub Integration](./GITHUB_INTEGRATION.md) - GitHub API and automation

### Quality & Testing
- [Quality Gates](./QUALITY_GATES.md) - Automated quality validation
- [Testing Guide](./TESTING_GUIDE.md) - Testing infrastructure and automation
- [Security Guide](./SECURITY_GUIDE.md) - Security best practices

### Development & Contribution
- [Architecture Guide](./ARCHITECTURE.md) - System architecture and design
- [Contributing Guide](./CONTRIBUTING.md) - Development and contribution guidelines
- [Release Process](./RELEASE_PROCESS.md) - Release management and deployment

## 🎯 Key Features

### 🏗️ Project Initialization
- Interactive wizard for new and existing projects
- Template-based project scaffolding
- Automatic dependency management
- Git repository setup and configuration
- Development environment validation

### 🔍 Debug Mode
- CI-like console output with verbose logging
- Debug interface with orchestrator integration (CTRL+D)
- Comprehensive error reporting and troubleshooting
- Performance monitoring and profiling
- Multi-language debugging support (Node.js, Python)

### 🚀 CI/CD Pipeline Management
- Automated workflow validation and setup
- GitHub Actions integration and management
- Build pipeline configuration and optimization
- Test automation and quality enforcement
- Deployment pipeline monitoring

### 💰 Token Accounting
- Real-time token usage tracking
- Cost calculation with provider-specific pricing
- Usage limits and quota management
- Token efficiency optimization
- Comprehensive cost reporting

### 🔧 Quality Gates
- Automated code scanning and analysis
- LLM-based code review integration
- Data preparation for quality assessment
- Decision-making algorithms for quality validation
- E2E self-verification workflows

### 🐙 GitHub Integration
- GitHub SDK integration for API operations
- Pull request creation and management
- Issue tracking and workflow automation
- Repository management and collaboration
- Code review automation

### 📊 Shared Context System
- Cross-PRP context window management
- Status tracking for all active PRPs
- Incident logging and resolution tracking
- Blocker identification and management
- Progress monitoring and reporting

## 🔧 Configuration

The CLI uses a `.prprc` configuration file for customization:

```json
{
  "name": "my-project",
  "type": "typescript",
  "debug": {
    "enabled": true,
    "level": "verbose"
  },
  "cicd": {
    "platform": "github",
    "autoSetup": true
  },
  "quality": {
    "gates": ["eslint", "prettier", "test"],
    "autoFix": true
  }
}
```

## 🎮 Command Structure

```bash
prp <command> [options]

Commands:
  init          Initialize a new or existing project
  debug         Start debug mode with CI-like output
  build         Build and optimize your project
  test          Run tests and quality checks
  deploy        Deploy your application
  status        Show project and PRP status
  config        Manage CLI configuration
  token         Show token usage and costs
```

## 🆘 Getting Help

```bash
# General help
prp --help

# Command-specific help
prp init --help
prp debug --help

# Configuration help
prp config --help

# Show current configuration
prp config show
```

## 🔄 Version Management

```bash
# Check current version
prp --version

# Update to latest version
npm update -g @prp/cli

# Check for updates
prp update check
```

## 📝 Examples

### Initialize a New TypeScript Project
```bash
prp init my-typescript-app --template typescript --with-eslint --with-prettier
```

### Start Debug Mode
```bash
prp debug --level verbose --output-file debug.log
```

### Run Quality Gates
```bash
prp test --quality-gates --fix-issues
```

### Deploy with CI/CD
```bash
prp deploy --environment production --with-tests
```

## 🌟 Best Practices

1. **Always use `prp init`** for new projects to ensure proper setup
2. **Enable debug mode** during development for better visibility
3. **Configure quality gates** to maintain code quality standards
4. **Use token accounting** to monitor AI operation costs
5. **Leverage GitHub integration** for automated workflows
6. **Keep .prprc updated** with project-specific configurations

## 🤝 Community & Support

- [GitHub Repository](https://github.com/prp/cli)
- [Issue Tracker](https://github.com/prp/cli/issues)
- [Discord Community](https://discord.gg/prp)
- [Documentation Website](https://docs.prp.dev)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

---

For detailed information on any topic, please refer to the specific documentation sections listed above.