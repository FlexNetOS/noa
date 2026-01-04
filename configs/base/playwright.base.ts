/**
 * NOA Shared Playwright Configuration
 * 
 * This is the base Playwright configuration that all projects should extend.
 * It provides consistent settings across all E2E test suites.
 * 
 * Usage in project playwright.config.ts:
 * ```typescript
 * import { baseConfig, createProjectConfig } from '../../config/playwright.base';
 * export default createProjectConfig({
 *   testDir: './e2e',
 *   webServer: { command: 'pnpm dev', url: 'http://localhost:3000' }
 * });
 * ```
 * 
 * @see https://playwright.dev/docs/test-configuration
 */

import { defineConfig, devices, PlaywrightTestConfig } from '@playwright/test';

/**
 * Base configuration shared across all NOA projects
 */
export const baseConfig: PlaywrightTestConfig = {
    /* Run tests in files in parallel */
    fullyParallel: true,

    /* Fail the build on CI if you accidentally left test.only in the source code. */
    forbidOnly: !!process.env.CI,

    /* Retry on CI only */
    retries: process.env.CI ? 2 : 0,

    /* Opt out of parallel tests on CI. */
    workers: process.env.CI ? 1 : undefined,

    /* Reporter to use. See https://playwright.dev/docs/test-reporters */
    reporter: process.env.CI
        ? [ [ 'html', { open: 'never' } ], [ 'github' ] ]
        : 'html',

    /* Shared settings for all the projects below. */
    use: {
        /* Collect trace when retrying the failed test. */
        trace: 'on-first-retry',

        /* Screenshot on failure */
        screenshot: 'only-on-failure',

        /* Video on first retry */
        video: 'on-first-retry',
    },

    /* Configure projects for major browsers */
    projects: [
        {
            name: 'chromium',
            use: { ...devices[ 'Desktop Chrome' ] },
        },
        {
            name: 'firefox',
            use: { ...devices[ 'Desktop Firefox' ] },
        },
        {
            name: 'webkit',
            use: { ...devices[ 'Desktop Safari' ] },
        },
        /* Test against mobile viewports. */
        {
            name: 'Mobile Chrome',
            use: { ...devices[ 'Pixel 5' ] },
        },
        {
            name: 'Mobile Safari',
            use: { ...devices[ 'iPhone 12' ] },
        },
    ],
};

/**
 * Device presets for common testing scenarios
 */
export const devicePresets = {
    desktop: [ 'chromium', 'firefox', 'webkit' ],
    mobile: [ 'Mobile Chrome', 'Mobile Safari' ],
    chromiumOnly: [ 'chromium' ],
    all: [ 'chromium', 'firefox', 'webkit', 'Mobile Chrome', 'Mobile Safari' ],
};

/**
 * Create a project-specific Playwright configuration by extending the base config
 * 
 * @param overrides - Project-specific configuration overrides
 * @returns Complete Playwright configuration
 */
export function createProjectConfig ( overrides: {
    testDir: string;
    baseURL?: string;
    webServer?: {
        command: string;
        url: string;
        reuseExistingServer?: boolean;
        stdout?: 'ignore' | 'pipe';
        stderr?: 'ignore' | 'pipe';
    };
    projects?: typeof baseConfig.projects;
    use?: Partial<typeof baseConfig.use>;
} ): PlaywrightTestConfig
{
    return defineConfig( {
        ...baseConfig,
        testDir: overrides.testDir,
        use: {
            ...baseConfig.use,
            baseURL: overrides.baseURL || process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:3000',
            ...overrides.use,
        },
        projects: overrides.projects || baseConfig.projects,
        webServer: overrides.webServer ? {
            ...overrides.webServer,
            reuseExistingServer: overrides.webServer.reuseExistingServer ?? !process.env.CI,
            stdout: overrides.webServer.stdout ?? 'ignore',
            stderr: overrides.webServer.stderr ?? 'pipe',
        } : undefined,
    } );
}

export default baseConfig;
