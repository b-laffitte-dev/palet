# Stack technique — Application Palet Vendéen

## Décisions de base (fondation)

| Domaine | Choix | Notes |
|---|---|---|
| Backend | **Rust** | API et logique métier |
| Frontend / IHM | **TypeScript** | Typage partagé avec le backend via schémas |
| Base de données | **PostgreSQL** | Données joueurs, clubs, fédérations, tournois, championnat |
| Design system | **Un design system dédié** | Composants partagés, inspiré de l'identité du palet vendéen |
| Diffusion | **Multi-canal** : Web, iOS, Android | Cible finale |
| Premier livrable | **Application web** | Fondation du projet |

## Principes

- L'application web est la fondation : l'architecture doit rester compatible avec les cibles iOS et Android (API backend découplée, design system réutilisable).
- Le backend Rust expose une API consommée par tous les canaux.
- Le design system est défini en amont pour garantir la cohérence visuelle entre les canaux.

## Historique

- Décision initiale : Rust (backend), TypeScript (IHM), PostgreSQL (base de données), design system dédié, multi-canal (web, iOS, Android) avec application web comme point de départ.
