# Architecture — Application Palet Vendéen

## Vue d'ensemble

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│  Web (TS)   │  │  iOS        │  │  Android    │   → Design system partagé
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       └────────────────┼────────────────┘
                        │ HTTPS / JSON
              ┌─────────▼──────────┐
              │  API Rust (Axum)   │  ← OpenAPI générée (Utoipa)
              └─────────┬──────────┘
                        │ SQLx (vérifié à la compilation)
              ┌─────────▼──────────┐
              │    PostgreSQL      │
              └────────────────────┘
```

## Stack

| Domaine | Choix |
|---|---|
| Backend | Rust + Axum + SQLx + Utoipa (OpenAPI) |
| Base de données | PostgreSQL, migrations SQLx (SQL versionné), UUIDv7 |
| Types API | OpenAPI → openapi-typescript → openapi-fetch |
| Compilateur TS | TS 5.9 en prod / TS7 (tsgo) en track parallèle, bascule à la stabilité |
| Linter | oxlint (+ eslint-plugin-react-hooks si besoin) |
| Formatter | oxfmt (`oxfmt.toml` versionné) |
| Tests | vitest + Playwright (web), cargo test (backend) |
| UI | React + Vite (Next.js si SEO nécessaire), Tailwind + shadcn/ui |
| Observabilité | tracing + OpenTelemetry |
| CI/CD | GitHub Actions |

## Décisions techniques

### Backend (Rust)
- **Axum** : framework web moderne, écosystème Tokio/Tower.
- **SQLx** : requêtes SQL vérifiées à la compilation contre le schéma réel ; pas d'ORM lourde, SQL explicite (adapté aux classements/championnats).
- **Utoipa** : génération OpenAPI depuis le code — le contrat API typé de bout en bout.
- Migrations **expand/contract** (rétrocompatibles, rollback possible).
- Auth : JWT courts + refresh tokens (table `sessions`), Argon2 pour les mots de passe, rôles (joueur, responsable club, fédération, admin) via middlewares Axum.

### Frontend (TypeScript)
- **TypeScript strict** : `strict: true`, `noUncheckedIndexedAccess`, pas de `any` implicite.
- **TS 7 (tsgo)** : compilateur natif ~10x plus rapide, pas encore stable. Stratégie : build/CI bloquant sur TS 5.9, `tsgo --noEmit` en job CI **non bloquant** en parallèle, usage local via VS Code. Bascule en bloquant dès la stabilité.
- **oxlint** : linter Rust, 50-100x plus rapide qu'ESLint. Catégories bloquantes : `correctness`, `suspicious`, `no-unused-vars` ; `pedantic` en warning. Compléter avec `eslint-plugin-react-hooks` si les règles typées manquantes deviennent critiques.
- **oxfmt** : formatter du même écosystème Ox ; `oxfmt --check` bloquant en CI, formatage en pre-commit. Fallback ponctuel `dprint` si un cas de formatage n'est pas géré.
- TanStack Query (gestion serveur) + TanStack Table (classements).

### Base de données (PostgreSQL)
- Conventions : `snake_case`, clés primaires UUIDv7, `timestamptz`, enums natifs pour les statuts (ex. tournoi : `planifie|en_cours|termine|annule`).
- Domaine cœur : `federation > club > equipe > joueur` et `competition > phase > rencontre/ronde > partie`, classements calculés via vues ou tables matérialisées.
- RLS à étudier pour les données sensibles de clubs ; sinon contraintes fortes + index sur les colonnes de classement.

### Multi-canal
- L'API OpenAPI + JSON est consommée par tous les canaux.
- Design system documenté (Storybook), composants responsives réutilisables.
- Option future : Expo (React Native) pour iOS/Android, partage direct du design system et de la logique TS.

### Qualité & CI

Pipeline TS : `oxfmt --check` → `oxlint` → `tsc --noEmit` (bloquant, TS 5.9) → `tsgo --noEmit` (non bloquant) → `vitest` → `playwright`.

Pipeline Rust : `cargo fmt` → `cargo clippy -D warnings` → `cargo test`.

Déploiement : conteneurs Docker, Postgres managé, option hébergeur EU (Clever Cloud, Fly.io, Railway).

## Les 3 leviers de réussite

1. **Contrat API typé de bout en bout** (OpenAPI généré du Rust → types TS) : élimine les bugs d'intégration.
2. **SQL vérifié à la compilation** (SQLx) : schéma et requêtes ne peuvent pas diverger silencieusement.
3. **Design system en composants documentés** dès le départ : le passage iOS/Android devient une réutilisation, pas une réécriture.

## Ordre de mise en œuvre

1. Monorepo : `backend/` (Cargo) + `web/` (Vite/Next) + `docs/` + `docker-compose.yml`
2. Backend : squelette Axum + SQLx + première migration (schéma club/joueur/fédération)
3. Génération OpenAPI → types TS (avant d'écrire beaucoup de frontend)
4. Design system : fondations (tokens, couleurs, typo) + composants clés issus de la maquette homepage
5. Itération fonctionnelle : Clubs → Tournois → Championnat → Fiches joueur
