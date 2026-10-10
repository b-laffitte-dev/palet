# Plan de Migration des Données Existantes - Palet Vendéen

## 📋 Vue d'ensemble

Ce document décrit le plan de migration des données existantes depuis les systèmes actuels (FNSMR, CVDP, clubs locaux) vers la nouvelle application Palet Vendéen.

---

## 🎯 Objectifs

1. **Centralisation** : Regrouper toutes les données dispersées dans un système unifié
2. **Normalisation** : Standardiser les formats et structures de données
3. **Historique** : Préserver l'historique des compétitions et joueurs
4. **Intégrité** : Garantir la cohérence et l'exactitude des données
5. **Sécurité** : Protéger les données personnelles des utilisateurs

---

## 📊 Sources de Données Existantes

### 1. FNSMR (Fédération Nationale)

| Type de Donnée | Format | Accès | Fréquence de Mise à Jour |
|---------------|--------|-------|--------------------------|
| Clubs affiliés | CSV/Excel | API REST | Mensuelle |
| Joueurs licenciés | CSV/Excel | API REST | Hebdomadaire |
| Championnat de France | Base de données | Export SQL | En temps réel |
| Coupe de France | Base de données | Export SQL | En temps réel |
| Règles officielles | PDF | Manuel | Annuel |
| Classements nationaux | Base de données | API REST | Hebdomadaire |

**Contact** : FNSMR - [www.le-palet.com](https://www.le-palet.com/)

### 2. CVDP (Commission Vendée)

| Type de Donnée | Format | Accès | Fréquence |
|---------------|--------|-------|-----------|
| Clubs de Vendée | Excel | Email/SharePoint | Mensuelle |
| Calendrier départemental | Google Calendar | API | En temps réel |
| Résultats des championnats | Excel | Email | Après chaque journée |
| Classement D1/D2 | Excel | Email | Hebdomadaire |
| Statistiques joueurs | Excel | Email | Mensuelle |

**Contact** : CVDP - Intégré à FNSMR

### 3. Clubs Locaux

| Club | Type de Donnée | Format | Méthode de Collecte |
|------|---------------|--------|---------------------|
| La Roche-sur-Yon PC | Effectifs, résultats | Excel/Papier | Email/Scan |
| Clos Fontenois | Effectifs, résultats | Excel/Papier | Email/Scan |
| Luçon Palet Club | Effectifs, résultats | Excel/Papier | Email/Scan |
| Les Sables | Effectifs, résultats | Excel/Papier | Email/Scan |
| Challans | Effectifs, résultats | Excel/Papier | Email/Scan |
| Montaigu | Effectifs, résultats | Excel/Papier | Email/Scan |
| Saint-Gilles | Effectifs, résultats | Excel/Papier | Email/Scan |

### 4. Systèmes Externes

| Système | Type de Donnée | Format | Intégration Possible |
|---------|---------------|--------|---------------------|
| HelloAsso | Inscriptions tournois | API | Oui |
| Facebook | Événements, actualités | API Graph | Oui |
| Google Maps | Localisation clubs | API | Oui |
| Meteo France | Conditions météo | API | Oui |

---

## 🗺️ Stratégie de Migration

### Approche Globale

```
Phase 1 : Analyse (2-4 semaines)
    │
    ├── 1.1 Audit des données existantes
    ├── 1.2 Cartographie des sources
    ├── 1.3 Identification des écarts
    └── 1.4 Plan de transformation

Phase 2 : Préparation (2-3 semaines)
    │
    ├── 2.1 Nettoyage des données
    ├── 2.2 Normalisation des formats
    ├── 2.3 Développement des scripts ETL
    └── 2.4 Tests des processus de migration

Phase 3 : Migration (4-8 semaines)
    │
    ├── 3.1 Migration des données de référence
    ├── 3.2 Migration historique
    ├── 3.3 Synchronisation continue
    └── 3.4 Vérification et validation

Phase 4 : Post-Migration (2-4 semaines)
    │
    ├── 4.1 Monitoring et correction
    ├── 4.2 Formation des utilisateurs
    └── 4.2 Documentation finale
```

### Priorités de Migration

| Priorité | Type de Donnée | Justification |
|----------|---------------|---------------|
| 🔴 **Critique** | Clubs, Joueurs (licences actives) | Nécessaire pour le fonctionnement de base |
| 🟡 **Important** | Championnat en cours | Continuité des compétitions |
| 🟡 **Important** | Calendrier | Planification des événements |
| 🟢 **Moyen** | Historique des 3 dernières saisons | Analyse et statistiques |
| 🟢 **Moyen** | Résultats des tournois | Archivage |
| 🔵 **Faible** | Historique complet (>3 ans) | Archivage long terme |

---

## 📁 Schéma de Données Cible

### Modèle Entité-Relation

```
┌─────────────────────┐
│        USER          │
├─────────────────────┤
│ id (PK)             │
│ email              │
│ username           │
│ firstName         │
│ lastName          │
│ passwordHash      │
│ role              │
│ status            │
│ emailVerified     │
│ createdAt         │
│ updatedAt         │
└──────────┬─────────┘
           │
           │ 1:N
           ▼
┌─────────────────────┐
│       PLAYER        │
├─────────────────────┤
│ id (PK, FK→User)   │
│ licenseNumber      │
│ clubId (FK)        │
│ category          │
│ birthDate         │
│ dominantHand      │
│ playStyle         │
│ stats             │ (JSON)
│ trophies          │ (JSON)
│ createdAt         │
│ updatedAt         │
└──────────┬─────────┘
           │
           │ N:M
           ▼
┌─────────────────────┐       ┌─────────────────────┐
│        TEAM         │       │       CLUB          │
├─────────────────────┤       ├─────────────────────┤
│ id (PK)             │       │ id (PK)             │
│ name                │       │ name                │
│ clubId (FK)         │───▶   │ code                │
│ captainId (FK→Player)│       │ address             │
│ players (JSON)      │       │ city                │
│ stats              │       │ postalCode          │
│ status             │       │ league              │
│ createdAt          │       │ division            │
│ updatedAt          │       │ status              │
└─────────────────────┘       │ contactEmail        │
                                │ contactPhone        │
                                │ website             │
                                │ social              │
                                │ createdAt          │
                                │ updatedAt          │
                                └─────────────────────┘
                                         │
                                         │ 1:N
                                         ▼
                              ┌─────────────────────┐
                              │     COMPETITION     │
                              ├─────────────────────┤
                              │ id (PK)             │
                              │ name                │
                              │ code                │
                              │ type                │
                              │ category            │
                              │ material            │
                              │ season              │
                              │ format              │
                              │ status              │
                              │ organizerId (FK→Club)│
                              │ startDate           │
                              │ endDate             │
                              │ currentJournee      │
                              │ totalJournees       │
                              │ stats               │ (JSON)
                              │ eligibility         │ (JSON)
                              │ createdAt          │
                              │ updatedAt          │
                              └──────────┬─────────┘
                                         │
                                         │ 1:N
                                         ▼
┌─────────────────────┐       ┌─────────────────────┐
│       MATCH         │       │      TEAM_IN_       │
├─────────────────────┤       │    COMPETITION      │
│ id (PK)             │       ├─────────────────────┤
│ competitionId (FK)  │───▶   │ id (PK)             │
│ type                │       │ competitionId (FK)│
│ journee             │       │ teamId (FK)        │
│ phase               │       │ status             │
│ team1Id (FK→Team)   │       │ points             │
│ team2Id (FK→Team)   │       │ stats              │
│ team1Score          │       │ createdAt          │
│ team2Score          │       │ updatedAt          │
│ winner              │       └─────────────────────┘
│ status              │
│ date                │
│ time                │
│ location            │
│ isLive              │
│ verified            │
│ createdAt          │
│ updatedAt          │
└─────────────────────┘
```

### Correspondance des Tables

| Table Source (FNSMR) | Table Cible | Mapping | Transformation |
|---------------------|-------------|--------|----------------|
| fnsmr_clubs | Club | Direct | Normalisation des noms |
| fnsmr_players | User + Player | Split | Séparation user/player |
| fnsmr_licences | Player | Join | Intégration dans Player |
| fnsmr_competitions | Competition | Direct | Normalisation type/category |
| fnsmr_matches | Match | Direct | Parsing des dates |
| fnsmr_teams | Team | Direct | Ajout stats |
| fnsmr_rankings | ChampionshipClassification | Transformation | Calcul des points |

---

## 🔄 Processus de Migration

### 1. Audit des Données Existantes

#### Checklist d'Audit

- [ ] Inventaire complet des données par source
- [ ] Qualité des données (complétude, exactitude)
- [ ] Format des données (CSV, Excel, SQL, API)
- [ ] Fréquence de mise à jour
- [ ] Propriétaire des données
- [ ] Accès aux données (API, export manuel, etc.)
- [ ] Historique disponible
- [ ] Données sensibles (RGPD)

#### Outils d'Audit

```bash
# Analyse des fichiers CSV
csvkit info donnees_clubs.csv
csvkit stats donnees_clubs.csv

# Analyse des fichiers Excel
pandas.read_excel('donnees_joueurs.xlsx').info()

# Validation des données
python validate_data.py --input donnees_clubs.csv --schema club_schema.json
```

### 2. Nettoyage des Données

#### Problèmes Courants

| Problème | Solution | Outil |
|----------|----------|-------|
| Données manquantes | Valeurs par défaut | Python/Pandas |
| Doublons | Déduplication | SQL DISTINCT |
| Formats incohérents | Normalisation | Python (dateutil) |
| Caractères spéciaux | Nettoyage | Python (unidecode) |
| Erreurs de saisie | Correction manuelle | Excel/CSV |
| Données obsolètes | Filtrage par date | SQL WHERE |

#### Scripts de Nettoyage

```python
# clean_clubs.py
import pandas as pd
from unidecode import unidecode

def clean_club_data(input_path, output_path):
    df = pd.read_csv(input_path)
    
    # Nettoyer les noms
    df['name'] = df['name'].str.strip().str.title()
    df['name'] = df['name'].apply(lambda x: unidecode(x))
    
    # Nettoyer les codes
    df['code'] = df['code'].str.strip().str.upper()
    
    # Nettoyer les villes
    df['city'] = df['city'].str.strip().str.title()
    
    # Normaliser les statuts
    df['status'] = df['status'].str.lower().map({
        'actif': 'active',
        'inactif': 'inactive',
        'suspendu': 'suspended'
    })
    
    # Sauvegarder
    df.to_csv(output_path, index=False)
    return df
```

### 3. Transformation des Données

#### Mapping des Champs

**Exemple : Clubs**

| Champ Source | Champ Cible | Type | Transformation |
|--------------|--------------|------|----------------|
| id | id | UUID | Generer UUID v4 |
| nom | name | string | Trim + Title |
| surnom | nickname | string | Null si vide |
| code | code | string | Uppercase |
| adresse | address | string | Trim |
| ville | city | string | Trim + Title |
| code_postal | postalCode | string | Null si vide |
| ligue | league | string | "Vendée" par défaut |
| division | division | string | Normaliser (D1, D2, etc.) |
| nb_licencies | playersCount | integer | 0 si vide |
| nb_equipes | teamsCount | integer | 0 si vide |
| email | contactEmail | string | Null si vide |
| telephone | contactPhone | string | Null si vide |
| site_web | website | string | Null si vide |

**Exemple : Joueurs**

| Champ Source | Champ Cible | Type | Transformation |
|--------------|--------------|------|----------------|
| id | id | UUID | Generer UUID v4 |
| licence | licenseNumber | string | Normaliser format |
| nom | lastName | string | Trim + Title |
| prenom | firstName | string | Trim + Title |
| email | email | string | Normaliser (lowercase) |
| date_naissance | birthDate | date | Parser format FR |
| age | age | integer | Calculer à partir de birthDate |
| main_dominante | dominantHand | enum | Mapper à right/left/ambidextrous |
| club_id | clubId | UUID | Match avec Club.id |
| categorie | category | string | Normaliser (senior, veteran, etc.) |
| statut_licence | licenseStatus | enum | Mapper à valid/expired/pending |

### 4. Chargement des Données

#### Ordre de Chargement

```
1. Clubs (pas de dépendance)
   ↓
2. Users (pas de dépendance)
   ↓
3. Players (dépend de Clubs et Users)
   ↓
4. Teams (dépend de Clubs et Players)
   ↓
5. Competitions (dépend de Clubs)
   ↓
6. Team_in_Competition (dépend de Teams et Competitions)
   ↓
7. Matches (dépend de Competitions et Teams)
   ↓
8. Classification/Stats (dépend de Matches)
```

#### Scripts de Chargement

```typescript
// scripts/migrate/clubs.ts
import { db } from '../../backend/db';
import { clubs } from './data/clubs.json';

async function migrateClubs() {
  for (const club of clubs) {
    await db.insert(clubTable).values({
      id: crypto.randomUUID(),
      name: club.name,
      nickname: club.nickname || null,
      code: club.code,
      address: club.address || null,
      city: club.city,
      postalCode: club.postalCode || null,
      league: club.league || 'Vendée',
      division: club.division || 'D2',
      status: club.status || 'active',
      playersCount: club.playersCount || 0,
      teamsCount: club.teamsCount || 0,
      contactEmail: club.contactEmail || null,
      contactPhone: club.contactPhone || null,
      website: club.website || null,
      social: club.social || { facebook: null, twitter: null, instagram: null },
      createdAt: new Date(),
      updatedAt: new Date(),
    });
  }
  console.log(`Migrated ${clubs.length} clubs`);
}
```

### 5. Vérification et Validation

#### Tests de Validation

```typescript
// tests/migration/clubs.test.ts
import { db } from '../../backend/db';
import { describe, it, expect } from 'vitest';

describe('Club Migration', () => {
  it('should have migrated all clubs', async () => {
    const clubs = await db.select().from(clubTable);
    expect(clubs.length).toBeGreaterThan(100);
  });

  it('should have valid data formats', async () => {
    const clubs = await db.select().from(clubTable);
    
    for (const club of clubs) {
      expect(club.name).toBeTypeOf('string');
      expect(club.name.length).toBeGreaterThan(0);
      expect(club.code).toMatch(/^[A-Z0-9]{2,4}$/);
      expect(club.status).toBeOneOf(['active', 'inactive', 'suspended']);
      expect(new Date(club.createdAt).getTime()).not.toBeNaN();
    }
  });

  it('should have unique codes', async () => {
    const codes = await db.selectDistinct().from(clubTable).select('code');
    const uniqueCodes = new Set(codes.map(c => c.code));
    expect(uniqueCodes.size).toBe(codes.length);
  });
});
```

---

## 🔒 Gestion de la Sécurité

### RGPD et Données Personnelles

#### Données Sensibles

| Type de Donnée | Classification | Traitement |
|---------------|----------------|------------|
| Email | Personnel | Chiffrement |
| Téléphone | Personnel | Chiffrement |
| Adresse | Personnel | Chiffrement |
| Date de naissance | Personnel | Pseudonymisation possible |
| Photo | Personnel | Stockage sécurisé |
| Numéro de licence | Identifiant | Masquage partiel |

#### Mesures de Sécurité

1. **Chiffrement des données sensibles**
   - Utilisation de bcrypt pour les mots de passe
   - Chiffrement AES-256 pour les données personnelles

2. **Accès contrôlé**
   - RBAC (Role-Based Access Control)
   - Audit des accès aux données

3. **Conformité RGPD**
   - Droit d'accès, de modification, de suppression
   - Conservation limitée des données
   - Notification en cas de violation

4. **Sauvegardes**
   - Sauvegardes quotidiennes automatiques
   - Test des restaurations
   - Stockage géographique redondant

### Migration des Comptes Utilisateurs

#### Stratégie

1. **Nouveaux utilisateurs** : Inscription via l'application
2. **Utilisateurs existants** : Import depuis FNSMR avec :
   - Email comme identifiant principal
   - Mot de passe temporaire généré
   - Obligation de réinitialisation du mot de passe
3. **Joueurs sans email** : Création de comptes avec :
   - Identifiant basé sur le numéro de licence
   - Mot de passe temporaire envoyé par courrier postal

---

## 📅 Calendrier de Migration

### Timeline

```
Mois 1 (Octobre 2026)
├── Semaine 1-2 : Audit des données
├── Semaine 3-4 : Nettoyage et préparation

Mois 2 (Novembre 2026)
├── Semaine 1-2 : Développement des scripts ETL
├── Semaine 3-4 : Tests des scripts

Mois 3 (Décembre 2026)
├── Semaine 1 : Migration des clubs
├── Semaine 2 : Migration des joueurs
├── Semaine 3 : Migration des compétitions
├── Semaine 4 : Migration des matchs

Mois 4 (Janvier 2027)
├── Semaine 1-2 : Validation et correction
├── Semaine 3-4 : Synchronisation continue

Mois 5 (Février 2027)
├── Semaine 1-2 : Formation des utilisateurs
├── Semaine 3-4 : Documentation finale
```

### Jalon Clés

| Date | Jalon | Description |
|------|-------|-------------|
| 15/10/2026 | Audit Complété | Inventaire complet des données |
| 30/10/2026 | Données Nettoyées | Toutes les données sources nettoyées |
| 15/11/2026 | Scripts ETL Prêts | Scripts de transformation testés |
| 15/12/2026 | Migration Clubs | Tous les clubs migrés |
| 31/12/2026 | Migration Joueurs | Tous les joueurs migrés |
| 15/01/2027 | Migration Compétitions | Toutes les compétitions migrées |
| 31/01/2027 | Validation Complétée | Toutes les données validées |
| 15/02/2027 | Go Live | Application en production |

---

## 💰 Budget et Ressources

### Ressources Humaines

| Rôle | Temps Alloué | Responsabilités |
|------|--------------|-----------------|
| Chef de Projet | 20% | Coordination, suivi |
| Architecte Données | 50% | Conception, mapping |
| Développeur ETL | 80% | Développement scripts |
| Testeur QA | 40% | Validation des données |
| Expert Métier | 20% | Validation fonctionnelle |
| Support Technique | 10% | Résolution des problèmes |

### Budget Estimé

| Poste | Coût |
|-------|------|
| Développement ETL | 15 000 - 20 000 € |
| Nettoyage Données | 5 000 - 10 000 € |
| Tests et Validation | 5 000 - 8 000 € |
| Formation | 3 000 - 5 000 € |
| **Total** | **28 000 - 43 000 €** |

---

## 🎓 Formation

### Plan de Formation

#### 1. Formation Technique

**Public** : Développeurs, Administrateurs système

**Contenu** :
- Architecture de la base de données
- Scripts de migration
- Procédures de sauvegarde/restauration
- Monitoring des performances

**Durée** : 2 jours

#### 2. Formation Utilisateurs

**Public** : Responsables de clubs, Arbitres, Organisateurs

**Contenu** :
- Utilisation de l'application
- Gestion des joueurs et équipes
- Saisie des résultats
- Consultation des statistiques

**Durée** : 1 jour

#### 3. Formation Administrateurs

**Public** : Administrateurs fédéraux et départementaux

**Contenu** :
- Gestion des utilisateurs
- Configuration des compétitions
- Validation des résultats
- Génération de rapports

**Durée** : 2 jours

---

## 📊 Suivi et Reporting

### Indicateurs de Suivi

| Métrique | Cible | Mesure |
|----------|-------|--------|
| Nombre de clubs migrés | 100+ | Compte |
| Nombre de joueurs migrés | 10 000+ | Compte |
| Nombre de compétitions migrées | 50+ | Compte |
| Nombre de matchs migrés | 5 000+ | Compte |
| Taux d'erreur de migration | < 1% | Pourcentage |
| Temps de migration | < 48h | Durée |
| Temps de validation | < 2 semaines | Durée |

### Rapports

1. **Rapport Quotidien**
   - Progression de la migration
   - Problèmes rencontrés
   - Données migrées dans la journée

2. **Rapport Hebdomadaire**
   - Résumé de la semaine
   - Statistiques de qualité
   - Prochaines étapes

3. **Rapport Final**
   - Synthèse complète
   - Leçons apprises
   - Recommandations

---

## 🚨 Gestion des Risques

### Risques Identifiés

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Données manquantes | Moyen | Élevé | Audit préalable, valeurs par défaut |
| Format de données incompatibles | Élevé | Moyen | Normalisation pré-migration |
| Erreurs de mapping | Moyen | Élevé | Validation automatisée + manuelle |
| Temps de migration trop long | Faible | Moyen | Exécution par lots, optimisation |
| Perte de données | Faible | Critique | Sauvegardes complètes avant migration |
| Non-acceptation par les utilisateurs | Moyen | Élevé | Implication précoce, formation |
| Problèmes de performance | Moyen | Moyen | Tests de charge, optimisation |

### Plan de Contingence

1. **Problème de données** : Arrêt de la migration, correction, reprise
2. **Problème technique** : Rollback vers les systèmes existants
3. **Problème d'acceptation** : Sessions de feedback, ajustements
4. **Dépassement de budget** : Priorisation des fonctionnalités, report

---

## 📝 Checklist de Migration

### Avant la Migration

- [ ] Audit complet des données
- [ ] Nettoyage des données source
- [ ] Schéma de base de données validé
- [ ] Scripts de migration développés
- [ ] Scripts de migration testés
- [ ] Environnement de staging prêt
- [ ] Sauvegardes complètes réalisées
- [ ] Équipe de migration formée
- [ ] Plan de communication établi
- [ ] Checklist de validation préparée

### Pendant la Migration

- [ ] Migration des clubs
- [ ] Migration des utilisateurs
- [ ] Migration des joueurs
- [ ] Migration des équipes
- [ ] Migration des compétitions
- [ ] Migration des matchs
- [ ] Migration des statistiques
- [ ] Validation des données migrées
- [ ] Correction des erreurs
- [ ] Tests de performance

### Après la Migration

- [ ] Vérification complète des données
- [ ] Tests utilisateurs
- [ ] Formation des utilisateurs
- [ ] Documentation mise à jour
- [ ] Monitoring activé
- [ ] Support post-migration
- [ ] Feedback collecté
- [ ] Améliorations identifiées

---

## 📞 Contacts

### Équipe de Migration

| Rôle | Nom | Email | Téléphone |
|------|-----|-------|----------|
| Chef de Projet | À définir | chef.projet@palet-vendeen.fr | +33 X XX XX XX XX |
| Architecte Données | À définir | architecte@palet-vendeen.fr | +33 X XX XX XX XX |
| Développeur ETL | À définir | dev.etl@palet-vendeen.fr | +33 X XX XX XX XX |
| Testeur QA | À définir | qa@palet-vendeen.fr | +33 X XX XX XX XX |

### Partenaires Externes

| Organisation | Contact | Email | Site Web |
|-------------|---------|-------|----------|
| FNSMR | À définir | contact@le-palet.com | [www.le-palet.com](https://www.le-palet.com/) |
| CVDP | À définir | contact@cvdp.fr | Intégré à FNSMR |

---

*Document créé le 09/10/2026*
*À mettre à jour régulièrement pendant la migration*
