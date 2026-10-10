# Spécifications Fonctionnelles Détaillées - Palet Vendéen

## 📋 Introduction

Ce document décrit les spécifications fonctionnelles détaillées pour l'application Palet Vendéen. Il sert de référence pour le développement, les tests et la validation du système.

**Version** : 1.0  
**Date** : 09/10/2026  
**Auteurs** : Équipe Technique Palet Vendéen  
**Statut** : En cours de développement

---

## 🎯 Objectifs du Système

### Objectifs Principaux

1. **Centraliser** la gestion des joueurs, clubs, compétitions et résultats
2. **Digitaliser** les processus manuels (inscriptions, arbitrage, classements)
3. **Améliorer** l'expérience utilisateur pour les joueurs, clubs et fédérations
4. **Automatiser** le calcul des scores, classements et statistiques
5. **Faciliter** la communication entre les acteurs du palet vendéen
6. **Préserver** l'historique et le patrimoine du sport

### Objectifs Secondaires

1. **Monétiser** via partenariats et services premium
2. **Étendre** l'audience au-delà de la Vendée
3. **Innover** avec des fonctionnalités avancées (IA, mobile, etc.)
4. **Intégrer** avec les systèmes existants (FNSMR, fédérations régionales)

---

## 👥 Acteurs du Système

### Liste des Acteurs

| Acteur | Description | Rôle dans le système |
|--------|-------------|---------------------|
| **Joueur** | Pratiquant du palet vendéen | Consulter son profil, ses stats, s'inscrire aux tournois |
| **Capitaine d'équipe** | Joueur responsable d'une équipe | Gérer les inscriptions, déclarer les résultats |
| **Responsable Club** | Représentant d'un club | Gérer les joueurs, équipes, matériel du club |
| **Arbitre** | Officiel de compétition | Saisir les résultats, valider les matchs |
| **Organisateur** | Responsable d'un tournoi | Créer, configurer, gérer les compétitions |
| **Administrateur Départemental** | Représentant CVDP/CDSMR | Valider les compétitions, gérer les clubs |
| **Administrateur National** | Représentant FNSMR | Superviser toutes les opérations |
| **Super Administrateur** | Admin technique | Gérer les utilisateurs, configurations système |
| **Supporter** | Fan du palet vendéen | Consulter les résultats, classements, actualités |
| **Visiteur** | Utilisateur non connecté | Accès limité aux informations publiques |

### Matrice des Permissions

| Fonctionnalité | Joueur | Capitaine | Club Admin | Arbitre | Organisateur | Dépt Admin | National Admin | Super Admin | Supporter | Visiteur |
|---------------|--------|-----------|------------|---------|--------------|------------|----------------|------------|-----------|----------|
| Consulter profil | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Modifier profil | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Créer équipe | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Gérer joueurs | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Inscrire équipe | ❌ | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Saisir résultats | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Valider résultats | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Créer tournoi | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Configurer tournoi | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Gérer clubs | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Valider clubs | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Gérer utilisateurs | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ |
| Config système | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| Consulter résultats | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Consulter classements | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Consulter actualités | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## 🎯 Fonctionnalités par Module

---

## 🏆 Module 1 : Compétitions

### 1.1 Gestion des Championnat

#### 1.1.1 Création d'un Championnat

**Description** : Permettre aux administrateurs de créer un nouveau championnat

**Acteurs** : Organisateur, Administrateur Départemental, Administrateur National

**Préconditions** :
- Utilisateur connecté avec permissions suffisantes
- Saison non terminée

**Flux Principal** :
1. Utilisateur accède à la page "Créer Championnat"
2. Système affiche le formulaire de création
3. Utilisateur remplit :
   - Nom du championnat
   - Code unique
   - Saison (ex: 2024-2025)
   - Type (Championnat Départemental, Régional, etc.)
   - Catégorie (Individuel, Doublette, Triplette)
   - Matériel (Fonte, Laiton, Bois)
   - Format (Ligue, Poules + Élimination)
   - Nombre de journées
   - Date de début et de fin
   - Clubs participants (ou ouvert à tous)
   - Règles spécifiques
4. Utilisateur valide le formulaire
5. Système valide les données
6. Système crée le championnat
7. Système notifie les clubs concernés

**Flux Alternatifs** :
- **Erreur de validation** : Système affiche les erreurs, utilisateur corrige
- **Code existant** : Système propose un code généré automatiquement
- **Annulation** : Utilisateur peut annuler à tout moment

**Postconditions** :
- Championnat créé et visible dans la liste
- Notifications envoyées aux clubs
- Championnat disponible pour les inscriptions

**Priorité** : Haute

---

#### 1.1.2 Gestion des Journées

**Description** : Planifier et gérer les journées de championnat

**Fonctionnalités** :
- Création de journée avec date, heure, lieu
- Assignment des matchs aux journées
- Notification automatique aux équipes
- Gestion des reports de journée
- Suivi de la progression (J1, J2, ..., J18)

**Flux** :
1. Admin accède à la gestion des journées
2. Sélectionne un championnat
3. Ajoute une nouvelle journée avec :
   - Numéro de journée
   - Date
   - Lieu (terrain)
   - Matchs prévus
4. Système génère automatiquement les matchs si format liguer
5. Système notifie les capitaines des équipes concernées

**Règles Métier** :
- Maximum 18 journées pour un championnat standard
- 15 points pour la victoire en finale, 13 points autrement
- Chaque équipe affronte toutes les autres en format ligue
- En format poule : 4-6 équipes par poule

---

#### 1.1.3 Suivi des Classements

**Description** : Calcul automatique des classements

**Algorithme de Classement** :
```
1. Par points (décroissant)
2. Par différence de points (points marqués - points encaissés)
3. Par points marqués (Bp - Bonus Points)
4. Si égalité parfaite : match de barrage
```

**Fonctionnalités** :
- Calcul automatique après chaque journée
- Historique des classements par journée
- Visualisation des tendances (flèches ↗↘→)
- Statut automatique :
  - 🟢 **Qualifié** : Top 2 pour la finale
  - 🟡 **Barrage** : 3ème place (si applicable)
  - 🔴 **Relégué** : Bottom 2 (descente D2)
  - ✅ **Sauvé** : Autres

**Affichage** :
- Tableau de classement avec :
  - Position
  - Club/Équipe
  - MJ (Matchs Joués)
  - G (Gagnés)
  - P (Perdus)
  - N (Nuls)
  - DIFF (Différence)
  - PTS (Points)
  - FORME (5 derniers résultats)

---

### 1.2 Gestion des Tournois

#### 1.2.1 Types de Tournois

| Type | Format | Participants | Durée | Points |
|------|--------|--------------|-------|--------|
| **Open** | Élimination directe | 32-128 | 1 jour | 13 |
| **Championnat** | Ligue | 8-48 | 1-6 mois | 13/15 |
| **Coupe** | Mixte (poules + élim) | 16-64 | 2-3 jours | 15 |
| **Qualificatif** | Élimination | 16-32 | 1 jour | 13 |
| **Amical** | Libre | 2-16 | 1 jour | 13 |

#### 1.2.2 Phases d'un Tournoi

```
┌─────────────────────────────────────────────┐
│                TOURNOI                         │
├─────────────────────────────────────────────┤
│  Phase 1: Inscition (si ouvert)              │
│  ├── Date limite                              │
│  ├── Frais d'inscription                      │
│  └── Validation manuelle (si requis)         │
│                                                  │
│  Phase 2: Poules (si format mixte)          │
│  ├── 4-6 équipes par poule                   │
│  ├── Chaque équipe affronte les autres       │
│  └── Top N qualifiés pour phase suivante     │
│                                                  │
│  Phase 3: Élimination Directe                │
│  ├── 8ème de finale                           │
│  ├── 1/4 de finale                            │
│  ├── 1/2 finale                               │
│  └── Finale                                  │
│                                                  │
│  Phase 4: Classement Final                   │
│  ├── Podium (1er, 2ème, 3ème)                 │
│  └── Trophées et récompenses                 │
└─────────────────────────────────────────────┘
```

#### 1.2.3 Arbitrage Automatique

**Fonctionnalités** :
- Génération automatique des matchs
- Équilibrage des poules (éviter les équipes fortes ensemble)
- Prise en compte des contraintes :
  - Un joueur ne peut pas affronter un coéquipier de club
  - Éviter les rencontres répétées
  - Respecter les prérequis de classement

**Algorithme** :
```
1. Trier les équipes par classement
2. Répartir en serpent dans les poules
3. Vérifier les contraintes
4. Ajuster manuellement si nécessaire
5. Générer le calendrier
```

---

### 1.3 Saisie des Résultats

#### 1.3.1 Saisie Manuelle

**Acteurs** : Arbitre, Organisateur

**Processus** :
1. Sélection du match à déclarer
2. Saisie des scores :
   - Score équipe 1
   - Score équipe 2
   - Ou score par manche (optionnel)
3. Validation des scores
4. Déclaration du vainqueur (automatique ou manuel)
5. Validation définitive

**Règles** :
- Score maximum par manche : illimité (mais typiquement 10-15)
- Victoria à 13 points (15 en finale)
- Match nul possible si score égal après toutes les manches
- Nombre de manches : variable (jusqu'à ce qu'une équipe atteigne 13/15)

#### 1.3.2 Saisie Mobile

**Application Mobile** (à développer) :
- Interface simplifiée pour les arbitres
- Mode hors-ligne avec synchronisation automatique
- Scan QR code pour identifier le match
- Photo des feuilles de match pour archivage

**Fonctionnalités** :
- ✅ Saisie rapide des scores
- ✅ Historique des matchs arbitrés
- ✅ Statistiques personnelles
- 📋 Validation par signature électronique

#### 1.3.3 Validation des Résultats

**Flux** :
1. Résultat saisi par l'arbitre
2. Notification au capitaine de l'équipe gagnante
3. Confirmation par le capitaine (optionnel)
4. Validation finale par l'organisateur
5. Mise à jour automatique du classement

**Statuts des Résultats** :
- 📝 **En attente** : Résultat saisi mais non validé
- ✅ **Validé** : Résultat confirmé
- ⚠️ **Contesté** : Réclamation en cours
- ❌ **Annulé** : Match annulé

---

## 👥 Module 2 : Acteurs

### 2.1 Gestion des Joueurs

#### 2.1.1 Profil Joueur

**Champs du Profil** :

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | ✅ | Identifiant unique |
| firstName | string | ✅ | Prénom |
| lastName | string | ✅ | Nom |
| fullName | string | ✅ | Nom complet (calculé) |
| alias | string | ❌ | Surnom |
| email | string | ✅ | Email (unique) |
| photo | URL | ❌ | Photo de profil |
| birthDate | date | ✅ | Date de naissance |
| age | number | ✅ | Âge (calculé) |
| birthPlace | string | ❌ | Lieu de naissance |
| nationality | string | ✅ | Nationalité |
| dominantHand | enum | ✅ | Main dominante (right/left/ambidextrous) |
| playStyle | string | ❌ | Style de jeu |
| phone | string | ❌ | Téléphone |
| address | string | ❌ | Adresse |
| city | string | ❌ | Ville |
| postalCode | string | ❌ | Code postal |
| clubId | UUID | ❌ | Club actuel |
| clubNumber | string | ❌ | Numéro de club |
| category | enum | ✅ | Catégorie (senior, veteran, junior, etc.) |
| licenseNumber | string | ✅ | Numéro de licence |
| licenseDate | date | ✅ | Date de la licence |
| licenseStatus | enum | ✅ | Statut (valid/expired/pending) |
| registrationDate | date | ✅ | Date d'inscription |
| status | enum | ✅ | Statut (active/inactive/suspended/banned) |

#### 2.1.2 Statistiques Joueur

**Champs des Stats** :

```typescript
interface PlayerStats {
  // Généraux
  matchesPlayed: number;       // Nombre de matchs joués
  wins: number;                // Victoires
  draws: number;               // Matchs nuls
  losses: number;              // Défaites
  
  // Points
  totalPoints: number;         // Points totaux marqués
  averagePoints: number;       // Moyenne par match
  pointsPerManche: number;     // Moyenne par manche
  bestScore: number;           // Meilleur score en un match
  
  // Performances
  winRate: number;            // Taux de victoires (0-1)
  accuracy: number;            // Précision (0-1)
  bestStreak: number;          // Meilleure série de victoires
  maxWinStreak: number;       // Meilleure série de victoires
  
  // Classement
  currentRank: number | null;    // Classement actuel
  currentDivision: string;     // Division actuelle
  currentPoints: number;       // Points actuels
  previousRank: number | null;  // Classement précédent
  rankTrend: 'up' | 'down' | 'stable'; // Tendances
  newHighlights: number;       // Nouveaux records
  
  // Par position
  byPosition: {
    first: { matches: number; points: number; winRate?: number };
    second: { matches: number; points: number; winRate?: number };
    [key: string]: { matches: number; points: number; winRate?: number };
  };
  
  // Par compétition
  byCompetition: Record<string, {
    matches: number;
    wins: number;
    draws: number;
    losses: number;
    totalPoints: number;
    winRate: number;
  }>;
  
  // Par saison
  bySeason: Record<string, {
    matches: number;
    wins: number;
    draws: number;
    losses: number;
    totalPoints: number;
    averagePoints: number;
    pointsPerManche: number;
    winRate: number;
    bestScore: number;
    bestStreak: number;
    rank: number;
    division: string;
  }>;
}
```

#### 2.1.3 Historique du Joueur

**Match History** :
```typescript
interface PlayerMatchHistory {
  wins: number;
  losses: number;
  draws: number;
  total: number;
  recentMatches: Match[];          // 10 derniers matchs
  byCompetition: Record<string, {
    wins: number;
    losses: number;
    draws: number;
    total: number;
  }>;
  bySeason: Record<string, {
    wins: number;
    losses: number;
    draws: number;
    total: number;
  }>;
}
```

### 2.2 Gestion des Clubs

#### 2.2.1 Profil Club

**Champs du Profil** :

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | ✅ | Identifiant unique |
| name | string | ✅ | Nom du club |
| nickname | string | ❌ | Surnom |
| code | string | ✅ | Code court (ex: LR, CF) |
| logo | URL | ❌ | Logo du club |
| address | string | ❌ | Adresse postale |
| city | string | ✅ | Ville |
| postalCode | string | ❌ | Code postal |
| league | string | ✅ | Ligue (ex: Vendée) |
| division | string | ✅ | Division (D1, D2, etc.) |
| status | enum | ✅ | Statut (active/inactive/suspended) |
| playersCount | number | ✅ | Nombre de joueurs |
| teamsCount | number | ✅ | Nombre d'équipes |
| contactEmail | string | ❌ | Email de contact |
| contactPhone | string | ❌ | Téléphone de contact |
| website | URL | ❌ | Site web |
| social | object | ❌ | Réseaux sociaux |

#### 2.2.2 Effectifs du Club

**Gestion des Joueurs** :
- Ajout/Suppression de joueurs
- Transfert entre clubs
- Gestion des licences
- Statistiques par catégorie (senior, junior, etc.)

**Gestion des Équipes** :
- Création d'équipes
- Assignment des joueurs
- Désignation du capitaine
- Historique des équipes

#### 2.2.3 Matériel du Club

**Inventaire** :
- Plaques de plomb (45x45cm, 20kg)
- Palets fonte (54mm, ~100g)
- Palets laiton (40mm, ~50g)
- Maîtres (petits palets cibles)
- Autres (règles, chronomètres, etc.)

**Fonctionnalités** :
- Suivi des prêts/retours
- Maintenance (vérification poids/dimensions)
- Réservation pour tournois
- Alertes de contrôle (saturisme, plomb)

### 2.3 Gestion des Équipes

#### 2.3.1 Composition d'Équipe

**Types d'Équipes** :
- **Individuel** : 1 joueur
- **Doublette** : 2 joueurs
- **Triplette** : 3 joueurs

**Règles** :
- Un joueur peut appartenir à plusieurs équipes
- Maximum 1 équipe par compétition pour un joueur
- Équipes mixtes (homme/femme) autorisées
- Catégorie déterminée par le joueur le plus âgé

**Statut des Équipes** :
- ✅ **Active** : Équipe complète et validée
- ⚠️ **Incomplete** : Manque des joueurs
- ❌ **Inactive** : Équipe dissoute
- 🏆 **Championne** : Équipe ayant gagné une compétition

#### 2.3.2 Statistiques d'Équipe

```typescript
interface TeamStats {
  matches: number;          // Matchs joués
  wins: number;            // Victoires
  draws: number;           // Nuls
  losses: number;          // Défaites
  pointsFor: number;       // Points marqués
  pointsAgainst: number;   // Points encaissés
  diff: number;            // Différence (pointsFor - pointsAgainst)
  points: number;           // Points totaux (selon barème)
  winRate: number;         // Taux de victoires
  bestScore: number;       // Meilleur score
  bestStreak: number;      // Meilleure série
  currentRank: number;     // Classement actuel
}
```

---

## 📅 Module 3 : Calendrier

### 3.1 Gestion des Événements

**Types d'Événements** :
- 🏆 Championnat (journée)
- 🏆 Tournoi
- 🎯 Open
- 🥂 Coupe
- 📋 Réunion
- 🎉 Événement social

**Champs d'un Événement** :
```typescript
interface Event {
  id: string;
  title: string;
  description: string;
  type: 'championship' | 'tournament' | 'open' | 'cup' | 'meeting' | 'social';
  status: 'draft' | 'published' | 'cancelled' | 'completed';
  startDate: Date;
  endDate: Date;
  time: string;              // Heure de début
  duration: number;          // Durée en minutes
  location: string;          // Lieu
  address: string;           // Adresse complète
  organizerId: string;       // Club/Organisateur
  maxParticipants: number;   // Limite de participants
  registeredCount: number;   // Inscrits
  fee: number | null;        // Frais de participation
  category: string;          // Catégorie (senior, veteran, etc.)
  material: 'fonte' | 'laiton' | 'bois';
  format: string;
  isPublic: boolean;         // Visible par tous
  registrationDeadline: Date | null;
  canRegister: boolean;      // Inscriptions ouvertes
}
```

### 3.2 Inscription aux Événements

**Flux d'Inscription** :
1. Joueur/Capitaine accède à l'événement
2. Système vérifie les prérequis :
   - Licence valide
   - Catégorie compatible
   - Club affilié (si requis)
   - Limite de participants non atteinte
3. Joueur remplit le formulaire :
   - Équipe (pour les compétitions par équipe)
   - Partenaire(s) (pour doublette/triplette)
   - Paiement (si requis)
4. Système valide l'inscription
5. Confirmation envoyée par email
6. Notification à l'organisateur

**Statuts d'Inscription** :
- 📝 **En attente** : Paiement non reçu
- ✅ **Confirmée** : Inscription validée
- ⚠️ **Liste d'attente** : Limite atteinte
- ❌ **Annulée** : Inscription annulée

### 3.3 Calendrier Personnalisé

**Fonctionnalités** :
- Synchronisation avec Google Calendar
- Notification des événements à venir
- Filtrage par type, club, catégorie
- Export ICS (iCalendar)
- Rappels automatiques (email, push)

---

## 📊 Module 4 : Statistiques et Reporting

### 4.1 Tableaux de Bord

#### 4.1.1 Tableau de Bord Joueur

**Widgets** :
- 📈 Évolution du classement
- 🎯 Statistiques personnelles
- 🏆 Historique des compétitions
- 📊 Répartition par position (1ère/2ème)
- 🔥 Meilleure série de victoires
- 💪 Performances par compétition

#### 4.1.2 Tableau de Bord Club

**Widgets** :
- 📈 Classement général
- 👥 Évolution des effectifs
- 🏆 Palmarès des équipes
- 🎯 Statistiques des joueurs
- 📅 Prochains événements
- 💰 Budget et finances

#### 4.1.3 Tableau de Bord Administrateur

**Widgets** :
- 📊 Activité globale
- 📈 Évolution des licences
- 🏆 Résumé des compétitions
- 💰 Revenus des inscriptions
- 📋 Répartition par catégorie
- 🎯 Tendances et prédictions

### 4.2 Rapports

#### 4.2.1 Rapports Standard

| Rapport | Description | Fréquence | Format |
|---------|-------------|-----------|--------|
| Bilan Saison | Résumé de la saison | Annuel | PDF/Excel |
| Classement Général | Tous les clubs/joueurs | Hebdomadaire | PDF/Excel |
| Statistiques Club | Détail par club | Mensuel | PDF |
| Statistiques Joueur | Détail par joueur | Sur demande | PDF |
| Résumé Compétition | Détail d'une compétition | Après compétition | PDF |
| Analyse des Tendances | Évolution sur 5 ans | Annuel | PDF |

#### 4.2.2 Rapports Personnalisés

**Fonctionnalités** :
- Sélection des critères (période, club, joueur, compétition)
- Choix des métriques à afficher
- Filtrage avancé
- Export en PDF/Excel/CSV
- Planification automatique

### 4.3 Analytics Avancés

**Fonctionnalités** :
- Prédiction des résultats (basée sur l'historique)
- Analyse des performances par condition météo
- Comparaison entre clubs/régions
- Identification des talents émergents
- Analyse des tendances de jeu

**Algorithmes** :
- Moyenne mobile pondérée pour les prédictions
- Régression linéaire pour l'analyse des tendances
- Clustering pour la détection des profils de joueurs
- Classification pour la prédiction des résultats

---

## 📢 Module 5 : Communication

### 5.1 Notifications

**Types de Notifications** :

| Type | Canal | Description |
|------|-------|-------------|
| Match à venir | Email, Push | Rappel 24h avant |
| Résultat disponible | Email, Push | Après validation |
| Nouveau classement | Email, Push | Après chaque journée |
| Inscription confirmée | Email | Confirmation d'inscription |
|aiement reçu | Email | Confirmation de paiement |
| Message privé | Email, In-App | Communication entre utilisateurs |
| Annonce générale | Email, In-App | Messages administratifs |

**Préférences de Notification** :
```typescript
interface NotificationPreferences {
  email: {
    results: boolean;       // Résultats
    news: boolean;          // Actualités
    reminders: boolean;     // Rappels
    messages: boolean;      // Messages privés
    announcements: boolean; // Annonces générales
  };
  push: {
    results: boolean;       // Résultats
    matches: boolean;       // Matchs à venir
    news: boolean;          // Actualités
  };
  sms: {
    reminders: boolean;     // Rappels urgents
  };
}
```

### 5.2 Messagerie

**Fonctionnalités** :
- Messagerie privée entre utilisateurs
- Discussion par équipe
- Chat de groupe (clubs, compétitions)
- Historique des conversations
- Pièces jointes (PDF, images)
- Mentions (@utilisateur)

**Sécurité** :
- Chiffrement des messages
- Modération automatique (détection de spam)
- Signalement des messages inappropriés
- Archivage automatique

### 5.3 Newsletter

**Fonctionnalités** :
- Création de newsletters
- Sélection des destinataires (par club, catégorie, etc.)
- Personnalisation du contenu
- Aperçu avant envoi
- Statistiques d'ouverture
- Gestion des désabonnements

---

## 🎨 Module 6 : Interface Utilisateur

### 6.1 Design System

**Composants Principaux** :
- Boutons (primaire, secondaire, outline, ghost, gold, danger, link)
- Cartes (default, flat, hoverable, interactive)
- Formulaires (Input, Select, Checkbox, Radio, Switch, Date, TextArea)
- Tableaux (avec tri, pagination, sélection)
- Modales
- Notifications (Toast)
- Onglets
- Avatars
- Badges
- Indicateurs de statuts

**Design Tokens** :
- Couleurs (primary, gold, green, red, etc.)
- Espacements (xs, s, m, l, xl)
- Typographie (caption, body-s, body-m, body-l, heading-xs, etc.)
- Bordures (radius, width)
- Ombres

### 6.2 Responsive Design

**Breakpoints** :
- Mobile : < 640px
- Tablette : 640px - 1024px
- Desktop : > 1024px

**Adaptation** :
- Navigation : Hamburger menu sur mobile
- Tableaux : Scroll horizontal ou version mobile
- Formulaires : Empilement vertical
- Cartes : Taille adaptée

### 6.3 Accessibilité

**Standards** :
- WCAG 2.1 AA
- Contrastes minimums
- Navigation clavier
- Textes alternatifs
- Labels explicites
- ARIA attributes

**Fonctionnalités** :
- Mode sombre (défaut)
- Mode clair (optionnel)
- Taille de texte ajustable
- Lecteur d'écran compatible

---

## 🔐 Module 7 : Sécurité

### 7.1 Authentification

**Méthodes** :
- Email + Mot de passe
- OAuth (Google, Facebook, Apple)
- 2FA (TOTP, SMS)
- SSO (pour les fédérations)

**Flux d'Authentification** :
```
┌──────────┐     ┌────────────┐     ┌────────────┐
│ Login    │────▶│ Validate    │────▶│ Set Session │
│ Form     │     │ Credentials │     │ & Tokens    │
└──────────┘     └────────────┘     └────────────┘
                                     │
                    ┌────────────────────┴────────────────────┐
                    │                                      │
                    ▼                                      ▼
            ┌───────────────┐                      ┌───────────────┐
            │ 2FA Required  │                      │ Redirect      │
            │ (if enabled)  │                      │ to Dashboard  │
            └───────────────┘                      └───────────────┘
                    │
                    ▼
            ┌───────────────┐
            │ Verify 2FA    │────────────────────▶ Access Granted
            │ Code          │
            └───────────────┘
```

**Gestion des Tokens** :
- Access Token : JWT, durée 15 minutes
- Refresh Token : Durée 7 jours
- Stockage : sessionStorage (par défaut), localStorage (si 