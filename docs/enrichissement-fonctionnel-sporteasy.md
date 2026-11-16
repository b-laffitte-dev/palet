# Enrichissement Fonctionnel — Inspiration SportEasy

> **Complément fonctionnel** de `docs/specifications-fonctionnelles.md`.
> Source d'inspiration : la plateforme **SportEasy** (gestion d'équipe et de club
> de sport amateur : calendrier partagé, convocations, disponibilités, suivi des
> présences, compositions, statistiques, messagerie, covoiturage, sondages,
> médias, cagnotte).
>
> **Statut** : fonctionnel uniquement. Aucune implémentation, aucune technologie
> prescrite. Ce document décrit le **quoi** et le **pourquoi**, pas le **comment**.
>
> **Version** : 1.0
> **Date** : 16/11/2026
> **Auteurs** : Équipe Technique Palet Vendéen

---

## 1. Objectif et périmètre

Les modules 1 à 7 de `specifications-fonctionnelles.md` couvrent la **fédération**
(compétitions officielles, résultats, classements, profils, communication
institutionnelle). Il manque la **vie d'équipe au quotidien** : entre deux
journées de championnat, qui est disponible, qui joue, qui conduit, qui ramène
les maitres, comment on valide la feuille de match.

Cet enrichissement reprend les mécanismes qui font le succès de SportEasy et
les adapte au **palet vendéen** (doublette / triplette / tête-à-tête, manches,
maitre, victoire à 13 points — 15 en finale, clubs de Vendée 85 et Deux-Sèvres
79, déplacements fréquents entre communes).

**Dans le périmètre** : espace équipe, disponibilités, convocations, composition,
feuille de match digitale, déplacements, tâches/sondages, fil d'équipe, médias,
assiduité, cagnotte, page publique.

**Hors périmètre** (volontairement) : tout ce qui est déjà couvert ailleurs —
saisie des résultats officiels et classements (Module 1), profils joueurs/clubs
(Module 2), notifications génériques et newsletter (Module 5), auth (Module 7).
Ici on décrit uniquement la dimension **"équipe au jour le jour"**.

---

## 2. Principes directeurs empruntés à SportEasy

| Principe SportEasy | Traduction pour le palet vendéen |
|---|---|
| Un seul endroit pour l'info d'équipe (plus de groupes WhatsApp/SMS éparpillés) | Un **espace équipe** par équipe de club : calendrier, dispo, annonces, médias |
| Répondre à une convocation en un clic (oui / non / peut-être) | Le joueur déclare sa **disponibilité** pour chaque match, entraînement ou tournoi |
| Relances automatiques auprès des non-répondants | Le capitaine **relance en un clic** ceux qui n'ont pas répondu avant la clôture |
| Composition d'équipe avant le match (titulaires / remplaçants) | Le capitaine compose sa **doublette / triplette** avec les postes (tireur, milieu, pointeur) |
| Bilan d'assiduité (présences aux matchs et entraînements) | **Taux de présence** par joueur, valorisé en fin de saison |
| Covoiturage pour les déplacements | Organisation des **places en voiture** pour les matchs à l'extérieur |
| Tâches "qui ramène quoi" | Qui ramène les **palets, les maitres, la plaque**, etc. |
| Sondages | Choisir une date d'entraînement, le lieu d'un repas d'équipe, etc. |
| Fil d'actu avec accusé de lecture | Les **annonces importantes** du capitaine vues par tous ("lu par X membres") |
| Album médias de l'équipe | Photos/vidéos des matchs et événements du club |
| Cagnotte / cotisations | Cotisations, **amendes d'absence**, petites dépenses du club |
| Vote du joueur du match | **Meilleur joueur de la manche/match** voté par les coéquipiers |
| Page publique de l'équipe | La page club/équipe visible par les **supporters et visiteurs** |

---

## 3. Modules fonctionnels détaillés

Chaque module décrit : acteurs, parcours principal, règles métier, données
fonctionnelles clés, priorité.

---

### E1. Espace équipe (le « vestiaire numérique »)

**Description** : un espace dédié par équipe de club (ex. « AS La Roche D1
Triplette Fonte »), distinct de la fiche club publique. Un club peut gérer
plusieurs équipes (D1, D2, vétérans, jeunes…).

**Acteurs** : Responsable Club, Capitaine d'équipe, Joueur, Supporter (lecture
publique limitée).

**Parcours** :
1. Le Responsable Club crée une équipe dans son club (nom, catégorie, format
   habituel doublette/triplette, matériel fonte/laiton, couleur de maillot/logo).
2. Il invite des membres par **lien d'invitation ou code d'équipe** ; le joueur
   rejoint en créant/se connectant à son compte.
3. Chaque membre a un **rôle dans l'équipe** : Capitaine, Responsable, Joueur,
   Remplaçant, Supporter.
4. L'équipe dispose d'un tableau de bord : prochain événement, dernières annonces,
   dernières photos, bilan de dispo du prochain match.

**Données fonctionnelles** : équipe (id, nom, club, catégorie, format, matériel,
couleur), adhésion équipe (membre, rôle, date d'entrée, poste habituel).

**Priorité** : MVP.

---

### E2. Calendrier d'équipe

**Description** : le calendrier partagé de l'équipe, qui agrège les événements
liés à cette équipe.

**Types d'événements** : match officiel (journée de championnat, tournoi),
match amical, entraînement, déplacement (voir E6), événement social (repas,
AG…), réunion.

**Parcours** :
1. Le Capitaine (ou Responsable Club) crée un événement : date, heure, lieu
   (adresse du boulodrome/plaque), type, adversaire (pour un match), lien vers
   la compétition si officielle.
2. Les matchs officiels importés du championnat apparaissent automatiquement avec
   leur lieu (domicile/extérieur).
3. Chaque membre voit le calendrier de **ses** équipes et peut l'**exporter**
   vers son agenda personnel (format standard iCalendar) et activer des
   **rappels** (ex. J-1 à 20h, 2h avant).
4. Changement de dernière minute (horaire, lieu, annulation) : l'événement est
   modifié et une notification part à tous les membres concernés.

**Règles métier** :
- Un match officiel de championnat ne peut être modifié que par l'organisateur
  de la compétition ; l'équipe peut seulement ajouter un commentaire logistique.
- Les événements sont visibles des membres de l'équipe ; la page publique
  (E12) n'affiche que les matchs et résultats.

**Priorité** : MVP.

---

### E3. Disponibilités (réponses aux événements)

**Description** : pour chaque événement demandant une réponse, chaque membre
indique sa disponibilité.

**Réponses possibles** : ✅ Disponible / ❌ Indisponible / ❓ Incertain, avec
**commentaire libre** et, en cas d'indisponibilité, un **motif** (travail,
blessure, famille, autre).

**Parcours** :
1. À la création de l'événement (ou sur demande du capitaine), une **demande de
   disponibilité** est ouverte avec une **date/heure de clôture** paramétrable
   (ex. veille 20h pour un match du samedi).
2. Chaque membre répond en un clic depuis le calendrier ou la notification.
3. Le Capitaine voit en temps réel le récap : disponibles, indisponibles,
   incertains, **non-répondants**.
4. Avant la clôture, le capitaine peut **relancer automatiquement** (une
   notification) uniquement les membres n'ayant pas répondu.
5. Après clôture, les réponses sont figées (modifiables seulement avec
   dérogation du capitaine, tracée).

**Données fonctionnelles** : réponse (membre, événement, statut, commentaire,
motif, date de réponse, relancé oui/non).

**Priorité** : MVP.

---

### E4. Convocations et composition d'équipe

**Description** : à partir des disponibilités, le capitaine convoque et compose
l'équipe pour un match.

**Parcours** :
1. Le capitaine ouvre la **convocation** du prochain match.
2. Il voit les disponibles (E3) et sélectionne les titulaires selon le format :
   - tête-à-tête : 1 joueur ;
   - doublette : 2 joueurs ;
   - triplette : 3 joueurs, avec **poste de chacun** : pointeur, milieu, tireur.
3. Il désigne des **remplaçants** (membres de l'équipe convoqués mais non
   titulaires) et le capitaine de la feuille de match.
4. Il valide : les convoqués reçoivent une **notification de convocation** avec
   lieu, heure, adversaire, consignes (qui ramène quoi, covoiturage).
5. La composition peut être **partagée publiquement** (E12) et exportée pour la
   feuille de match papier si le règlement l'exige.

**Règles métier** :
- On ne peut sélectionner comme titulaire que des membres ayant répondu
  « disponible » (ou « incertain » avec confirmation du capitaine).
- Le poste « tireur » est celui qui engage le maitre en début de manche ; en
  triplette, chaque joueur a un poste fixe pour le match, modifiable en cours
  de match et tracé sur la feuille de match (E5).
- Un joueur ne peut être titularisé dans deux équipes du même club le même
  jour (alerte de conflit).

**Données fonctionnelles** : convocation (événement, titulaires + postes,
remplaçants, capitaine, date d'envoi, statut envoyé/validé).

**Priorité** : MVP.

---

### E5. Feuille de match digitale

**Description** : la feuille de match officielle du palet, saisie et signée
numériquement, en complément de la saisie des résultats de compétition
(Module 1).

**Parcours** :
1. Avant le match : la feuille reprend la composition convoquée (E4).
2. Pendant/après le match, le capitaine ou l'arbitre saisit :
   - les joueurs présents et leur poste ;
   - les **remplaçants entrés en cours de match** (et la manche concernée) ;
   - le **score manche par manche** (points de chaque équipe) ;
   - les points marqués par joueur (si suivi individuel souhaité) ;
   - les incidents éventuels (maitre sorti, contestation, blessure).
3. Les deux capitaines **valident/co-signent** la feuille, puis l'arbitre
   (ou l'organisateur) la valide définitivement.
4. La feuille validée alimente automatiquement le résultat officiel (Module 1)
   et l'historique de l'équipe (E9/E10).

**Règles métier** :
- Score cible : **13 points**, **15 en finale** (« la balle ») ; en cas d'égalité
  à 12-12, belle en 15 points.
- Le nombre de remplaçants autorisés et les règles de remplacement sont ceux du
  règlement de la compétition concernée.
- Une feuille non signée par les deux capitaines reste en statut « en attente ».

**Statuts** : 📝 Brouillon → ✅ Signée par les capitaines → ✅ Validée
(officielle) → ⚠️ Contestée → ❌ Annulée.

**Priorité** : MVP (saisie), V2 (signature électronique avancée).

---

### E6. Déplacements et covoiturage

**Description** : organisation logistique des matchs à l'extérieur (très
fréquent en championnat départemental : déplacement entre communes de 85/79).

**Parcours** :
1. Sur un événement « à l'extérieur », un membre (conducteur) propose un
   **trajet en voiture** : point de départ, heure de départ, nombre de places,
   participation aux frais (indicative, ex. au km).
2. Les autres membres s'inscrivent comme **passagers** (places attribuées,
   liste d'attente si complet).
3. Le conducteur ou un passager peut annuler (libération de la place, avec
   notification).
4. Le capitaine voit le récap des voitures disponibles pour son équipe.

**Données fonctionnelles** : trajet (événement, conducteur, départ, heure,
places totales/prises), inscription passager (membre, trajet, statut confirmé/
en attente/annulé).

**Priorité** : V2.

---

### E7. Tâches et sondages

**Description** : deux mécanismes d'organisation collective très utilisés sur
SportEasy.

**Tâches « qui fait quoi »** :
- Sur un événement, le capitaine crée des tâches : « Ramener les palets fonte »,
  « Ramener les maitres », « Ramener la plaque de plomb », « Faire le tirage au
  sort », « Préparer le goûter d'après-match ».
- Chaque membre peut s'assigner une tâche (ou être désigné), avec échéance.
- Statut : à faire / faite.

**Sondages** :
- Le capitaine crée un sondage à choix multiples (ex. « Date préférée pour
  l'entraînement de la semaine prochaine ? », « Qui est partant pour le repas
  de fin de saison le samedi X ? »).
- Chaque membre vote (plusieurs choix possibles selon le sondage) ; le résultat
  est visible en temps réel ; clôture manuelle ou à date fixée.

**Priorité** : V2 (tâches), V2 (sondages).

---

### E8. Fil d'équipe : annonces et messages

**Description** : la communication centralisée de l'équipe, qui remplace les
groupes de messagerie parallèles.

**Deux niveaux** :
1. **Annonces du capitaine/responsable** (fil d'actu) : message important avec
   **accusé de lecture** (« vu par N membres sur M »), possibilité d'épingler,
   de joindre un média. Exemples : « Changement d'horaire samedi », «
   Inscription au tournoi de X avant vendredi ».
2. **Messages libres de l'équipe** : discussion entre membres de l'équipe,
   avec mentions (@joueur), pièces jointes (photos, PDF), réactions (👍 ✅).

**Règles** :
- Un membre quitte l'équipe → il perd l'accès au fil (les annonces lues restent
  dans son historique personnel).
- Modération : le capitaine/responsable peut supprimer un message de son équipe.

**Priorité** : MVP (annonces avec lu/vu), V2 (discussion libre).

---

### E9. Médias de l'équipe (album)

**Description** : un album photo/vidéo par équipe, alimenté par les membres.

**Parcours** :
1. Chaque membre peut publier des photos/vidéos, rattachées à un événement
   (match, tournoi, repas) ou libres.
2. Les autres membres peuvent aimer/commenter.
3. Le responsable peut rendre certaines photos **publiques** (visibles sur la
   page publique E12) — ex. photo de l'équipe championne.
4. Taille/format et stockage : contraintes à définir côté technique (hors
   périmètre de ce document).

**Priorité** : V2.

---

### E10. Présences, assiduité et « meilleur joueur »

**Description** : le suivi de qui participe à quoi, et la valorisation des
joueurs.

**Bilan d'assiduité** :
- Pour chaque membre : nombre d'événements auxquels il était convoqué, nombre
  de présences (matchs joués, entraînements), nombre d'absences justifiées /
  non justifiées, **taux de présence** global et par type d'événement (match /
  entraînement).
- Visible par le capitaine/responsable (et par le joueur pour soi-même).
- Valorisation de fin de saison (ex. tableau d'assiduité du club).

**Vote du meilleur joueur** :
- Après un match, chaque membre de l'équipe peut voter pour le **meilleur
  joueur du match** (MVP) parmi les titulaires.
- Le MVP du match est affiché sur la feuille de match (E5) et cumulé dans les
  stats du joueur (Module 2).

**Forme de l'équipe** : à partir des feuilles de match, série en cours (V/D/N),
forme récente des 5 derniers matchs — affichée sur le tableau de bord équipe.

**Priorité** : V2 (assiduité), V3 (vote MVP, forme).

---

### E11. Cagnotte du club

**Description** : la petite comptabilité de vie de club, inspirée de la cagnotte
SportEasy.

**Fonctionnalités** :
- **Cotisations** : le trésorier enregistre les cotisations annuelles par membre
  (montant, date, payé/impayé), avec relances automatiques avant échéance.
- **Amendes** : amende forfaitaire en cas d'absence non justifiée à un match
  officiel (montant paramétrable par le club, ex. 2 €), appliquée depuis la
  feuille de match/les dispo (E3, E5).
- **Dépenses** : saisie de petites dépenses (achat de maitres, palets, buvette)
  avec justificatif photo, validation par le trésorier.
- **Solde et mouvements** : le trésorier voit le solde et l'historique ; les
  membres voient leur propre solde (cotisé / amendes / solde dû).

**Règles** : visibilité différenciée (trésorier : tout ; membre : son solde
uniquement). Paiement réel hors plateforme (virement/espèces) — la plateforme
suit, ne encaisse pas.

**Priorité** : V3.

---

### E12. Page publique de l'équipe / du club

**Description** : la vitrine publique, visible par les supporters et visiteurs
non connectés (acteur « Visiteur » du Module acteurs).

**Contenu** : nom/logo/couleurs de l'équipe, effectif (noms, postes habituels,
photos avec consentement), calendrier des prochains matchs, derniers résultats,
classement si championnat (lien Module 1), photos publiques (E9), palmarès.

**Règles** : aucune donnée privée (disponibilités, messages, cagnotte,
coordonnées persos) n'y est publiée. Le responsable club contrôle la
visibilité.

**Priorité** : V2.

---

## 4. Parcours utilisateurs types

### Scénario A — « Samedi, journée de championnat à l'extérieur »
1. Lundi : le capitaine voit le match de samedi importé du calendrier (E2) et
   ouvre la demande de dispo (E3) avec clôture vendredi 20h.
2. Les joueurs répondent oui/non en un clic ; vendredi 18h le capitaine relance
   les 2 non-répondants.
3. Vendredi 21h : il convoque (E4) — triplette avec postes — et publie la compo
   dans le fil d'équipe (E8).
4. Un membre propose un covoiturage 4 places (E6) ; 3 coéquipiers s'inscrivent.
5. Le capitaine crée la tâche « qui ramène les maitres ? » (E7).
6. Samedi après le match : feuille de match saisie manche par manche, co-signée
   par les deux capitaines (E5) ; le résultat part en validation officielle.
7. Dimanche : le capitaine publie 3 photos du match (E9) et lance le vote MVP
   (E10) ; le bilan d'assiduité se met à jour.

### Scénario B — « Organiser un tournoi amical du club »
1. Le responsable crée l'événement tournoi (E2), ouvre les inscriptions équipe
   par équipe du club et externes (Module 3, §3.2).
2. Sondage (E7) pour fixer la date du repas de clôture.
3. Tâches (E7) : qui installe les plaques, qui fait le tirage au sort, qui tient
   le buvette.
4. Le jour J : feuilles de match digitales pour chaque partie (E5), photos (E9).

### Scénario C — « Fin de saison, assiduité et cotisations »
1. Le trésorier consulte le bilan d'assiduité (E10) et les cotisations (E11) :
   relance des impayés.
2. Les amendes d'absence non justifiée ont été appliquées automatiquement depuis
   les feuilles de match.
3. Le responsable publie la photo de l'équipe sur la page publique (E12) et le
   bilan de la saison.

---

## 5. Règles métier palet à respecter dans ces modules

- **Formats** : tête-à-tête (1), doublette (2), triplette (3).
- **Postes** (triplette) : pointeur, milieu, tireur ; en doublette : un tireur
  et un pointeur/milieu selon la compo choisie par le capitaine.
- **Score** : victoire à **13 points**, **15 en finale** ; égalité 12-12 → belle
  en 15.
- **Manche** : le maitre est placé par une équipe (2 essais), puis les palets
  sont lancés un à un ; on marque 1 point par palet plus proche du maitre que le
  premier palet adverse.
- **Matériel** : fonte (54 mm, ~100 g, lancer 3,80 m) ou laiton (40 mm, ~50 g,
  2,80 m) sur plaque de plomb 45×45 cm — le matériel de l'équipe est précisé
  dans E1 et rappelé dans les tâches (E7).
- **Remplaçants** : selon le règlement de la compétition ; toujours tracé sur la
  feuille de match (E5).

---

## 6. Matrice acteurs × nouveaux modules

| Module | Joueur | Capitaine | Resp. Club | Arbitre | Organisateur | Dept/National Admin | Supporter/Visiteur |
|---|---|---|---|---|---|---|---|
| E1 Espace équipe | membre | gère | crée/gère | ❌ | ❌ | supervise | ❌ |
| E2 Calendrier équipe | consulte | crée | crée | importe matchs | crée officiels | valide officiels | lit (public) |
| E3 Disponibilités | répond | consulte/reli | consulte | ❌ | ❌ | ❌ | ❌ |
| E4 Convocations/compo | convoqué | crée | crée | valide | valide | ❌ | lit compo (public) |
| E5 Feuille de match | co-signe | saisit/co-signe | ❌ | saisit/valide | valide | supervise | lit résultat |
| E6 Covoiturage | passager/conducteur | voit récap | voit récap | ❌ | ❌ | ❌ | ❌ |
| E7 Tâches/sondages | répond/participe | crée | crée | ❌ | crée | ❌ | ❌ |
| E8 Annonces/fil | lit/répond | publie | publie | ❌ | annonce | annonce | lit annonces publiques |
| E9 Médias | publie/aime | publie/modère | modère | ❌ | ❌ | ❌ | lit médias publics |
| E10 Assiduité/MVP | vote/consulte soi | consulte | consulte | alimente | ❌ | consulte | lit bilan public |
| E11 Cagnotte | voit son solde | ❌ | trésorier | ❌ | ❌ | ❌ | ❌ |
| E12 Page publique | gère son profil public | | publie | ❌ | ❌ | ❌ | consulte |

---

## 7. Priorisation

| Tranche | Modules | Justification |
|---|---|---|
| **MVP** | E1, E2, E3, E4, E5 (saisie), E8 (annonces) | Cœur de la valeur SportEasy : savoir qui joue, composer, officialiser le match, informer |
| **V2** | E5 (signature), E6, E7, E9, E10 (assiduité), E12 | Logistique et valorisation, forte valeur perçue mais moins urgente |
| **V3** | E10 (MVP/forme), E11 | Confort et vie sociale du club |

---

## 8. Hors périmètre et questions ouvertes

**Hors périmètre** : paiement en ligne (la cagnotte suit, n'encaisse pas) ;
profil « parent » (les catégories jeunes existent mais la gestion parent/enfant
n'est pas retenue pour l'instant) ; live score public en temps réel (déjà traité
côté home/Module 1) ; arbitrage vidéo.

**Questions ouvertes pour le lead** :
1. La feuille de match digitale (E5) doit-elle **remplacer** la saisie résultats
   du Module 1 ou seulement l'alimenter ? (Recommandation : l'alimenter, le
   Module 1 reste la source officielle du classement.)
2. Niveau de visibilité des disponibilités : toute l'équipe, ou seulement
   capitaine + responsables ?
3. Montant et règles des amendes d'absence (E11) : fixés par le club ou par la
   commission départementale ?
4. La page publique (E12) couvre-t-elle le **club entier** ou chaque **équipe**
   séparément ? (Recommandation : les deux, l'équipe étant la page par défaut.)
