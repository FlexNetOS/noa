import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Start from the login page for each test
    await page.goto('/login');
  });

  test('should display login page correctly', async ({ page }) => {
    // Check for key elements
    await expect(page.getByText('Welcome Back')).toBeVisible();
    await expect(page.getByText('Sign in to ML DevOps Platform')).toBeVisible();
    await expect(page.getByRole('button', { name: /sign in/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /continue with google/i })).toBeVisible();
  });

  test('should show error for invalid credentials', async ({ page }) => {
    // Fill in invalid credentials
    await page.fill('input[type="email"]', 'invalid@example.com');
    await page.fill('input[type="password"]', 'wrongpassword');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Wait for error message
    await expect(page.getByText(/invalid email or password/i)).toBeVisible();
  });

  test('should successfully login with valid credentials', async ({ page }) => {
    // Fill in valid credentials
    await page.fill('input[type="email"]', 'john@doe.com');
    await page.fill('input[type="password"]', 'johndoe123');
    
    // Submit form
    await page.click('button[type="submit"]');
    
    // Wait for redirect to home page
    await page.waitForURL('/');
    await expect(page).toHaveURL('/');
  });

  test('should navigate to signup page', async ({ page }) => {
    // Click signup link
    await page.click('a[href="/signup"]');
    
    // Verify we're on signup page
    await expect(page).toHaveURL('/signup');
    await expect(page.getByText('Create Account')).toBeVisible();
  });
});

test.describe('Signup Flow', () => {
  test('should display signup page correctly', async ({ page }) => {
    await page.goto('/signup');
    
    await expect(page.getByText('Create Account')).toBeVisible();
    await expect(page.getByText('Get started with ML DevOps Platform')).toBeVisible();
    await expect(page.getByRole('button', { name: /create account/i })).toBeVisible();
  });

  test('should validate required fields', async ({ page }) => {
    await page.goto('/signup');
    
    // Try to submit without filling fields
    await page.click('button[type="submit"]');
    
    // HTML5 validation should prevent submission
    const emailInput = page.locator('input[type="email"]');
    await expect(emailInput).toHaveAttribute('required', '');
  });

  test('should navigate back to login', async ({ page }) => {
    await page.goto('/signup');
    
    // Click login link
    await page.click('a[href="/login"]');
    
    // Verify we're on login page
    await expect(page).toHaveURL('/login');
    await expect(page.getByText('Welcome Back')).toBeVisible();
  });
});
