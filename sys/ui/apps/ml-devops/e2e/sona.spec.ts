import { test, expect } from '@playwright/test';
import path from 'path';

const authFile = path.join(__dirname, '../.auth/user.json');

test.describe('SONA Orchestration', () => {
  test.use({ storageState: authFile });

  test.beforeEach(async ({ page }) => {
    await page.goto('/sona');
  });

  test('should display SONA dashboard', async ({ page }) => {
    await expect(page.getByText('SONA Orchestration')).toBeVisible();
    await expect(page.getByText('Sequential Orchestration for Neural Agents')).toBeVisible();
  });

  test('should show workflow templates', async ({ page }) => {
    // Check for templates section
    await expect(page.getByText('Workflow Templates')).toBeVisible();
    
    // Check for at least one template
    const templates = page.locator('[class*="card"]').filter({ hasText: 'Paper' }).or(
      page.locator('[class*="card"]').filter({ hasText: 'Web' })
    );
    await expect(templates.first()).toBeVisible();
  });

  test('should display statistics', async ({ page }) => {
    // Check for stats cards
    await expect(page.getByText(/\d+.*templates/i)).toBeVisible();
    await expect(page.getByText(/\d+.*strategies/i)).toBeVisible();
    await expect(page.getByText(/\d+.*agents/i)).toBeVisible();
  });

  test('should navigate between tabs', async ({ page }) => {
    // Look for tabs
    const overviewTab = page.locator('[role="tab"]').filter({ hasText: 'Overview' });
    const workflowsTab = page.locator('[role="tab"]').filter({ hasText: 'Workflows' }).or(
      page.locator('[role="tab"]').filter({ hasText: 'Builder' })
    );
    
    if (await workflowsTab.count() > 0) {
      await workflowsTab.first().click();
      await expect(workflowsTab.first()).toHaveAttribute('data-state', 'active');
    }
  });
});
