# Contributing to Taskosaur

Thank you for your interest in contributing to Taskosaur! We welcome contributions from the community and are grateful for your help in making this project better.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/Taskosaur/taskosaur.git
   cd taskosaur
   ```
3. **Follow the setup instructions** in the [README.md](README.md)
4. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## How to Contribute

### Bug Reports
- Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md)
- Include clear reproduction steps
- Provide environment details (OS, Node.js version, etc.)
- Include relevant screenshots or error messages

### Feature Requests
- Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md)
- Explain the problem your feature solves
- Provide examples and use cases
- Consider the scope and impact

### Code Contributions
- Check existing [issues](https://github.com/Taskosaur/taskosaur/issues) for good first issues
- Look for issues labeled `good first issue` or `help wanted`
- Comment on issues you'd like to work on
- Follow our coding standards and commit guidelines

## Development Setup

### Prerequisites
- Node.js 22+ and npm
- PostgreSQL 13+
- Redis 6+

### Setup Steps
1. **Install dependencies**:
   ```bash
   npm install
   cd backend && npm install
   cd ../frontend && npm install
   ```

2. **Environment setup**:
   ```bash
   # Copy environment files
   cp backend/.env.example backend/.env
   cp frontend/.env.example frontend/.env.local
   
   # Update the .env files with your configuration
   ```

3. **Database setup**:
   ```bash
   cd backend
   
   # Generate Prisma client
   npm run prisma:generate
   
   # Run database migrations
   npm run prisma:migrate:dev
   
   # (Optional) Seed the database
   npm run seed:core seed
   ```

4. **Start development servers**:
   ```bash
   # Terminal 1 - Backend
   cd backend && npm run start:dev
   
   # Terminal 2 - Frontend
   cd frontend && npm run dev
   ```

## Coding Standards

### TypeScript & JavaScript
- Use TypeScript for all new code
- Enable strict mode with null checks
- Provide explicit type annotations
- Avoid `any` type unless absolutely necessary

### Code Style
- Use Prettier for code formatting
- Run ESLint and fix all warnings
- Follow existing naming conventions:
  - PascalCase for classes, interfaces, types
  - camelCase for variables, functions
  - kebab-case for file names

### Backend (NestJS)
- Use dependency injection via constructors
- Implement proper error handling with try/catch
- Use DTOs for request/response validation
- Follow the existing module structure

### Frontend (Next.js/React)
- Use functional components with hooks
- Implement proper error boundaries
- Use TypeScript interfaces for props
- Follow the existing component structure

### Database
- Write clear, descriptive migration names
- Include both up and down migrations
- Test migrations on sample data
- Document schema changes

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

### Commit Message Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples
```bash
feat(auth): add password reset functionality
fix(tasks): resolve kanban drag and drop issue
docs: update API documentation for task endpoints
```

## Pull Request Process

1. **Ensure your code follows our standards**:
   ```bash
   # Backend
   cd backend
   npm run lint
   npm run format
   
   # Frontend
   cd frontend
   npm run lint
   ```

2. **Update documentation** if needed
3. **Write clear PR description**:
   - Describe what changes you made
   - Reference related issues
   - Include screenshots for UI changes
   - List any breaking changes

4. **Request review** from maintainers
5. **Address feedback** promptly
6. **Squash commits** if requested

### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Code builds successfully
- [ ] Manual testing completed
- [ ] No console errors or warnings

## Screenshots (if applicable)
Add screenshots here

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes or breaking changes documented
```

## Issue Reporting

### Before Creating an Issue
1. **Search existing issues** to avoid duplicates
2. **Check the documentation** and FAQ
3. **Try the latest version** to see if it's already fixed

### Creating a Good Issue
- Use appropriate issue templates
- Provide clear, descriptive titles
- Include steps to reproduce
- Add relevant labels
- Include environment information
- Attach screenshots or logs if helpful

## Community

- **Discussions**: [GitHub Discussions](https://github.com/Taskosaur/taskosaur/discussions)
- **Issues**: [GitHub Issues](https://github.com/Taskosaur/taskosaur/issues)
- **Email**: support@taskosaur.com

## Labels

We use labels to categorize issues and PRs:

- `bug`: Something isn't working
- `enhancement`: New feature or request
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `documentation`: Improvements or additions to documentation
- `duplicate`: This issue or PR already exists
- `wontfix`: This will not be worked on

## Development Priorities

Check our [Development Roadmap](README.md#development-roadmap) to see what we're currently working on and what's planned for future releases.

## Questions?

If you have questions about contributing, feel free to:
- Open a [Discussion](https://github.com/Taskosaur/taskosaur/discussions)
- Create an issue with the `question` label
- Reach out via email

Thank you for contributing to Taskosaur!