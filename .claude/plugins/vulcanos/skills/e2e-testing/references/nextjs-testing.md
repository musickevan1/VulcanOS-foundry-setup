# Next.js Testing Reference

Playwright patterns specific to Next.js App Router and Pages Router applications.

## Project Setup

### playwright.config.ts for Next.js
```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,

  use: {
    baseURL: process.env.BASE_URL || 'http://localhost:3000',
    trace: 'on-first-retry',
  },

  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
  ],

  // Start Next.js dev server
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
```

### Directory Structure
```
project/
├── app/                    # App Router
│   ├── page.tsx
│   ├── layout.tsx
│   └── users/
│       └── page.tsx
├── e2e/                    # E2E tests
│   ├── home.spec.ts
│   ├── users.spec.ts
│   └── fixtures/
│       └── auth.ts
└── playwright.config.ts
```

## App Router Testing

### Server Components
```typescript
// Server components render immediately - no loading state
test('server component renders data', async ({ page }) => {
  await page.goto('/users');

  // Data should be present (no client-side fetch)
  await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible();
  await expect(page.getByRole('listitem')).toHaveCount(10);
});
```

### Client Components
```typescript
// Client components may have loading states
test('client component loads data', async ({ page }) => {
  await page.goto('/dashboard');

  // May see loading state first
  // Then wait for actual content
  await expect(page.getByTestId('stats')).toBeVisible();
});
```

### Loading States (loading.tsx)
```typescript
test('shows loading state while fetching', async ({ page }) => {
  // Slow down API to observe loading
  await page.route('/api/slow', route =>
    route.fulfill({ delay: 2000, body: '{}' })
  );

  await page.goto('/slow-page');
  await expect(page.getByText('Loading...')).toBeVisible();
  await expect(page.getByTestId('content')).toBeVisible();
});
```

### Error Boundaries (error.tsx)
```typescript
test('shows error boundary on failure', async ({ page }) => {
  // Force API error
  await page.route('/api/users', route =>
    route.fulfill({ status: 500 })
  );

  await page.goto('/users');
  await expect(page.getByText('Something went wrong')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Try again' })).toBeVisible();
});
```

### Not Found (not-found.tsx)
```typescript
test('shows 404 for invalid routes', async ({ page }) => {
  const response = await page.goto('/nonexistent-page');
  expect(response?.status()).toBe(404);
  await expect(page.getByText('Page not found')).toBeVisible();
});
```

### Layouts
```typescript
test('layout persists across navigation', async ({ page }) => {
  await page.goto('/');

  // Layout elements present
  await expect(page.getByRole('navigation')).toBeVisible();
  await expect(page.getByRole('banner')).toBeVisible();

  // Navigate - layout should persist
  await page.getByRole('link', { name: 'About' }).click();
  await expect(page.getByRole('navigation')).toBeVisible();
});
```

### Parallel Routes
```typescript
test('parallel routes render independently', async ({ page }) => {
  await page.goto('/dashboard');

  // Multiple parallel slots
  await expect(page.getByTestId('team-slot')).toBeVisible();
  await expect(page.getByTestId('analytics-slot')).toBeVisible();
});
```

### Intercepting Routes
```typescript
test('modal intercepts route', async ({ page }) => {
  await page.goto('/photos');

  // Click photo - should open modal (intercepted)
  await page.getByRole('link', { name: 'Photo 1' }).click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await expect(page).toHaveURL('/photos/1');

  // Direct navigation - should show full page
  await page.goto('/photos/1');
  await expect(page.getByRole('dialog')).toBeHidden();
  await expect(page.getByTestId('photo-page')).toBeVisible();
});
```

## Route Handlers (API Routes)

### Testing API Routes
```typescript
test.describe('API: /api/users', () => {
  test('GET returns users', async ({ request }) => {
    const response = await request.get('/api/users');
    expect(response.ok()).toBeTruthy();

    const data = await response.json();
    expect(data.users).toBeInstanceOf(Array);
  });

  test('POST creates user', async ({ request }) => {
    const response = await request.post('/api/users', {
      data: { name: 'John', email: 'john@test.com' }
    });
    expect(response.status()).toBe(201);

    const user = await response.json();
    expect(user.id).toBeDefined();
    expect(user.name).toBe('John');
  });

  test('validates required fields', async ({ request }) => {
    const response = await request.post('/api/users', {
      data: { name: '' }
    });
    expect(response.status()).toBe(400);

    const error = await response.json();
    expect(error.message).toContain('email');
  });
});
```

### Mocking API Routes
```typescript
test('handles API errors gracefully', async ({ page }) => {
  // Intercept and mock API response
  await page.route('/api/users', route =>
    route.fulfill({
      status: 500,
      contentType: 'application/json',
      body: JSON.stringify({ error: 'Server error' })
    })
  );

  await page.goto('/users');
  await expect(page.getByText('Failed to load users')).toBeVisible();
});

// Mock with delay
await page.route('/api/users', async route => {
  await new Promise(r => setTimeout(r, 2000));
  await route.fulfill({ body: '[]' });
});

// Mock with dynamic response
await page.route('/api/users/*', route => {
  const userId = route.request().url().split('/').pop();
  route.fulfill({
    body: JSON.stringify({ id: userId, name: 'Test User' })
  });
});
```

## Authentication Patterns

### NextAuth.js Testing
```typescript
// fixtures/auth.ts
import { test as base } from '@playwright/test';

export const test = base.extend({
  // Authenticated page
  authenticatedPage: async ({ page }, use) => {
    // Set auth cookie/session
    await page.goto('/api/auth/signin');
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Password').fill('password');
    await page.getByRole('button', { name: 'Sign in' }).click();
    await page.waitForURL('/dashboard');
    await use(page);
  },
});

// Using authenticated fixture
test('protected route accessible when logged in', async ({ authenticatedPage }) => {
  await authenticatedPage.goto('/settings');
  await expect(authenticatedPage.getByRole('heading', { name: 'Settings' })).toBeVisible();
});
```

### Middleware Auth Testing
```typescript
test('middleware redirects unauthenticated users', async ({ page }) => {
  await page.goto('/protected');
  await expect(page).toHaveURL('/login?callbackUrl=%2Fprotected');
});

test('middleware allows authenticated users', async ({ page, context }) => {
  // Set auth token
  await context.addCookies([{
    name: 'auth-token',
    value: 'valid-token',
    domain: 'localhost',
    path: '/',
  }]);

  await page.goto('/protected');
  await expect(page).toHaveURL('/protected');
});
```

## Server Actions

### Testing Form Actions
```typescript
test('server action submits form', async ({ page }) => {
  await page.goto('/contact');

  await page.getByLabel('Name').fill('John');
  await page.getByLabel('Email').fill('john@test.com');
  await page.getByLabel('Message').fill('Hello!');

  // Submit triggers server action
  await page.getByRole('button', { name: 'Send' }).click();

  // Wait for action to complete (may redirect or show success)
  await expect(page.getByText('Message sent!')).toBeVisible();
});
```

### Testing Optimistic Updates
```typescript
test('shows optimistic update then confirms', async ({ page }) => {
  await page.goto('/todos');

  // Add todo
  await page.getByLabel('New todo').fill('Buy milk');
  await page.getByRole('button', { name: 'Add' }).click();

  // Optimistic: appears immediately (may have pending state)
  await expect(page.getByText('Buy milk')).toBeVisible();

  // After server confirms, pending state removed
  await expect(page.getByTestId('todo-pending')).toBeHidden();
});
```

## Static/Dynamic Rendering

### Testing ISR (Incremental Static Regeneration)
```typescript
test('ISR page updates after revalidation', async ({ page, request }) => {
  // Visit ISR page
  await page.goto('/posts/1');
  const originalContent = await page.getByTestId('content').textContent();

  // Trigger revalidation
  await request.post('/api/revalidate?path=/posts/1');

  // Reload and check for update
  await page.reload();
  // Content may have changed (depends on your data)
});
```

### Testing generateStaticParams
```typescript
test('static paths are generated', async ({ request }) => {
  // These paths should exist (pre-rendered)
  const paths = ['/posts/1', '/posts/2', '/posts/3'];

  for (const path of paths) {
    const response = await request.get(path);
    expect(response.ok()).toBeTruthy();
  }
});

test('dynamic path returns 404 without fallback', async ({ page }) => {
  const response = await page.goto('/posts/99999');
  expect(response?.status()).toBe(404);
});
```

## Image Optimization

```typescript
test('next/image loads optimized images', async ({ page }) => {
  await page.goto('/gallery');

  // Image should use Next.js optimization
  const image = page.getByRole('img', { name: 'Hero' });
  await expect(image).toBeVisible();

  // Check srcset for responsive images
  const srcset = await image.getAttribute('srcset');
  expect(srcset).toContain('_next/image');
});
```

## Internationalization (i18n)

```typescript
test.describe('i18n', () => {
  test('default locale loads', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByText('Welcome')).toBeVisible();
  });

  test('switches to French', async ({ page }) => {
    await page.goto('/fr');
    await expect(page.getByText('Bienvenue')).toBeVisible();
  });

  test('locale switcher works', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('combobox', { name: 'Language' }).selectOption('fr');
    await expect(page).toHaveURL('/fr');
  });
});
```

## Common Next.js Testing Patterns

### Navigation with Link
```typescript
test('Link component navigates without reload', async ({ page }) => {
  await page.goto('/');

  // Listen for navigation (should be client-side)
  const navigationPromise = page.waitForURL('/about');
  await page.getByRole('link', { name: 'About' }).click();
  await navigationPromise;

  // Page didn't fully reload - check layout element still in DOM
  await expect(page.getByRole('navigation')).toBeVisible();
});
```

### Search Params
```typescript
test('search params update URL', async ({ page }) => {
  await page.goto('/products');

  await page.getByRole('combobox', { name: 'Sort' }).selectOption('price-asc');
  await expect(page).toHaveURL(/sort=price-asc/);

  await page.getByLabel('Search').fill('shoes');
  await page.getByLabel('Search').press('Enter');
  await expect(page).toHaveURL(/q=shoes/);
});
```

### useRouter Testing
```typescript
test('programmatic navigation works', async ({ page }) => {
  await page.goto('/form');

  await page.getByLabel('Name').fill('John');
  await page.getByRole('button', { name: 'Submit' }).click();

  // Form handler uses router.push('/success')
  await expect(page).toHaveURL('/success');
});
```
