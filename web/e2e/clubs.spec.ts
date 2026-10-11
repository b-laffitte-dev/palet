/**
 * E2E Tests for CDSP 79 Clubs Pages
 * Tests clubs listing and detail pages
 */

import { test, expect } from '@playwright/test';

test.describe('CDSP 79 Clubs', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/clubs');
  });

  test('should load clubs page successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/Clubs|Annuaire/);
    await expect(page.locator('main')).toBeVisible();
  });

  test('should display clubs list', async ({ page }) => {
    const clubsList = page.locator('[role="list"]');
    await expect(clubsList).toBeVisible();
  });

  test('should display club cards', async ({ page }) => {
    const clubCards = page.locator('.bg-brand-green-900/50, .bg-brand-green-900');
    await expect(clubCards).toHaveCountGreaterThan(0);
  });

  test('should navigate to club detail page', async ({ page }) => {
    // Click on first club card
    const firstClub = page.locator('[role="listitem"]').first();
    await firstClub.click();
    
    // Should navigate to club detail
    await expect(page).toHaveURL(/(.*)\/clubs\/[a-zA-Z0-9-]+/);
    await expect(page.locator('h1')).toBeVisible();
  });
});

test.describe('CDSP 79 Club Detail', () => {
  test('should load club detail page directly', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    await expect(page).toHaveTitle(/Club/);
    await expect(page.locator('h1')).toContainText(/Club/);
  });

  test('should display club information', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    // Check for club name
    const clubName = page.locator('h1');
    await expect(clubName).toBeVisible();
    
    // Check for club stats
    const stats = page.locator('section').filter({ hasText: /Statistiques/ });
    await expect(stats).toBeVisible();
  });

  test('should display club players tab', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    const playersTab = page.getByRole('tab', { name: /Effectif/ });
    await expect(playersTab).toBeVisible();
    await playersTab.click();
    
    const playersContent = page.getByRole('tabpanel');
    await expect(playersContent).toBeVisible();
  });

  test('should display club teams tab', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    const teamsTab = page.getByRole('tab', { name: /Équipes/ });
    await expect(teamsTab).toBeVisible();
    await teamsTab.click();
    
    const teamsContent = page.getByRole('tabpanel');
    await expect(teamsContent).toBeVisible();
  });

  test('should display club classification tab', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    const classificationTab = page.getByRole('tab', { name: /Classement/ });
    await expect(classificationTab).toBeVisible();
    await classificationTab.click();
    
    const classificationContent = page.getByRole('tabpanel');
    await expect(classificationContent).toBeVisible();
  });

  test('should have back to clubs link', async ({ page }) => {
    await page.goto('/clubs/club-001');
    
    const backLink = page.getByRole('link', { name: /Retour aux clubs/ });
    await expect(backLink).toBeVisible();
    await backLink.click();
    
    await expect(page).toHaveURL('/clubs');
  });
});
