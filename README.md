# Palet Vendéen — Application web

Application de fédération des joueurs, clubs et fédérations de palet vendéen : tournois, championnat, fiches clubs et joueurs. Inspiration : les plateformes du rugby (Top 14) appliquées au palet vendéen.

## Démarrage rapide

```bash
docker compose up --build
```

- Web : http://localhost:5173
- API : http://localhost:3000/health
- Swagger UI : http://localhost:3000/swagger-ui

## Structure du monorepo

- `backend/` — API Rust (Axum + SQLx + Utoipa), migrations dans `backend/migrations`
- `web/` — Application web TypeScript (React + Vite + TanStack Query)
- `docs/` — Documentation de référence
  - `docs/stack-technique.md` — décisions de base
  - `docs/architecture.md` — architecture de référence

## Développement local

Backend :

```bash
cd backend
DATABASE_URL=postgres://palet:palet@localhost:5432/palet cargo run
```

Web :

```bash
cd web
npm install
npm run dev
```

Génération des types TS depuis l'OpenAPI (le backend doit tourner ou le fichier openapi doit être exporté) :

```bash
./scripts/generate-types.sh
```

## Qualité

- Backend : `cargo fmt`, `cargo clippy -D warnings`, `cargo test`
- Web : `oxfmt --check src` (via `npm run format:check`), `oxlint` (via `npm run lint`), `npm run build` (inclut `tsc --noEmit`)
