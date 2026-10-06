# Architecture — Application Palet Vendéen

## Vue d'ensemble

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│  Web (TS)   │  │  iOS        │  │  Android    │   → Design system partagé
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       └────────────────┼────────────────┘
                        │ HTTPS / JSON
              ┌─────────▼──────────────┐
              │ API Node/TS (Hono)     │  ← OpenAPI générée (zod-openapi)
              └─────────┬──────────────┘
                        │ Drizzle ORM (schéma typé)
              ┌─────────▼──────────────┐
              │       PostgreSQL       │
              └────────────────────────┘
```

## Stack

| Domaine | Choix |
|---|---|
| Backend | Node.js + TypeScript (Hono + @hono/zod-openapi + Drizzle ORM + Zod) |
| Base de données | PostgreSQL, migrations Drizzle (SQL versionné), UUID |
| Types API | OpenAPI → openapi-typescript → openapi-fetch |
| Compilateur TS | TS 5.9 en prod / TS7 (tsgo) en track parallèle, bascule à la stabilité |
| Linter | oxlint (+ eslint-plugin-react-hooks si besoin) |
| Formatter | oxfmt |
| Tests | vitest (+ supertest pour l'API), Playwright (web) |
| UI | React + Vite (Next.js si SEO nécessaire), Tailwind + shadcn/ui |
| Observabilité | pino (logs structurés), OpenTelemetry |
| CI/CD | GitHub Actions |

## Décisions techniques

### Backend (Node.js + TypeScript)
- **Hono** : framework web rapide et léger, excellente intégration TypeScript.
- **@hono/zod-openapi** : définition des routes avec des schémas Zod qui génèrent l'OpenAPI — validation d'entrée/sortie et contrat API dans une seule source de vérité.
- **Drizzle ORM** : schéma typé en TypeScript, SQL généré explicite, migrations versionnées (`drizzle-kit generate`). Pas d'ORM lourd, requêtes proches du SQL (adapté aux classements complexes).
- Migrations **expand/contract** (rétrocompatibles, rollback possible).
- Auth (à venir) : JWT courts + refresh tokens (table `sessions`), argon2 pour les mots de passe, rôles (joueur, responsable club, fédération, admin).

### Frontend (TypeScript)
- **TypeScript strict** : `strict: true`, `noUncheckedIndexedAccess`, pas de `any` implicite.
- **TS 7 (tsgo)** : compilateur natif ~10x plus rapide, pas encore stable. Stratégie : build/CI bloquant sur TS 5.9, `tsgo --noEmit` en job CI **non bloquant** en parallèle, usage local via VS Code. Bascule en bloquant dès la stabilité.
- **oxlint** : linter Rust, 50-100x plus rapide qu'ESLint. Catégories bloquantes : `correctness`, `suspicious`, `no-unused-vars` ; `pedantic` en warning.
- **oxfmt** : formatter du même écosystème Ox ; `oxfmt --check` bloquant en CI, formatage en pre-commit.
- TanStack Query (gestion serveur) + TanStack Table (classements).

### Base de données (PostgreSQL)
- Conventions : `snake_case`, clés primaires UUID (gen_random_uuid), `timestamptz`, enums natifs pour les statuts (ex. tournoi : `planifie|en_cours|termine|annule`).
- Domaine cœur : `federation > club > equipe > joueur` et `competition > phase > rencontre/ronde > partie`, classements calculés via vues ou tables matérialisées.
- RLS à étudier pour les données sensibles de clubs ; sinon contraintes fortes + index sur les colonnes de classement.

### Multi-canal
- L'API OpenAPI + JSON est consommée par tous les canaux.
- Design system documenté (Storybook), composants responsives réutilisables.
- Option future : Expo (React Native) pour iOS/Android, partage direct du design system et de la logique TS.

### Qualité & CI

Pipeline backend : `oxfmt --check` → `oxlint` → `tsc --noEmit` → `vitest`.

Pipeline web : `oxfmt --check` → `oxlint` → `tsc --noEmit` (bloquant, TS 5.9) → `tsgo --noEmit` (non bloquant) → `vitest` → `playwright`.

Déploiement : conteneurs Docker, Postgres managé, option hébergeur EU (Clever Cloud, Fly.io, Railway).

## Les 3 leviers de réussite

1. **Contrat API typé de bout en bout** (Zod → OpenAPI → types TS) : une seule source de vérité du schéma aux types.
2. **Schéma DB typé** (Drizzle) : schéma et requêtes ne peuvent pas diverger silencieusement.
3. **Design system en composants documentés** dès le départ : le passage iOS/Android devient une réutilisation, pas une réécriture.

## Ordre de mise en œuvre

1. Monorepo : `backend/` (Node/TS Hono) + `web/` (Vite/React) + `docs/` + `docker-compose.yml`
2. Backend : squelette Hono + Drizzle + première migration (fédérations/clubs/joueurs)
3. Génération OpenAPI → types TS (avant d'écrire beaucoup de frontend)
4. Design system : fondations (tokens, couleurs, typo) + composants clés issus de la maquette homepage
5. Itération fonctionnelle : Clubs → Tournois → Championnat → Fiches joueur
