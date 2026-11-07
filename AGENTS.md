# AGENTS.md — Palet Vendéen

## Activation automatique des skills (signaux faibles)

Avant de commencer **toute tâche**, scanner le message de l'utilisateur à la
recherche des signaux faibles déclarés dans le champ `triggers:` du frontmatter
de chaque `skills/*/SKILL.md` :

1. Lire les frontmatters de tous les skills (`skills/*/SKILL.md`).
2. Pour chaque skill dont un trigger matche la demande — mot-clé, intention ou
   synonyme (ex. « corrige ce bug » → `rust-tdd`, « c'est trop le bazar dans ce
   fichier » → `kiss`, « la page déborde sur téléphone » → `responsive-mobile`) —
   lire le `SKILL.md` **complet avant d'écrire la moindre ligne de code**, puis
   appliquer ses règles pendant toute la tâche.
3. Plusieurs skills peuvent s'activer en même temps : les combiner.
   Ex. « ajoute une page de classement sur mobile, testée » active
   `dioxus-patterns` + `responsive-mobile` + `rust-tdd` (+ `kiss` et
   `rust-idioms` qui s'appliquent à toute tâche Rust).
4. Si aucun trigger ne matche, aucun skill n'est chargé.
5. Appliquer les skills en silence : ne pas annoncer le mécanisme à l'utilisateur.

## Conventions du dépôt

- Workspace Rust : `rust-backend` (Actix-web + SQLx + PostgreSQL),
  `rust-frontend` (Dioxus 0.7 → WASM web), `rust-mobile` (Dioxus 0.7 mobile,
  hors workspace), `rust-shared` (types partagés).
- Commandes de qualité (voir `README.md`) : `cargo fmt --all`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
  et `cargo check -p palet-frontend --target wasm32-unknown-unknown` pour le WASM.
- Skills disponibles : voir `skills/README.md`.
- Documentation projet : `docs/` (architecture, design system, guides responsive).
