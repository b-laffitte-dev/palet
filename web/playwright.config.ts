import { defineConfig, devices } from '@playwright/test';

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv#how-does-it-work
 */
import dotenv from 'dotenv';
dotenv.config({ path: '.env.local' });

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: './e2e',
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [
    ['html', { outputFolder: 'playwright-report' }],
    ['list'],
  ],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: process.env.CI ? 'http://localhost:4173' : 'http://localhost:5173',

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',

    /* Record video only when retrying */
    video: 'retry-with-video',

    /* Take screenshot on every test */
    screenshot: 'on',
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: 'firefox',
      use: { 
        ...devices['Desktop Firefox'],
        headless: true 
      },
    },
  ],

  /* Run your local dev server before starting the tests */
  webServer: {
    command: 'npm run build && npm run preview',
    url: 'http://localhost:4173',
    reuseExistingServer: !process.env.CI,
    timeout: 300000, // 5 minutes for build + preview server to start
  },

  // Screenshots configuration
  expect: {
    toHaveScreenshot: {
      maxDiffPixels: 1000,
    },
  },
});
