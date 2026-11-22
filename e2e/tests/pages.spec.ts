import { test, expect, Page, APIRequestContext } from '@playwright/test';

const API = process.env.E2E_API_URL ?? 'http://localhost:3000';

interface Issues {
  consoleErrors: string[];
  pageErrors: string[];
  failedRequests: string[];
  httpErrors: string[];
}

// Bruit spécifique au serveur de dev `dx serve` : index.html tente d'importer
// ./pkg/palet_frontend.js (présent seulement en prod wasm-pack) — le try/catch
// l'attrape, mais le navigateur logue quand même une erreur console.
// Le favicon n'existe pas sous dx serve. On les filtre, ce ne sont pas des
// erreurs applicatives.
function isDevServerNoise(text: string, url?: string): boolean {
  return (
    text.includes('palet_frontend.js') ||
    text.includes('Failed to load module script') ||
    (url ?? '').includes('favicon') ||
    (url ?? '').includes('/pkg/')
  );
}

function watch(page: Page, issues: Issues) {
  page.on('console', (msg) => {
    if (msg.type() === 'error') {
      const url = msg.location()?.url ?? '';
      // 401 = endpoint protégé sans session : géré par l'app (redirection/texte),
      // le navigateur logue la ressource mais ce n'est pas une erreur applicative.
      if (msg.text().includes('401')) return;
      if (!isDevServerNoise(msg.text(), url)) issues.consoleErrors.push(`${msg.text()} [${url}]`);
    }
  });
  page.on('pageerror', (err) => issues.pageErrors.push(String(err)));
  page.on('requestfailed', (req) => {
    if (isDevServerNoise('', req.url())) return;
    issues.failedRequests.push(
      `${req.method()} ${req.url()} :: ${req.failure()?.errorText ?? 'failed'}`,
    );
  });
  page.on('response', (res) => {
    if (res.status() >= 400 && !isDevServerNoise('', res.url())) {
      issues.httpErrors.push(`${res.status()} ${res.url()}`);
    }
  });
}

async function visit(page: Page, path: string): Promise<Issues> {
  const issues: Issues = {
    consoleErrors: [],
    pageErrors: [],
    failedRequests: [],
    httpErrors: [],
  };
  watch(page, issues);
  await page.goto(path, { waitUntil: 'domcontentloaded', timeout: 30_000 });
  // attendre que l'app WASM ait rendu quelque chose dans #main
  await page.waitForSelector('#main *', { timeout: 20_000 }).catch(() => {});
  // laisser les appels API asynchrones se terminer
  await page.waitForTimeout(2_500);
  return issues;
}

async function firstId(request: APIRequestContext, path: string): Promise<string | null> {
  const res = await request.get(`${API}${path}`);
  if (!res.ok()) return null;
  const json: any = await res.json();
  const arr = json?.data?.data ?? json?.data ?? [];
  return Array.isArray(arr) && arr.length > 0 ? arr[0].id : null;
}

let ids: Record<string, string | null> = {};

test.beforeAll(async ({ request }) => {
  ids = {
    club: await firstId(request, '/clubs'),
    championnat: await firstId(request, '/championnats'),
    competition: await firstId(request, '/competitions'),
    joueur: await firstId(request, '/joueurs'),
    actualite: await firstId(request, '/actualites'),
    match: await firstId(request, '/matchs'),
  };
  console.log('IDs seed utilisés :', JSON.stringify(ids));
});

const staticRoutes = [
  '/',
  '/championnats',
  '/tournois',
  '/clubs',
  '/joueurs',
  '/actualites',
  '/federation',
  '/connexion',
  '/inscription',
  '/mon-compte',
  '/mon-club',
  '/arbitrage',
  '/notifications',
  '/recherche/palet',
  '/404',
];

const detailRoutes = (): string[] => {
  const r: string[] = [];
  if (ids.championnat) r.push(`/championnats/${ids.championnat}`);
  if (ids.competition) r.push(`/tournois/${ids.competition}`);
  if (ids.club) r.push(`/clubs/${ids.club}`);
  if (ids.joueur) r.push(`/joueurs/${ids.joueur}`);
  if (ids.actualite) r.push(`/actualites/${ids.actualite}`);
  if (ids.match) r.push(`/arbitrage/${ids.match}`);
  return r;
};

const allRoutes = [...staticRoutes, ...detailRoutes(), '/cette-route-nexiste-pas'];

for (const route of allRoutes) {
  test(`page ${route} sans erreur`, async ({ page }) => {
    const issues = await visit(page, route);

    const bodyHtml = await page.locator('#main').innerHTML();
    expect(
      bodyHtml.trim().length,
      `${route} : #main est vide (page blanche ?)`,
    ).toBeGreaterThan(50);

    // le bandeau de la home (pv-topbar) doit être présent sur toutes les pages
    await expect(
      page.locator('.pv-topbar'),
      `${route} : bandeau pv-topbar absent`,
    ).toHaveCount(1);

    // le header de la home (pv-header : logo + nav + CTA) aussi
    await expect(
      page.locator('header.pv-header'),
      `${route} : header pv-header absent`,
    ).toHaveCount(1);

    expect(
      issues.pageErrors,
      `${route} : exceptions JS non capturées`,
    ).toEqual([]);

    expect(
      issues.consoleErrors,
      `${route} : erreurs console`,
    ).toEqual([]);

    // 401 = endpoint protégé consulté sans être connecté (comportement prévu)
    const apiHttpErrors = issues.httpErrors.filter(
      (u) => !u.startsWith('401 ') && !u.includes('8081'),
    );
    expect(apiHttpErrors, `${route} : réponses HTTP >= 400`).toEqual([]);

    const apiFailed = issues.failedRequests.filter((u) => u.includes(':3000'));
    expect(apiFailed, `${route} : requêtes API ayant échoué`).toEqual([]);
  });
}

test('route inconnue affiche la page 404 (catch-all, pas de page blanche)', async ({ page }) => {
  const issues = await visit(page, '/cette-route-nexiste-pas');
  const text = await page.locator('#main').innerText();
  console.log(`route inconnue -> contenu: ${text.slice(0, 200)}`);
  expect(issues.pageErrors).toEqual([]);
  expect(text).toContain('404');
  expect(text).not.toContain('Failed to parse route');
});

test('classement du championnat affiche les noms de clubs, pas des UUID', async ({ page }) => {
  if (!ids.championnat) return;
  const issues = await visit(page, `/championnats/${ids.championnat}`);
  const text = await page.locator('#main').innerText();
  expect(issues.pageErrors).toEqual([]);
  // le seed référence des id de clubs (ex. 38c14d11... = Niort Palet Club)
  expect(text).not.toMatch(/\b[0-9a-f]{8}\b/);
  expect(text).toContain('Niort');
});

test('menu actif dynamique dans le SiteHeader', async ({ page }) => {
  const cases: Array<[string, string]> = [
    ['/championnats', 'Championnat'],
    ['/tournois', 'Tournois'],
    ['/clubs', 'Clubs'],
    ['/joueurs', 'Joueurs'],
    ['/actualites', 'Actualités'],
    ['/federation', 'Fédération'],
  ];
  for (const [route, label] of cases) {
    await visit(page, route);
    const active = page.locator('header.pv-header a.pv-nav-link.active');
    await expect(active, `${route} : exactement un lien actif attendu`).toHaveCount(1);
    expect(await active.innerText(), `${route} : menu actif`).toBe(label);
  }
  // page détail : la section parente reste active
  if (ids.club) {
    await visit(page, `/clubs/${ids.club}`);
    const active = page.locator('header.pv-header a.pv-nav-link.active');
    await expect(active).toHaveCount(1);
    expect(await active.innerText()).toBe('Clubs');
  }
  // home : aucun lien actif
  await visit(page, '/');
  await expect(page.locator('header.pv-header a.pv-nav-link.active')).toHaveCount(0);
});

test('menu actif dynamique dans le Header sticky', async ({ page }) => {
  const clubsLink = () =>
    page.locator('header:not(.pv-header) nav a').filter({ hasText: 'Clubs' }).first();
  await visit(page, '/clubs');
  await expect(clubsLink(), '/clubs : lien Clubs actif').toHaveClass(/underline/);
  await visit(page, '/tournois');
  await expect(clubsLink(), '/tournois : lien Clubs inactif').not.toHaveClass(/underline/);
});

test('pas de débordement horizontal en mobile (375px)', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  for (const route of ['/', '/championnats', '/clubs', '/joueurs', '/connexion']) {
    const issues = await visit(page, route);
    const overflow = await page.evaluate(() => {
      const el = document.scrollingElement ?? document.documentElement;
      return { scrollWidth: el.scrollWidth, clientWidth: el.clientWidth };
    });
    console.log(`${route} mobile: scrollWidth=${overflow.scrollWidth} clientWidth=${overflow.clientWidth}`);
    expect(
      overflow.scrollWidth,
      `${route} : débordement horizontal (${overflow.scrollWidth} > ${overflow.clientWidth})`,
    ).toBeLessThanOrEqual(overflow.clientWidth + 1);
    expect(issues.pageErrors, `${route} : exceptions JS`).toEqual([]);
  }
});
