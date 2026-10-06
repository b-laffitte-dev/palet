# Palet Vendéen — Application web

Application de fédération des joueurs, clubs et fédérations de palet vendéen : tournois, championnat, fiches clubs et joueurs. Inspiration : les plateformes du rugby (Top 14) appliquées au palet vendéen.

## Démarrage rapide

```bash
docker compose up --build
```

- Web : http://localhost:5173
- API : http://localhost:3000/health
- OpenAPI : http://localhost:3000/api-docs/openapi.json

## Structure du monorepo

- `backend/` — API Node.js + TypeScript (Hono + Drizzle + Zod), migrations dans `backend/db/migrations`
- `web/` — Application web TypeScript (React + Vite + TanStack Query)
- `docs/` — Documentation de référence
  - `docs/stack-technique.md` — décisions de base
  - `docs/architecture.md` — architecture de référence

## Développement local

Backend :

```bash
cd backend
npm install
DATABASE_URL=postgres://palet:palet@localhost:5432/palet npm run dev
```

Migrations :

```bash
cd backend
npm run db:generate   # génère une migration depuis db/schema.ts
npm run db:migrate     # applique les migrations
```

Web :

```bash
cd web
npm install
npm run dev
```

Génération des types TS pour le web depuis l'OpenAPI :

```bash
./scripts/generate-types.sh
```

## Qualité

- Backend et web : `oxfmt --check`, `oxlint`, `tsc --noEmit`, `vitest` (via les scripts npm de chaque package)
