# Matrice des Acteurs et des Droits — Palet Vendéen (v1)

> Matrice de contrôle d'accès **fonctionnelle** (qui peut faire quoi, et où).
> Complète la « Matrice des Permissions » simplifiée (✅/❌) de
> `docs/specifications-fonctionnelles.md` en précisant les **niveaux de droit**
> et les **périmètres**.
>
> **Statut** : fonctionnel uniquement, aucune implémentation prescrite.
>
> **Version** : 1.0
> **Date** : 16/11/2026
> **Auteurs** : Équipe Technique Palet Vendéen

---

## 1. Conventions de droits

| Code | Droit | Signification |
|---|---|---|
| 👁 **L** | Lecture | Consulter / voir |
| ➕ **C** | Création | Créer une ressource |
| ✏️ **M** | Modification | Modifier (souvent « de ses propres données » ou « de son périmètre ») |
| 🗑 **S** | Suppression | Supprimer / dissoudre |
| ✅ **V** | Validation | Approuver, valider, co-signer, relancer, modérer |
| ⚙️ **A** | Administration | Gérer les membres, les rôles, les paramètres d'un périmètre |
| ❌ | Aucun | Pas d'accès |

**Périmètre** : tout droit s'applique dans un périmètre délimité —
`équipe` (ses équipes), `club`, `compétition`, `département`, `national`,
`système`. Exemple : un Capitaine a `A` sur **ses équipes**, mais ❌ ailleurs.

**Rôles cumulables** : un utilisateur peut cumuler plusieurs acteurs
(ex. Joueur + Capitaine + Arbitre). Les droits s'additionnent.

---

## 2. Acteurs et périmètres

| # | Acteur | Définition | Périmètre naturel |
|---|---|---|---|
| 1 | **Visiteur** | Non connecté | Public uniquement |
| 2 | **Supporter** | Compte créé, non rattaché à un club | Public + son compte |
| 3 | **Joueur** | Licencié, rattaché à un club et à une/des équipes | Ses données, son club, ses équipes |
| 4 | **Capitaine d'équipe** | Joueur avec rôle « capitaine » sur une équipe | Son équipe (droits étendus) |
| 5 | **Responsable Club** | Membre du bureau d'un club | Son club |
| 6 | **Trésorier** | Rôle financier d'un club (souvent cumulé avec Resp. Club) | Cagnotte de son club (E11) |
| 7 | **Arbitre** | Officiel habilité sur des compétitions | Compétitions désignées |
| 8 | **Organisateur** | Responsable de tournois/competitions qu'il a créés | Ses compétitions |
| 9 | **Admin Départemental** | Représentant CVDP 85 / CDSP 79 / etc. | Son département |
| 10 | **Admin National** | Représentant FNSMR | National (tous départements) |
| 11 | **Super Admin** | Administrateur technique de la plateforme | Système entier (audité) |

**Hiérarchie** : National ⊃ Départemental ⊃ Club ⊃ Équipe ⊃ Joueur.
Un niveau supérieur englobe le périmètre inférieur (ex. l'Admin National voit
et peut agir partout, l'Admin Départemental seulement dans son 85/79/…).

---

## 3. Matrice détaillée par domaine

Légende des colonnes : **Vis** = Visiteur, **Sup** = Supporter, **Jou** = Joueur,
**Capt** = Capitaine, **RC** = Responsable Club, **Trés** = Trésorier,
**Arb** = Arbitre, **Org** = Organisateur, **AD** = Admin Dép., **AN** = Admin Nat.,
**SA** = Super Admin.

### A. Consultation publique (home, résultats, classements, calendriers publics, actualités, pages publiques club/équipe E12, profils publics)

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Voir résultats & classements | L | L | L | L | L | L | L | L | L | L | L |
| Voir calendrier public compétitions | L | L | L | L | L | L | L | L | L | L | L |
| Voir actualités | L | L | L | L | L | L | L | L | L | L | L |
| Voir page publique club/équipe (E12) | L | L | L | L | L | L | L | L | L | L | L |
| Voir profil public joueur (nom, poste, stats, palmarès) | L | L | L | L | L | L | L | L | L | L | L |
| Voir données privées (dispo, messages, cagnotte, coordonnées) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

### B. Compte et profil personnel

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Créer un compte / se connecter | C | C | C | C | C | C | C | C | C | C | C |
| Voir son propre profil complet | ❌ | L | L | L | L | L | L | L | L | L | L |
| Modifier son propre profil | ❌ | M | M | M | M | M | M | M | M | M | M |
| Voir profil complet d'un autre membre (coordonnées, licence) | ❌ | ❌ | L (son club/équipes) | L (son équipe) | L (son club) | ❌ | L (compétitions désignées) | L (ses compétitions) | L (son dép.) | L | L |
| Modifier le profil d'un autre (licence, statut) | ❌ | ❌ | ❌ | ❌ | M (son club) | ❌ | ❌ | ❌ | M (son dép.) | M | M |
| Gérer ses préférences de notifications | ❌ | M | M | M | M | M | M | M | M | M | M |
| Supprimer son compte (droit à l'oubli) | ❌ | S | S | S | S | S | S | S | S | S | S |

### C. Championnats et compétitions officielles

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Consulter championnats, poules, journées, règlement | L | L | L | L | L | L | L | L | L | L | L |
| Créer un championnat officiel | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | C (proposition) | C/V (validation) | C/V | C/V |
| Configurer championnat (format, journées, règles 13/15) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | M (les siens) | M (son dép.) | M | M |
| Générer le calendrier / tirage au sort / poules | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | C (les siens) | C (son dép.) | C | C |
| Reporter / annuler une journée | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | M (les siens) | M (son dép.) | M | M |
| Inscrire son club/équipe à une compétition | ❌ | ❌ | ❌ | C (son équipe) | C (son club) | ❌ | ❌ | ❌ | C (son dép.) | C | C |
| Valider les inscriptions | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | V (les siennes) | V (son dép.) | V | V |

### D. Matchs, résultats et feuilles de match

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Consulter matchs et résultats | L | L | L | L | L | L | L | L | L | L | L |
| Saisir un résultat / feuille de match (E5) | ❌ | ❌ | ❌ | C (matchs de son équipe) | ❌ | ❌ | C (compétitions désignées) | C (ses compétitions) | C (son dép.) | C | ❌ |
| Co-signer la feuille de match | ❌ | ❌ | V (si capitaine adverse/équipier désigné) | V | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Valider définitivement un résultat (mise à jour classement) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | V | V (ses compétitions) | V (son dép.) | V | ❌ |
| Contester un résultat | ❌ | ❌ | C (son équipe) | C | C (son club) | ❌ | ❌ | V (traite) | V (son dép.) | V | ❌ |
| Annuler un match | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | M | M (ses compétitions) | M (son dép.) | M | ❌ |
| Voir feuilles de match validées (historique) | L (résultat) | L | L | L (son équipe) | L (son club) | ❌ | L (désignées) | L (siennes) | L (son dép.) | L | L |

> Règle transverse : **séparation des devoirs** — celui qui saisit ne valide pas
> seul (co-signature + validation arbitre/organisateur/admin).

### E. Clubs et licences

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Consulter fiche club publique | L | L | L | L | L | L | L | L | L | L | L |
| Modifier fiche club (adresse, contacts, logo, réseaux) | ❌ | ❌ | ❌ | ❌ | M (son club) | ❌ | ❌ | ❌ | M (son dép.) | M | M |
| Créer / supprimer un club | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | C/S (son dép.) | C/S | C/S |
| Valider/affilier un club | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | V (son dép.) | V | V |
| Gérer les membres d'un club (ajouter/retirer joueur) | ❌ | ❌ | ❌ | ❌ | A (son club) | ❌ | ❌ | ❌ | A (son dép.) | A | A |
| Gérer licences (numéro, date, statut) | ❌ | ❌ | L (la sienne) / M (demande) | ❌ | M (son club) | ❌ | ❌ | ❌ | M (son dép.) | M | M |
| Transférer un joueur d'un club à l'autre | ❌ | ❌ | C (demande) | ❌ | V (club d'accueil) | ❌ | ❌ | ❌ | V (son dép.) | V | ❌ |

### F. Espace équipe — Modules E1 à E12 (`enrichissement-fonctionnel-sporteasy.md`)

Périmètre `équipe` pour Joueur/Capitaine ; `club` pour Responsable Club.

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E1 Créer une équipe dans son club | ❌ | ❌ | ❌ | ❌ | C | ❌ | ❌ | ❌ | C (son dép.) | C | C |
| E1 Gérer membres et rôles de l'équipe | ❌ | ❌ | ❌ | A (son équipe) | A (son club) | ❌ | ❌ | ❌ | L (son dép.) | L | A |
| E1 Rejoindre une équipe (lien/code invitation) | ❌ | C | C | C | C | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E1 Quitter une équipe / se retirer | ❌ | ❌ | S (soi) | S (soi) | M | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E2 Voir calendrier de ses équipes | ❌ | ❌ | L | L | L (club) | ❌ | L (désignées) | L (siennes) | L | L | L |
| E2 Créer/modifier événement d'équipe (entraînement, amical, social) | ❌ | ❌ | ❌ | C/M (son équipe) | C/M (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E2 Modifier un match officiel importé | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | M (ses compétitions) | M (son dép.) | M | M |
| E3 Répondre à une demande de dispo | ❌ | ❌ | C/M (soi) | C/M (soi) | ❌ | ❌ | C/M (si concerné) | ❌ | ❌ | ❌ | ❌ |
| E3 Voir les dispo individuelles des membres | ❌ | ❌ | ❌ | L (son équipe) | L (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | L |
| E3 Relancer les non-répondants | ❌ | ❌ | ❌ | V (son équipe) | V (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E4 Convoquer / composer l'équipe (postes, remplaçants) | ❌ | ❌ | ❌ | C/M/V (son équipe) | C/M/V (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E4 Voir la composition publiée | L (si publique) | L (si publique) | L | L | L | ❌ | L | L | L | L | L |
| E5 Saisir la feuille de match de son équipe | ❌ | ❌ | ❌ | C/M (son équipe) | ❌ | ❌ | C/M (désignées) | C/M (siennes) | ❌ | ❌ | ❌ |
| E6 Proposer un trajet / s'inscrire comme passager | ❌ | ❌ | C/M (soi) | C/M (soi) | L | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E6 Voir les trajets de l'équipe | ❌ | ❌ | L (son équipe) | L | L (club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E7 Créer tâches/sondages | ❌ | ❌ | ❌ | C (son équipe) | C (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E7 S'assigner une tâche / voter | ❌ | ❌ | C/M (soi) | C/M (soi) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E8 Publier une annonce (avec lu/vu) | ❌ | ❌ | ❌ | C (son équipe) | C (son club) | ❌ | ❌ | ❌ | C (son dép.) | C (national) | ❌ |
| E8 Lire le fil de son équipe | ❌ | ❌ | L | L | L (club) | ❌ | ❌ | ❌ | ❌ | ❌ | L |
| E8 Modérer (supprimer un message) | ❌ | ❌ | S (son propre message) | S (son équipe) | S (son club) | ❌ | ❌ | ❌ | S (son dép.) | S | S |
| E9 Publier des médias | ❌ | ❌ | C (ses équipes) | C | C (club) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E9 Modérer/rendre public un média | ❌ | ❌ | S (siens) | S (son équipe) | S/V (son club) | ❌ | ❌ | ❌ | ❌ | ❌ | S |
| E10 Voir le bilan d'assiduité | ❌ | ❌ | L (le sien) | L (son équipe) | L (son club) | ❌ | ❌ | ❌ | L (son dép.) | L | L |
| E10 Voter pour le MVP | ❌ | ❌ | C (matchs de son équipe) | C | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| E11 Voir la cagnotte (solde, mouvements) | ❌ | ❌ | ❌ | ❌ | ❌ | A (son club) | ❌ | ❌ | L (son dép.) | L | L |
| E11 Voir son propre solde | ❌ | ❌ | L (si membre cotisant) | L | L | L | ❌ | ❌ | ❌ | ❌ | ❌ |
| E11 Gérer cotisations/amendes/dépenses | ❌ | ❌ | ❌ | ❌ | ❌ | C/M/V (son club) | ❌ | ❌ | L (son dép.) | L | M |
| E12 Publier/gérer la page publique | ❌ | ❌ | ❌ | ❌ | M/V (son club) | ❌ | ❌ | ❌ | M (son dép.) | M | M |

### G. Statistiques et rapports

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Voir stats publiques (classements, meilleurs joueurs) | L | L | L | L | L | L | L | L | L | L | L |
| Voir ses propres stats détaillées | ❌ | L (limitées) | L | L | L | L | L | L | L | L | L |
| Voir stats de son équipe | ❌ | ❌ | L | L | L (club) | ❌ | ❌ | ❌ | ❌ | ❌ | L |
| Voir stats de son club (joueurs, équipes, finances) | ❌ | ❌ | ❌ | ❌ | L/A (son club) | L (finances) | ❌ | ❌ | L (son dép.) | L | L |
| Exporter rapports (PDF/Excel/CSV) | ❌ | ❌ | L (perso) | L (équipe) | C (club) | C (finances) | ❌ | C (ses compétitions) | C (son dép.) | C | C |
| Voir rapports fédéraux (tendances, 5 ans) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | L (son dép.) | L/A | L |

### H. Communication générale et notifications

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Recevoir notifications selon ses préférences | ❌ | L | L | L | L | L | L | L | L | L | L |
| Envoyer message privé | ❌ | C | C | C | C | C | C | C | C | C | ❌ |
| Recevoir annonces (fil) | L (publiques) | L | L (ses équipes/club) | L | L | ❌ | L (désignées) | L | L (son dép.) | L | L |
| Publier annonce générale (newsletter site) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | C (son dép.) | C | ❌ |
| Gérer modèles de newsletter, stats d'ouverture | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | M (son dép.) | M | M |

### I. Administration système et audit

| Fonctionnalité | Vis | Sup | Jou | Capt | RC | Trés | Arb | Org | AD | AN | SA |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Gérer les comptes utilisateurs (créer, suspendre, supprimer) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | A (national) | A |
| Attribuer/révoquer les rôles (arbitre, organisateur, admin dép.) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | A | A |
| Configurer le système (règles globales, barèmes, saisons) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | A |
| Consulter la piste d'audit (qui a fait quoi, quand) | ❌ | ❌ | ❌ | ❌ | L (son club, actions le concernant) | ❌ | ❌ | L (ses compétitions) | L (son dép.) | L | L |
| Corriger un résultat déjà validé (cas exceptionnel) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | V (son dép., tracé) | V (tracé) | V (tracé, audité) |

---

## 4. Matrice de synthèse (niveau maximal par domaine)

| Acteur | Public | Profil | Compétitions | Matchs/résultats | Clubs/licences | Espace équipe E1–E12 | Stats | Comm. | Admin système |
|---|---|---|---|---|---|---|---|---|---|
| Visiteur | L | ❌ | L | L | L | ❌ | L (public) | L (annonces publiques) | ❌ |
| Supporter | L | M (soi) | L | L | L | ❌ (sauf équipes rejointes) | L (perso limitée) | C (messages privés) | ❌ |
| Joueur | L | M (soi) | L + C (inscriptions équipe) | V (co-signature) | L (soi) | C/M/L (ses équipes) | L (soi/équipe) | C/L | ❌ |
| Capitaine | L | M (soi) | C (inscrire équipe) | C + V (son équipe) | ❌ | **A (son équipe)** | L (équipe) | C (annonces équipe) | ❌ |
| Responsable Club | L | M (soi) | C (inscrire club) | L | **A (son club)** | **A (équipes du club)** | C (rapports club) | C (annonces club) | L (audit club) |
| Trésorier | L | M (soi) | ❌ | ❌ | ❌ | **A (cagnotte E11)** | L (finances) | ❌ | ❌ |
| Arbitre | L | M (soi) | L (désignées) | **C + V (désignées)** | ❌ | C (feuilles désignées) | ❌ | L | ❌ |
| Organisateur | L | M (soi) | **C/M/V (ses compétitions)** | **C/V (ses compétitions)** | ❌ | ❌ | C (ses compétitions) | ❌ | L (audit) |
| Admin Départemental | L | M (son dép.) | **C/M/V (son dép.)** | **C/V (son dép.)** | **C/M/V (son dép.)** | L (son dép.) | C (son dép.) | C (annonces dép.) | L (audit dép.) |
| Admin National | L | M | **A** | **V** | **A** | L | C | C | **A (utilisateurs/rôles)** |
| Super Admin | L | M | M | ❌ (jamais de saisie sportive) | M | L | L | ❌ | **A (système, audité)** |

---

## 5. Règles transverses

1. **Moindre privilège** : on donne le droit minimal nécessaire ; les droits
   étendus (A, V) ne sont jamais globaux sauf Admin National / Super Admin.
2. **Périmètre obligatoire** : Capitaine = A sur *ses* équipes uniquement ;
   Responsable Club = A sur *son* club ; Admin Dép. = sur *son* département.
3. **Séparation des devoirs** sur les résultats officiels : saisie (capitaine ou
   arbitre) ≠ co-signature (capitaine adverse) ≠ validation finale
   (arbitre/organisateur/admin). Aucune personne ne fait les trois étapes seule.
4. **Visibilité publique vs privée** : résultats, classements, calendriers
   officiels, pages publiques, profils publics = visibles de tous (Visiteur) ;
   disponibilités individuelles, messages, covoiturage, cagnotte, coordonnées
   personnelles = jamais publics.
5. **Délégation et révocation** : le Responsable Club peut nommer/révoquer les
   capitaines de son club ; l'Admin Dép. nomme les organisateurs/arbitres de son
   département ; l'Admin National nomme les Admin Dép. ; seul le Super Admin
   touche à la configuration système. Toute révocation prend effet immédiatement.
6. **Fin de saison / changement de club** : les droits d'équipe et de club sont
   automatiquement révoqués à la sortie du membre (transfert, dissolution) ;
   l'historique (feuilles de match, résultats) reste consultable.
7. **Audit** : toute action C/M/S/V/A laisse une trace (qui, quoi, quand) ;
   consultable selon la matrice (I).
8. **Super Admin technique** : accès système complet mais **jamais de saisie ou
   modification de résultats sportifs** hors procédure de correction tracée.

---

## 6. Questions ouvertes pour le lead

1. Un joueur peut-il refuser d'être visible dans le profil public de son club
   (photo, nom) ? (Recommandation : oui, option par joueur.)
2. Les disponibilités individuelles (E3) sont-elles visibles par tous les membres
   de l'équipe ou seulement capitaine/responsable ? (Matrice ci-dessus : seulement
   capitaine/RC.)
3. Faut-il un rôle « Secrétaire » distinct du Responsable Club pour la saisie des
   résultats, ou le capitaine suffit ?
4. Délégation Admin Dép. → Admin National : faut-il un niveau « régional »
   (ex. ligue Pays de la Loire) entre les deux ?
5. Durée de conservation de la piste d'audit et droit de consultation du joueur
   sur ses propres traces (RGPD) ?
