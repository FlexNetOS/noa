import { test, expect } from '@playwright/test';
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

test.describe('MOE Chat Interface', () => {
  test.use({ storageState: authFile });

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display chat interface', async ({ page }) => {
    // Check for chat input
    const chatInput = page.locator('textarea[placeholder*="message"]').or(page.locator('input[placeholder*="message"]'));
    await expect(chatInput.first()).toBeVisible();
    
    // Check for send button
    const sendButton = page.locator('button[type="submit"]').last();
    await expect(sendButton).toBeVisible();
  });

  test('should send a message', async ({ page }) => {
    // Find chat input
    const chatInput = page.locator('textarea[placeholder*="message"]').or(page.locator('input[placeholder*="message"]')).first();
    
    // Type a message
    await chatInput.fill('Hello, test message');
    
    // Send message
    const sendButton = page.locator('button[type="submit"]').last();
    await sendButton.click();
    
    // Verify message appears in chat
    await expect(page.getByText('Hello, test message')).toBeVisible({ timeout: 10000 });
  });

  test('should show loading state while processing', async ({ page }) => {
    // Find chat input
    const chatInput = page.locator('textarea[placeholder*="message"]').or(page.locator('input[placeholder*="message"]')).first();
    
    // Type a message
    await chatInput.fill('Test loading state');
    
    // Send message
    const sendButton = page.locator('button[type="submit"]').last();
    await sendButton.click();
    
    // Check for loading indicator (spinner, disabled state, etc.)
    const loadingIndicator = page.locator('[class*="animate-spin"]').or(page.locator('[disabled]'));
    await expect(loadingIndicator.first()).toBeVisible({ timeout: 1000 });
  });
});
