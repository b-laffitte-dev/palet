import { test, expect } from '@playwright/test';

// Fumée des endpoints consommés par les pages (base API :3000).
const API = process.env.E2E_API_URL ?? 'http://localhost:3000';

test('GET /health', async ({ request }) => {
  const res = await request.get(`${API}/health`);
  expect(res.ok()).toBeTruthy();
  const json: any = await res.json();
  console.log('health:', JSON.stringify(json.data));
});

const listEndpoints = [
  '/clubs',
  '/championnats',
  '/competitions',
  '/joueurs',
  '/equipes',
  '/matchs',
  '/actualites',
  '/divisions',
  '/recherche?q=palet',
];

test('GET /notifications sans session -> 401 (protégé par auth)', async ({ request }) => {
  const res = await request.get(`${API}/notifications`);
  expect(res.status()).toBe(401);
});

for (const ep of listEndpoints) {
  test(`GET ${ep} repond 200 avec ApiResponse`, async ({ request }) => {
    const res = await request.get(`${API}${ep}`);
    const body = await res.text();
    console.log(`${ep} -> ${res.status()} ${body.slice(0, 120)}`);
    expect(res.status(), `${ep} -> ${res.status()} ${body.slice(0, 200)}`).toBe(200);
    const json: any = JSON.parse(body);
    expect(json.success, `${ep} : success=false`).toBe(true);
  });
}

test('les endpoints de dispo/arbitrage existent', async ({ request }) => {
  const matchs = await (await request.get(`${API}/matchs`)).json();
  const id = matchs?.data?.data?.[0]?.id;
  if (!id) return;
  for (const ep of [`/matchs/${id}`, `/matchs/${id}/dispos`, `/matchs/competition/x`]) {
    const res = await request.get(`${API}${ep}`);
    console.log(`GET ${ep} -> ${res.status()}`);
    expect(res.status(), `GET ${ep}`).toBeLessThan(500);
  }
});
