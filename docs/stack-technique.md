# Stack technique — Application Palet Vendéen

## Décisions de base (fondation)

| Domaine | Choix | Notes |
|---|---|---|
| Backend | **Node.js + TypeScript** | API et logique métier (Hono + Drizzle) |
| Frontend / IHM | **TypeScript** | Typage partagé avec le backend via OpenAPI |
| Base de données | **PostgreSQL** | Données joueurs, clubs, fédérations, tournois, championnat |
| Design system | **Un design system dédié** | Composants partagés, inspiré de l'identité du palet vendéen |
| Diffusion | **Multi-canal** : Web, iOS, Android | Cible finale |
| Premier livrable | **Application web** | Fondation du projet |

## Principes

- L'application web est la fondation : l'architecture doit rester compatible avec les cibles iOS et Android (API backend découplée, design system réutilisable).
- Le backend expose une API OpenAPI consommée par tous les canaux.
- Le design system est défini en amont pour garantir la cohérence visuelle entre les canaux.

## Historique

- Décision initiale : backend Rust, TypeScript (IHM), PostgreSQL, design system dédié, multi-canal (web, iOS, Android) avec application web comme point de départ.
- **Révision** : le backend passe de Rust à **Node.js + TypeScript** (Hono + Drizzle + Zod). Cohérence de langage sur toute la stack, écosystème npm unifié, génération OpenAPI conservée. Les documents d'architecture restent la référence pour le reste des choix.
