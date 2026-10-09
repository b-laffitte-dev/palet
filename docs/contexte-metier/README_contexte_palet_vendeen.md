# **📁 Dossier Contexte Métier - Palet Vendéen**

> *Collection complète d'informations pour la construction de solutions métiers autour du palet vendéen*

---

## **📚 Contenu du Dossier**

### **1. Document Principal**
📄 **[`contexte_metier_palet_vendeen.md`](contexte_metier_palet_vendeen.md)**
- **Taille** : ~17 Ko
- **Format** : Markdown
- **Contenu** : Document complet et détaillé avec 10 sections majeures
- **Public cible** : Équipes techniques, architectes, développeurs, chefs de projet
- **Utilisation** : Référence exhaustive pour comprendre le domaine

**Sections incluses** :
1. Introduction et contexte historique
2. Matériel et équipement (dimensions, poids, caractéristiques)
3. Règles officielles du jeu (détaillées)
4. Déroulement des parties et matchs
5. Organisation des championnats (tous niveaux)
6. Structure organisationnelle (FNSMR, CVDP, clubs)
7. Variantes du jeu (fonte vs laiton, autres variantes)
8. Aspects culturels et sécurité
9. Lexique et terminologie complète
10. Données techniques de référence

---

### **2. Synthèse Technique (JSON)**
📄 **[`contexte_metier_palet_vendeen_synthese.json`](contexte_metier_palet_vendeen_synthese.json)**
- **Taille** : ~9 Ko
- **Format** : JSON structuré
- **Contenu** : Toutes les données organisées en objets JSON
- **Public cible** : Développeurs, data scientists, systèmes automatisés
- **Utilisation** : Intégration dans des applications, bases de données, APIs

**Structure JSON** :
```json
{
  "palet_vendeen": {
    "metadata": {...},
    "histoire": {...},
    "geographie": {...},
    "materiel": {...},
    "regles": {...},
    "competitions": {...},
    "organisation": {...},
    "variantes": {...},
    "culture_et_securite": {...},
    "lexique": {...},
    "besoins_metier": {...}
  }
}
```

---

### **3. Résumé Exécutif**
📄 **[`contexte_metier_palet_vendeen_resume.md`](contexte_metier_palet_vendeen_resume.md)**
- **Taille** : ~6 Ko
- **Format** : Markdown
- **Contenu** : Version synthétique et actionnable
- **Public cible** : Direction, chefs de projet, parties prenantes métiers
- **Utilisation** : Présentation rapide, prise de décision, roadmap

**Sections clés** :
- En bref (définition, chiffres clés)
- Structure des compétitions (hiérarchie, formats)
- Données clés (matériel, joueurs, classement)
- Besoins fonctionnels identifiés
- Opportunités d'innovation
- Prochaines étapes (feuille de route)

---

## **🎯 Comment Utiliser Ces Documents**

### **Pour les Développeurs**
```
1. Commencez par le JSON pour intégrer les données dans votre système
2. Consultez le document principal pour les détails métiers
3. Utilisez le résumé pour comprendre les priorités
```

### **Pour les Chefs de Projet**
```
1. Lisez le résumé exécutif pour la vision globale
2. Explorez le document principal pour les détails
3. Utilisez le JSON pour les spécifications techniques
```

### **Pour la Direction**
```
1. Le résumé exécutif suffit pour la compréhension globale
2. Le document principal pour approfondir un point spécifique
3. Le JSON pour les indicateurs clés (KPIs)
```

---

## **🔍 Index des Informations**

### **Recherche Rapide**

| **Thème** | **Où trouver** | **Section/Fichier** |
|-----------|----------------|---------------------|
| **Règles du jeu** | Document principal | Section 3 |
| **Dimensions matérielles** | Document principal / JSON | Section 2 / materiel |
| **Calendrier compétitions** | Document principal / JSON | Section 5 / competitions.calendrier_2026 |
| **Organisation (FNSMR, CVDP)** | Document principal / JSON | Section 6 / organisation |
| **Besoins fonctionnels** | Résumé exécutif / JSON | besoins_metier |
| **Lexique** | Document principal / JSON | Section 9 / lexique |
| **Données techniques** | Document principal | Section 10 |

---

## **📊 Statistiques et Chiffres Clés**

### **Matériel**
- Plaque : 45cm × 45cm, **≥ 20kg**, plomb
- Palets fonte : **54mm Ø, ~100g**, lancer à **3,80m**
- Palets laiton : **40mm Ø, ~50g**, lancer à **2,80m**
- Maître : petit palet de couleur distincte

### **Compétitions**
- **Points pour gagner** : 13 (15 en finale)
- **Format** : 1v1, 2v2 (doublette), 3v3 (triplette)
- **Classement** : Points → Différence → Points marqués (Bp)

### **Organisation**
- **Fédération** : FNSMR (nationale)
- **Commissions** : CVDP (Vendée), CDSP (Deux-Sèvres), etc.
- **Clubs** : 100+ clubs en Vendée et départements voisins

---

## **🚀 Prochaines Étapes Recommandées**

### **1. Validation du Contexte**
- [ ] Vérifier les informations avec la **CVDP** ou **FNSMR**
- [ ] Interviews des organisateurs de tournois
- [ ] Analyse des processus actuels (inscriptions, arbitrage)

### **2. Priorisation des Besoins**
- [ ] Atelier avec les parties prenantes
- [ ] Cartographie des processus métiers
- [ ] Identification des points de friction

### **3. Conception Technique**
- [ ] Spécifications fonctionnelles détaillées
- [ ] Architecture système
- [ ] Maquettes UI/UX

### **4. Développement**
- [ ] MVP pour la gestion des compétitions
- [ ] Intégration avec les systèmes existants
- [ ] Tests utilisateurs

---

## **📞 Contacts et Ressources**

### **Organisations**
- **FNSMR** : [https://www.le-palet.com/](https://www.le-palet.com/)
- **CVDP** : Commission Vendéenne Du Palet (intégré à FNSMR)
- **CDSMR 85** : Comité Départemental du Sport en Milieu Rural de Vendée

### **Clubs Locaux (Exemples)**
- Club des Essarts (Vendée)
- Palet du Marais Champagnelais
- La Sportive (Saint Pardoux)

### **Partenaires**
- Crédit Mutuel Océan
- Département de la Vendée
- Agence Nationale du Sport

---

## **⚠️ Notes Importantes**

### **Points d'Attention**
1. **Sécurité** : Risque de saturnisme avec les plaques de plomb → à prendre en compte dans la gestion du matériel
2. **Variantes régionales** : Règles légèrement différentes selon les départements (fonte vs laiton)
3. **Saisonnalité** : Compétitions de septembre à juin, avec pic d'activité au printemps
4. **Évolutivité** : Nombre de participants variable (de 2 à 276+ pour les grands tournois)

### **Limites des Documents**
- Les informations sont basées sur des sources publiques (sites officiels, Wikipedia)
- Certaines données (comme les contacts exacts) nécessitent une vérification directe
- Les calendriers peuvent évoluer (consulter [le-palet.com](https://www.le-palet.com/) pour les mises à jour)

---

## **📝 Historique des Versions**

| **Version** | **Date** | **Modifications** | **Auteur** |
|-------------|----------|------------------|-----------|
| 1.0 | 09/10/2026 | Création initiale (recherche complète + structuration) | Benzo |

---

## **🎓 Ressources Complémentaires**

### **Liens Utiles**
- [Site officiel du Palet (FNSMR)](https://www.le-palet.com/)
- [Wikipédia - Palet vendéen](https://fr.wikipedia.org/wiki/Palet_vend%C3%A9en)
- [Règles du jeu - Palet-Vendéen.fr](https://www.palet-vendeen.fr/regles-du-palet-vendeen)
- [Règles - PétanqueShop](https://www.petanqueshop.com/regle-palet-vendeen.html)
- [Règles - Decathlon Conseils](https://conseilsport.decathlon.fr/jeu-de-palets-vendeens-les-regles-du-jeu)

### **Documents Externes**
- Règlement officiel du palet (version 2009) - FNSMR
- Championnat de Vendée Fonte 2025-2026 - CVDP
- PDF des règles (disponible sur certains sites)

---

## **💡 Conseils pour la Suite**

1. **Commencez par le résumé exécutif** si vous avez peu de temps
2. **Utilisez le JSON** pour intégrer les données dans vos systèmes
3. **Consultez le document principal** pour approfondir un sujet spécifique
4. **Validez les informations** avec les acteurs du terrain (CVDP, clubs)
5. **Priorisez les besoins** en fonction des retours utilisateurs

---

*Dossier créé le 09/10/2026*
*Dernière mise à jour : 09/10/2026*
*Pour toute question : consulter les documents ou contacter la FNSMR/CVDP*
