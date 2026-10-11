/**
 * Global setup for Playwright tests
 * This file is used to perform one-time setup before all tests
 */

import { chromium, FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  // Perform authentication if needed
  // For CDSP 79, we might need to log in for some tests
  const browser = await chromium.launch();
  const page = await browser.newPage();

  // Save signed in state to 'storageState.json'
  // This can be used in tests to avoid logging in repeatedly
  await page.context().storageState({ path: 'storageState.json' });
  await browser.close();
}

export default globalSetup;
