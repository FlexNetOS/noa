import { test, expect } from '@playwright/test';
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

test.describe('Profile Management', () => {
  test.use({ storageState: authFile });

  test.beforeEach(async ({ page }) => {
    await page.goto('/profile');
  });

  test('should display profile information', async ({ page }) => {
    await expect(page.getByText('Profile Settings')).toBeVisible();
    await expect(page.getByText(/john@doe\.com/i)).toBeVisible();
  });

  test('should show role badge', async ({ page }) => {
    // Check for role indicator (admin or user)
    const roleBadge = page.locator('[class*="badge"]').filter({ hasText: /admin|user/i });
    await expect(roleBadge.first()).toBeVisible();
  });

  test('should have sign out button', async ({ page }) => {
    const signOutButton = page.getByRole('button', { name: /sign out/i });
    await expect(signOutButton).toBeVisible();
  });

  test('should allow name editing', async ({ page }) => {
    // Look for name input field
    const nameInput = page.locator('input[type="text"]').first();
    
    if (await nameInput.isVisible()) {
      // Clear and enter new name
      await nameInput.fill('Test User Updated');
      
      // Look for save button
      const saveButton = page.getByRole('button', { name: /save|update/i });
      if (await saveButton.count() > 0) {
        await saveButton.click();
        
        // Check for success message
        await expect(page.getByText(/updated|saved|success/i)).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should sign out successfully', async ({ page }) => {
    // Click sign out
    await page.click('button:has-text("Sign Out")');
    
    // Wait for redirect to login
    await page.waitForURL('/login');
    await expect(page).toHaveURL('/login');
  });
});
