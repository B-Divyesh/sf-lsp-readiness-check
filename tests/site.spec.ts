import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFile } from 'node:child_process';
import { mkdtemp, readFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { promisify } from 'node:util';

const exec = promisify(execFile);

test('landing explains the job and opens the sample in one click', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('h1')).toHaveText('Verify tooling before an agent edits');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.getByText('5/5')).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
});

test('@claim:sample-probe the demo shows navigation, formatting, tests, and a signed packet', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Run sample probe' }).click();
  await expect(page.locator('#terminal-output')).toContainText('READY — agent edits may start');
  await expect(page.locator('#terminal-output')).toContainText('TypeScript language server');
  await expect(page.locator('#terminal-output')).toContainText('Rust language server');
  await expect(page.locator('#terminal-output')).toContainText('42 tests passed');
  await expect(page.locator('#terminal-output')).toContainText('Signature: Ed25519');
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(['demo:lsp-readiness-check']);
});

test('@claim:local-operation the CLI has no network client and the demo makes no cross-origin request', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Run sample probe' }).click();
  await page.getByRole('button', { name: 'Replay output' }).click();
  const foreign = requests.filter((url) => new URL(url).origin !== 'http://127.0.0.1:4173');
  expect(foreign).toEqual([]);
  const cargo = await readFile(join(process.cwd(), 'Cargo.toml'), 'utf8');
  const library = await readFile(join(process.cwd(), 'src/lib.rs'), 'utf8');
  const command = await readFile(join(process.cwd(), 'src/main.rs'), 'utf8');
  expect(`${cargo}\n${library}`).not.toMatch(/reqwest|hyper|TcpStream|UdpSocket/);
  expect(command).toContain('"--network"');
  expect(command).toContain('"none"');
  expect(command).toContain('"--cap-drop=ALL"');
  expect(command).toContain(':/source:ro');
});

test('@claim:signed-packet the CLI creates a packet whose signature verifies', async () => {
  const directory = await mkdtemp(join(tmpdir(), 'lsp-readiness-claim-'));
  const result = await exec(join(process.cwd(), 'target/release/lsp-readiness'), ['demo']);
  const match = result.stdout.match(/Signed packet: (.+)/);
  expect(match).not.toBeNull();
  const packet = JSON.parse(await readFile(match![1].trim(), 'utf8'));
  expect(packet.algorithm).toBe('Ed25519');
  expect(packet.payload.ready).toBe(true);
  const saved = join(directory, 'packet.json');
  await import('node:fs/promises').then(({ writeFile }) => writeFile(saved, JSON.stringify(packet)));
  const verified = await exec(join(process.cwd(), 'target/release/lsp-readiness'), ['verify', saved, '--json']);
  expect(JSON.parse(verified.stdout)).toEqual({ valid: true, algorithm: 'Ed25519' });
});

test('@claim:offline-demo the demo reloads offline after the first visit', async ({ browser }) => {
  const context = await browser.newContext({ serviceWorkers: 'allow' });
  const page = await context.newPage();
  await page.goto('http://127.0.0.1:4173/demo');
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Review a completed readiness probe');
  await context.close();
});

test('@claim:private-ci-price the plan states its price and included work', async ({ page }) => {
  await page.goto('/#pricing');
  const pricing = page.locator('#pricing');
  await expect(pricing).toContainText('$49 per repository each month');
  await expect(pricing).toContainText('Required checks for private CI');
  await expect(pricing.getByRole('link', { name: /Buy private CI/ })).toHaveAttribute('href', 'https://api.sociobot.in/api/v1/products/lsp-readiness-check/checkout');
});

test('a returned license is stored, verified, and removed from the URL', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/lsp-readiness-check/verify?license=test-license', (route) => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/?license=test-license');
  await expect(page).toHaveURL('http://127.0.0.1:4173/');
  await expect.poll(() => page.evaluate(() => localStorage.getItem('sb_license:lsp-readiness-check'))).toBe('test-license');
  await expect.poll(() => page.evaluate(() => JSON.parse(localStorage.getItem('sb_license_verdict:lsp-readiness-check') ?? '{}').valid)).toBe(true);
});

for (const route of ['/', '/demo', '/privacy', '/terms', '/does-not-exist']) {
  test(`route ${route} has one h1 and no serious accessibility findings`, async ({ page }) => {
    const consoleErrors: string[] = [];
    page.on('console', (message) => { if (message.type() === 'error') consoleErrors.push(message.text()); });
    page.on('pageerror', (error) => consoleErrors.push(error.message));
    await page.goto(route);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
    expect(results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
    expect(consoleErrors).toEqual([]);
  });
}

test('each internal link resolves', async ({ page, request }) => {
  await page.goto('/');
  const links = await page.locator('a').evaluateAll((anchors) => [...new Set(anchors.map((anchor) => anchor.getAttribute('href')).filter((href): href is string => Boolean(href?.startsWith('/'))))]);
  for (const link of links) {
    const response = await request.get(link);
    expect(response.ok(), link).toBe(true);
  }
});

test('keyboard navigation reaches the primary demo action', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('main')).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeFocused();
});

test('mobile layout stays within the viewport', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  const width = await page.evaluate(() => document.documentElement.scrollWidth);
  expect(width).toBeLessThanOrEqual(390);
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
});
