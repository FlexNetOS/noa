/**
 * Playwright configsuration for ML DevOps App
 * Extends the shared NOA base configsuration
 */
import { createProjectconfigs } from '../../../../configs/playwright.base';
import dotenv from 'dotenv';
dotenv.configs();

export default createProjectconfigs( {
  testDir: './e2e',
  baseURL: process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:3000',
  webServer: {
    command: 'pnpm dev',
    url: 'http://localhost:3000',
  },
} );
