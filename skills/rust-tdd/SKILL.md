---
name: rust-tdd
description: Appliquer le TDD dans le workspace Rust Palet Vendéen (backend Actix, shared, frontend/mobile Dioxus 0.7) — cycle red-green-refactor, organisation des tests, outils et commandes de test, exemples concrets.
triggers:
  - test, tests, TDD, couverture, bug, régression, "corrige ce bug", "ça ne marche pas", "comportement attendu"
  - unitaire, intégration, e2e, assert, "cargo test", wasm-bindgen-test, proptest, mock, fixture
  - recette, QA, "ajoute un test", "scénario de test", "ça plante", exception, panic en prod
---

# TDD en Rust — Palet Vendéen

## Le cycle (toujours dans cet ordre)

1. **RED** — écrire un test qui échoue (le comportement n'existe pas encore).
2. **GREEN** — écrire le code minimal qui fait passer le test. Pas plus.
3. **REFACTOR** — nettoyer (noms, extraction, idiomes) en gardant les tests au vert.

Règles : petits pas, un comportement = un test, commit à chaque cycle vert,
ne jamais écrire de code de prod sans test préalable pour la logique métier
(règles de classement, scores, calculs de championnat dans `rust-shared` et
`rust-backend/src/services/`).

## Organisation des tests dans ce workspace

- **Tests unitaires** : `#[cfg(test)] mod tests` en bas du fichier testé
  (déjà le cas dans `rust-backend/src/config.rs`, `db.rs`, `services/palet_rules_service.rs`).
- **Tests d'intégration** : dossier `tests/` à la racine de chaque crate
  (ex. `rust-backend/tests/api_clubs.rs` avec `actix_web::test`).
- **Tests de sérialisation** des types partagés : dans `rust-shared/src/*` ou
  `rust-shared/tests/` — toujours tester la round-trip JSON (serde).
- **Frontend/mobile Dioxus** :
  - composants : rendre le `rsx!` avec `dioxus-ssr` et comparer avec
    `pretty-assertions` (voir exemples ci-dessous) ;
  - hooks/state : piloter un `VirtualDom` à la main (pattern `test_hook` de la
    doc officielle Dioxus 0.7) ;
  - WASM/navigateur : `wasm-bindgen-test` (déjà en dev-dependency de `palet-frontend`) ;
  - E2E : Playwright avec `dx serve` comme serveur web (config `webServer` de
    la doc Dioxus 0.7).

## Commandes

```bash
cargo test --workspace                          # tous les tests natifs
cargo nextest run                               # plus rapide, meilleur rapport (à installer : cargo install cargo-nextest)
cargo llvm-cov --workspace --html               # couverture (cargo install cargo-llvm-cov)
cargo test -p palet-frontend --target wasm32-unknown-unknown  # logique pure du frontend
wasm-pack test --headless --chrome -p palet-frontend              # tests wasm-bindgen-test
cd rust-frontend && dx test                                     # si configuré
```

## Crates de test recommandées (dev-dependencies)

| Crate | Usage |
|---|---|
| `rstest` | tests paramétrés (`#[case]`) et fixtures (`#[fixture]`) — évite de copier-coller les setups |
| `proptest` | property-based testing sur les règles métier (ex. un classement reste trié après n matchs) |
| `mockall` | mocker les traits (repositories, clients API) dans le backend |
| `pretty-assertions` + `dioxus-ssr` | comparer le rendu rsx des composants Dioxus |
| `actix-web` `test` module | tests d'API HTTP du backend |

## Règles de nommage et de structure

- Nom explicite : `fn classement_reste_trie_apres_ajout_match()` ou
  `given_invalid_score_when_complete_match_then_400()`.
- Structure **AAA** : Arrange (préparer les données) / Act (appeler) / Assert (vérifier).
- Un `assert!` ciblé par test quand c'est possible ; plusieurs petites assertions
  liées au même comportement restent acceptables.
- Ne teste pas le code trivial (getters, constructeurs) ; teste les règles métier.

## Exemple — test unitaire (style du projet)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn victoire_donne_trois_points() {
        // Arrange
        let m = Match::new(...);
        // Act
        let points = points_classement(&m, Resultat::Victoire);
        // Assert
        assert_eq!(points, 3);
    }
}
```

## Exemple — test de composant Dioxus 0.7 (doc officielle)

```rust
use dioxus::prelude::*;

fn assert_rsx_eq(first: Element, second: Element) {
    let first = dioxus_ssr::render_element(first);
    let second = dioxus_ssr::render_element(second);
    pretty_assertions::assert_str_eq!(first, second);
}

#[test]
fn bouton_compteur_rend_le_compteur() {
    assert_rsx_eq(
        rsx! { button { "Compteur : 0" } },
        rsx! { Compteur { initial: 0 } },
    );
}
```

(dev-dependencies : `dioxus-ssr`, `pretty-assertions`)

## Check-list avant de merger

- [ ] Chaque nouvelle règle métier a au moins un test écrit avant le code.
- [ ] `cargo test --workspace` passe.
- [ ] `cargo clippy --workspace -- -D warnings` passe.
- [ ] Les tests couvrent les cas nominaux **et** les cas d'erreur (400/404/validation).

## Références

- Guide testing Dioxus 0.7 : https://dioxuslabs.com/learn/0.7/guides/testing/web/
- The Rust Book — tests : https://doc.rust-lang.org/book/ch11-00-testing.html
- rstest : https://docs.rs/rstest · proptest : https://docs.rs/proptest · mockall : https://docs.rs/mockall
