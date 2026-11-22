import { test, Page, APIRequestContext } from '@playwright/test';

// Spec d'inspection (non bloquante) : dump le texte visible de chaque page
// pour repérer les états d'erreur/vides affichés malgré des réponses 200.
const API = process.env.E2E_API_URL ?? 'http://localhost:3000';

async function firstId(request: APIRequestContext, path: string): Promise<string | null> {
  const res = await request.get(`${API}${path}`);
  if (!res.ok()) return null;
  const json: any = await res.json();
  const arr = json?.data?.data ?? json?.data ?? [];
  return Array.isArray(arr) && arr.length > 0 ? arr[0].id : null;
}

test('inspect toutes les pages', async ({ page, request }) => {
  const ids: Record<string, string | null> = {
    club: await firstId(request, '/clubs'),
    championnat: await firstId(request, '/championnats'),
    competition: await firstId(request, '/competitions'),
    joueur: await firstId(request, '/joueurs'),
    actualite: await firstId(request, '/actualites'),
    match: await firstId(request, '/matchs'),
  };
  const routes = [
    '/', '/championnats', '/tournois', '/clubs', '/joueurs', '/actualites',
    '/federation', '/connexion', '/inscription', '/mon-compte', '/mon-club',
    '/arbitrage', '/notifications', '/recherche/palet', '/404',
    '/cette-route-nexiste-pas',
  ];
  if (ids.championnat) routes.push(`/championnats/${ids.championnat}`);
  if (ids.competition) routes.push(`/tournois/${ids.competition}`);
  if (ids.club) routes.push(`/clubs/${ids.club}`);
  if (ids.joueur) routes.push(`/joueurs/${ids.joueur}`);
  if (ids.actualite) routes.push(`/actualites/${ids.actualite}`);
  if (ids.match) routes.push(`/arbitrage/${ids.match}`);

  const consoleErrs: string[] = [];
  const httpErrs: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error' && !m.text().includes('401') && !m.text().includes('palet_frontend.js') && !m.text().includes('Failed to load module script'))
      consoleErrs.push(`${m.text()} [${m.location()?.url ?? ''}]`);
  });
  page.on('pageerror', (e) => consoleErrs.push(`PAGEERROR: ${e}`));
  page.on('response', (r) => {
    if (r.status() >= 400 && r.status() !== 401 && !r.url().includes('favicon') && !r.url().includes('/pkg/'))
      httpErrs.push(`${r.status()} ${r.url()}`);
  });

  for (const route of routes) {
    await page.goto(route, { waitUntil: 'domcontentloaded' });
    await page.waitForSelector('#main *', { timeout: 15_000 }).catch(() => {});
    await page.waitForTimeout(2_000);
    const text = (await page.locator('#main').innerText()).replace(/\s+/g, ' ').trim();
    console.log(`\n=== ${route} ===\n${text.slice(0, 400)}`);
  }
  console.log('\n=== console/page errors ===\n' + (consoleErrs.join('\n') || '(aucune)'));
  console.log('\n=== http >= 400 (hors 401) ===\n' + (httpErrs.join('\n') || '(aucune)'));
});
