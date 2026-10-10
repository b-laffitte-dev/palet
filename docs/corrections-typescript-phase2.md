# Corrections TypeScript - Phase 2

## Contexte
Lors de la Phase 2 (Conception), les 6 pages de détail ont été créées avec des mocks de données. Cependant, plusieurs erreurs TypeScript empêchaient la compilation.

## Problèmes identifiés et corrections

### 1. Champs manquants dans les mocks

**Problème** : Les interfaces `Player`, `Team`, `Club`, `Match` étendent `Timestamped` qui nécessite `createdAt: Date` et `updatedAt: Date`.

**Correction** : Ajout systématique de ces champs à tous les mocks :
```typescript
createdAt: new Date('YYYY-MM-DD'),
updatedAt: new Date('YYYY-MM-DD'),
```

### 2. Incompatibilité null/undefined

**Problème** : Plusieurs types utilisent `Type | null` mais les mocks ou les accesseurs renvoient `Type | undefined`.

**Correction** : 
- Utilisation de l'opérateur `!` pour affirmer la non-nullité : `mockTeams[0]!`
- Remplacement de `undefined` par `null` dans les types
- Utilisation de `?? undefined` pour convertir `null` en `undefined`

### 3. Props incorrectes des composants Avatar

**Problème** : Les composants `UserAvatar` et `TeamAvatar` attendent des props spécifiques :
- `UserAvatar` : `user: { id?: string, name?: string, email?: string, photo?: string, initials?: string }`
- `TeamAvatar` : `team: { id?: string, name?: string, logo?: string, abbreviation?: string }`

Mais le code utilisait `name={...}` ou `src={...}` directement.

**Correction** :
```typescript
// Avant
<UserAvatar src={player.photo} name={player.fullName} size="xl" />

// Après
<UserAvatar user={{ id: player.id, name: player.fullName, photo: player.photo ?? undefined }} size="xl" />

// Pour TeamAvatar
<TeamAvatar team={{ id: team.id, name: team.name, logo: team.logo ?? undefined, abbreviation: team.club?.code }} size="m" />
```

### 4. Imports manquants

**Problème** : Les composants `CardHeader`, `CardContent`, `CardFooter`, `UserAvatar`, `TeamAvatar` n'étaient pas importés.

**Correction** : Ajout systématique à toutes les pages de détail :
```typescript
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, UserAvatar, TeamAvatar } from '../components';
```

### 5. Type de phase incorrect

**Problème** : Le type `PhaseType` est `'knockout' | 'group' | 'final'` mais le code utilisait `'groups'`.

**Correction** : Remplacement de toutes les occurrences de `'groups'` par `'group'`.

### 6. Champs manquants dans les interfaces

**Problème** : 
- `MatchTeam` nécessite un champ `seed: number`
- `PositionStats` nécessite `first` et `second`

**Correction** : Ajout des champs manquants dans les mocks.

### 7. Références circulaires dans les mocks

**Problème** : Les mocks de joueurs faisanent référence à eux-mêmes (ex: `currentTeams: [{ players: [mockPlayer] }]`).

**Correction** : Simplification des mocks en retirant les références circulaires ou en utilisant des indices explicites.

## Fichiers corrigés

1. **ClubDetail.tsx**
   - Ajout de `createdAt/updatedAt` aux mockPlayers
   - Correction des imports
   - Correction des TeamAvatar props
   - Résolution des problèmes null/undefined

2. **PlayerDetail.tsx**
   - Ajout de `createdAt/updatedAt` à mockPlayer
   - Correction des imports (Progress composé manuellement)
   - Correction des UserAvatar props
   - Résolution des références circulaires
   - Correction du type CompetitionStats
   - Ajout de `seed` aux MatchTeam

3. **TournamentDetail.tsx**
   - Ajout de `createdAt/updatedAt` aux mockPlayers
   - Correction des imports
   - Correction de `phase.type` : 'groups' → 'group'
   - Résolution des problèmes null/undefined
   - Correction des TeamAvatar props

4. **ChampionshipDetail.tsx**
   - Correction des imports
   - Correction de `topTeam` : Team|undefined → Team|null
   - Correction des TeamAvatar props
   - Résolution des problèmes null/undefined
   - Suppression des propriétés dupliquées (totalJournees)

5. **MatchDetail.tsx**
   - Ajout de `createdAt/updatedAt` aux mockPlayers
   - Correction des imports
   - Correction des UserAvatar props
   - Résolution des problèmes null/undefined

6. **TeamDetail.tsx**
   - Ajout de `createdAt/updatedAt` aux mockPlayers
   - Correction des imports
   - Correction des UserAvatar/TeamAvatar props
   - Résolution des problèmes de type (maxWinStreak)

## Résultat

Toutes les erreurs TypeScript ont été corrigées. La commande `npm run build` passe avec succès.

## Prochaines étapes

- [ ] Vérifier les tests unitaires
- [ ] Passer à la Phase 3 : Développement des fonctionnalités principales
- [ ] Intégration avec le backend
- [ ] Tests end-to-end
