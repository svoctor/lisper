const { test, expect } = require('@playwright/test');

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
