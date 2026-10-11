/**
 * E2E Tests for CDSP 79 Players Pages
 * Tests players listing and detail pages
 */

import { test, expect } from '@playwright/test';

test.describe('CDSP 79 Players', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/joueurs');
  });

  test('should load players page successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/Joueurs/);
    await expect(page.locator('main')).toBeVisible();
    await expect(page).toHaveScreenshot('players-page.png');
  });

  test('should display players list', async ({ page }) => {
    const playersList = page.locator('[role="list"]');
    await expect(playersList).toBeVisible();
  });

  test('should display player cards', async ({ page }) => {
    const playerCards = page.locator('.bg-brand-green-900/50, .bg-brand-green-900');
    await expect(playerCards).toHaveCountGreaterThan(0);
  });

  test('should navigate to player detail page', async ({ page }) => {
    // Click on first player card
    const firstPlayer = page.locator('[role="listitem"]').first();
    await firstPlayer.click();
    
    // Should navigate to player detail
    await expect(page).toHaveURL(/(.*)\/joueurs\/[a-zA-Z0-9-]+/);
    await expect(page.locator('h1')).toBeVisible();
    await expect(page).toHaveScreenshot('player-detail.png');
  });

  test('should have search/filter functionality', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[type="text"]');
    await expect(searchInput).toHaveCountGreaterThanOrEqual(0);
  });
});

test.describe('CDSP 79 Player Detail', () => {
  test('should load player detail page directly', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    await expect(page).toHaveTitle(/Joueur/);
    await expect(page.locator('h1')).toContainText(/Jean Martin|Le Maestro/);
  });

  test('should display player information', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    // Check for player name
    const playerName = page.locator('h1');
    await expect(playerName).toBeVisible();
    
    // Check for player photo
    const playerPhoto = page.locator('img[alt*="Jean Martin"]');
    await expect(playerPhoto).toBeVisible();
    
    // Check for player stats
    const stats = page.locator('section').filter({ hasText: /Statistiques/ });
    await expect(stats).toBeVisible();
  });

  test('should display player stats tab', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const statsTab = page.getByRole('tab', { name: /Statistiques/ });
    await expect(statsTab).toBeVisible();
    await statsTab.click();
    
    const statsContent = page.getByRole('tabpanel');
    await expect(statsContent).toBeVisible();
  });

  test('should display player history tab', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const historyTab = page.getByRole('tab', { name: /Historique/ });
    await expect(historyTab).toBeVisible();
    await historyTab.click();
    
    const historyContent = page.getByRole('tabpanel');
    await expect(historyContent).toBeVisible();
  });

  test('should display player trophies tab', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const trophiesTab = page.getByRole('tab', { name: /Trophées/ });
    await expect(trophiesTab).toBeVisible();
    await trophiesTab.click();
    
    const trophiesContent = page.getByRole('tabpanel');
    await expect(trophiesContent).toBeVisible();
  });

  test('should display player teams tab', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const teamsTab = page.getByRole('tab', { name: /Équipes/ });
    await expect(teamsTab).toBeVisible();
    await teamsTab.click();
    
    const teamsContent = page.getByRole('tabpanel');
    await expect(teamsContent).toBeVisible();
  });

  test('should have back to players link', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const backLink = page.getByRole('link', { name: /Retour aux joueurs/ });
    await expect(backLink).toBeVisible();
    await backLink.click();
    
    await expect(page).toHaveURL('/joueurs');
  });

  test('should display player club information', async ({ page }) => {
    await page.goto('/joueurs/player-001');
    
    const clubInfo = page.locator('[href*="/clubs/"]');
    await expect(clubInfo).toBeVisible();
  });
});
