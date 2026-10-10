# Architecture Technique - Palet Vendéen

## Vue d'ensemble

L'application Palet Vendéen est une **Single Page Application (SPA)** moderne construite avec une architecture **monorepo** séparant le frontend (React) du backend (Node.js).

---

## 🏗️ Architecture Globale

```
┌─────────────────────────────────────────────────────────────────┐
│                        MONOREPO                                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐    ┌─────────────────────┐              │
│  │      FRONTEND        │    │       BACKEND        │              │
│  │   (React + Vite)     │    │   (Node.js + Hono)   │              │
│  │                     │    │                     │              │
│  │  - web/             │    │  - backend/          │              │
│  │    - src/           │    │    - src/            │              │
│  │      - routes/      │    │      - routes/       │              │
│  │      - components/  │    │      - controllers/   │              │
│  │      - hooks/       │    │      - services/      │              │
│  │      - types/       │    │      - db/            │              │
│  │      - api/         │    │      - migrations/    │              │
│  │      - styles/      │    │    - db/schema.ts    │              │
│  │      - main.tsx     │    │    - index.ts        │              │
│  │    - vite.config.ts │    │    - package.json    │              │
│  │    - tsconfig.json  │    │    - tsconfig.json   │              │
│  └─────────────────────┘    └─────────────────────┘              │
│                                                                  │
│  ┌─────────────────────┐                                         │
│  │      DOCUMENTATION   │                                         │
│  │   (docs/)           │                                         │
│  │  - contexte-metier/ │                                         │
│  │  - stack-technique.md│                                         │
│  │  - architecture.md   │ ← Ce document                          │
│  └─────────────────────┘                                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎨 Design System

### Philosophie

Le design system suit une approche **modulaire et typée** avec :
- **TypeScript strict** pour la sécurité des types
- **CVA (Class Variance Authority)** pour la gestion des variantes
- **Tailwind CSS** pour le styling utilitaire
- **Composants réutilisables** avec props typées

### Structure des Composants

```
components/
├── common/           # Composants génériques réutilisables
│   ├── Button.tsx    # Boutons avec variantes
│   ├── Card.tsx      # Cartes avec variantes
│   ├── Input.tsx     # Champs de formulaire
│   ├── Select.tsx    # Sélecteurs
│   ├── Modal.tsx     # Modales
│   ├── Toast.tsx     # Notifications
│   ├── Tabs.tsx      # Onglets
│   ├── Pagination.tsx # Pagination
│   ├── Table.tsx     # Tableaux
│   ├── Avatar.tsx    # Avatars
│   ├── Badge.tsx     # Badges
│   ├── Form.tsx      # Composants de formulaire
│   ├── Icon.tsx      # Icônes (Lucide React)
│   └── LiveIndicator.tsx # Indicateurs de live
│
├── domain/           # Composants métier spécifiques
│   ├── MatchHero.tsx      # Hero pour les matchs
│   ├── LiveScores.tsx     # Scores en direct
│   ├── ClassificationTable.tsx # Tableau de classement
│   ├── TournamentCard.tsx # Carte de tournoi
│   ├── PlayerCard.tsx     # Carte de joueur
│   ├── ClubCard.tsx      # Carte de club
│   └── MatchCard.tsx      # Carte de match
│
└── layout/            # Composants de mise en page
    ├── Layout.tsx     # Layout principal
    ├── Header.tsx     # En-tête
    ├── Footer.tsx     # Pied de page
    ├── TopBar.tsx     # Barre supérieure
    └── Navigation.tsx # Navigation
```

### Design Tokens (CSS Variables)

Les tokens de design sont définis dans `web/src/styles/design-system.css` :

```css
/* Colors */
--color-primary-900: #0f0f1a;
--color-primary-800: #1a1a2e;
--color-primary-700: #252542;
--color-primary-600: #303054;
--color-primary-500: #3b3b66;
--color-primary-400: #464678;
--color-primary-300: #636396;
--color-primary-200: #8080b4;
--color-gold-700: #b8860b;
--color-gold-600: #daa520;
--color-gold-500: #ffd700;

/* Spacing */
--spacing-xs: 0.5rem;
--spacing-s: 1rem;
--spacing-m: 1.5rem;
--spacing-l: 2rem;
--spacing-xl: 3rem;

/* Typography */
--text-caption: 0.75rem;
--text-body-s: 0.875rem;
--text-body-m: 1rem;
--text-body-l: 1.125rem;
--text-heading-xs: 1.25rem;
--text-heading-s: 1.5rem;
--text-heading-m: 2rem;
--text-heading-l: 2.5rem;
--text-heading-xl: 3rem;

/* Border Radius */
--radius-sm: 0.25rem;
--radius-m: 0.375rem;
--radius-l: 0.5rem;
--radius-xl: 0.75rem;
--radius-full: 9999px;

/* Shadows */
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
--shadow-m: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
--shadow-l: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
--shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
```

### Composants Clés

#### Button
```typescript
<Button 
  variant="primary" | "secondary" | "outline" | "ghost" | "gold" | "danger" | "link"
  size="xs" | "sm" | "m" | "l" | "xl"
  leftIcon="IconName"
  rightIcon="IconName"
  loading={boolean}
  disabled={boolean}
>
  Label
</Button>
```

#### Card
```typescript
<Card 
  variant="default" | "flat" | "hoverable" | "interactive"
  padding="none" | "sm" | "md" | "lg"
>
  <CardHeader>...</CardHeader>
  <CardContent>...</CardContent>
  <CardFooter>...</CardFooter>
</Card>
```

#### Form Components
```typescript
<Form layout="default" | "compact" | "tight">
  <TextField 
    label="Nom"
    placeholder="Entrez votre nom"
    value={value}
    onChange={setValue}
    required={true}
    leftIcon="User"
    error="Message d'erreur"
    helpText="Texte d'aide"
  />
  
  <SelectField
    label="Catégorie"
    options={options}
    value={selected}
    onChange={setSelected}
    searchable={true}
  />
  
  <CheckboxField
    label="Accepter les conditions"
    value={accepted}
    onChange={setAccepted}
  />
</Form>
```

#### Table
```typescript
<Table>
  <TableHeader>
    <TableRow>
      <TableHeadCell>Nom</TableHeadCell>
      <TableHeadCell>Statut</TableHeadCell>
      <TableHeadCell className="text-center">Actions</TableHeadCell>
    </TableRow>
  </TableHeader>
  <TableBody>
    {data.map((item) => (
      <TableRow key={item.id}>
        <TableCell>{item.name}</TableCell>
        <TableCell>
          <Badge variant="success">Actif</Badge>
        </TableCell>
        <TableCellWithActions>
          <Button variant="ghost" size="sm" leftIcon="Edit">
            Modifier
          </Button>
        </TableCellWithActions>
      </TableRow>
    ))}
  </TableBody>
</Table>
```

---

## 📁 Structure des Routes

### Configuration du Router

Fichier : `web/src/main.tsx`

```typescript
const router = createBrowserRouter([
  { path: '/', element: <Layout><Home /></Layout> },
  { path: '/championnats', element: <Layout><Championnats /></Layout> },
  { path: '/tournois', element: <Layout><Tournois /></Layout> },
  { path: '/clubs', element: <Layout><Clubs /></Layout> },
  { path: '/joueurs', element: <Layout><Joueurs /></Layout> },
  { path: '/actualites', element: <Layout><Actualites /></Layout> },
  { path: '/federation', element: <Layout><Federation /></Layout> },
  { path: '/connexion', element: <Connexion /> },
  { path: '/inscription', element: <Inscription /> },
  { path: '/verification-email', element: <VerificationEmail /> },
]);
```

### Pages Principales

| Route | Composant | Description |
|-------|-----------|-------------|
| `/` | Home | Page d'accueil avec matchs en direct, classement, tournois |
| `/championnats` | Championnats | Liste et détails des championnats |
| `/tournois` | Tournois | Liste et détails des tournois |
| `/clubs` | Clubs | Annuaire des clubs |
| `/joueurs` | Joueurs | Liste des joueurs et statistiques |
| `/actualites` | Actualites | Actualités et nouvelles |
| `/federation` | Federation | Informations fédérales |
| `/connexion` | Connexion | Connexion utilisateur |
| `/inscription` | Inscription | Inscription nouveau utilisateur |
| `/verification-email` | VerificationEmail | Vérification email |

---

## 🔌 Gestion des Données

### API Client

Fichier : `web/src/api.ts`

Utilisation de **hono/client** pour les appels API typés :

```typescript
import { api } from './api';

// GET request
const { data: players } = await api().GET('/api/players');

// POST request
const { data: newPlayer } = await api().POST('/api/players', {
  body: playerData
});

// PUT request
const { data: updatedPlayer } = await api().PUT(`/api/players/${id}`, {
  body: updateData
});

// DELETE request
const { data: deleted } = await api().DELETE(`/api/players/${id}`);
```

### React Query

Configuration centrale dans `main.tsx` :

```typescript
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      refetchOnWindowFocus: false,
      retry: 2,
    },
  },
});
```

Utilisation dans les composants :

```typescript
import { useQuery } from '@tanstack/react-query';
import { api } from '../api';

const { data, isLoading, error } = useQuery<Player[]>({
  queryKey: ['players'],
  queryFn: async () => {
    const response = await api().GET<Player[]>('/api/players');
    return response.data;
  },
});
```

---

## 🔐 Authentification

### Auth Provider

Fichier : `web/src/hooks/useAuth.tsx`

Fonctionnalités :
- Gestion des tokens (access, refresh)
- Stockage sécurisé (sessionStorage/localStorage)
- Support 2FA
- Rafraîchissement automatique des tokens
- Gestion des erreurs

```typescript
const { 
  user, 
  token, 
  isAuthenticated, 
  isLoading, 
  error, 
  requires2FA, 
  login, 
  logout, 
  register, 
  refreshToken 
} = useAuth();
```

### Flux d'authentification

```
┌─────────┐     ┌────────────┐     ┌─────────────┐
│ Connexion│────▶│ Validation │────▶│ Set Tokens  │
│  Form   │     │ Credentials│     │ & User Data │
└─────────┘     └────────────┘     └─────────────┘
                                       │
                                       ▼
                              ┌─────────────────┐
                              │ Redirect to    │
                              │ Previous Page  │
                              └─────────────────┘
```

---

## 🎯 Modèles de Données (Types)

### Structure des Types

Fichier : `web/src/types/index.ts`

#### Entités Principales

```typescript
// Club
interface Club {
  id: string;
  name: string;
  nickname: string | null;
  code: string;
  logo: string | null;
  address: string | null;
  city: string;
  postalCode: string | null;
  league: string;
  division: string;
  status: 'active' | 'inactive' | 'suspended';
  playersCount: number;
  teamsCount: number;
  contactEmail: string | null;
  contactPhone: string | null;
  website: string | null;
  social: { facebook: string | null; twitter: string | null; instagram: string | null };
  createdAt: Date;
  updatedAt: Date;
}

// Player
interface Player {
  id: string;
  firstName: string;
  lastName: string;
  fullName: string;
  alias: string | null;
  email: string;
  photo: string | null;
  status: 'active' | 'inactive' | 'suspended' | 'banned';
  birthDate: Date;
  age: number;
  birthPlace: string | null;
  nationality: string;
  dominantHand: 'right' | 'left' | 'ambidextrous';
  club: Club;
  clubId: string;
  category: string;
  licenseNumber: string;
  licenseStatus: 'valid' | 'expired' | 'pending';
  stats: PlayerStats;
  trophies: PlayerTrophies;
  currentTeams: PlayerTeam[];
  matchHistory: PlayerMatchHistory;
  preferences: PlayerPreferences;
  createdAt: Date;
  updatedAt: Date;
}

// Tournament
interface Tournament extends Competition {
  type: 'championship' | 'cup' | 'tournament' | 'friendly';
  category: 'individual' | 'doublette' | 'triplette' | 'team';
  material: 'fonte' | 'laiton' | 'bois';
  format: 'knockout' | 'league' | 'round-robin' | 'groups' | 'mixed';
  maxTeams: number | null;
  registeredTeams: number;
  pointsType: string;
  winPoints: number;
  drawPoints: number;
  lossPoints: number;
  fee: number | null;
  paymentRequired: boolean;
  paymentMethods: ('card' | 'paypal' | 'cash' | 'transfer')[];
  eligibility: TournamentEligibility;
  currentJournee: number | null;
  totalJournees: number | null;
  currentPhase: string | null;
  phases: TournamentPhase[];
  brackets: TournamentBracket | null;
  stats: TournamentStats;
}

// Match
interface Match {
  id: string;
  competitionId: string;
  competition: Competition;
  type: 'championship' | 'tournament' | 'cup' | 'friendly' | 'training';
  journee: number | null;
  phase: string | null;
  round: string | null;
  team1: MatchTeam;
  team2: MatchTeam;
  team1Score: number | null;
  team2Score: number | null;
  winner: 'team1' | 'team2' | null;
  manches: Manche[] | null;
  bestOf: number | null;
  currentManche: number | null;
  currentScores: number[] | null;
  status: 'pending' | 'scheduled' | 'in_progress' | 'completed' | 'cancelled' | 'postponed' | 'disputed';
  verified: boolean;
  date: Date | null;
  time: string | null;
  location: string | null;
  duration: number | null;
  canEdit: boolean;
  isLive: boolean;
  isRecent: boolean;
  startedAt: Date | null;
  endedAt: Date | null;
  createdAt: Date;
  updatedAt: Date;
}
```

---

## 🎨 Palette de Couleurs

| Couleur | Valeur | Usage |
|---------|--------|-------|
| Primary 900 | #0f0f1a | Fond principal |
| Primary 800 | #1a1a2e | Surfaces élevées |
| Primary 700 | #252542 | Bords, diviseurs |
| Gold 700 | #b8860b | Accent principal |
| Gold 600 | #daa520 | Accent secondaire |
| Gold 500 | #ffd700 | Surlignage |
| Green 600 | #16a34a | Succès |
| Red 600 | #dc2626 | Erreur/Danger |
| Amber 600 | #d97706 | Avertissement |

---

## 📐 Responsive Design

Breakpoints utilisés :
- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px
- `2xl`: 1536px

Exemple :
```typescript
<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
```

---

## 🔧 Bonnes Pratiques

### Organisation du Code

1. **Composants** : Un fichier = un composant
2. **Types** : Toujours typer les props et les états
3. **Nommage** : PascalCase pour les composants, camelCase pour les variables
4. **Imports** : Importer depuis les chemins relatifs, utiliser les exports index
5. **Accessibilité** : Toujours utiliser les attributs ARIA appropriés

### Gestion des Erreurs

```typescript
try {
  const response = await api().POST('/api/endpoint', { body: data });
  return response.data;
} catch (error: any) {
  if (error.status === 401) {
    // Gérer l'erreur d'authentification
  } else if (error.status === 404) {
    // Gérer la ressource introuvable
  } else {
    // Gérer l'erreur générique
  }
}
```

### Performance

- Utiliser `React.memo` pour les composants statiques
- Utiliser `useMemo` et `useCallback` pour les calculs coûteux
- Éviter les re-renders inutiles avec des dépendances précises
- Utiliser le lazy loading pour les gros composants

---

## 🚀 Déploiement

### Environnements

| Environnement | URL | Description |
|---------------|-----|-------------|
| Développement | http://localhost:5173 | Environnement local |
| Staging | À définir | Environnement de test |
| Production | À définir | Environnement de production |

### Docker

Construction et exécution :
```bash
# Build et start
cd /Users/benzo/workspaces/palet
docker compose up --build

# Arrêter
docker compose down

# Voir les logs
docker compose logs -f
```

### Services Docker

- **Web** : Port 5173 (Vite Dev Server)
- **API** : Port 3000 (Node.js + Hono)
- **Database** : Port 5432 (PostgreSQL)

---

## 📊 Métriques de Qualité

### Outils de Linting

```bash
# Backend
cd backend
npm run lint        # ESLint
npm run format:check # Prettier
npm run type:check  # TypeScript

# Frontend
cd web
npm run lint        # ESLint + TypeScript
npm run format:check # Prettier/Oxlint
```

### Tests

```bash
# Backend
cd backend
npm run test        # Vitest
npm run test:watch  # Vitest en mode watch

# Frontend
cd web
npm run test        # Vitest
```

---

## 🎯 Prochaines Étapes

### À Court Terme

1. ✅ **Design System** - Complété
2. ✅ **Correction des erreurs TypeScript** - Complété
3. ⏳ **Pages de détail** - Club, Joueur, Tournoi, Championnat
4. ⏳ **Tests unitaires** - Couverture complète
5. ⏳ **Documentation API** - OpenAPI/Swagger

### À Moyen Terme

1. **Pages administratives** - Gestion des clubs, joueurs, tournois
2. **Tableau de bord** - Analytics et statistiques
3. **Application mobile** - Version React Native
4. **Intégration continue** - CI/CD pipeline

### À Long Terme

1. **Notifications push** - En temps réel
2. **Chatbot** - Assistance et FAQ
3. **IA pour l'arbitrage** - Détection automatique
4. **Gamification** - Badges, récompenses, classements

---

## 📞 Contacts et Support

### Équipe Technique
- Responsable : À définir
- Développeurs : À définir
- Designers : À définir

### Canaux de Communication
- **Slack** : #palet-vendeen
- **Email** : support@palet-vendeen.fr
- **GitHub** : À définir

---

*Document mis à jour le 09/10/2026*
