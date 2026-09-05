import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFile, spawn } from 'node:child_process';
import { chmod, mkdir, mkdtemp, readFile, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { promisify } from 'node:util';
import routeMetadata from '../site/route-metadata.json' with { type: 'json' };

const exec = promisify(execFile);
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? 'http://127.0.0.1:4173';
const binary = join(process.cwd(), 'target/release/lsp-readiness');
const pinnedImage = 'registry.example/readiness@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const publicRouteMetadata = Object.entries(routeMetadata).filter(([path]) => path !== '/404');

function metaContent(html: string, attribute: 'name' | 'property', value: string): string | undefined {
  return html.match(new RegExp(`<meta ${attribute}="${value}" content="([^"]*)"`))?.[1];
}

type ClosedRun = { code: number; stdout: string; stderr: string; elapsed: number };

function runWithClosedStdin(command: string, args: string[], env: NodeJS.ProcessEnv): Promise<ClosedRun> {
  return new Promise((resolve, reject) => {
    const started = Date.now();
    const child = spawn(command, args, { env, stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => { stdout += chunk; });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    const timeout = setTimeout(() => {
      child.kill('SIGKILL');
      reject(new Error(`${args[0] ?? command} did not finish with stdin closed`));
    }, 3_000);
    child.on('error', (error) => { clearTimeout(timeout); reject(error); });
    child.on('close', (code) => {
      clearTimeout(timeout);
      resolve({ code: code ?? 2, stdout, stderr, elapsed: Date.now() - started });
    });
  });
}

async function fakeRuntime(directory: string): Promise<string> {
  const runtime = join(directory, 'capture-runtime');
  const payload = JSON.stringify({
    schema: 'https://lsp-readiness-check.sociobot.in/schema/v1', repository: 'repository', generated_at: 1,
    ready: true, languages: ['JavaScript / TypeScript'],
    capabilities: [
      { kind: 'lsp', name: 'TypeScript language server', command: 'typescript-language-server --stdio', status: 'ready', evidence: 'fixture response' },
      { kind: 'formatter', name: 'prettier', command: 'prettier', status: 'ready', evidence: 'fixture response' },
      { kind: 'tests', name: 'Repository tests', command: 'npm test', status: 'ready', evidence: 'fixture response' },
    ], source_digest: 'sha256:fixture',
  });
  await writeFile(runtime, `#!/bin/sh\nif [ -n "${'${LSP_READINESS_CAPTURE:-}'}" ]; then /usr/bin/printf '%s\\n' "$@" > "$LSP_READINESS_CAPTURE"; fi\n/bin/printf '%s\\n' '${payload}'\n`);
  await chmod(runtime, 0o700);
  return runtime;
}

async function checkWithCommandTraps(): Promise<{ source: string; before: string; marker: string; capture: string; key: string; result: ClosedRun }> {
  const boundary = await mkdtemp(join(tmpdir(), 'lsp-readiness-command-traps-'));
  const repository = join(boundary, 'repository');
  const traps = join(boundary, 'traps');
  const source = join(repository, 'source.ts');
  const marker = join(boundary, 'trap-ran');
  const capture = join(boundary, 'runtime-arguments');
  const key = join(boundary, 'signing.key');
  await mkdir(repository);
  await mkdir(traps);
  await writeFile(source, 'export const untouched = true;\n');
  await writeFile(marker, '');
  for (const command of ['npm', 'pnpm', 'yarn', 'bun', 'cargo', 'pip', 'pip3', 'poetry', 'uv', 'go', 'composer', 'typescript-language-server', 'rust-analyzer', 'prettier', 'rustfmt']) {
    const trap = join(traps, command);
    await writeFile(trap, '#!/bin/sh\n/bin/printf "%s\\n" "$0" >> "$LSP_READINESS_TRAP_LOG"\nexit 97\n');
    await chmod(trap, 0o700);
  }
  const runtime = await fakeRuntime(boundary);
  const before = await readFile(source, 'utf8');
  const result = await runWithClosedStdin(binary, [
    'check', repository, '--image', pinnedImage, '--runtime', runtime,
    '--output', join(boundary, 'packet.json'), '--key', key, '--json',
  ], { PATH: traps, LSP_READINESS_TRAP_LOG: marker, LSP_READINESS_CAPTURE: capture });
  return { source, before, marker, capture, key, result };
}

test('landing explains the job and opens the sample in one click', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('h1')).toHaveText('Verify tooling before an agent edits');
  await expect(page.locator('.hero-copy .eyebrow')).toHaveText('Repository check · command-line tool');
  await expect(page.getByRole('heading', { name: 'Signed JSON readiness report' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'How the repository check works' })).toBeVisible();
  await expect(page.getByText('Its signature makes tampering detectable (Ed25519).')).toBeVisible();
  await expect(page.getByText('The normal check uses a network-disabled container made from the exact development image you choose.')).toBeVisible();
  await expect(page.getByText('Use an image address with a SHA-256 digest so the same tools run each time.')).toBeVisible();
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/?demo=1$/);
  await expect(page.getByText('5/5')).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
});

test('the direct demo URL keeps isolated storage, a reset control, and a way back to real data', async ({ page }) => {
  await page.goto('/?demo=1');
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Start for real' })).toBeVisible();
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(['demo:lsp-readiness-check']);
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Review a completed readiness probe');
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(['demo:lsp-readiness-check']);
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL(/\/$/);
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual([]);
});

test('@claim:no-account the website and CLI demo run without credentials or an authentication request', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/?demo=1');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.getByRole('button', { name: 'Run sample probe' }).click();
  const result = await runWithClosedStdin(binary, ['demo'], { PATH: process.env.PATH ?? '' });
  expect(result.code, result.stderr).toBe(0);
  expect(result.stdout).toContain('READY — agent edits may start');
  expect(requests.filter((url) => /auth|login|account/i.test(new URL(url).pathname))).toEqual([]);
  expect(requests.every((url) => new URL(url).origin === new URL(baseURL).origin)).toBe(true);
});

test('@claim:sample-probe the shipped fixture runs 42 tests and produces the displayed signed packet', async ({ page }) => {
  const binary = join(process.cwd(), 'target/release/lsp-readiness');
  const fixtureTests = await exec('npm', ['test', '--prefix', 'examples/northstar-api']);
  expect(fixtureTests.stdout).toContain('# pass 42');
  const generated = JSON.parse((await exec(binary, ['demo', '--json'])).stdout);
  const published = JSON.parse(await readFile(join(process.cwd(), 'site/public/sample/northstar-api.lsp-readiness.json'), 'utf8'));
  expect(generated.payload.ready).toBe(true);
  expect(generated.payload.source_digest).toBe(published.payload.source_digest);
  expect(generated.payload.capabilities).toEqual(expect.arrayContaining([
    expect.objectContaining({ kind: 'tests', status: 'ready', evidence: '42 tests passed' }),
  ]));
  await exec(binary, ['verify', join(process.cwd(), 'site/public/sample/northstar-api.lsp-readiness.json'), '--json']);
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Run sample probe' }).click();
  await expect(page.locator('#terminal-output')).toContainText('READY — agent edits may start');
  await expect(page.locator('#terminal-output')).toContainText('TypeScript language server');
  await expect(page.locator('#terminal-output')).toContainText('Rust language server');
  await expect(page.locator('#terminal-output')).toContainText('42 tests passed');
  await expect(page.locator('#terminal-output')).toContainText('Tamper check: Ed25519 signature');
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(['demo:lsp-readiness-check']);
});

test('container rejects a mutable image before starting a runtime', async () => {
  const binary = join(process.cwd(), 'target/release/lsp-readiness');
  await expect(exec(binary, ['container', '.', '--image', 'ubuntu:latest', '--runtime', 'definitely-not-a-container-runtime']))
    .rejects.toMatchObject({ stderr: expect.stringContaining('immutable sha256 digest') });
});

test('@claim:local-operation normal checks use a locked-down container and the demo makes no cross-origin request', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Run sample probe' }).click();
  await page.getByRole('button', { name: 'Replay output' }).click();
  const foreign = requests.filter((url) => new URL(url).origin !== new URL(baseURL).origin);
  expect(foreign).toEqual([]);
  const checked = await checkWithCommandTraps();
  expect(checked.result.code, checked.result.stderr).toBe(0);
  const runtimeArguments = await readFile(checked.capture, 'utf8');
  expect(runtimeArguments).toContain('--network\nnone\n');
  expect(runtimeArguments).toContain('--read-only\n');
  expect(runtimeArguments).toContain('--cap-drop=ALL\n');
  expect(runtimeArguments).toContain('--security-opt=no-new-privileges\n');
  expect(runtimeArguments).toContain(':/source:ro\n');
  expect(await readFile(checked.source, 'utf8')).toBe(checked.before);
  const symlinkRegressions = await exec('cargo', ['test', 'repository_scan_skips']);
  expect(symlinkRegressions.stdout).toContain('2 passed');
});

test('@claim:no-tool-install a normal check never runs tool installers or changes the source mount', async () => {
  const checked = await checkWithCommandTraps();
  expect(checked.result.code, checked.result.stderr).toBe(0);
  expect(await readFile(checked.marker, 'utf8')).toBe('');
  expect(await readFile(checked.source, 'utf8')).toBe(checked.before);
});

test('@claim:no-dependency-install a normal check never runs dependency installers or changes the source mount', async () => {
  const checked = await checkWithCommandTraps();
  expect(checked.result.code, checked.result.stderr).toBe(0);
  expect(await readFile(checked.marker, 'utf8')).toBe('');
  expect(await readFile(checked.source, 'utf8')).toBe(checked.before);
});

test('@claim:noninteractive-ci every public CI command completes promptly with stdin closed', async () => {
  const boundary = await mkdtemp(join(tmpdir(), 'lsp-readiness-noninteractive-'));
  const repository = join(boundary, 'repository');
  await mkdir(repository);
  await writeFile(join(repository, 'source.ts'), 'export const checked = true;\n');
  const runtime = await fakeRuntime(boundary);
  const environment = { PATH: process.env.PATH ?? '' };
  const common = ['--image', pinnedImage, '--runtime', runtime, '--output', join(boundary, 'packet.json'), '--key', join(boundary, 'signing.key'), '--json'];
  for (const args of [
    ['check', repository, ...common],
    ['container', repository, ...common],
    ['demo', '--json'],
  ]) {
    const result = await runWithClosedStdin(binary, args, environment);
    expect(result.code, `${args.join(' ')}: ${result.stderr}`).toBe(0);
    expect(result.elapsed).toBeLessThan(3_000);
  }
  const demo = await runWithClosedStdin(binary, ['demo', '--json'], environment);
  expect(demo.code, demo.stderr).toBe(0);
  const packet = join(boundary, 'demo-packet.json');
  await writeFile(packet, demo.stdout);
  const verified = await runWithClosedStdin(binary, ['verify', packet, '--json'], environment);
  expect(verified.code, verified.stderr).toBe(0);
  expect(verified.stdout).toContain('"valid":true');
});

test('@claim:signing-key-permissions a first normal check creates its signing key with owner-only permissions', async () => {
  const checked = await checkWithCommandTraps();
  expect(checked.result.code, checked.result.stderr).toBe(0);
  expect((await stat(checked.key)).mode & 0o777).toBe(0o600);
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
  await page.goto(`${baseURL}/demo`);
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Review a completed readiness probe');
  await context.close();
});

test('a new service worker removes the previous release cache', async ({ browser }) => {
  const context = await browser.newContext({ serviceWorkers: 'allow' });
  const page = await context.newPage();
  await page.goto(`${baseURL}/demo`);
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.evaluate(async () => {
    const previous = await caches.open('lsp-readiness-v2');
    await previous.put('/stale-release', new Response('stale'));
    const registration = await navigator.serviceWorker.getRegistration();
    await registration?.unregister();
  });
  await page.reload();
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await expect.poll(() => page.evaluate(() => caches.keys())).toEqual(['lsp-readiness-v3']);
  await context.close();
});

test('the unavailable paid offer and billing endpoint are not presented', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('Buy private CI')).toHaveCount(0);
  await expect(page.locator('a[href*="api.sociobot.in"]')).toHaveCount(0);
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
    const actionableErrors = route === '/does-not-exist'
      ? consoleErrors.filter((message) => !/Failed to load resource: the server responded with a status of 404/.test(message))
      : consoleErrors;
    expect(actionableErrors).toEqual([]);
  });
}

test('public routes serve their own social metadata before JavaScript and retain it after hydration', async ({ page, request }) => {
  for (const [path, metadata] of publicRouteMetadata) {
    const response = await request.get(path);
    expect(response.ok(), path).toBe(true);
    const html = await response.text();
    const canonical = `https://lsp-readiness-check.sociobot.in${path}`;
    expect(html).toContain(`<title>${metadata.title}</title>`);
    expect(metaContent(html, 'name', 'description')).toBe(metadata.description);
    expect(html).toContain(`<link rel="canonical" href="${canonical}"`);
    expect(metaContent(html, 'property', 'og:title')).toBe(metadata.title);
    expect(metaContent(html, 'property', 'og:description')).toBe(metadata.description);
    expect(metaContent(html, 'property', 'og:url')).toBe(canonical);
    expect(metaContent(html, 'name', 'twitter:title')).toBe(metadata.title);
    expect(metaContent(html, 'name', 'twitter:description')).toBe(metadata.description);
    await page.goto(path);
    await expect(page).toHaveTitle(metadata.title);
    await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', metadata.description);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', canonical);
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', metadata.title);
    await expect(page.locator('meta[property="og:description"]')).toHaveAttribute('content', metadata.description);
    await expect(page.locator('meta[property="og:url"]')).toHaveAttribute('content', canonical);
    await expect(page.locator('meta[name="twitter:title"]')).toHaveAttribute('content', metadata.title);
    await expect(page.locator('meta[name="twitter:description"]')).toHaveAttribute('content', metadata.description);
  }
});

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
  const undersized = await page.locator('a, button').evaluateAll((elements) => elements
    .map((element) => ({ label: element.textContent?.trim(), rect: element.getBoundingClientRect() }))
    .filter(({ rect }) => rect.width > 0 && rect.height > 0 && (rect.width < 44 || rect.height < 44)));
  expect(undersized).toEqual([]);
});

test('mobile demo keeps its banner touch targets and terminal keyboard accessible', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/demo');
  await expect(page.locator('#terminal-output')).toHaveAttribute('tabindex', '0');
  const undersized = await page.locator('a, button').evaluateAll((elements) => elements
    .map((element) => element.getBoundingClientRect())
    .filter((rect) => rect.width > 0 && rect.height > 0 && (rect.width < 44 || rect.height < 44)));
  expect(undersized).toEqual([]);
  const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
  expect(results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
});

test('static hosting routes known pages through the app and returns a real 404 otherwise', async () => {
  const config = JSON.parse(await readFile(join(process.cwd(), 'site/public/staticwebapp.config.json'), 'utf8'));
  expect(config.routes).toEqual(expect.arrayContaining([
    expect.objectContaining({ route: '/demo', rewrite: '/demo/index.html' }),
    expect.objectContaining({ route: '/privacy', rewrite: '/privacy/index.html' }),
    expect.objectContaining({ route: '/terms', rewrite: '/terms/index.html' }),
  ]));
  expect(config.navigationFallback).toBeUndefined();
  expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html', statusCode: 404 });
  const notFound = await readFile(join(process.cwd(), 'site/public/404.html'), 'utf8');
  expect(notFound).not.toContain('http-equiv="refresh"');
});
