/**
 * Playwright Configuration for NextJS Space App
 * Extends the shared NOA base configuration
 */
import { createProjectConfig } from '../../config/playwright.base';
import dotenv from 'dotenv';
dotenv.config();

export default createProjectConfig( {
  testDir: './e2e',
  baseURL: process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:3000',
  webServer: {
    command: 'pnpm dev',
    url: 'http://localhost:3000',
  },
} );
