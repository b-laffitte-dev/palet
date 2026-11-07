---
name: kiss
description: Garder le code du projet simple (Keep It Simple, Stupid) — règles concrètes anti-sur-ingénierie appliquées au workspace Rust/Dioxus, checklist de revue de code.
triggers:
  - simple, simplifier, "trop complexe", "trop gros", illisible, "trop le bazar", refactor, nettoyer
  - KISS, "dette technique", over-engineering, "trop de couches", "trop abstrait", "trop générique"
  - "ça fait beaucoup", "rendre plus clair", "moins de code", "trop verbeux", complexité
---

# KISS — Keep It Simple, Stupid

> « La plupart des systèmes fonctionnent mieux quand on les garde simples. »
> Toute complexité inutile ajoute des points de panne et ralentit la lecture du code.

## Règles appliquées à ce projet

1. **YAGNI** — You Aren't Gonna Need It : pas de fonctionnalité « au cas où »,
   pas de configuration anticipée, pas d'API générique « pour plus tard ».
2. **Règle des trois** — n'extraire une abstraction (fonction, trait, composant,
   module) qu'à partir du **troisième** usage identique. Deux fois, c'est de la
   répétition acceptable.
3. **Une fonction = une idée, < ~30 lignes.** Si c'est plus long, se demander
   ce que la fonction fait *vraiment* et la découper.
4. **Un seul niveau d'abstraction par fonction.** Ne mélange pas « calculer le
   classement » et « formater du JSON » dans la même fonction.
5. **Noms qui disent l'intention** plutôt que des commentaires qui reformulent
   le code. Un commentaire qui explique *pourquoi* (contrainte métier du palet
   vendéen, ex. règles de score) est utile ; un commentaire qui dit *quoi* ne l'est pas.
6. **Supprimer le code mort** dès qu'il est repéré (code commenté, fonctions
   non appelées, dépendances inutilisées — `cargo machete` / `cargo udeps`).
7. **Préférer la bibliothèque standard** avant d'ajouter une dépendance.
   Avant tout nouvel ajout dans un `Cargo.toml`, se demander : « que m apporte
   cette crate que `std` ne fait pas en 5 lignes ? » (le workspace centralise
   déjà ses deps dans le `Cargo.toml` racine).
8. **La solution la plus simple qui marche maintenant**. Pas de design pattern
   complexe (factory, builder, visitor…) sans un besoin réel et nommable.
9. **Un commit = une idée.** Pas de « fix de tout en même temps ».

## Anti-patterns à éviter ici

- Sur-architecture en couches (controller/service/repository/… ) quand un
  handler direct suffit pour un CRUD simple.
- Génériques et traits complexes « pour rendre ça réutilisable » trop tôt.
- Macros maison qui compliquent la lecture.
- « Framework dans le framework » : réinventer dans Dioxus ce que le navigateur/CSS
  fait déjà nativement.
- Optimisation prématurée (cache, pool, async compliqué) sans mesure.

## Checklist de revue de PR (à cocher)

- [ ] Chaque fonction tient en un écran et fait une seule chose.
- [ ] Pas d'abstraction introduite avant le 3e usage.
- [ ] Pas de dépendance ajoutée sans justification écrite dans la PR.
- [ ] Pas de code mort, pas de `dbg!`/`println!` de debug oubliés.
- [ ] Les noms de variables/fonctions/components se comprennent sans lire le corps.
- [ ] Le code nouveau pourrait-il être plus simple ? Si oui, le simplifier avant merge.

## Exemple concret

Au lieu de :

```rust
// Mauvais : abstraction prématurée
trait ScoreStrategy { fn compute(&self, m: &Match) -> u8; }
struct WinStrategy; struct DrawStrategy; struct LossStrategy;
```

Écrire :

```rust
// Bien : simple, lisible, suffisant
fn points_classement(resultat: Resultat) -> u8 {
    match resultat {
        Resultat::Victoire => 3,
        Resultat::Nul => 1,
        Resultat::Defaite => 0,
    }
}
```

Si un jour un autre barème arrive, on extraira — à ce moment-là seulement.
