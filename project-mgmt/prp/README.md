# PRP - Product Requirement Prompts

> **Bootstrap context-driven development workflow based on Product Requirement Prompts (PRPs) and orchestrate execution with LOOP MODE**
>
> By Vasilisa Versus

## What is PRP?

**PRP** is both a methodology and a CLI tool that revolutionizes how you approach software development:

- **📋 PRP Methodology** - Context-driven development using Product Requirement Prompts as living documents
- **🔄 LOOP MODE** - Continuous agent iteration guided by signals and progress tracking
- **🤖 Agent Orchestration** - AI agents collaborate through standardized PRPs with emotional signals
- **⚡ Signal System** - Track progress and emotional state with 14+ signals (ATTENTION, BLOCKED, ENCANTADO, etc.)
- **🚀 Project Scaffolding** - Bootstrap new projects with best practices and complete infrastructure

### The PRP Workflow (Autonomous Orchestration)

**Key Principle:** AI Orchestrator makes decisions autonomously. Humans are subordinate agents, not decision makers.

Every development task follows this cycle:

1. **📖 Read ALL PRPs** - Load context across entire project
2. **🔍 Analyze ALL Signals** - Identify highest priority signal (10→1)
3. **⚡ React to Strongest Signal** - Work on highest priority across ALL PRPs
4. **🔨 Execute Work** - Implement changes, write code, solve problems
5. **💬 Update Progress** - Leave detailed comment in progress log
6. **🎯 Leave Signal** - Express current state (TIRED, CONFIDENT, BLOCKED, etc.)
7. **🔄 Loop** - Continue until DoD met or checkpoint reached

**Orchestrator Rules:**
- ❌ **NO QUESTIONS** to humans for decisions
- ✅ **DECIDE AUTONOMOUSLY** based on signal analysis
- ✅ **DOCUMENT** decisions in PRP progress log
- ✅ **EXECUTE** immediately without waiting
- ⚠️ **NUDGE** only for critical blocks (Priority 10)

**Quick Start with PRPs**:
```bash
# List all PRPs
ls PRPs/

# Start working on a PRP
# 1. Read the PRP file
# 2. Check Progress Log for latest signal
# 3. Follow signal's algorithm (see AGENTS.md)
# 4. Do work
# 5. Update Progress Log with your signal
```

**Signal Examples**:
- 🔴 **ATTENTION** (10) - New work or need user input (triggers NUDGE system)
- 🚫 **BLOCKED** (9) - Can't proceed, need external help
- ✅ **CONFIDENT** (3) - Work done, ready for review
- 🏁 **COMPLETED** (1) - PRP finished, DoD met

**For detailed workflow instructions, see [AGENTS.md](AGENTS.md)**
**For contributing guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md)**

[![npm version](https://img.shields.io/npm/v/@dcversus/prp)](https://www.npmjs.com/package/@dcversus/prp)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D20.0.0-brightgreen)](https://nodejs.org/)

## Project Scaffolding Features

- 🚀 **Interactive CLI** with beautiful TUI powered by Ink (React for terminal)
- 🎨 **Multiple Templates** - FastAPI, NestJS, React, TypeScript libraries, and more
- 📝 **Complete Project Setup** - LICENSE, README, CONTRIBUTING, Code of Conduct, and more
- 🤖 **AI Integration** - Optional AI-powered code generation (OpenAI, Anthropic, Google)
- 🔧 **GitHub Actions** - Pre-configured CI/CD workflows
- 📦 **Dependency Management** - Auto-install with npm, yarn, or pnpm
- 🐳 **Docker Support** - Optional Dockerfile and docker-compose configuration
- ⚙️ **Configurable** - Choose exactly what to include in your project

## Quick Start

### Using npx (no installation required)

```bash
npx @dcversus/prp
```

### Global Installation

```bash
npm install -g @dcversus/prp
prp
```

### Local Installation

```bash
npm install @dcversus/prp
npx prp
```

## Usage

### Interactive Mode (Default)

Simply run the command and follow the prompts:

```bash
prp
```

The interactive CLI will guide you through:
1. Project metadata (name, description, author)
2. Template selection (FastAPI, NestJS, React, etc.)
3. Feature selection (LICENSE, Code of Conduct, GitHub Actions, etc.)
4. AI integration options (optional)

### Non-Interactive Mode

Pass all options via command-line arguments:

```bash
prp \
  --name my-project \
  --description "My awesome project" \
  --author "Your Name" \
  --email "you@example.com" \
  --template react \
  --license MIT \
  --no-interactive
```

### Command-Line Options

```
Options:
  -n, --name <name>              Project name
  -d, --description <desc>       Project description
  -a, --author <author>          Author name
  -e, --email <email>            Author email
  -t, --template <template>      Template (fastapi, nestjs, react, typescript-lib, none)
  --no-interactive               Run in non-interactive mode
  --yes                          Use default values for all options
  --license <license>            License type (default: MIT)
  --no-git                       Skip git initialization
  --no-install                   Skip dependency installation
  -h, --help                     Display help
  -V, --version                  Display version
```

## Supported Templates

| Template | Description | Tech Stack |
|----------|-------------|------------|
| `none` | Minimal setup with docs only | N/A |
| `fastapi` | FastAPI Python web service | Python, FastAPI, Uvicorn |
| `nestjs` | NestJS TypeScript backend | TypeScript, NestJS, Node.js |
| `react` | React web application | TypeScript, React, Vite |
| `typescript-lib` | TypeScript library/package | TypeScript, Node.js |
| `express` | Express.js backend | TypeScript, Express, Node.js |

More templates coming soon!

## What Gets Generated

### Core Project Files

- `README.md` - Project documentation with badges and sections
- `LICENSE` - Your chosen license (default: MIT)
- `.gitignore` - Language/framework-specific ignores
- `package.json` / `requirements.txt` - Dependency management
- `.editorconfig` - Consistent coding styles

### Open Source Best Practices

- `CONTRIBUTING.md` - Contribution guidelines
- `CODE_OF_CONDUCT.md` - Contributor Covenant
- `SECURITY.md` - Security policy and vulnerability reporting
- `CHANGELOG.md` - Keep a Changelog format

### GitHub Templates

- `.github/ISSUE_TEMPLATE/` - Bug reports and feature requests
- `.github/PULL_REQUEST_TEMPLATE.md` - PR checklist
- `.github/workflows/ci.yml` - CI/CD workflows

### Development Tools

- ESLint configuration (for JS/TS projects)
- Prettier configuration (for JS/TS projects)
- TypeScript configuration (for TS projects)
- Jest/pytest configuration (testing)
- Docker configuration (optional)

## AI Integration

PRP can optionally integrate with AI coding assistants to generate boilerplate code:

### Supported Providers

- **OpenAI** (GPT-4, Codex) - `OPENAI_API_KEY`
- **Anthropic** (Claude) - `ANTHROPIC_API_KEY`
- **Google** (Gemini) - `GOOGLE_API_KEY`

### Configuration

Set your API key as an environment variable:

```bash
export OPENAI_API_KEY="your-api-key"
prp
```

Or create a `.prprc` file in your home directory:

```json
{
  "aiProvider": "openai",
  "apiKey": "your-api-key"
}
```

## Development

### Prerequisites

- Node.js >= 20.0.0
- npm >= 10.0.0

### Setup

```bash
# Clone the repository
git clone https://github.com/dcversus/prp.git
cd prp

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build
npm run build

# Run tests
npm test

# Lint and format
npm run lint
npm run format
```

### Project Structure

```
prp/
├── src/
│   ├── cli.ts              # CLI entry point
│   ├── index.ts            # Main module
│   ├── types.ts            # TypeScript types
│   ├── ui/                 # Ink UI components
│   │   ├── App.tsx
│   │   └── components/
│   ├── generators/         # Template generators
│   ├── templates/          # Template files
│   ├── ai/                 # AI integration
│   └── utils/              # Helper functions
├── tests/                  # Test files
├── dist/                   # Compiled output
└── docs/                   # Documentation
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

Inspired by:
- [Yeoman](https://yeoman.io/)
- [Cookiecutter](https://github.com/cookiecutter/cookiecutter)
- [create-react-app](https://create-react-app.dev/)
- [Telefonica Open Source Scaffold](https://github.com/Telefonica/opensource-scaffold)

Built with:
- [Ink](https://github.com/vadimdemedes/ink) - React for CLIs
- [Commander.js](https://github.com/tj/commander.js) - Command-line framework
- [TypeScript](https://www.typescriptlang.org/) - Type-safe JavaScript

## Links

- [GitHub Repository](https://github.com/dcversus/prp)
- [npm Package](https://www.npmjs.com/package/@dcversus/prp)
- [Issue Tracker](https://github.com/dcversus/prp/issues)
- [Changelog](CHANGELOG.md)

---

Made with ❤️ by [dcversus](https://github.com/dcversus)
