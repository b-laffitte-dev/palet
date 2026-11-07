# Contexte IA Vibe — Palet Vendéen (consolidé)

> Document de contexte unique pour l'agent IA (Mistral Vibe). Il consolide
> l'ensemble de `docs/` confronté à la **réalité du code**. Quand un document
> de `docs/` contredit ce fichier, c'est **le code qui gagne** (sauf décision
> contraire tranchée par le lead, listée en §9).
>
> Date de consolidation : 2026-11-07. Mettre à jour ce fichier quand le code
> évolue de façon structurelle (stack, routes, schéma DB).

---

## 1. Vision et domaine métier

Application de fédération des joueurs, clubs et fédérations de **palet vendéen**
(tournois, championnat, fiches clubs/joueurs), inspirée des plateformes du rugby
(Top 14). Le palet vendéen : jeu de précision lancé sur une plaque de plomb
(45×45 cm, ≥20 kg), palets fonte (54 mm, ~100 g, lancer 3,80 m) ou laiton
(40 mm, ~50 g, 2,80 m), maître comme cible. Formats 1v1 / 2v2 (doublette) /
3v3 (triplette). **Victoire à 13 points, 15 en finale** — implémenté et testé
dans `rust-backend/src/services/palet_rules_service.rs`.

Hiérarchie : FNSMR (national) → commissions départementales (CVDP 85, CDSP 79,
…) → 100+ clubs locaux.

Classement : **points → différence de points → points marqués (Bp)** — voir
`ClassificationRules` dans `rust-shared/src/championship_types.rs`.

Référence métier complète : `docs/contexte-metier/` (document principal, résumé
exécutif, synthèse JSON). Ces trois documents sont **valides et à jour**.

---

## 2. Stack réelle (d'après le code, pas d'après les vieux docs)

| Domaine | Réalité du code | Fichiers de preuve |
|---|---|---|
| Workspace | Cargo workspace Rust, resolver 2 | `Cargo.toml` |
| Backend | **Actix-web 4.17 + SQLx 0.9 (PostgreSQL) + utoipa 6 / Swagger UI + serde + tokio + config (TOML)** | `rust-backend/Cargo.toml`, `src/main.rs` |
| Frontend web | **Dioxus 0.7.10 (features web + router) → WASM**, servi par nginx | `rust-frontend/Cargo.toml`, `src/lib.rs`, `Dioxus.toml`, `nginx.conf` |
| Mobile | **Dioxus 0.7.10 (feature mobile)**, crate `palet-mobile` **hors workspace** (`exclude`) | `Cargo.toml`, `rust-mobile/Cargo.toml` |
| Types partagés | crate `palet-shared` (serde + utoipa + validator) | `rust-shared/Cargo.toml`, `src/*.rs` |
| Base de données | PostgreSQL 17 (docker), **pas encore de dossier `migrations/`** (db.rs le note explicitement) | `docker-compose.yml`, `rust-backend/src/db.rs` |
| Config backend | TOML `config/default.toml` + env `PALET__*` (préfixe `PALET__`) | `rust-backend/config/default.toml`, `src/config.rs` |
| Déploiement | `docker compose up --build` : db 5432, api 3000, web 8080→80 | `docker-compose.yml`, Dockerfiles |
| Qualité | `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo check -p palet-frontend --target wasm32-unknown-unknown` | `AGENTS.md`, `scripts/check-rust.sh` |

**Ne plus suivre** les docs qui décrivent Node.js/Hono/Drizzle/React/Vite/
Tailwind/Yew/Tauri : ce sont des versions antérieures du projet (voir §8).

---

## 3. Structure réelle du monorepo

```
palet/
├── Cargo.toml                  # workspace : rust-backend, rust-shared, rust-frontend (rust-mobile exclu)
├── AGENTS.md                   # règles agent + activation des skills
├── docker-compose.yml          # db (postgres:17), api (3000), web (8080)
├── rust-backend/               # API Actix-web
│   ├── config/default.toml
│   └── src/
│       ├── main.rs             # serveur + routes + Swagger /api-docs
│       ├── lib.rs              # AppState { db, config }
│       ├── config.rs  db.rs
│       ├── handlers/           # health, club, match, championship
│       ├── repositories/       # club, match, championship, player, team (+ enum_conv)
│       ├── services/           # championship, match, palet_rules (logique métier + tests TDD)
│       └── models/
├── rust-frontend/              # Dioxus web → WASM
│   ├── Dioxus.toml  index.html  nginx.conf  Dockerfile
│   ├── public/styles/          # design-system.css, responsive.css, global.css, components.css, compat.css, homepage.css
│   └── src/
│       ├── lib.rs              # Route enum + App (Router) + start() WASM
│       ├── main.rs             # shim binaire
│       ├── pages/              # home, championships(+detail), tournaments(+detail), clubs(+detail), players(+detail), actualites, federation, connexion, inscription, not_found
│       ├── components/
│       │   ├── common/         # badge, button, card, logo
│       │   ├── layout/         # app_layout, header, sticky_header, mobile_nav, footer
│       │   └── home/           # top_bar, site_header, hero, live_scores, standings, upcoming, top_players, spaces, home_footer
│       ├── services/api.rs     # client HTTP vers http://localhost:3000 (CORS any origin)
│       ├── hooks/  utils/     # format, validation
├── rust-mobile/                # Dioxus mobile (miroir de rust-frontend, hors workspace)
├── rust-shared/src/            # lib.rs (User, Club, Player, Team, Competition…), championship_types.rs, competition_types.rs, match_types.rs
├── .vibe/                      # config Vibe du dépôt (chargée si dossier « trusted »)
│   └── skills/                 # rust-tdd, kiss, rust-idioms, dioxus-patterns, responsive-mobile
├── scripts/                    # check-rust.sh, dev.sh (dx serve --port 8081)
└── docs/                       # voir §8 pour le statut de chaque document
```

---

## 4. API backend réelle (routes enregistrées dans `main.rs`)

Base : `http://localhost:3000` — Swagger UI : `/api-docs`.

| Ressource | Endpoints |
|---|---|
| Health | `GET /health` |
| Clubs | `GET /clubs`, `POST /clubs`, `GET /clubs/{id}` |
| Matchs | `GET /matchs`, `POST /matchs`, `GET/PUT/DELETE /matchs/{id}`, `GET /matchs/competition/{id}`, `GET /matchs/status/{status}`, `POST /matchs/{id}/start`, `POST /matchs/{id}/complete`, `POST /matchs/{id}/manche` |
| Championnats | `GET /championnats`, `POST /championnats`, `GET/PUT/DELETE /championnats/{id}`, `GET /championnats/league/{league}`, `GET /championnats/season/{season}`, `POST /championnats/{id}/start`, `POST /championnats/{id}/next-journee`, `GET/PUT /championnats/{id}/classification` |

Réponses enveloppées dans `ApiResponse<T>` (palet-shared). Le frontend consomme
via `rust-frontend/src/services/api.rs` (`ApiResponse<PaginatedResponse<T>>`).

**Non encore exposé** (repositories présents, handlers absents) : joueurs,
équipes, fédérations, auth (JWT prévu, `jwt_secret` déjà en config).

---

## 5. Frontend réel (routes Dioxus, `rust-frontend/src/lib.rs`)

```
/                     Home
/championnats         Championnats        /championnats/:id   ChampionnatDetail
/tournois             Tournois            /tournois/:id       TournamentDetail
/clubs                Clubs               /clubs/:id          ClubDetail
/joueurs              Joueurs             /joueurs/:id        PlayerDetail
/actualites           Actualites
/federation           Federation
/connexion            Connexion           /inscription        Inscription
/404                  NotFound
```

La home page est composée dans `components/home/` : top_bar, site_header,
hero (match en vedette), live_scores, standings (classement), upcoming
(prochains événements), top_players, spaces, home_footer.

Dev local : `scripts/dev.sh` → `dx serve --port 8081` (évite le conteneur web
sur 8080). Build WASM : `cargo build -p palet-frontend --target wasm32-unknown-unknown --release`.

---

## 6. Design system réel (CSS)

Les styles vivent dans `rust-frontend/public/styles/` (chargés par
`design-system.css` via `@import`) :

- `design-system.css` — variables CSS (couleurs, typo Inter, espacements,
  rayons, ombres) + import des autres feuilles
- `responsive.css` — media queries mobile-first
- `global.css`, `components.css`, `compat.css`, `homepage.css`

**Thème réellement appliqué** : sombre (`--bg-primary: #000000`,
`--text-primary: #ffffff`) avec couleurs de marque **vert `#00a651` et orange
`#ff8c00`** (`--color-brand-green`, `--color-brand-orange`, échelle 50–950),
grises neutres, sémantiques success/warning/danger/info. Les classes utilitaires
Tailwind-like (`bg-brand-green-500`, `flex`, `text-white`…) sont utilisées
directement dans le rsx (ex. `logo.rs`).

**Décision du lead (2026-11-07)** : la palette **CDSP 79 est l'identité
visuelle officielle** — vert `#00a651` + orange `#ff8c00` sur fond sombre, telle
qu'appliquée dans le CSS. Le texte UI « Palet Vendéen » reste le nom de
l'application (le projet couvre Vendée 85 et Deux-Sèvres 79). Ne pas
« corriger » le CSS vers la palette `#0d3b2e`/`#f5d576` des vieux docs.

Le design system documenté dans `docs/design-system/` (palette vert profond
`#0d3b2e` + doré `#f5d576`, composants React/Tailwind) décrit une **intention de
design**, pas l'implémentation actuelle : les composants réels sont en Rust
(`components/common|layout|home`), le styling en CSS vanilla, pas de
Storybook/Tailwind dans le code.

---

## 7. Règles métier implémentées (`palet_rules_service.rs`, testées TDD)

- Match à **13 points**, **15 en finale** (« la balle ») — `get_match_points_target`
- `best_of` valide : 13 ou 15 uniquement
- Calcul du vainqueur de manche/match, manche terminée, classement
  (`calculate_team_classification`) selon points → diff → Bp
- Les types métier sont dans `rust-shared` (`Match`, `Manche`, `Championship`,
  `ChampionshipTeam`, `ChampionshipClassification`, `Competition*`, `User`,
  `Club`, `Player`, `Team`, `Federation`, `AuthToken`, `ApiResponse`,
  `PaginatedResponse`…)

---

## 8. Statut des documents de `docs/` (confrontés au code)

| Document | Statut | Pourquoi |
|---|---|---|
| `contexte-metier/` (3 docs + JSON) | ✅ **Valide** — référence métier | Confirmé par le code (règles 13/15, classement) |
| `specifications-fonctionnelles.md` | ⚠️ **Métier valide, tech obsolète** | Acteurs/modules/règles bons ; sections React/Tokens/OpenAPI-zod à lire comme intention, pas comme stack |
| `plan-migration-donnees.md` | ⚠️ **Intention valide** | Sources FNSMR/CVDP réelles ; mais aucune migration SQL n'existe encore (`db.rs` le confirme) |
| `design-system/` (README, colors, typography, components, layouts, icons, pages/) | ⚠️ **Intention de design** | Palette/composants décrits ≠ CSS réel (§6). Les pages décrites correspondent aux routes réelles |
| `maquettes/homepage.svg|png` | ✅ Référence visuelle | La home Dioxus en est l'implémentation |
| `RESPONSIVE_*.md` (4 docs) | ⚠️ **Partiellement obsolète** | Principes mobile-first bons et appliqués (`responsive.css`, skill `responsive-mobile`) ; mais titres « CDSP 79 » et chemins `src/styles/` faux → réel : `rust-frontend/public/styles/`. `useResponsive.ts` n'existe pas (hooks Rust) |
| `README.md` (racine) | ✅ **Valide** | Colle au code (stack Rust, ports, endpoints) |

**Supprimés le 2026-11-07 (décision lead)** — docs de l'ancienne stack
Node.js/React, obsolètes et partiellement faux (Yew/Tauri) : `architecture.md`,
`architecture-technique.md`, `stack-technique.md`, `RUST_MIGRATION.md`,
`etat-avancement.md`, `etat-projet-phase2-finalisee.md`, `RESUME-PHASE-2.md`,
`corrections-typescript-phase2.md`, `corrections-types-frontend.md`.
Retrouvables dans l'historique git.

**Règle pour l'agent** : pour toute question de stack, d'architecture, de routes,
de composants ou de styles → se fier à ce fichier + au code. Les docs marqués
❌/⚠️ ne servent que pour le métier, la migration de données ou l'intention de
design.

---

## 9. Conflits et décisions

Résolus par le code (le code gagne) :

1. Stack backend/frontend/mobile : Rust Actix-web + Dioxus 0.7 (web et mobile),
   pas Node/React/Yew/Tauri.
2. Ports : web 8080, api 3000, db 5432 (docker-compose), pas de Vite 5173.
3. Structure backend : `handlers/ + repositories/ + services/`, pas `routes/`.
4. Chemins : styles dans `rust-frontend/public/styles/`, contexte métier dans
   `docs/contexte-metier/` (pas `/tmp/`).
5. Marque affichée dans l'UI : « Palet Vendéen » (logo.rs, top_bar.rs).

**Questions tranchées par le lead le 2026-11-07** :

- [x] Identité visuelle : la palette **CDSP 79** (vert `#00a651` + orange
  `#ff8c00`, fond sombre) est officielle — c'est le code qui gagne.
- [x] Docs obsolètes Node/TS (§8) : **supprimés** (voir historique git).

---

## 10. Skills et conventions pour l'agent (résumé de `AGENTS.md`)

Avant **toute tâche**, scanner les `triggers:` des `.vibe/skills/*/SKILL.md` et
charger les skills matchés (plusieurs possibles) : `rust-tdd` (test/bug/"ça ne
marche pas"), `kiss` (simplifier/refactor), `rust-idioms` (ownership/clippy/
"compile pas"), `dioxus-patterns` (composant/page/rsx/route/UI),
`responsive-mobile` (mobile/css/breakpoint/"ça déborde"). Les appliquer en
silence.

Conventions : workspace Rust ; commandes qualité `cargo fmt --all`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
`cargo check -p palet-frontend --target wasm32-unknown-unknown`. TDD obligatoire
pour la logique métier (voir skill `rust-tdd`). KISS partout. Pas de `unwrap`/
`panic` en prod (skill `rust-idioms`).

---

## 11. Conformité Mistral Vibe — structure des fichiers agent

Conventions officielles (docs.mistral.ai/vibe) appliquées à ce dépôt :

| Élément | Convention Vibe | Dans ce repo |
|---|---|---|
| Instructions agent | `AGENTS.md` à la racine du dépôt, découvert et injecté automatiquement dans le contexte | ✅ `AGENTS.md` |
| Config projet Vibe | `.vibe/` à la racine, chargé uniquement si le dossier est « trusted » | ✅ `.vibe/` |
| Skills | `.vibe/skills/<nom>/SKILL.md` — spec Agent Skills : frontmatter YAML avec `name` (égal au nom du dossier) + `description` (≤ 1024 caractères) obligatoires ; ici champ custom `triggers:` en plus. Chaque skill est invocable via `/<nom>` | ✅ `.vibe/skills/` (5 skills : rust-tdd, kiss, rust-idioms, dioxus-patterns, responsive-mobile) |
| Agents custom | `.vibe/agents/*.toml` (TOML) | aucun — pas nécessaire |
| Prompts nommés | `.vibe/prompts/*.md` | aucun — pas nécessaire |
| Fichier de contexte consolidé | doc de référence du projet, lié depuis `AGENTS.md` | ✅ `docs/CONTEXTE-IA-VIBE.md` (ce fichier), pointé par `AGENTS.md` |

Règle : ne pas remettre les skills dans un dossier `skills/` à la racine — Vibe
ne les découvrirait pas (hors `skill_paths` custom dans `.vibe/config.toml`).
