/**
 * E2E Tests for CDSP 79 Homepage
 * Tests the main landing page functionality
 */

import { test, expect } from '@playwright/test';

test.describe('CDSP 79 Homepage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should load homepage successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/CDSP 79|Palet/);
    await expect(page.locator('main')).toBeVisible();
    await expect(page).toHaveScreenshot('homepage-load.png');
  });

  test('should display header with navigation', async ({ page }) => {
    // Check for main header elements
    const header = page.locator('header');
    await expect(header).toBeVisible();
    
    // Check for logo or branding
    const logo = page.locator('[alt*="CDSP" i]');
    await expect(logo).toBeVisible();
    await expect(page).toHaveScreenshot('header-navigation.png');
  });

  test('should display hero section with championship info', async ({ page }) => {
    const hero = page.locator('section').filter({ hasText: /CHAMPIONNAT|DÉPARTEMENTAL/ });
    await expect(hero).toBeVisible();
    
    // Check for featured match
    const featuredMatch = page.locator('.text-brand-green-400').filter({ hasText: /EN DIRECT/i });
    await expect(featuredMatch).toBeVisible();
  });

  test('should display live scores ticker when matches are live', async ({ page }) => {
    const liveScores = page.locator('section').filter({ hasText: /Scores en Direct/i });
    await expect(liveScores).toBeVisible();
  });

  test('should display classification table', async ({ page }) => {
    const classification = page.locator('section').filter({ hasText: /Classement/i });
    await expect(classification).toBeVisible();
    
    // Check for table headers
    const tableHeaders = page.locator('th').filter({ hasText: /Position|Équipe|Joués|Victoires|Points/ });
    await expect(tableHeaders).toBeVisible();
  });

  test('should display tournaments section', async ({ page }) => {
    const tournaments = page.locator('section').filter({ hasText: /Compétitions|Tournois/ });
    await expect(tournaments).toBeVisible();
  });

  test('should display news section', async ({ page }) => {
    const news = page.locator('section').filter({ hasText: /Actualités/ });
    await expect(news).toBeVisible();
  });

  test('should display partners section', async ({ page }) => {
    const partners = page.locator('section').filter({ hasText: /Partenaires/ });
    await expect(partners).toBeVisible();
  });

  test('should display footer', async ({ page }) => {
    const footer = page.locator('footer');
    await expect(footer).toBeVisible();
  });

  test('should have responsive design - mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.reload();
    
    // Check mobile menu button
    const mobileMenu = page.locator('button').filter({ hasText: /Menu|☰/ });
    await expect(mobileMenu).toBeVisible();
    
    // Check content is still visible
    const mainContent = page.locator('main');
    await expect(mainContent).toBeVisible();
  });

  test('should have responsive design - tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.reload();
    
    // Check main content is visible
    const mainContent = page.locator('main');
    await expect(mainContent).toBeVisible();
  });

  test('should navigate to clubs page', async ({ page }) => {
    const clubsLink = page.getByRole('link', { name: /Clubs|clubs/i });
    await clubsLink.click();
    await expect(page).toHaveURL('/clubs');
    await expect(page.locator('h1')).toContainText(/Clubs|Annuaire/);
  });

  test('should navigate to players page', async ({ page }) => {
    const playersLink = page.getByRole('link', { name: /Joueurs|joueurs/i });
    await playersLink.click();
    await expect(page).toHaveURL('/joueurs');
    await expect(page.locator('h1')).toContainText(/Joueurs/);
  });

  test('should navigate to championships page', async ({ page }) => {
    const championshipsLink = page.getByRole('link', { name: /Championnats|championnats/i });
    await championshipsLink.click();
    await expect(page).toHaveURL('/championnats');
    await expect(page.locator('h1')).toContainText(/Championnats/);
  });

  test('should navigate to tournaments page', async ({ page }) => {
    const tournamentsLink = page.getByRole('link', { name: /Tournois|tournois/i });
    await tournamentsLink.click();
    await expect(page).toHaveURL('/tournois');
    await expect(page.locator('h1')).toContainText(/Tournois/);
  });

  test('should have proper color scheme', async ({ page }) => {
    // Check for CDSP 79 brand colors
    const bgColor = await page.locator('body').evaluate(el => 
      window.getComputedStyle(el).backgroundColor
    );
    // Should be dark background
    expect(bgColor).toMatch(/rgb\(0, 0, 0\)|#000000/);
    
    // Check for gold accent color
    const goldElements = page.locator('.text-brand-orange-pure, .bg-brand-orange-pure');
    await expect(goldElements).toBeVisible();
    await expect(page).toHaveScreenshot('color-scheme.png');
  });
});
