# Playwright Patterns Reference

Quick reference for common Playwright patterns and APIs.

## Setup & Configuration

### playwright.config.ts
```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,

  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },

  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
    { name: 'mobile', use: { ...devices['iPhone 13'] } },
  ],

  // Auto-start dev server
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

## Selector Cheat Sheet

| Method | Use Case | Example |
|--------|----------|---------|
| `getByRole` | Interactive elements | `getByRole('button', { name: 'Submit' })` |
| `getByTestId` | Explicit test hooks | `getByTestId('submit-btn')` |
| `getByText` | Visible text | `getByText('Welcome')` |
| `getByLabel` | Form inputs | `getByLabel('Email')` |
| `getByPlaceholder` | Placeholder text | `getByPlaceholder('Enter email')` |
| `getByAltText` | Images | `getByAltText('User avatar')` |
| `getByTitle` | Title attribute | `getByTitle('Settings')` |
| `locator` | CSS/XPath fallback | `locator('.custom-class')` |

## Role Query Options

```typescript
// Common roles
page.getByRole('button', { name: 'Submit' })
page.getByRole('link', { name: 'Home' })
page.getByRole('heading', { level: 1 })
page.getByRole('textbox', { name: 'Email' })
page.getByRole('checkbox', { name: 'Agree' })
page.getByRole('radio', { name: 'Option A' })
page.getByRole('combobox', { name: 'Country' })
page.getByRole('listbox')
page.getByRole('option', { name: 'USA' })
page.getByRole('dialog')
page.getByRole('alert')
page.getByRole('navigation')
page.getByRole('main')
page.getByRole('img', { name: 'Logo' })

// With states
page.getByRole('button', { name: 'Submit', disabled: true })
page.getByRole('checkbox', { checked: true })
page.getByRole('button', { pressed: true })
page.getByRole('option', { selected: true })
```

## Locator Filters & Chaining

```typescript
// Filter by text
page.getByRole('listitem').filter({ hasText: 'Product A' })

// Filter by nested locator
page.getByRole('listitem').filter({ has: page.getByRole('button') })

// Filter by NOT having
page.getByRole('listitem').filter({ hasNot: page.getByRole('checkbox', { checked: true }) })

// Chain locators
page.getByTestId('sidebar').getByRole('link', { name: 'Settings' })

// Nth element
page.getByRole('listitem').nth(0)
page.getByRole('listitem').first()
page.getByRole('listitem').last()
```

## Actions

```typescript
// Click
await element.click()
await element.dblclick()
await element.click({ button: 'right' })
await element.click({ modifiers: ['Shift'] })

// Fill (clears first)
await element.fill('text')

// Type (character by character)
await element.type('text', { delay: 100 })

// Press keys
await element.press('Enter')
await element.press('Control+A')
await page.keyboard.press('Escape')

// Select options
await element.selectOption('value')
await element.selectOption({ label: 'Option' })
await element.selectOption(['opt1', 'opt2'])

// Check/Uncheck
await element.check()
await element.uncheck()
await element.setChecked(true)

// Hover
await element.hover()

// Focus
await element.focus()

// Drag and drop
await source.dragTo(target)

// Upload files
await element.setInputFiles('path/to/file.pdf')
await element.setInputFiles(['file1.pdf', 'file2.pdf'])
```

## Assertions

```typescript
// Visibility
await expect(element).toBeVisible()
await expect(element).toBeHidden()
await expect(element).toBeAttached()

// Text
await expect(element).toHaveText('exact text')
await expect(element).toContainText('partial')
await expect(element).toHaveText(/regex/)

// Input values
await expect(input).toHaveValue('value')
await expect(input).toBeEmpty()

// State
await expect(element).toBeEnabled()
await expect(element).toBeDisabled()
await expect(checkbox).toBeChecked()
await expect(element).toBeFocused()
await expect(element).toBeEditable()

// Count
await expect(list.getByRole('listitem')).toHaveCount(5)

// Attributes
await expect(element).toHaveAttribute('href', '/link')
await expect(element).toHaveClass('active')
await expect(element).toHaveId('main')
await expect(element).toHaveCSS('color', 'rgb(0, 0, 0)')

// Page
await expect(page).toHaveURL('/path')
await expect(page).toHaveURL(/\/users\/\d+/)
await expect(page).toHaveTitle('Title')

// Soft assertions (don't stop on failure)
await expect.soft(element).toHaveText('text')

// Negation
await expect(element).not.toBeVisible()

// Timeout
await expect(element).toBeVisible({ timeout: 10000 })
```

## Waiting

```typescript
// Auto-waits built into actions
await page.click('button') // waits for button to be actionable

// Explicit waits
await page.waitForURL('/dashboard')
await page.waitForLoadState('load')
await page.waitForLoadState('domcontentloaded')
await page.waitForLoadState('networkidle')

// Wait for element
await page.waitForSelector('.dynamic')
await element.waitFor({ state: 'visible' })
await element.waitFor({ state: 'hidden' })

// Wait for response
const response = await page.waitForResponse('/api/data')
await page.waitForResponse(resp => resp.url().includes('/api/'))

// Wait for function
await page.waitForFunction(() => document.title === 'Ready')

// Wait for event
await page.waitForEvent('download')

// Timeout
await page.waitForSelector('.item', { timeout: 5000 })
```

## Fixtures & Hooks

```typescript
import { test, expect } from '@playwright/test';

// Before/After hooks
test.beforeAll(async () => {
  // Run once before all tests in file
});

test.afterAll(async () => {
  // Run once after all tests in file
});

test.beforeEach(async ({ page }) => {
  // Run before each test
  await page.goto('/');
});

test.afterEach(async ({ page }) => {
  // Run after each test
});

// Custom fixtures
const test = base.extend<{ userPage: Page }>({
  userPage: async ({ browser }, use) => {
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto('/login');
    await page.fill('#email', 'user@test.com');
    await page.click('button[type="submit"]');
    await use(page);
    await context.close();
  },
});
```

## Test Organization

```typescript
// Grouping
test.describe('Feature', () => {
  test('scenario 1', async ({ page }) => {});
  test('scenario 2', async ({ page }) => {});
});

// Serial execution (run in order)
test.describe.serial('Ordered tests', () => {
  test('first', async ({ page }) => {});
  test('second', async ({ page }) => {});
});

// Skip/focus
test.skip('skipped test', async ({ page }) => {});
test.only('focused test', async ({ page }) => {});
test.fixme('known broken', async ({ page }) => {});

// Conditional skip
test('only on chromium', async ({ page, browserName }) => {
  test.skip(browserName !== 'chromium', 'Chromium only');
});

// Annotations
test('slow test', async ({ page }) => {
  test.slow(); // triples timeout
});

// Tags
test('important @critical', async ({ page }) => {});
// Run with: npx playwright test --grep @critical
```

## API Testing

```typescript
test('API request', async ({ request }) => {
  // GET
  const response = await request.get('/api/users');
  expect(response.ok()).toBeTruthy();
  const data = await response.json();

  // POST
  const created = await request.post('/api/users', {
    data: { name: 'John', email: 'john@test.com' }
  });
  expect(created.status()).toBe(201);

  // With headers
  const authed = await request.get('/api/profile', {
    headers: { 'Authorization': 'Bearer token' }
  });
});
```

## Screenshot & Debugging

```typescript
// Screenshots
await page.screenshot({ path: 'screenshot.png' })
await page.screenshot({ fullPage: true })
await element.screenshot({ path: 'element.png' })

// Visual regression
await expect(page).toHaveScreenshot('homepage.png')
await expect(element).toHaveScreenshot()

// Trace
await context.tracing.start({ screenshots: true, snapshots: true })
await context.tracing.stop({ path: 'trace.zip' })

// Debug mode
await page.pause() // Opens inspector

// Console logs
page.on('console', msg => console.log(msg.text()))
```

## Running Tests

```bash
# Run all tests
npx playwright test

# Run specific file
npx playwright test tests/login.spec.ts

# Run tests matching pattern
npx playwright test -g "login"

# Run with UI mode
npx playwright test --ui

# Run in headed mode
npx playwright test --headed

# Run specific project
npx playwright test --project=chromium

# Run with trace
npx playwright test --trace on

# Debug mode
npx playwright test --debug

# Generate code
npx playwright codegen localhost:3000
```
