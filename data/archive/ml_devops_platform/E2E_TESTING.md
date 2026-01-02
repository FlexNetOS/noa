# End-to-End Testing Guide

## Overview

The ML DevOps Platform includes a comprehensive E2E testing suite using Playwright. Tests cover all critical user journeys including authentication, navigation, chat interactions, SONA workflows, and profile management.

## Test Structure

### Test Files

```
nextjs_space/e2e/
├── auth.setup.ts          # Authentication setup for tests
├── auth.spec.ts           # Login and signup flows
├── navigation.spec.ts     # Multi-page navigation
├── chat.spec.ts           # MOE chat interface
├── sona.spec.ts           # SONA orchestration
└── profile.spec.ts        # Profile management
```

### Configuration

- **Config File**: `playwright.config.ts`
- **Base URL**: `http://localhost:3000` (configurable)
- **Browsers**: Chromium, Firefox, WebKit, Mobile Chrome, Mobile Safari
- **Parallel Execution**: Enabled by default
- **Retries**: 2 retries on CI, 0 locally

## Running Tests

### Prerequisites

1. Install dependencies:
```bash
cd nextjs_space
yarn install
```

2. Install Playwright browsers:
```bash
yarn playwright install chromium
```

### Running All Tests

```bash
yarn test
```

### Running Specific Test Files

```bash
# Run auth tests only
yarn test e2e/auth.spec.ts

# Run navigation tests
yarn test e2e/navigation.spec.ts

# Run chat tests
yarn test e2e/chat.spec.ts
```

### Running in UI Mode

Playwright UI mode provides an interactive testing experience:

```bash
yarn test:ui
```

Features:
- Visual test runner
- Time travel debugging
- Step-by-step execution
- Network inspection
- DOM snapshots

### Running in Debug Mode

```bash
yarn test:debug
```

Stops at the first failure and opens the Playwright inspector.

### Viewing Test Reports

After running tests, view the HTML report:

```bash
yarn test:report
```

## Test Coverage

### Authentication (auth.spec.ts)

- ✅ Display login page correctly
- ✅ Show error for invalid credentials
- ✅ Successfully login with valid credentials
- ✅ Navigate to signup page
- ✅ Display signup page correctly
- ✅ Validate required fields
- ✅ Navigate back to login

### Navigation (navigation.spec.ts)

- ✅ Navigate to main dashboard
- ✅ Navigate to SONA page
- ✅ Navigate to DeepCode page
- ✅ Navigate to Documentation page
- ✅ Navigate to Profile page

### Chat Interface (chat.spec.ts)

- ✅ Display chat interface
- ✅ Send a message
- ✅ Show loading state while processing

### SONA Orchestration (sona.spec.ts)

- ✅ Display SONA dashboard
- ✅ Show workflow templates
- ✅ Display statistics
- ✅ Navigate between tabs

### Profile Management (profile.spec.ts)

- ✅ Display profile information
- ✅ Show role badge
- ✅ Have sign out button
- ✅ Allow name editing
- ✅ Sign out successfully

## Writing New Tests

### Basic Test Structure

```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/your-page');
  });

  test('should do something', async ({ page }) => {
    // Your test code
    await expect(page.getByText('Expected Text')).toBeVisible();
  });
});
```

### Authenticated Tests

For tests that require authentication, use the auth file:

```typescript
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

test.describe('Authenticated Feature', () => {
  test.use({ storageState: authFile });

  test('should access protected page', async ({ page }) => {
    await page.goto('/protected');
    // Test authenticated content
  });
});
```

### Common Patterns

#### Clicking Elements

```typescript
// By role
await page.getByRole('button', { name: 'Submit' }).click();

// By text
await page.getByText('Click Me').click();

// By selector
await page.click('button[type="submit"]');
```

#### Filling Forms

```typescript
await page.fill('input[type="email"]', 'test@example.com');
await page.fill('input[type="password"]', 'password123');
```

#### Waiting for Navigation

```typescript
await page.click('button');
await page.waitForURL('/expected-url');
```

#### Assertions

```typescript
// Visibility
await expect(page.getByText('Hello')).toBeVisible();

// URL
await expect(page).toHaveURL('/expected');

// Count
await expect(page.locator('.item')).toHaveCount(5);

// Attribute
await expect(page.locator('input')).toHaveAttribute('required', '');
```

## Best Practices

### 1. Use Semantic Selectors

Prefer role-based and text-based selectors:

```typescript
// Good
await page.getByRole('button', { name: 'Submit' });
await page.getByText('Welcome Back');

// Avoid
await page.locator('#button-123');
await page.locator('.css-class-xyz');
```

### 2. Wait for Elements

Always wait for elements to be visible:

```typescript
await expect(page.getByText('Loading complete')).toBeVisible();
```

### 3. Independent Tests

Each test should be independent and not rely on other tests:

```typescript
// Good: Each test starts fresh
test.beforeEach(async ({ page }) => {
  await page.goto('/login');
});

// Avoid: Tests depend on execution order
test('login', async ({ page }) => { /* ... */ });
test('use logged in state', async ({ page }) => { /* depends on previous */ });
```

### 4. Use Test Fixtures

Create reusable fixtures for common operations:

```typescript
const test = base.extend({
  authenticatedPage: async ({ page }, use) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');
    await page.waitForURL('/');
    await use(page);
  },
});
```

### 5. Group Related Tests

```typescript
test.describe('Authentication', () => {
  test.describe('Login', () => {
    // Login tests
  });

  test.describe('Signup', () => {
    // Signup tests
  });
});
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install dependencies
        run: yarn install

      - name: Install Playwright
        run: yarn playwright install --with-deps chromium

      - name: Run tests
        run: yarn test
        env:
          CI: true

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: playwright-report
          path: playwright-report/
```

## Debugging

### Visual Debugging

Run tests in headed mode:

```bash
yarn playwright test --headed
```

### Debug Mode

```bash
yarn test:debug
```

### Screenshots on Failure

Playwright automatically captures screenshots on failure. Find them in:
```
test-results/
```

### Traces

Traces are captured on first retry. View them:

```bash
yarn playwright show-trace test-results/path-to-trace.zip
```

### Console Logs

Capture console logs in tests:

```typescript
test('should log errors', async ({ page }) => {
  const logs: string[] = [];
  page.on('console', msg => logs.push(msg.text()));

  await page.goto('/');

  expect(logs).not.toContain('error');
});
```

## Troubleshooting

### Tests Timeout

Increase timeout for slow operations:

```typescript
test('slow operation', async ({ page }) => {
  test.setTimeout(60000); // 60 seconds
  await page.goto('/');
});
```

### Element Not Found

Use `waitForSelector` before interaction:

```typescript
await page.waitForSelector('button[type="submit"]');
await page.click('button[type="submit"]');
```

### Flaky Tests

Add retry logic or use `waitFor`:

```typescript
await expect(async () => {
  await expect(page.getByText('Dynamic Content')).toBeVisible();
}).toPass({ timeout: 10000 });
```

## Resources

- [Playwright Documentation](https://playwright.dev/)
- [Playwright Best Practices](https://playwright.dev/docs/best-practices)
- [Playwright API Reference](https://playwright.dev/docs/api/class-playwright)
- [Debugging Guide](https://playwright.dev/docs/debug)

## Maintenance

### Updating Dependencies

```bash
yarn add -D @playwright/test@latest
yarn playwright install
```

### Updating Test Data

Modify `e2e/auth.setup.ts` to update test credentials.

### Adding New Test Files

1. Create new file in `e2e/` directory
2. Follow naming convention: `feature.spec.ts`
3. Import necessary utilities
4. Write tests using Playwright API
5. Run tests to verify

---

**Note**: Tests use the test account `john@doe.com` with password `johndoe123`. This account is seeded automatically during database initialization.
