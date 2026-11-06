# Checklist de Test Responsive Manuel - CDSP 79

## 📱 Instructions

1. Ouvrez l'application dans votre navigateur
2. Utilisez les outils de développement (F12 ou Ctrl+Shift+I)
3. Activez le mode "Device Toolbar" (Ctrl+Shift+M dans Chrome)
4. Testez chaque appareil de référence ci-dessous
5. Vérifiez chaque point de la checklist
6. Notez les problèmes dans la section correspondante

---

## 🎯 Appareils de Référence à Tester

### Mobile (Portrait)
- **iPhone SE** : 320×568px
- **iPhone 12** : 390×844px
- **iPhone 15** : 393×852px
- **Galaxy S20** : 384×854px
- **Pixel 7** : 412×915px

### Mobile (Paysage)
- **iPhone SE** : 568×320px
- **iPhone 12** : 844×390px

### Tablettes
- **iPad Mini** : 768×1024px (Portrait) / 1024×768px (Paysage)
- **iPad Air** : 820×1180px (Portrait)

### Desktop
- **Petit écran** : 1024×768px
- **Laptop** : 1280×720px, 1366×768px
- **Standard** : 1440×900px, 1920×1080px

---

## ✅ Checklist par Composant

### Layout & Structure

#### Header
- [ ] Logo visible et lisible
- [ ] Nom "CDSP 79" visible
- [ ] Tagline visible sur desktop (masqué sur mobile si nécessaire)
- [ ] Menu hamburger visible sur mobile (< 1024px)
- [ ] Navigation horizontale visible sur desktop (>= 1024px)
- [ ] Menu hamburger fonctionne au clic
- [ ] Menu mobile s'ouvre correctement
- [ ] Menu mobile contient tous les liens de navigation
- [ ] Menu mobile a un bouton de fermeture
- [ ] Menu mobile se ferme en cliquant en dehors
- [ ] Menu mobile se ferme en appuyant sur Échap
- [ ] Bouton "S'inscrire" visible (sur desktop et mobile)
- [ ] Bouton "Connexion" visible
- [ ] Bouton "Mon espace" visible
- [ ] Indicateur "EN DIRECT" visible

#### TopBar
- [ ] Barre noire en haut de la page
- [ ] Texte "CDSP 79 - Palet Fonte" visible
- [ ] Couleur or du texte
- [ ] Hauteur correcte (40px)
- [ ] Bordure inférieure visible
- [ ] Contenu ne dépasse pas horizontalement

#### Layout Principal
- [ ] Min-height: 100vh
- [ ] Fond sombre visible
- [ ] Espacement (padding) adapté à la taille d'écran
- [ ] Pas de débordement horizontal

#### Footer
- [ ] Logo CDSP 79 visible
- [ ] Description lisible
- [ ] Icônes des réseaux sociaux visibles
- [ ] Sections de liens organisées en grille
- [ ] Grille adaptée : 1 colonne mobile, 2-3 colonnes desktop
- [ ] Copyright visible et lisible
- [ ] Informations de version visibles

---

### Navigation

#### Desktop (> 1024px)
- [ ] Tous les liens de navigation visibles
- [ ] Liens alignés horizontalement
- [ ] Espacement suffisant entre les liens
- [ ] Liens actifs mis en évidence (bordure or)
- [ ] Survol des liens fonctionne (couleur or)

#### Mobile (< 1024px)
- [ ] Menu hamburger visible
- [ ] Menu hamburger cliquable
- [ ] Menu mobile s'ouvre
- [ ] Liens verticaux
- [ ] Liens de taille suffisante pour le toucher
- [ ] Liens actifs visibles
- [ ] Séparateurs entre les sections
- [ ] Bouton de fermeture visible
- [ ] Logo en bas du menu mobile

---

### Cartes

#### MatchCard
- [ ] Compact sur mobile
- [ ] Complète sur desktop
- [ ] Noms des équipes visibles
- [ ] Scores visibles
- [ ] Statut visible (En direct, Terminé, etc.)
- [ ] Couleurs de statut correctes
- [ ] Cliquable sur toute la surface
- [ ] Pas de débordement

#### PlayerCard
- [ ] Photo/avatar visible
- [ ] Nom complet visible
- [ ] Surnom visible (si présent)
- [ ] Club visible
- [ ] Statistiques visibles
- [ ] Badge de win rate visible
- [ ] Cliquable

#### TournamentCard
- [ ] Type visible (Tournoi, Coupe)
- [ ] Nom visible
- [ ] Date visible
- [ ] Statut visible
- [ ] Nombre d'équipes visible
- [ ] Bouton d'action visible
- [ ] Couleurs adaptées au type

#### ClubCard
- [ ] Logo visible
- [ ] Nom du club visible
- [ ] Ville visible
- [ ] Division visible
- [ ] Nombre de joueurs visible
- [ ] Nombre d'équipes visible

---

### Tableaux

#### ClassificationTable
- [ ] Titre visible
- [ ] Sous-titre visible (journée, saison)
- [ ] En-têtes de colonnes visibles
- [ ] Données visibles
- [ ] Scroll horizontal sur mobile
- [ ] Première colonne fixe sur mobile (si implémenté)
- [ ] Couleurs de fond par statut (qualifié, barrage, relégué)
- [ ] Lignes survolables
- [ ] Badges de statut visibles

#### LiveScores
- [ ] Cartes de scores visibles
- [ ] Noms des équipes visibles
- [ ] Scores visibles
- [ ] Statut visible (en cours, terminé)
- [ ] Défilement horizontal sur mobile
- [ ] Mise à jour automatique

#### MatchHero
- [ ] Logos des équipes visibles
- [ ] Noms des équipes visibles
- [ ] Score visible
- [ ] Statut visible
- [ ] Informations du match visibles
- [ ] Boutons d'action visibles

---

### Formulaires

#### Input
- [ ] Largeur adaptée
- [ ] Label visible
- [ ] Placeholder visible
- [ ] Valeur saisie visible
- [ ] Bordure visible
- [ ] Focus visible
- [ ] Erreurs visibles

#### Select
- [ ] Largeur adaptée
- [ ] Label visible
- [ ] Options sélectionnables
- [ ] Valeur sélectionnée visible
- [ ] Flèche visible

#### Button
- [ ] Taille adaptée
- [ ] Texte lisible
- [ ] Cliquable
- [ ] States (hover, active) visibles
- [ ] Icônes visibles (si présentes)

---

### Modales

#### Modal
- [ ] Centrée sur l'écran
- [ ] Largeur adaptée
- [ ] Fond sombre (overlay)
- [ ] Contenu lisible
- [ ] Bouton de fermeture visible
- [ ] Scrollable si contenu long
- [ ] Fermeture au clic en dehors
- [ ] Fermeture avec Échap

#### Toast
- [ ] Position fixe
- [ ] Largeur adaptée
- [ ] Texte lisible
- [ ] Couleurs adaptées au variant
- [ ] Disparition automatique

---

### Onglets (Tabs)

#### Tabs
- [ ] Onglets visibles
- [ ] Texte lisible
- [ ] Onglet actif visible
- [ ] Survol fonctionne
- [ ] Contenu des onglets visible
- [ ] Défilement horizontal sur mobile

---

### Pagination

#### Pagination
- [ ] Boutons visibles
- [ ] Numéro de page actuel visible
- [ ] Boutons Précédent/Suivant visibles
- [ ] Boutons de page visibles
- [ ] Texte lisible
- [ ] Cliquable
- [ ] Espacement suffisant

---

## 📋 Tests par Taille d'Écran

### 320px (iPhone SE)
- [ ] Pas de débordement horizontal
- [ ] Texte lisible (minimum 12px)
- [ ] Boutons cliquables (minimum 44×44px)
- [ ] Menu hamburger visible et fonctionnel
- [ ] Contenu principal accessible
- [ ] Footer accessible
- [ ] Images redimensionnées

### 390px (iPhone 12)
- [ ] Même vérifications que 320px
- [ ] Un peu plus d'espace pour le contenu
- [ ] Tableaux légèrement plus larges

### 768px (iPad Portrait)
- [ ] Transition entre mobile et tablette
- [ ] Navigation peut être horizontale ou verticale
- [ ] Plus de colonnes visibles dans les tableaux
- [ ] Cartes en 2 colonnes

### 1024px (Desktop Minimum)
- [ ] Navigation horizontale complète
- [ ] Plus de colonnes dans les grilles
- [ ] Tableaux plus larges
- [ ] Plus d'espace pour le contenu

### 1280px (Desktop Standard)
- [ ] Expérience desktop complète
- [ ] Toutes les colonnes visibles
- [ ] Espacement confortable
- [ ] Images à taille normale

### 1440px+ (Large Desktop)
- [ ] Même vérifications que 1280px
- [ ] Plus d'espace disponible
- [ ] Contenu bien centré

---

## 🔍 Problèmes Courants à Vérifier

### Débordement Horizontal
- [ ] `overflow-x: hidden` ou `auto` sur les conteneurs
- [ ] Largeurs en pourcentage (%) ou viewport (vw)
- [ ] Pas de `width: fixed` non géré
- [ ] Images avec `max-width: 100%`

### Texte Illisible
- [ ] Taille de police minimum 12px sur mobile
- [ ] Contraste suffisant (4.5:1 minimum)
- [ ] Texte ne se chevauche pas
- [ ] Espacement entre lignes suffisant

### Éléments Non Cliquables
- [ ] Boutons minimum 44×44px sur mobile
- [ ] Liens minimum 44×44px sur mobile
- [ ] Espacement suffisant entre les éléments
- [ ] Z-index correct pour les overlays

### Images
- [ ] Images avec `max-width: 100%`
- [ ] Images avec `height: auto`
- [ ] Images optimisées pour mobile
- [ ] Images lazy-loaded si nécessaire

### Performance
- [ ] Temps de chargement < 3s sur mobile (3G)
- [ ] Scroll fluide (60fps)
- [ ] Pas de blocage du thread principal
- [ ] Cache efficace

---

## 📝 Rapport de Bugs

Utilisez ce tableau pour noter les problèmes trouvés :

| Composant | Appareil | Problème | Gravité | Statut |
|-----------|----------|----------|---------|--------|
| Exemple   | iPhone SE | Texte trop petit | Haute | À corriger |
|           |          |          |         |        |
|           |          |          |         |        |

---

## ✅ Validation Finale

Avant de considérer le responsive design comme terminé, vérifiez que :

- [ ] Tous les appareils de référence ont été testés
- [ ] Aucun débordement horizontal sur aucun appareil
- [ ] Tout le texte est lisible sur tous les appareils
- [ ] Toutes les fonctionnalités sont accessibles sur mobile
- [ ] La navigation est intuitive sur mobile
- [ ] Les formulaires sont utilisables sur mobile
- [ ] Les tableaux sont consultables sur mobile
- [ ] Les performances sont acceptables sur mobile
- [ ] L'accessibilité est maintenue sur mobile

---

## 🎯 Sign-off

**Testeur :** ___________________

**Date :** ___________________

**Version :** ___________________

**Statut :** [ ] ✅ Approuvé [ ] ⚠️ En cours [ ] ❌ Échoué

**Commentaires :**
________________________________________________________
