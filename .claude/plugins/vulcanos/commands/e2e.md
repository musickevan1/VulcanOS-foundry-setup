---
description: Generate Playwright E2E tests for React/Next.js applications
argument-hint: [target] [--run]
allowed-tools: Read, Write, Glob, Grep, Bash
---

# /e2e - Playwright Test Generator

Generate end-to-end tests for React/Next.js applications using Playwright.

## Usage

```
/e2e                              # Interactive - asks what to test
/e2e login flow                   # Generate tests for described flow
/e2e src/components/Button.tsx    # Generate tests for specific component
/e2e --run                        # Generate and run tests after
```

## Workflow

### 1. Discovery Phase

Before generating tests, analyze the project:

```bash
# Find Playwright config
glob: playwright.config.{ts,js}

# Find existing test patterns
glob: **/*.spec.ts, **/e2e/**/*.ts

# Find app structure (Next.js)
glob: app/**/page.tsx, pages/**/*.tsx

# Find components
glob: src/components/**/*.tsx, components/**/*.tsx
```

### 2. Context Gathering

Read and understand:
- `playwright.config.ts` - baseURL, testDir, browser settings
- Existing tests - learn project conventions (selectors, utilities, page objects)
- Target component/route - identify interactive elements and user flows

### 3. Test Generation

Generate tests following these principles:

**Selector Priority (best to worst):**
1. `page.getByRole()` - accessible, resilient
2. `page.getByTestId()` - explicit test hooks
3. `page.getByText()` - visible content
4. `page.locator()` - CSS/XPath (last resort)

**Test Structure:**
```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/path');
  });

  test('should do expected behavior', async ({ page }) => {
    // Arrange - setup if needed

    // Act - user interactions
    await page.getByRole('button', { name: 'Submit' }).click();

    // Assert - verify outcomes
    await expect(page.getByText('Success')).toBeVisible();
  });

  test('should handle error case', async ({ page }) => {
    // Test error/edge cases
  });
});
```

### 4. Validation (if --run specified)

```bash
npx playwright test path/to/generated.spec.ts
```

Report results and iterate on failures.

## Test Patterns by Type

### Page/Route Tests
- Navigation works
- Content renders
- Interactive elements function
- Forms submit correctly
- Error states display

### Component Tests
- Renders with different props
- User interactions work
- State changes correctly
- Accessibility basics

### Flow Tests (login, checkout, etc.)
- Happy path end-to-end
- Validation errors
- Edge cases
- Session/auth handling

## Output Location

Place generated tests in:
1. Project's existing test directory (from playwright.config.ts testDir)
2. Default: `tests/` or `e2e/` directory
3. Next to component if component test: `Component.spec.ts`

## Ask the User

If target is ambiguous, ask:
- "What user flow should this test cover?"
- "Should I test happy path only, or include error cases?"
- "Where should I put the test file?"
