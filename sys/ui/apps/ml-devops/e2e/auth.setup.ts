import { test as setup, expect } from '@playwright/test';
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

setup('authenticate', async ({ page }) => {
  // Navigate to login page
  await page.goto('/login');

  // Fill in the login form with test credentials
  await page.fill('input[type="email"]', 'john@doe.com');
  await page.fill('input[type="password"]', 'johndoe123');

  // Click the sign in button
  await page.click('button[type="submit"]');

  // Wait for navigation to complete
  await page.waitForURL('/');

  // Verify we're logged in by checking for some authenticated content
  await expect(page).toHaveURL('/');

  // Save signed-in state to 'authFile'
  await page.context().storageState({ path: authFile });
});
