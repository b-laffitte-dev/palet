# Palet Vendéen — Application web

Application de fédération des joueurs, clubs et fédérations de palet vendéen : tournois, championnat, fiches clubs et joueurs. Inspiration : les plateformes du rugby (Top 14) appliquées au palet vendéen.

## Démarrage rapide

```bash
docker compose up --build
```

- **Web** : http://localhost:8080
- **API** : http://localhost:3000/health
- **OpenAPI** : http://localhost:3000/api-docs

## Structure du monorepo

C'est un workspace Rust :

- `rust-backend/` — API Rust (Actix-web + SQLx + PostgreSQL + utoipa/OpenAPI)
- `rust-frontend/` — Application web Rust (Dioxus → WASM, servi par nginx)
- `rust-shared/` — Types partagés entre backend et frontend (sérialisation, schémas)
- `rust-mobile/` — Application mobile (Dioxus mobile, hors workspace par défaut)
- `docs/` — Documentation de référence

## Développement local

### Prérequis

- Rust stable (voir `rust-toolchain`)
- Cible WASM : `rustup target add wasm32-unknown-unknown`
- PostgreSQL 17 (ou `docker compose up db`)

### Backend

```bash
cargo run -p palet-backend
# ou avec une URL de base de données personnalisée :
PALET__DATABASE_URL=postgres://palet:palet@localhost:5432/palet cargo run -p palet-backend
```

### Frontend (WASM)

```bash
# Build WASM
cargo build -p palet-frontend --target wasm32-unknown-unknown --release

# Servir localement (nécessite wasm-pack ou trunk)
# Avec wasm-pack :
cd rust-frontend && wasm-pack build --target web --out-name palet_frontend --out-dir ./pkg
# Puis servir le dossier avec n'importe quel serveur HTTP statique
```

### Docker

```bash
docker compose up --build
# db   : PostgreSQL 17 sur le port 5432
# api  : Backend Rust sur le port 3000
# web  : Frontend WASM + nginx sur le port 8080
```

## Qualité

```bash
# Vérification de compilation
cargo check --workspace
cargo check -p palet-frontend --target wasm32-unknown-unknown

# Build
cargo build --workspace
cargo build -p palet-frontend --target wasm32-unknown-unknown --release

# Tests
cargo test --workspace

# Lint
cargo clippy --workspace
```

## API

L'API REST expose les ressources suivantes :

| Ressource | Endpoints |
|-----------|-----------|
| Health | `GET /health` |
| Clubs | `GET/POST /clubs`, `GET /clubs/{id}` |
| Matchs | `GET/POST /matchs`, `GET/PUT/DELETE /matchs/{id}`, `GET /matchs/competition/{id}`, `GET /matchs/status/{status}`, `POST /matchs/{id}/start`, `POST /matchs/{id}/complete`, `POST /matchs/{id}/manche` |
| Championnats | `GET/POST /championnats`, `GET/PUT/DELETE /championnats/{id}`, `GET /championnats/league/{league}`, `GET /championnats/season/{season}`, `POST /championnats/{id}/start`, `POST /championnats/{id}/next-journee`, `GET/PUT /championnats/{id}/classification` |

Documentation OpenAPI interactive : `http://localhost:3000/api-docs`
