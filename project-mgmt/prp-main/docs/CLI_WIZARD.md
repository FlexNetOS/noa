# CLI Wizard Documentation

## Overview

The CLI Wizard is an interactive project initialization system for PRP CLI that provides a seamless way to create new projects with AI agent configuration, template selection, and automated setup.

## Features

### 🎯 Core Functionality
- **Interactive Wizard Mode**: Step-by-step project configuration with prompts
- **Default Mode**: Quick initialization with sensible defaults
- **Template System**: Multiple project templates (fast, minimal, all, landing-page)
- **Agent Configuration**: AI agent selection and customization
- **PRP Integration**: Product Requirement Prompt definition and management

### 🎵 Special Features
- **Dancing Monkeys Detection**: Automatically detects PRP for landing pages with animated monkeys
- **Multi-Provider Support**: Configuration for various AI providers (Anthropic, OpenAI, GLM)
- **CI Mode Support**: Non-interactive operation for CI/CD environments
- **Security Restrictions**: Blocks init command in CI mode for security

## Usage

### Basic Usage

```bash
# Interactive mode with wizard
prp init my-project

# Default mode with quick setup
prp init my-project --default

# Specify template
prp init my-project --template landing-page

# Define PRP inline
prp init my-project --prp "Build a modern web application with React and TypeScript"

# Special dancing monkeys deployment
prp init my-project --default --prp "Deliver gh-page with animated dancing monkeys spawn around"
```

### Advanced Options

```bash
# Specify custom agents
prp init my-project --agents "robo-developer,robo-ux-ui-designer,robo-aqa"

# Skip authentication setup
prp init my-project --skip-auth

# CI mode (non-interactive)
prp init my-project --ci --template minimal

# Verbose logging
prp init my-project --verbose
```

## Templates

### Fast Template (Default)
- Essential PRP CLI files
- Basic project structure
- Agent configuration (robo-developer, robo-aqa)
- Ready for immediate development

### Minimal Template
- Bare essentials only
- Basic package.json
- AGENTS.md (required)
- .gitignore
- Ready for custom setup

### All Template
- Complete setup with all features
- Full project structure
- Comprehensive configuration
- All development tools included
- CI/CD pipeline setup
- Documentation and examples

### Landing Page Template
- Static HTML landing page
- Responsive CSS design
- Animated dancing monkeys (optional)
- GitHub Pages deployment ready
- SEO optimization

## Agent Configuration

### Available Agents

1. **robo-developer** - Software development and implementation
2. **robo-aqa** - Quality assurance and testing
3. **robo-ux-ui-designer** - User experience and interface design
4. **robo-system-analyst** - System analysis and requirements
5. **robo-devops-sre** - DevOps and site reliability
6. **robo-orchestrator** - Project orchestration and coordination

### Agent Customization

Each agent can be configured with:
- **Model Selection**: Choose AI model (GPT-4, Claude, Gemini, etc.)
- **Token Limits**: Set maximum token usage
- **Capabilities**: Enable/disable specific capabilities
- **Tools**: Configure available tools and integrations

## Wizard Flow

### Step 1: Welcome
- Display PRP CLI banner
- Show helpful tips
- Present overview

### Step 2: Project Name
- Validate project name format
- Check for existing directories
- Suggest defaults

### Step 3: Template Selection
- Present available templates
- Show template features
- Allow preview of template contents

### Step 4: PRP Definition
- Interactive PRP editor
- Validation and guidance
- Templates and examples

### Step 5: Agent Configuration
- Select agents for project
- Customize agent settings
- Review capabilities

### Step 6: Additional Options
- Git initialization
- Dependency installation
- GitHub repository creation
- CI/CD pipeline setup

### Step 7: Confirmation
- Review configuration summary
- Confirm or modify settings
- Proceed with setup

## Special Features

### Dancing Monkeys Detection

The wizard automatically detects PRP requests for landing pages with animated dancing monkeys:

```bash
# These PRPs will trigger landing-page template with dancing monkeys
prp init my-project --prp "Deliver gh-page with animated dancing monkeys"
prp init my-project --prp "Create landing page with dancing monkeys"
prp init my-project --prp "gh-page with animated monkeys spawn around"
```

### CI Mode Security

For security reasons, the init command is blocked in CI mode:
- Detects `CI_MODE=true` environment variable
- Prevents interactive authentication setup
- Provides alternative workflow guidance
- Maintains security boundaries

### Progress Tracking

The wizard provides real-time progress updates:
- Step-by-step progress indicators
- Error handling and recovery
- State persistence and recovery
- Event emission for integration

## Configuration Files

### .prprc
Main project configuration file:
```json
{
  "version": "1.0.0",
  "projectId": "unique-project-id",
  "agents": {
    "enabled": ["robo-developer", "robo-aqa"],
    "configurations": {
      "robo-developer": {
        "model": "gpt-4",
        "maxTokens": 8000
      }
    }
  },
  "templates": {
    "default": "fast"
  },
  "features": {
    "git": true,
    "npm": true,
    "testing": true
  }
}
```

### AGENTS.md
Agent workflow and signal configuration:
```markdown
# My Project - Agent Guidelines

## Available Agents
- robo-developer: Software development
- robo-aqa: Quality assurance

## Signal Workflow
- [Tt]: Test verification
- [Qb]: Quality bugs
- [Cc]: Completion
```

## Error Handling

### Common Errors

1. **Invalid Template**: Template not found in registry
2. **Project Name Exists**: Directory already exists
3. **Authentication Failed**: Provider setup issues
4. **Network Issues**: Template download failures
5. **Permission Errors**: File system access issues

### Recovery Strategies

- **Retry Mechanism**: Automatic retry for transient failures
- **Rollback**: Cleanup on partial failures
- **Guidance**: Clear error messages and next steps
- **Logging**: Detailed logs for troubleshooting

## Integration Points

### CLI Integration
- Registered as `prp init` command
- Integrates with existing CLI structure
- Shares configuration and authentication

### Template System
- Extensible template registry
- Custom template support
- Template versioning and updates

### Agent System
- Dynamic agent discovery
- Capability-based selection
- Runtime configuration

## Testing

### Unit Tests
- Individual component testing
- Template rendering validation
- Configuration validation

### Integration Tests
- End-to-end wizard flow
- File system operations
- CLI command integration

### Manual Testing
- Interactive mode testing
- Error scenario testing
- User experience validation

## Future Enhancements

### Planned Features
- **Template Gallery**: Visual template browser
- **Project Templates**: Community-contributed templates
- **Advanced Configuration**: More granular agent settings
- **Team Collaboration**: Shared project configurations
- **Cloud Integration**: Remote template storage

### Technical Improvements
- **Performance**: Faster template processing
- **Caching**: Template and dependency caching
- **Validation**: Enhanced input validation
- **Accessibility**: Improved screen reader support
- **Internationalization**: Multi-language support

## Contributing

### Adding New Templates
1. Create template definition in `src/commands/template-manager.ts`
2. Add template files and configuration
3. Write tests for new template
4. Update documentation

### Adding New Agents
1. Define agent configuration in `src/commands/agent-configurator.ts`
2. Specify capabilities and tools
3. Add agent validation logic
4. Update wizard prompts

### Extending Wizard Flow
1. Modify wizard steps in `src/commands/wizard.ts`
2. Add new prompts and validation
3. Update state management
4. Write integration tests

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/dcversus/prp/issues
- Documentation: https://github.com/dcversus/prp/docs
- Community: Discord server (link in README)

---

*Generated by PRP CLI v0.5.0*