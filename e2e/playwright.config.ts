import { defineConfig } from '@playwright/test';

// Suite e2e Palet Vendéen : pages Dioxus servies par `dx serve` (8081),
// API Actix sur 3000. Les deux serveurs sont gérés hors Playwright
// (voir scripts/dev.sh) ; on peut les surcharger via E2E_BASE_URL / E2E_API_URL.
export default defineConfig({
  testDir: './tests',
  timeout: 60_000,
  retries: 0,
  workers: 1,
  reporter: [
    ['list'],
    ['json', { outputFile: 'test-results/results.json' }],
  ],
  use: {
    baseURL: process.env.E2E_BASE_URL ?? 'http://localhost:8081',
    viewport: { width: 1280, height: 800 },
    ignoreHTTPSErrors: true,
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
});
