# Migration vers Rust - Palet Vendéen

Ce document décrit la migration du projet Palet Vendéen de Node.js/TypeScript vers Rust.

## Structure du Projet Rust

```
palet/
├── Cargo.toml                    # Workspace root
├── rust-backend/                 # Backend API (Actix-web)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs              # Point d'entrée principal
│   │   ├── config.rs            # Configuration de l'application
│   │   ├── db.rs                # Connexion et opérations base de données
│   │   ├── handlers/            # Gestionnaires de requêtes
│   │   │   ├── mod.rs
│   │   │   ├── health_handler.rs
│   │   │   └── club_handler.rs
│   │   ├── models/              # Modèles de données
│   │   │   └── mod.rs
│   │   └── routes/              # Routes de l'API
│   │       └── mod.rs
│   ├── config/
│   │   └── default.toml         # Configuration par défaut
│   └── Dockerfile
│
├── rust-frontend/                # Frontend Web (Yew)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs              # Point d'entrée principal
│   │   ├── components/          # Composants UI
│   │   │   ├── mod.rs
│   │   │   ├── layout/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── layout.rs
│   │   │   │   ├── header.rs
│   │   │   │   └── footer.rs
│   │   │   └── common/
│   │   │       ├── mod.rs
│   │   │       ├── button.rs
│   │   │       ├── card.rs
│   │   │       └── club_card.rs
│   │   ├── pages/               # Pages de l'application
│   │   │   ├── mod.rs
│   │   │   ├── home.rs
│   │   │   ├── clubs.rs
│   │   │   └── ...
│   │   ├── services/            # Services (API client)
│   │   │   ├── mod.rs
│   │   │   └── api.rs
│   │   └── types/
│   │       └── mod.rs
│   ├── nginx.conf
│   └── Dockerfile
│
├── rust-shared/                  # Types et utilitaires partagés
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs               # Types partagés entre backend et frontend
│
├── rust-mobile/                  # Application Mobile (Tauri)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── ios.rs
│   │   └── android.rs
│   └── Dockerfile
│
└── docker-compose.rust.yml      # Configuration Docker pour Rust
```

## Technologies Utilisées

### Backend
- **Framework**: Actix-web 4.0
- **Base de données**: SQLx avec PostgreSQL
- **Sérialisation**: Serde
- **Validation**: Validator
- **Documentation API**: Utoipa + Swagger UI
- **Configuration**: Config + dotenv
- **Logging**: env_logger + log

### Frontend
- **Framework**: Yew 0.21
- **Routing**: yew-router
- **HTTP Client**: reqwest
- **Storage**: gloo (localStorage, sessionStorage)
- **WASM**: wasm-bindgen
- **Styling**: CSS classique avec classes Tailwind-like

### Mobile (iOS/Android)
- **Framework**: Tauri 2.0
- **Partage de code**: palet-shared
- **Compatibilité**: iOS et Android

### Base de Données
- **ORM**: SQLx (pas d'ORM, requêtes SQL directes)
- **Migrations**: Intégrées dans le code Rust
- **Types**: UUID, Chrono pour les timestamps

## Comparaison avec l'Ancienne Structure

### Backend (Node.js → Rust)

| Fonctionnalité | Node.js (Hono) | Rust (Actix-web) |
|---------------|----------------|------------------|
| Framework | Hono | Actix-web |
| ORM | Drizzle | SQLx |
| Validation | Zod | Validator |
| Base de données | PostgreSQL | PostgreSQL |
| Documentation | Zod OpenAPI | Utoipa |
| Configuration | .env | Config + dotenv |

### Frontend (React → Yew)

| Fonctionnalité | React | Yew |
|---------------|-------|-----|
| Framework | React 19 | Yew 0.21 |
| Routing | react-router | yew-router |
| State Management | React Query | use_state + use_effect |
| HTTP Client | fetch + openapi-fetch | reqwest |
| Styling | Tailwind CSS | CSS classique |
| Composants | TSX | HTML macro (html!) |

## Configuration

### Variables d'Environnement

Le backend utilise les variables d'environnement suivantes (préfixe `PALET__`):

```bash
PALET__HOST=0.0.0.0
PALET__PORT=3000
PALET__DATABASE_URL=postgres://palet:palet@localhost:5432/palet
PALET__DB_MAX_CONNECTIONS=20
PALET__DB_TIMEOUT_SECS=30
PALET__CORS_MAX_AGE=86400
PALET__ENVIRONMENT=development
PALET__LOG_LEVEL=info
PALET__JWT_SECRET=change-me-in-production
PALET__JWT_EXPIRATION_HOURS=24
```

## Déploiement

### Avec Docker

```bash
# Backend
docker build -t palet-backend -f rust-backend/Dockerfile .
docker run -p 3000:3000 --env PALET__DATABASE_URL=... palet-backend

# Frontend
docker build -t palet-frontend -f rust-frontend/Dockerfile .
docker run -p 8080:8080 palet-frontend

# Avec Docker Compose
docker compose -f docker-compose.rust.yml up --build
```

### Développement Local

```bash
# Backend
cd rust-backend
cargo run

# Frontend
cd rust-frontend
cargo run --features web

# Mobile
cd rust-mobile
cargo run --features mobile
```

## Migration de la Base de Données

La structure de la base de données a été préservée:

- `federations` → Table des fédérations
- `clubs` → Table des clubs
- `joueurs` → Table des joueurs

Les migrations sont gérées directement dans le code Rust (`db.rs`).

## Types Partagés

Le crate `palet-shared` contient tous les types communs entre le backend et le frontend:

- `User`, `Club`, `Player`, `Federation`
- `Tournament`, `Championship`, `Match`
- `AuthToken`, `AuthState`
- `ApiResponse`, `ApiError`
- `Pagination`, `PaginatedResponse`

## Compatibilité Mobile

L'application mobile utilise Tauri 2.0 pour:

1. **iOS**: Support natif via Tauri
2. **Android**: Support natif via Tauri
3. **Partage de code**: Utilisation de `palet-shared` pour les types communs

### Configuration iOS/Android

Voir les fichiers:
- `rust-mobile/src/ios.rs`
- `rust-mobile/src/android.rs`

## Bonnes Pratiques

### Backend

1. **Gestion des erreurs**: Utiliser `ApiError` pour les réponses d'erreur
2. **Validation**: Utiliser `validator` pour valider les requêtes
3. **Logging**: Utiliser `log` et `env_logger` pour le logging
4. **Documentation**: Utiliser `utoipa` pour la documentation OpenAPI

### Frontend

1. **Gestion d'état**: Utiliser `use_state` et `use_effect` de Yew
2. **Requêtes API**: Utiliser `ApiService` pour les appels API
3. **Routing**: Utiliser `yew-router` pour la navigation
4. **Styling**: Utiliser des classes CSS cohérentes

## Prochaines Étapes

1. **Implémenter les endpoints manquants**:
   - Authentification (login, register, logout)
   - Gestion des tournois
   - Gestion des championnats
   - Gestion des matchs
   - Gestion des joueurs

2. **Compléter le frontend**:
   - Pages détaillées (ClubDetail, PlayerDetail, etc.)
   - Formulaires (Connexion, Inscription)
   - Composants manquants

3. **Configurer Tauri pour mobile**:
   - Configuration iOS
   - Configuration Android
   - Tests sur simulateurs

4. **Optimisations**:
   - Cache des requêtes
   - Lazy loading
   - Optimisation des images

5. **Tests**:
   - Tests unitaires
   - Tests d'intégration
   - Tests E2E

## Ressources Utiles

- [Actix-web Documentation](https://actix.rs/)
- [Yew Documentation](https://yew.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Tauri Documentation](https://tauri.app/)
- [Rust Documentation](https://doc.rust-lang.org/)
