---
name: rust-idioms
description: Écrire du Rust idiomatique dans le workspace Palet Vendéen — ownership, Result/Option, itérateurs, clippy/fmt, gestion d'erreurs, async, conventions du projet.
triggers:
  - rust, ownership, borrow, emprunt, lifetime, "durée de vie", unwrap, expect, panic, Result, Option, "?"
  - clippy, fmt, "cargo check", "compile pas", "erreur de compilation", ne compile plus
  - clone, itérateur, map, filter, collect, async, tokio, spawn, idiome, "plus idiomatique", borrow checker
---

# Idiomes Rust — conventions du projet

## Ownership & types

- Préférer `&str` en paramètre de fonction, `String` seulement quand on possède la donnée.
- Accepter `impl AsRef<str>` / `impl AsRef<Path>` aux frontières pour être flexible.
- Ne pas cloner « pour être sûr » : emprunter (`&T`) d'abord, `Clone` quand c'est nécessaire.
- `Cow<'a, str>` pour éviter un clone quand la valeur est parfois empruntée, parfois owned.

## Erreurs : jamais de panic en production

- Retourner `Result<T, E>` ; propager avec `?`.
- Backend : définir les erreurs du domaine avec `thiserror` (ex. `AppError` avec
  `#[from]` pour `sqlx::Error`, `validator`, etc.) et les convertir en réponses
  HTTP (400/404/500) au niveau des handlers.
- **Jamais d'`.unwrap()` hors des tests** (`#[cfg(test)]`, exemples, doctests).
  En prod, au pire `.expect("message explicite")`. Mieux : `?` + erreur métier.
- `Option` avec combinateurs (`map`, `and_then`, `unwrap_or_else`) plutôt que `match` systématique.

## Itérateurs plutôt que boucles indexées

```rust
// Bien
let total: u32 = matchs.iter().map(|m| m.score_a).sum();
let noms: Vec<String> = joueurs.iter().filter(|j| j.actif).map(|j| j.nom.clone()).collect();
```

## Petits idiomes à utiliser partout

- `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]` sur les types de `rust-shared`.
- `Default` via `#[derive(Default)]` quand c'est évident, sinon `impl Default`.
- `From`/`TryFrom` pour les conversions (éviter les méthodes `to_*` maison).
- `#[must_use]` sur les fonctions pures dont le résultat ne doit pas être ignoré.
- Doc `///` sur toute l'API publique + exemples doctests (`cargo test --doc`) pour la logique métier.
- `matches!`, `let else` (edition 2021, Rust récent) pour les patterns.
- Formatter les erreurs avec `log`/`tracing` (`log::error!("...: {e:?}")`), pas de `println!` en prod.

## Qualité : les deux commandes à passer avant chaque commit

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings   # zéro warning toléré
cargo check -p palet-frontend --target wasm32-unknown-unknown   # le frontend compile en WASM
```

- `cargo fmt` : style uniforme, ne pas désactiver.
- Clippy : corriger les lints (`needless_clone`, `ptr_arg`, `single_match`…) plutôt
  que `#[allow]` — un `allow` doit être justifié en commentaire.

## Structure du workspace (rappel)

- Dépendances versions centralisées dans `[workspace.dependencies]` du `Cargo.toml` racine.
- `rust-shared` = types + règles métier purs (testables sans DB ni navigateur).
- `rust-backend` = Actix-web + SQLx ; logique métier dans `services/`, accès DB dans `repositories/`.
- `rust-frontend` / `rust-mobile` = Dioxus 0.7 ; logique UI testable dans `utils/` et `hooks/`.

## Async

- Backend : `tokio` via Actix ; `#[actix_web::main]` / `#[actix_web::test]` pour les tests d'API.
- Dioxus : `spawn(async move { ... })` pour les effets async, `use_resource`/`use_future`
  pour les données ; ne jamais bloquer le thread UI.

## Références

- The Rust Book : https://doc.rust-lang.org/book/
- Rust API Guidelines : https://rust-lang.github.io/api-guidelines/
- Clippy lints : https://rust-lang.github.io/rust-clippy/
