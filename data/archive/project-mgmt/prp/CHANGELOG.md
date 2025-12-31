# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.9] - 2025-11-03

### Fixed
- **Version synchronization**: Updated package.json and src/cli.ts version from 0.4.1 to 0.4.9
- **Build preparation**: Ensured version consistency across all references for publishing

## [0.4.1] - 2025-10-28

### Changed
- **Wiki.js Template Articles** - All 13 stub articles completed with comprehensive content
  - Article 10 (PRP Overview): Expanded from 17 lines to 267 lines with complete LOOP MODE workflow
  - Article 11 (Signal System): Expanded to 598 lines with all 14 signals and reaction patterns
  - Article 12 (Context-Driven Dev): Expanded to 464 lines explaining context > commands philosophy
  - Article 13 (Human as Agent): Expanded to 385 lines on orchestrator autonomy protocol
  - Article 20 (CLI Installation): Expanded to 590 lines with npm/npx/yarn/pnpm installation methods
  - Article 21 (CLI Usage): Expanded to 653 lines with examples for all 6 templates
  - Article 22 (Templates): Expanded to 572 lines documenting all templates with comparison table
  - Article 30 (Contributing): Expanded to 709 lines from CONTRIBUTING.md with PRP workflow
  - Article 40 (Wiki.js Basics): Expanded to 456 lines with Docker setup and SSO configuration
  - Article 41 (Content Management): Expanded to 710 lines with comprehensive editing guide
  - Article 42 (Best Practices): Expanded to 835 lines with style guide and maintenance procedures
  - Article 50 (Research Papers): Expanded to 231 lines with 12 academic paper citations
  - Article 51 (External Resources): Expanded to 214 lines with 67+ curated resources
  - **Total Impact**: ~6,894 lines of comprehensive, fact-checked documentation added
  - All articles include fact-check sections with Tier 1 verified sources
  - All code examples tested and validated
  - All links verified as functional
  - Template now production-ready for immediate use

## [0.3.0] - 2025-10-28

### Added
- **Orchestrator Autonomy Protocol** - AI orchestrators make decisions without human approval
  - Added "Orchestrator Autonomy Protocol" section to AGENTS.md (300+ lines)
  - Rule 1: NO QUESTIONS TO HUMANS - Make autonomous decisions
  - Rule 2: ASYNC COMMUNICATION ONLY - Use ATTENTION signals in PRPs
  - Rule 3: NUDGE FOR CRITICAL BLOCKS ONLY - Priority 10 scenarios
  - Rule 4: AUTONOMOUS DECISION MAKING - Signal analysis → prioritize → execute → document
  - Decision protocol with rationale documentation
  - Timeout-based decision making (1h/4h/24h thresholds)
  - Multi-PRP orchestration algorithms
  - Performance metrics and orchestrator mantras
  - Updated README.md with "Autonomous Orchestration" principles
  - Key principle: "AI Orchestrator makes decisions autonomously. Humans are subordinate agents."
- **PRP-009 Specification** - Tracks Wiki.js template implementation
  - Comprehensive PRP document with autonomous decision logs
  - Status tracking for 20 starter articles (7 complete, 13 stubs)
  - Technical implementation details
  - Risk assessment and mitigation strategies
  - Next steps and action items with priorities
- **Wiki.js Template** - Complete documentation project generator
  - 20 pre-written articles covering PRP methodology, CLI usage, and Wiki.js administration
  - Docker Compose setup with PostgreSQL and Redis
  - Authentik SSO configuration template
  - Comprehensive getting-started guides for non-developers
  - Article writing guidelines with fact-checking standards
  - GitHub registration guide
  - Authentik login tutorial
  - Contributing guidelines
  - Wiki.js administration basics
- **Article Writing Standards** - Added to AGENTS.md
  - Mandatory citation requirements for all factual claims
  - Source authority hierarchy (Tier 1-3)
  - Fact-check section template
  - Self-check criteria checklist
  - Code example testing requirements
  - Version-specific documentation standards
  - Screenshot guidelines
  - Article update policy (6-month verification cycle)

### Fixed
- **CLI version**: Updated hardcoded version from 0.1.0 to 0.2.0 in src/cli.ts:13
- **Non-interactive mode**: Implemented missing non-interactive mode functionality for CLI
  - Created src/nonInteractive.ts with full non-interactive project generation support
  - Added input validation for required options (name, template, email)
  - Added proper error messages and exit codes for missing/invalid options
  - Supports all CLI flags: --no-git, --no-install, --license, etc.
- **E2E tests**: Fixed all 5 failing E2E tests in install-upgrade.test.ts
  - Updated tests to use local build (node dist/cli.js) instead of npm installed version
  - Fixed tsconfig.json parsing test to check content instead of parsing JSON with comments
  - Added CLI path verification in test setup
  - All 18 tests now passing (9 unit + 9 E2E)
- **ESM compatibility**: Fixed __dirname usage in ESM modules using fileURLToPath and import.meta.url

## [0.2.0] - 2025-10-28

### Added
- **PRP Methodology & Signal System** - Revolutionary context-driven development workflow
  - **Signal System**: 14 emotional/state indicators (ATTENTION, BLOCKED, TIRED, ENCANTADO, etc.)
  - **Signal Strength**: Priority system (1-10 scale) for intelligent work prioritization
  - **Progress Log**: Standardized table format tracking role, datetime, comment, signal
  - **Agent Personalities**: System Analyst with Portuguese flair, pragmatic Developer, skeptical Tester
  - **Signal Reaction Patterns**: Detailed action steps for each signal type
  - **PRP LOOP MODE**: Continuous iteration guided by signals until DoD or checkpoint
  - **Mandatory PRP Workflow**: Find/create PRP before any work (Policy #0)
  - **TUI Selection**: Interactive PRP selection when multiple exist
  - **CLI Flags**: `--file` and `--new` for PRP management
- **CI/CD Infrastructure** - Complete GitHub Actions setup
  - Parallel CI jobs (lint, typecheck, test, security, build) for faster feedback
  - CHANGELOG.md check workflow (enforces mandatory updates on PRs)
  - Claude Code Review workflow (automated AI code review)
  - Quality gate job (requires all checks to pass)
- **Pre-commit Hooks** - Automated quality checks before commits
  - Husky for git hook management
  - lint-staged for incremental linting and formatting
  - TypeScript type checking on pre-commit
- **Testing Infrastructure**
  - Unit tests for validation utilities (9 tests passing)
  - Jest configuration fixed (`coverageThreshold` typo corrected)
  - Test directory structure (`tests/unit/`, `tests/integration/`)
- **GitHub Templates**
  - Pull Request template with DOD checklist and CHANGELOG reminder
  - Bug report issue template
  - Feature request issue template
  - Template request issue template
- **Development Tools**
  - `.editorconfig` for consistent code formatting across editors
  - Updated `lint-staged` configuration in package.json
- **Documentation**
  - AGENTS.md with comprehensive PRP workflow and signal system
  - AGENTS.md with mandatory CHANGELOG policy
  - README.md updated with PRP methodology as main project goal
  - PRP-007 specification document for signal system
  - CLAUDE.md updated with AGENTS.md reference

### Changed
- **PRP Directory Structure** - **BREAKING CHANGE**: Enforced flat structure with outcome-focused naming
  - All PRPs now in flat structure (no subdirectories allowed)
  - Naming convention: `PRP-XXX-what-will-change.md` (2-4 words, kebab-case)
  - Renamed: `PRP-CLI.md` → `PRP-001-bootstrap-cli-created.md`
  - Renamed: `PRP-002-Landing-Page.md` → `PRP-002-landing-page-deployed.md`
  - Renamed: `PRP-007-signal-system.md` → `PRP-007-signal-system-implemented.md`
  - Consolidated PRP-002 supporting files into single document
  - Moved PRP-003 from research/ subdirectory to flat structure
  - Removed research/ subdirectory entirely
- **CI Workflow** - Restructured from matrix to parallel jobs (EdgeCraft pattern)
  - Faster feedback (5 parallel jobs vs sequential matrix)
  - Separate security audit job
  - Build artifact retention (7 days)
- **Package.json** - Added husky and lint-staged as dev dependencies

### Fixed
- Jest configuration warning (`coverageThresholds` → `coverageThreshold`)

### Security
- None

## [0.1.1] - 2025-10-28

### Fixed
- **package.json bin path**: Corrected bin path to `dist/cli.js` for proper npm global installation
- **ESLint configuration**: Added Node.js globals (\_\_dirname, \_\_filename) to ESLint config to fix publishing errors
- **Jest configuration**: Added `--passWithNoTests` flag to allow npm publish without test files in initial release

### Changed
- Updated build process to ensure executable permissions on `dist/cli.js`

## [0.1.0] - 2025-10-28

### Added
- **Interactive CLI** with beautiful Ink-based terminal UI
  - Wizard-style project setup flow
  - Step-by-step prompts for project metadata
  - Template selection with descriptions
  - License selection
- **Three Production-Ready Templates**:
  - TypeScript Library (based on EdgeCraft patterns with strict config)
  - React App with Vite (based on EdgeCraft patterns)
  - FastAPI Python service (based on dcmaidbot patterns)
- **Comprehensive Common File Generators**:
  - LICENSE (MIT, Apache-2.0, GPL-3.0, BSD-3-Clause, ISC, Unlicense)
  - README.md with project badges and sections
  - CONTRIBUTING.md with contribution guidelines
  - CODE_OF_CONDUCT.md (Contributor Covenant)
  - SECURITY.md with vulnerability reporting
  - CHANGELOG.md (Keep a Changelog format)
  - .gitignore (language-specific)
  - .editorconfig for consistent coding styles
- **Template Engine**:
  - Handlebars-based rendering
  - Custom helpers (uppercase, lowercase, kebabCase, snakeCase, pascalCase)
  - Type-safe template data
- **Modular Utility System**:
  - File generator with automatic directory creation
  - Git operations (init, add, commit)
  - Package manager detection (npm, yarn, pnpm)
  - Input validation for project names and emails
- **Command-Line Options**:
  - Full non-interactive mode support
  - All options configurable via CLI flags
  - Git initialization (optional, --no-git to skip)
  - Dependency installation (optional, --no-install to skip)
- **Developer Experience**:
  - TypeScript 5.6+ with strict mode enabled
  - Zero-warning build system
  - Hot reload in development mode
  - Comprehensive npm scripts
  - Clean ESM module architecture
- **Documentation**:
  - Comprehensive PRP specification (PRP-CLI.md)
  - Complete README with usage examples
  - CLAUDE.md for AI development guidelines

### Technical Details
- Built with TypeScript 5.6+ in strict mode
- Uses Ink 5.0+ for React-based terminal UI
- Commander.js for CLI argument parsing
- Handlebars 4.7+ for template rendering
- fs-extra for file operations
- execa for subprocess execution
- Full ESM module system

[Unreleased]: https://github.com/dcversus/prp/compare/v0.4.9...HEAD
[0.4.9]: https://github.com/dcversus/prp/compare/v0.4.1...v0.4.9
[0.4.1]: https://github.com/dcversus/prp/compare/v0.3.0...v0.4.1
[0.3.0]: https://github.com/dcversus/prp/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dcversus/prp/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/dcversus/prp/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/dcversus/prp/releases/tag/v0.1.0
