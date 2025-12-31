# Contributing to PRP

Thank you for your interest in contributing to PRP! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Node version, npm version)
- Any relevant logs or screenshots

### Suggesting Features

Feature requests are welcome! Please create an issue with:
- A clear, descriptive title
- Detailed description of the feature
- Use cases and benefits
- Any relevant examples or mockups

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Install dependencies**: `npm install`
3. **Make your changes**:
   - Follow the code style guidelines
   - Add tests for new functionality
   - Update documentation as needed
4. **Run validation**: `npm run validate`
5. **Commit your changes** using Conventional Commits format
6. **Push to your fork** and submit a pull request

#### Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Include tests for new features
- Update CHANGELOG.md under `[Unreleased]` section
- Ensure all tests pass: `npm test`
- Ensure code is properly formatted: `npm run format:check`
- Ensure no linting errors: `npm run lint`

## 📡 Working with Signals

**PRP uses a Signal System to track progress and communicate state.**

### What is a Signal?

A **signal** is an emotional/status indicator that tells other agents and contributors what state the work is in. Every PRP progress log entry must include a signal.

### How to Leave a Signal

When updating a PRP Progress Log, add a signal in the last column:

```markdown
| Role | DateTime | Comment | Signal |
|------|----------|---------|--------|
| developer | 2025-10-28 | Implemented auth module, all tests passing. | ✅ CONFIDENT |
```

### Common Signals

- **🔴 ATTENTION** (10) - Need user input or new PRP created
- **🚫 BLOCKED** (9) - Cannot proceed, external dependency needed
- **🚨 URGENT** (9) - Time-sensitive, immediate action required
- **😫 TIRED** (6) - Work incomplete, needs inventory
- **✅ CONFIDENT** (3) - Work complete, ready for review
- **🎯 VALIDATED** (2) - Reviewed and approved
- **🏁 COMPLETED** (1) - DoD met, PRP done

**Full signal reference**: See [AGENTS.md](AGENTS.md#signal-system)

### How to Read Signals

**Before starting work on a PRP**:
1. Read the entire PRP
2. Check Progress Log for latest signal
3. React to strongest signal (highest strength number)
4. Follow the signal's algorithm (see AGENTS.md)

**Example**:
- If you see **BLOCKED** → Check blocker details, see if you can resolve it
- If you see **TIRED** → Review inventory, continue where left off
- If you see **CONFIDENT** → Review work, create PR if ready

## Development Setup

### Prerequisites

- Node.js >= 20.0.0
- npm >= 10.0.0
- Git

### Setup Steps

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/prp.git
cd prp

# Add upstream remote
git remote add upstream https://github.com/dcversus/prp.git

# Install dependencies
npm install

# Run in development mode
npm run dev

# Run tests
npm test

# Run tests in watch mode
npm run test:watch
```

### Available Scripts

- `npm run dev` - Run CLI in development mode
- `npm run build` - Build for production
- `npm test` - Run tests
- `npm run test:watch` - Run tests in watch mode
- `npm run test:coverage` - Generate coverage report
- `npm run lint` - Lint code
- `npm run lint:fix` - Auto-fix linting issues
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check code formatting
- `npm run typecheck` - Run TypeScript type checking
- `npm run validate` - Run all checks (typecheck + lint + test)

## Code Style

### TypeScript

- Use strict mode (no `any` unless absolutely necessary)
- Prefer interfaces over types for object shapes
- Use meaningful variable and function names
- Add JSDoc comments for public APIs

### React Components (Ink)

- Use functional components with hooks
- Properly type all props
- Keep components small and focused
- Extract reusable logic to custom hooks

### File Naming

- TypeScript files: `camelCase.ts`
- React components: `PascalCase.tsx`
- Test files: `*.test.ts` or `*.test.tsx`

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(generators): add Vue.js template support
fix(ui): correct spinner positioning in progress view
docs(readme): update installation instructions
test(generators): add tests for React template
```

## Testing

### Writing Tests

- Place tests next to the code they test or in `tests/` directory
- Use descriptive test names
- Follow AAA pattern (Arrange, Act, Assert)
- Mock external dependencies

```typescript
describe('generateProject', () => {
  it('should create project directory with correct structure', async () => {
    // Arrange
    const options = { name: 'test-app', template: 'react' };
    const targetPath = '/tmp/test-app';

    // Act
    await generateProject(options, targetPath);

    // Assert
    expect(fs.existsSync(targetPath)).toBe(true);
    expect(fs.existsSync(path.join(targetPath, 'package.json'))).toBe(true);
  });
});
```

### Running Tests

```bash
# Run all tests
npm test

# Run specific test file
npm test -- generators/react.test.ts

# Run tests in watch mode
npm run test:watch

# Generate coverage report
npm run test:coverage
```

## Documentation

### Code Documentation

All public functions, classes, and interfaces should have JSDoc comments:

```typescript
/**
 * Generates a new project based on provided options.
 *
 * @param options - Project configuration options
 * @param targetPath - Directory where project will be created
 * @returns Promise that resolves when generation is complete
 * @throws {Error} If target directory already exists
 */
export async function generateProject(
  options: ProjectOptions,
  targetPath: string
): Promise<void> {
  // Implementation
}
```

### README Updates

When adding features:
- Update the feature list
- Add usage examples
- Update command-line options
- Add to supported templates if applicable

### CHANGELOG Updates

All changes should be documented in CHANGELOG.md under the `[Unreleased]` section:

```markdown
## [Unreleased]

### Added
- New Vue.js template support

### Changed
- Improved error messages in interactive mode

### Fixed
- Fixed incorrect file permissions on generated scripts
```

## Adding New Templates

To add support for a new project template:

1. Create template directory: `src/templates/<template-name>/`
2. Add template files with Handlebars placeholders
3. Create generator: `src/generators/<template-name>.ts`
4. Export from `src/generators/index.ts`
5. Add to `Template` type in `src/types.ts`
6. Update CLI in `src/cli.ts`
7. Write tests in `tests/generators/<template-name>.test.ts`
8. Update README.md

Example generator structure:

```typescript
import { GeneratorContext, FileToGenerate } from '../types.js';

export async function generateVueProject(
  context: GeneratorContext
): Promise<FileToGenerate[]> {
  const files: FileToGenerate[] = [];

  // Add package.json
  files.push({
    path: 'package.json',
    content: generatePackageJson(context.options),
  });

  // Add other files...

  return files;
}
```

## Review Process

1. All PRs require at least one approval
2. All CI checks must pass
3. Code must follow style guidelines
4. Tests must pass with adequate coverage
5. Documentation must be updated

## Getting Help

- 📖 Read [CLAUDE.md](CLAUDE.md) for detailed development guidelines
- 💬 Ask questions in [GitHub Discussions](https://github.com/dcversus/prp/discussions)
- 🐛 Report bugs in [GitHub Issues](https://github.com/dcversus/prp/issues)

## License

By contributing to PRP, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

Thank you for contributing to PRP! 🚀
