import { expect, test, type APIRequestContext } from '@playwright/test';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';

const apiBase = 'http://127.0.0.1:8787/api/v1';
const auth = (subject: string) => ({ Authorization: `Bearer test-${subject}` });

test.skip(Boolean(process.env.PLAYWRIGHT_BASE_URL), 'Local API contract tests use release-disabled test identities.');

async function seedRepository(request: APIRequestContext, subject: string, name: string) {
  const response = await request.post(`${apiBase}/test/repositories`, {
    headers: auth(subject),
    data: { owner: `${subject}-team`, name, private: true },
  });
  expect(response.status(), await response.text()).toBe(201);
  return response.json() as Promise<{ id: string; report_token: string }>;
}

test('the API reports its SQLite migration health and requires authentication', async ({ request }) => {
  const health = await request.get('http://127.0.0.1:8787/healthz');
  expect(health.status()).toBe(200);
  expect(await health.json()).toEqual({ status: 'ok', database: 'ok', schema_version: 1 });
  const repositories = await request.get(`${apiBase}/repositories`);
  expect(repositories.status()).toBe(401);
  expect(await repositories.json()).toMatchObject({ error: 'unauthorized' });
});

test('@claim:tenant-isolation repository IDs and policies cannot cross organization boundaries', async ({ request }) => {
  const repository = await seedRepository(request, 'tenant-a', 'private-api');
  const report = JSON.parse(await readFile(join(process.cwd(), 'site/public/sample/northstar-api.lsp-readiness.json'), 'utf8'));
  const uploaded = await request.post(`${apiBase}/repositories/${repository.id}/runs`, {
    headers: { Authorization: `Report ${repository.report_token}` },
    data: { pull_request: 12, report },
  });
  expect(uploaded.status(), await uploaded.text()).toBe(201);

  const ownerList = await request.get(`${apiBase}/repositories`, { headers: auth('tenant-a') });
  expect(ownerList.status()).toBe(200);
  expect((await ownerList.json()).repositories).toEqual([
    expect.objectContaining({ id: repository.id, owner: 'tenant-a-team', name: 'private-api', private: true }),
  ]);

  const otherList = await request.get(`${apiBase}/repositories`, { headers: auth('tenant-b') });
  expect(otherList.status()).toBe(200);
  expect((await otherList.json()).repositories).toEqual([]);
  const otherExport = await request.get(`${apiBase}/account/export`, { headers: auth('tenant-b') });
  expect(otherExport.status()).toBe(200);
  expect((await otherExport.json()).runs).toEqual([]);

  const guessedRepository = await request.get(`${apiBase}/repositories/${repository.id}`, { headers: auth('tenant-b') });
  expect(guessedRepository.status()).toBe(404);
  const guessedPolicy = await request.put(`${apiBase}/repositories/${repository.id}/policy`, {
    headers: auth('tenant-b'),
    data: { required_lsp: true, required_formatters: true, required_tests: true },
  });
  expect(guessedPolicy.status()).toBe(404);
});

test('@claim:packet-upload-no-source uploads accept a signed report and reject source-shaped fields', async ({ request }) => {
  const repository = await seedRepository(request, 'upload-team', 'northstar-private');
  const report = JSON.parse(await readFile(join(process.cwd(), 'site/public/sample/northstar-api.lsp-readiness.json'), 'utf8'));

  const withSource = structuredClone(report);
  withSource.payload.source_files = [{ path: 'src/secret.ts', content: 'source-sentinel' }];
  const rejected = await request.post(`${apiBase}/repositories/${repository.id}/runs`, {
    headers: { Authorization: `Report ${repository.report_token}` },
    data: { pull_request: 17, external_run_id: 'github-1001', report: withSource },
  });
  expect(rejected.status()).toBe(422);

  const wrongToken = await request.post(`${apiBase}/repositories/${repository.id}/runs`, {
    headers: { Authorization: 'Report lrk_wrong' },
    data: { report },
  });
  expect(wrongToken.status()).toBe(401);

  const accepted = await request.post(`${apiBase}/repositories/${repository.id}/runs`, {
    headers: { Authorization: `Report ${repository.report_token}` },
    data: { pull_request: 17, external_run_id: 'github-1001', report },
  });
  expect(accepted.status(), await accepted.text()).toBe(201);
  expect(await accepted.json()).toMatchObject({ ready: true });

  const exported = await request.get(`${apiBase}/account/export`, { headers: auth('upload-team') });
  expect(exported.status()).toBe(200);
  const body = await exported.json();
  expect(body.runs).toHaveLength(1);
  expect(JSON.stringify(body)).not.toContain('src/secret.ts');
  expect(JSON.stringify(body)).not.toContain('source-sentinel');
});

test('@claim:export-delete an owner can export tenant data and permanently delete it', async ({ request }) => {
  const repository = await seedRepository(request, 'privacy-owner', 'delete-me');
  const policy = await request.put(`${apiBase}/repositories/${repository.id}/policy`, {
    headers: auth('privacy-owner'),
    data: { required_lsp: true, required_formatters: false, required_tests: true },
  });
  expect(policy.status(), await policy.text()).toBe(200);

  const before = await request.get(`${apiBase}/account/export`, { headers: auth('privacy-owner') });
  expect(before.status()).toBe(200);
  expect(await before.json()).toMatchObject({
    repositories: [expect.objectContaining({ id: repository.id, name: 'delete-me' })],
    policies: [expect.objectContaining({ repository_id: repository.id, required_formatters: false })],
  });

  const deleted = await request.delete(`${apiBase}/account`, { headers: auth('privacy-owner') });
  expect(deleted.status()).toBe(204);

  const after = await request.get(`${apiBase}/repositories`, { headers: auth('privacy-owner') });
  expect(after.status()).toBe(200);
  expect((await after.json()).repositories).toEqual([]);
});

test('the authenticated app loads repository policy and CI token controls from the API', async ({ page, request }) => {
  const repository = await seedRepository(request, 'browser-owner', 'browser-private');
  await page.addInitScript(() => sessionStorage.setItem('session:lsp-readiness-check:access-token', 'test-browser-owner'));
  await page.goto(`/app/repositories/${repository.id}/policy`);
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Set repository requirements');
  await expect(page.getByRole('group', { name: 'browser-owner-team/browser-private' })).toBeVisible();
  await page.getByRole('checkbox', { name: 'Require formatters' }).uncheck();
  await page.getByRole('button', { name: 'Save requirements' }).click();
  await expect(page.getByText('Saved policy version 1.')).toBeVisible();
  await page.getByRole('button', { name: 'Create CI report token' }).click();
  await expect(page.getByText('Copy this token now.')).toBeVisible();
  await expect(page.locator('.token-value')).toContainText('lrk_');
});

test('@claim:rate-limit repeated authenticated requests return 429 with Retry-After', async ({ request }) => {
  let limited = null;
  for (let attempt = 0; attempt < 45; attempt += 1) {
    const response = await request.get(`${apiBase}/session`, { headers: auth('rate-limit-team') });
    if (response.status() === 429) {
      limited = response;
      break;
    }
    expect(response.status(), await response.text()).toBe(200);
  }
  expect(limited, 'expected the configured fixed-window allowance to be exhausted').not.toBeNull();
  expect(Number(limited!.headers()['retry-after'])).toBeGreaterThan(0);
  expect(await limited!.json()).toMatchObject({ error: 'rate_limit' });
});
