---
name: e2e-testing
description: This skill should be used when the user asks to "generate e2e tests", "create playwright tests", "write end-to-end tests", "e2e test for", "playwright test for", "integration tests for React", "integration tests for Next.js", or needs guidance on Playwright test structure, page object patterns, or E2E testing best practices.
version: 1.0.0
---

# E2E Testing Skill - Playwright for React/Next.js

You are an expert in writing Playwright end-to-end tests for React and Next.js applications. Generate high-quality, maintainable tests that follow best practices.

## Core Principles

1. **User-centric tests** - Test what users see and do, not implementation details
2. **Resilient selectors** - Use accessible queries that survive refactoring
3. **Isolated tests** - Each test should be independent and repeatable
4. **Clear assertions** - Test visible outcomes, not internal state
5. **Meaningful coverage** - Happy paths + critical error cases

## Discovery Workflow

Before generating tests, gather project context:

```
1. Find playwright.config.ts → baseURL, testDir, webServer config
2. Find existing tests (*.spec.ts) → learn project conventions
3. Find app structure → routes, components, layouts
4. Find test utilities → page objects, fixtures, helpers
```

## Selector Strategy (Priority Order)

### 1. Role Queries (Preferred)
```typescript
// Buttons, links, headings - accessible and resilient
page.getByRole('button', { name: 'Submit' })
page.getByRole('link', { name: 'Dashboard' })
page.getByRole('heading', { level: 1 })
page.getByRole('textbox', { name: 'Email' })
page.getByRole('checkbox', { name: 'Remember me' })
```

### 2. Test IDs (Explicit)
```typescript
// When role queries aren't specific enough
page.getByTestId('login-form')
page.getByTestId('user-avatar')
```

### 3. Text/Label (Visible Content)
```typescript
// For content that's visible to users
page.getByText('Welcome back')
page.getByLabel('Password')
page.getByPlaceholder('Enter your email')
```

### 4. Locators (Last Resort)
```typescript
// Only when above options fail
page.locator('.custom-dropdown')
page.locator('[data-state="open"]')
```

## Test Structure Template

```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature: User Authentication', () => {
  // Shared setup
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
  });

  test('should login with valid credentials', async ({ page }) => {
    // Fill form
    await page.getByLabel('Email').fill('user@example.com');
    await page.getByLabel('Password').fill('password123');

    // Submit
    await page.getByRole('button', { name: 'Sign in' }).click();

    // Verify redirect and welcome
    await expect(page).toHaveURL('/dashboard');
    await expect(page.getByRole('heading', { name: 'Welcome' })).toBeVisible();
  });

  test('should show error for invalid credentials', async ({ page }) => {
    await page.getByLabel('Email').fill('wrong@example.com');
    await page.getByLabel('Password').fill('wrongpass');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page.getByText('Invalid credentials')).toBeVisible();
    await expect(page).toHaveURL('/login');
  });

  test('should validate required fields', async ({ page }) => {
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page.getByText('Email is required')).toBeVisible();
  });
});
```

## Assertion Patterns

### Visibility
```typescript
await expect(element).toBeVisible();
await expect(element).toBeHidden();
await expect(element).not.toBeVisible();
```

### Content
```typescript
await expect(element).toHaveText('Expected text');
await expect(element).toContainText('partial');
await expect(page).toHaveTitle('Page Title');
```

### State
```typescript
await expect(element).toBeEnabled();
await expect(element).toBeDisabled();
await expect(element).toBeChecked();
await expect(input).toHaveValue('entered text');
```

### Navigation
```typescript
await expect(page).toHaveURL('/expected/path');
await expect(page).toHaveURL(/\/users\/\d+/);
```

### Count
```typescript
await expect(page.getByRole('listitem')).toHaveCount(5);
```

## Waiting Patterns

```typescript
// Auto-waiting (built into most actions)
await page.click('button'); // Waits for button to be actionable

// Explicit waits when needed
await page.waitForURL('/dashboard');
await page.waitForResponse('/api/users');
await page.waitForLoadState('networkidle');

// Wait for element state
await expect(element).toBeVisible({ timeout: 10000 });
```

## Page Object Pattern

For complex pages, create reusable page objects:

```typescript
// pages/LoginPage.ts
import { Page, Locator } from '@playwright/test';

export class LoginPage {
  readonly page: Page;
  readonly emailInput: Locator;
  readonly passwordInput: Locator;
  readonly submitButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.emailInput = page.getByLabel('Email');
    this.passwordInput = page.getByLabel('Password');
    this.submitButton = page.getByRole('button', { name: 'Sign in' });
  }

  async goto() {
    await this.page.goto('/login');
  }

  async login(email: string, password: string) {
    await this.emailInput.fill(email);
    await this.passwordInput.fill(password);
    await this.submitButton.click();
  }
}

// Using in tests
test('login flow', async ({ page }) => {
  const loginPage = new LoginPage(page);
  await loginPage.goto();
  await loginPage.login('user@example.com', 'password');
  await expect(page).toHaveURL('/dashboard');
});
```

## Next.js Specific Patterns

### App Router Testing
```typescript
// Test server components render
test('server component renders data', async ({ page }) => {
  await page.goto('/users');
  // Server components should have content immediately
  await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible();
});

// Test loading states
test('shows loading state', async ({ page }) => {
  await page.goto('/slow-page');
  await expect(page.getByText('Loading...')).toBeVisible();
  await expect(page.getByTestId('content')).toBeVisible();
});
```

### API Route Testing
```typescript
test('api route returns data', async ({ request }) => {
  const response = await request.get('/api/users');
  expect(response.ok()).toBeTruthy();
  const data = await response.json();
  expect(data.users).toHaveLength(10);
});
```

### Middleware/Auth Testing
```typescript
test('redirects unauthenticated users', async ({ page }) => {
  await page.goto('/protected');
  await expect(page).toHaveURL('/login?redirect=/protected');
});
```

## Common Patterns by Test Type

### Form Tests
```typescript
test.describe('Contact Form', () => {
  test('submits successfully', async ({ page }) => {
    await page.goto('/contact');
    await page.getByLabel('Name').fill('John Doe');
    await page.getByLabel('Email').fill('john@example.com');
    await page.getByLabel('Message').fill('Hello!');
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.getByText('Message sent!')).toBeVisible();
  });

  test('validates email format', async ({ page }) => {
    await page.goto('/contact');
    await page.getByLabel('Email').fill('invalid-email');
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.getByText('Invalid email')).toBeVisible();
  });
});
```

### Navigation Tests
```typescript
test('navigation menu works', async ({ page }) => {
  await page.goto('/');

  // Test each nav link
  await page.getByRole('link', { name: 'About' }).click();
  await expect(page).toHaveURL('/about');

  await page.getByRole('link', { name: 'Products' }).click();
  await expect(page).toHaveURL('/products');
});
```

### Modal/Dialog Tests
```typescript
test('confirmation modal works', async ({ page }) => {
  await page.goto('/items');
  await page.getByRole('button', { name: 'Delete' }).first().click();

  // Modal appears
  await expect(page.getByRole('dialog')).toBeVisible();
  await expect(page.getByText('Are you sure?')).toBeVisible();

  // Confirm deletion
  await page.getByRole('button', { name: 'Confirm' }).click();
  await expect(page.getByRole('dialog')).toBeHidden();
});
```

## Output Guidelines

1. **File naming**: `feature-name.spec.ts` or match existing convention
2. **Location**: Use project's testDir or standard `tests/e2e/`
3. **Imports**: Use `@playwright/test` unless project has custom setup
4. **Comments**: Add brief comments explaining non-obvious test logic

## When to Ask User

- "What user flows should this test cover?"
- "Should I include error/edge case tests?"
- "Does this app have authentication I should account for?"
- "Are there existing test utilities or page objects I should use?"
