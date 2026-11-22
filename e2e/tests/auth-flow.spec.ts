import { test, expect } from '@playwright/test';

// Flux d'auth via l'UI : connexion valide, mauvais mot de passe, inscription.
const API = process.env.E2E_API_URL ?? 'http://localhost:3000';
const TEST_EMAIL = 'test.e2e@example.com';
const TEST_PASSWORD = 'Password123!';

test.beforeAll(async ({ request }) => {
  // s'assurer que le compte de test existe (idempotent)
  await request.post(`${API}/auth/register`, {
    data: {
      email: TEST_EMAIL,
      password: TEST_PASSWORD,
      first_name: 'Test',
      last_name: 'E2E',
      license_number: 'LIC-E2E-1',
    },
  });
});

test('connexion valide -> redirection / et /mon-compte affiche le profil', async ({ page }) => {
  await page.goto('/connexion');
  await page.getByLabel(/Email/i).fill(TEST_EMAIL);
  await page.getByLabel(/Mot de passe/i).fill(TEST_PASSWORD);
  await page.getByRole('button', { name: 'Se connecter' }).click();
  await page.waitForURL(/\/$/, { timeout: 15_000 });
  await page.goto('/mon-compte');
  await page.waitForSelector('#main *', { timeout: 15_000 });
  await page.waitForTimeout(1_500);
  const text = await page.locator('#main').innerText();
  expect(text).toContain(TEST_EMAIL);
  expect(text).toContain('Test');
});

test('mauvais mot de passe -> message d\'erreur affiché, pas de redirection', async ({ page }) => {
  await page.goto('/connexion');
  await page.getByLabel(/Email/i).fill(TEST_EMAIL);
  await page.getByLabel(/Mot de passe/i).fill('MauvaisMot2Passe!');
  await page.getByRole('button', { name: 'Se connecter' }).click();
  await page.waitForSelector('.page-error', { timeout: 15_000 });
  const err = await page.locator('.page-error').innerText();
  console.log('erreur affichée:', err);
  expect(page.url()).toContain('/connexion');
});

test('inscription via le formulaire -> compte créé et connecté', async ({ page, request }) => {
  const email = `e2e-${Date.now()}@example.com`;
  await page.goto('/inscription');
  await page.getByLabel(/Prénom/i).fill('Jean');
  await page.getByLabel(/^Nom/i).fill('Dupont');
  await page.getByLabel(/Email/i).fill(email);
  await page.getByLabel(/Mot de passe/i).first().fill('Password123!');
  await page.getByLabel(/Confirmer/i).fill('Password123!');
  await page.getByRole('button', { name: /Créer mon compte/i }).click();
  await page.waitForURL(/\/$/, { timeout: 15_000 });
  // le compte existe côté API
  const login = await request.post(`${API}/auth/login`, {
    data: { email, password: 'Password123!' },
  });
  expect(login.ok(), `login API après inscription UI: ${await login.text()}`).toBeTruthy();
});

test('inscription avec mots de passe différents -> erreur de validation', async ({ page }) => {
  await page.goto('/inscription');
  await page.getByLabel(/Prénom/i).fill('Jean');
  await page.getByLabel(/^Nom/i).fill('Dupont');
  await page.getByLabel(/Email/i).fill(`e2e-mismatch-${Date.now()}@example.com`);
  await page.getByLabel(/Mot de passe/i).first().fill('Password123!');
  await page.getByLabel(/Confirmer/i).fill('AutreChose456!');
  await page.getByRole('button', { name: /Créer mon compte/i }).click();
  await page.waitForSelector('.page-error', { timeout: 10_000 });
  const err = await page.locator('.page-error').innerText();
  console.log('erreur validation:', err);
  expect(page.url()).toContain('/inscription');
});
