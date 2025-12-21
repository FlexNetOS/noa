import { test, expect } from '@playwright/test';
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

test.describe('Authenticated Navigation', () => {
  test.use({ storageState: authFile });

  test('should navigate to main dashboard', async ({ page }) => {
    await page.goto('/');
    
    await expect(page.getByText('ML DevOps Platform')).toBeVisible();
    await expect(page.getByText('Unified AI Assistant')).toBeVisible();
  });

  test('should navigate to SONA page', async ({ page }) => {
    await page.goto('/');
    
    // Click SONA link
    await page.click('a[href="/sona"]');
    
    // Verify we're on SONA page
    await expect(page).toHaveURL('/sona');
    await expect(page.getByText('SONA Orchestration')).toBeVisible();
  });

  test('should navigate to DeepCode page', async ({ page }) => {
    await page.goto('/');
    
    // Click DeepCode link
    await page.click('a[href="/deepcode"]');
    
    // Verify we're on DeepCode page
    await expect(page).toHaveURL('/deepcode');
    await expect(page.getByText('DeepCode')).toBeVisible();
  });

  test('should navigate to Documentation page', async ({ page }) => {
    await page.goto('/');
    
    // Click Documentation link
    await page.click('a[href="/docs"]');
    
    // Verify we're on docs page
    await expect(page).toHaveURL('/docs');
    await expect(page.getByText('Documentation')).toBeVisible();
  });

  test('should navigate to Profile page', async ({ page }) => {
    await page.goto('/profile');
    
    await expect(page.getByText('Profile Settings')).toBeVisible();
    await expect(page.getByRole('button', { name: /sign out/i })).toBeVisible();
  });
});
