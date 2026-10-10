# Corrections Types TypeScript - Frontend Palet Vendéen

*Date: 2026-10-09*

## Résumé

Correction complète des erreurs TypeScript dans le frontend pour permettre une compilation sans erreur.

## Fichiers Modifiés

### 1. `/web/src/types/index.ts`

#### Interfaces Ajoutées
- **PlayerPreferences**: Alias de `UserPreferences` (utilisé dans Player)
- **PlayerMatchHistory**: Interface pour l'historique des matchs d'un joueur
- **PositionStats**: Statistiques par position avec winRate optionnel
- **SeasonStats**: Statistiques par saison
- **TeamCompetition**: Interface pour les compétitions d'une équipe
- **TeamCompetitionResults**: Résultats d'une compétition pour une équipe
- **TeamResults**: Résultats globaux pour une équipe
- **PlayerResults**: Résultats globaux pour un joueur

#### Modifications d'Interfaces Existantes
- **Tournament**: Changé `type: 'tournament' | 'cup'` en `type: CompetitionType` pour accepter tous les types de compétition
- **Team**: 
  - Ajouté `logo: string | null`
  - Changé `captain: Player` en `captain: Player | null`
  - Changé `captainId: string` en `captainId: string | null`
- **PlayerTeam**: Changé `captain: Player` en `captain: Player | null`
- **PositionStats**: Rendré `winRate` optionnel dans tous les sous-objets

### 2. `/web/src/components/common/Button.tsx`

- **Exported ButtonProps**: Ajouté `export` à l'interface ButtonProps
- **Correction taille icônes**: Remplacé `size` par `size || 'm'` pour éviter null dans Icon
- **Correction nom icône**: Changé `"loader-2"` en `"Loader2"`

### 3. `/web/src/components/common/Card.tsx`

- **Exported CardProps**: Ajouté `export` à l'interface CardProps

### 4. `/web/src/components/common/Icon.tsx`

- **Import LucideIcon**: Ajouté import pour typer correctement les icônes
- **Correction type sizeMap**: Changé en `Record<Exclude<IconProps['size'], undefined>, number>` pour éviter undefined comme clé
- **Typage LucideIcon**: Ajouté cast vers `LucideIconType | undefined` et utilisé variable intermédiaire

### 5. `/web/src/components/common/Badge.tsx`

- **Exported BadgeProps**: Ajouté `export` à l'interface BadgeProps

### 6. `/web/src/components/common/UserMenu.tsx`

- **Correction noms icônes**: Converti tous les noms d'icônes en PascalCase:
  - `"chevron-down"` → `"ChevronDown"`
  - `"user"` → `"User"`
  - `"home"` → `"Home"`
  - `"target"` → `"Target"`
  - `"bar-chart-3"` → `"BarChart3"`
  - `"bell"` → `"Bell"`
  - `"shield"` → `"Shield"`
  - `"trophy"` → `"Trophy"`
  - `"users"` → `"Users"`
  - `"log-out"` → `"LogOut"`

### 7. `/web/src/components/common/ClubCard.tsx`

- **Correction import Button**: Changé de `import { Button }` à `import Button` (default export)

### 8. `/web/src/components/domain/TournamentCard.tsx`

- **Correction import Button**: Changé de `import { Button }` à `import Button`
- **Simplification getTypeLabel**: Retiré les types non valides ('open', 'championship', 'federal')
- **Correction bug**: Changé `tournament.tournament.status` en `tournament.status`
- **Correction getStatusBadge**: Retiré les status non valides pour Tournament ('scheduled', 'postponed')

### 9. `/web/src/components/domain/MatchHero.tsx`

- **Exported MatchHeroProps**: Ajouté `export` à l'interface

### 10. `/web/src/components/domain/LiveScores.tsx`

- **Exported LiveScoreMatch**: Ajouté `export` à l'interface

### 11. `/web/src/components/domain/*` (tous les fichiers)

- **Export Props**: Exporté toutes les interfaces *Props pour les rendre disponibles

### 12. `/web/src/components/index.ts`

- **Correction exports Button**: Changé de named export à default export
- **Correction exports Card**: Changé de named export à default export
- **Suppression duplicates**: Retiré les exports en double de IconName, BadgeProps, BadgeVariant
- **Correction exports types**: Changé pour importer depuis les fichiers spécifiques au lieu de dossiers

### 13. `/web/src/api.ts`

- **Signature POST**: Changé pour accepter `body` comme paramètre séparé
- **Typage**: `<T>(path: string, body?: any, options?: RequestInit)` au lieu de `<T>(path: string, options?: RequestInit & { body?: any })`

### 14. `/web/src/hooks/useAuth.tsx`

- **Typage réponses API**: Ajouté generics aux appels POST: `api().POST<{ user: User; token: AuthToken }>(...)`
- **Correction appels POST**: Adapté tous les appels à la nouvelle signature (body comme paramètre séparé)
- **Typage refreshToken**: Changé return type de `Promise<void>` à `Promise<AuthToken>` dans l'interface AuthContextType
- **Simplification navigation**: Remplacé `window.location.state?.from?.pathname` par '/' pour éviter les erreurs

### 15. `/web/src/routes/Home.tsx`

#### Corrections structurelles
- **Mocks reconstruits**: 
  - `mockCompetition`: Ajouté `code` manquant
  - `mockTeam1`, `mockTeam2`: Ajouté `status`, `competitions`, `logo`
  - `mockFeaturedMatch`: Reconstruit avec une structure propre et organisée

- **Typage useQuery**: Ajouté generics à tous les useQuery:
  - `useQuery<Match>(...)`
  - `useQuery<LiveScoreMatch[]>(...)`
  - `useQuery<ChampionshipClassification>(...)`
  - `useQuery<Tournament[]>(...)`
  - `useQuery<Player[]>(...)`

#### Corrections mocks
- **Organizers**: Changé tous les organizers de Tournament pour être des Club valides (avec toutes les propriétés requises)
- **Classification Teams**: 
  - Ajouté `logo: null` après clubId
  - Ajouté `status: 'active'`
  - Ajouté `competitions: []`
  - Ajouté `createdAt: new Date(), updatedAt: new Date()`
  - Retiré le `logo: null` dupliqué (après name)
- **Imports**: Ajouté `Competition`, `Team`, `LiveScoreMatch` aux imports

### 16. `/web/src/components/layout/Footer.tsx`

- **Correction nom icône**: Changé `"target"` en `"Target"`

### 17. `/web/src/components/layout/Layout.tsx`

- **Typage API**: Ajouté cast `as Array<any>` à response.data pour éviter l'erreur de length sur unknown

### 18. `/web/src/main.tsx`

- **Suppression cacheTime**: Retiré `cacheTime` qui n'est plus supporté dans @tanstack/react-query v5

## Dépendances Installées

- `react-router-dom` et `@types/react-router-dom` pour la gestion du routage

## Résultat

✅ **Compilation TypeScript réussie sans aucune erreur**

Toutes les interfaces sont maintenant correctement typées, les imports sont valides, et le code compile sans erreur.
