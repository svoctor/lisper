const { test, expect } = require('@playwright/test');

test.beforeEach(async ({ page }) => {
  page.on('pageerror', error => console.error('BROWSER PAGE ERROR:', error.stack || error.message));
  page.on('console', message => {
    if (message.type() === 'error') console.error('BROWSER CONSOLE ERROR:', message.text());
  });
});


test('loads the interpreter and evaluates the sample program', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Lisper');
  const editor = page.locator('textarea');
  await expect(editor).toBeVisible();
  await expect(editor).toHaveValue(/\(def a 10\)/);
  await expect(page.locator('main')).toContainText('21');
});

test('evaluates user-edited Lisp expressions', async ({ page }) => {
  await page.goto('/');
  const editor = page.locator('textarea');
  await expect(editor).toBeVisible();
  await editor.fill('(+ 7 8)');
  await expect(page.locator('main')).toContainText('15');
});

test('supports multiple definitions within one evaluation', async ({ page }) => {
  await page.goto('/');
  const editor = page.locator('textarea');
  await expect(editor).toBeVisible();
  await editor.fill('(def x 12)\n(* x 3)');
  await expect(page.locator('main')).toContainText('36');
});

test('switches between light and dark themes without losing editor functionality', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'light' });
  await page.goto('/');

  const editor = page.locator('textarea');
  const toggle = page.locator('button').filter({ has: page.locator('svg circle') });
  await expect(editor).toBeVisible();
  await expect(toggle).toBeVisible();

  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  const lightBackground = await page.locator('body').evaluate(element =>
    getComputedStyle(element).getPropertyValue('--background').trim()
  );
  expect(lightBackground).toBe('white');

  await toggle.click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  const darkBackground = await page.locator('body').evaluate(element =>
    getComputedStyle(element).getPropertyValue('--background').trim()
  );
  expect(darkBackground).toBe('#252526');

  await editor.fill('(+ 7 8)');
  await expect(page.locator('main')).toContainText('15');

  await toggle.click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await expect(editor).toHaveValue('(+ 7 8)');
});
