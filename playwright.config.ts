import { defineConfig } from '@playwright/test';

const externalBaseURL = process.env.PLAYWRIGHT_BASE_URL;
const apiPort = 8787;
const apiDatabase = `target/playwright-api-${process.pid}.db`;

export default defineConfig({
  testDir: './tests',
  timeout: 30_000,
  retries: 0,
  workers: 1,
  use: {
    baseURL: externalBaseURL ?? 'http://127.0.0.1:4173',
    browserName: 'chromium',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  webServer: externalBaseURL ? undefined : [
    {
      command: 'npm run preview',
      url: 'http://127.0.0.1:4173',
      reuseExistingServer: false,
    },
    {
      command: `DATABASE_PATH=${apiDatabase} API_ORIGIN=http://127.0.0.1:${apiPort} PUBLIC_ORIGIN=http://127.0.0.1:4173 PORT=${apiPort} REQUESTS_PER_MINUTE=40 LSP_READINESS_TEST_AUTH=1 cargo run -p lsp-readiness-api -- serve`,
      url: `http://127.0.0.1:${apiPort}/healthz`,
      reuseExistingServer: false,
      timeout: 120_000,
    },
  ],
});
