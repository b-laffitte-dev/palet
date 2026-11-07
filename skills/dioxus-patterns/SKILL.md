---
name: dioxus-patterns
description: Patterns et bonnes pratiques Dioxus 0.7 (version du projet : 0.7.10) pour rust-frontend (web WASM) et rust-mobile — signals, props, hooks, rsx, routing, styling CSS, tests de composants.
triggers:
  - dioxus, composant, rsx, signal, use_signal, use_memo, props, hook, use_resource, use_future, spawn
  - route, router, navigation, page, UI, interface, frontend, affichage, render, "re-rend", "re-render"
  - "dx serve", wasm, bouton, formulaire, liste, composant, écran, "ajoute une page", "modifie l'affichage"
---

# Patterns Dioxus 0.7 — Palet Vendéen

Le projet utilise **Dioxus 0.7.10** (`rust-frontend` web + `rust-mobile` mobile).

## Composants

```rust
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarteClubProps {
    pub nom: String,
    #[props(default = false)]
    pub vedette: bool,
    pub children: Element,          // contenu passé entre les balises
}

#[component]
pub fn CarteClub(props: CarteClubProps) -> Element {
    rsx! {
        article { class: "carte-club",
            h3 { {props.nom} }
            {props.children}
        }
    }
}
```

- Toujours `#[derive(Props, Clone, PartialEq)]` — `PartialEq` évite les re-rendus inutiles.
- Props petites et sérialisables quand elles traversent des composants.
- Un composant = un fichier dans `src/components/`, une page = un fichier dans `src/pages/`.

## State : les signals (cœur de Dioxus 0.7)

```rust
let mut compteur = use_signal(|| 0);        // state local
let total = use_memo(|| matchs().iter().map(|m| m.points).sum::<u32>()); // dérivé, recalculé quand les signaux lus changent
use_effect(|| log::info!("compteur = {compteur}")); // effet de bord

// state global/partagé
use_context_provider(|| use_signal(|| Utilisateur::default()));
let user: Signal<Utilisateur> = use_context();
```

- Lire un signal dans le `rsx!` (`{compteur}`) abonne le composant : il re-rend quand il change.
- Lire finement : préférer `compteur.read()` dans une closure plutôt que de tout embarquer.
- Ne pas mettre de `String`/`Vec` énorme dans un signal sans raison : préférer `use_memo` dérivé.

## Async & data fetching

```rust
let clubs = use_resource(|| async move { api::get_clubs().await });
rsx! {
    match &*clubs.read() {
        Some(Ok(list)) => rsx! { for c in list { CarteClub { nom: c.nom.clone() } } },
        Some(Err(e)) => rsx! { p { "Erreur: {e}" } },
        None => rsx! { p { "Chargement…" } },
    }
}
// ou spawn pour une action :
onclick: move |_| spawn(async move { api::creer_match(...).await; clubs.restart(); })
```

- `use_resource` pour les lectures (re-fetch avec `.restart()`), `spawn` pour les actions.

## rsx : les idiomes utiles

```rust
rsx! {
    button {
        class: "btn",                       // classe de base
        class: if actif { "btn-actif" },    // classe conditionnelle (cumulable)
        disabled: !formulaire_valide,
        onclick: move |evt| envoyer(evt),
        "Envoyer ({compteur})"              // texte dynamique
    }
    for (i, item) in items.iter().enumerate() {
        li { key: "{item.id}", "{i}. {item.nom}" }   // key obligatoire dans les listes
    }
}
```

- `key:` sur chaque élément d'une liste (stabilité du DOM).
- `class: if cond { "..." }` pour les classes conditionnelles (0.7).
- Événements : `move |evt: Event<MouseData>| ...`.

## Routing (dioxus-router 0.7)

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")] Home,
    #[route("/clubs/:id")] ClubDetail { id: String },
    #[route("/404")] NotFound,
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
// navigation : use_navigator().push(Route::ClubDetail { id })
// lien : Link { to: Route::Clubs, "Voir les clubs" }
```

## Styling (web)

Dioxus 0.7 utilise le **CSS standard** — pas de système maison :

- Feuilles de style servies depuis `public/` (déjà en place : `public/styles/design-system.css`,
  `responsive.css`, …) et liées dans `index.html`, ou via `document::Stylesheet { href: asset!("/assets/main.css") }`.
- Classes utilitaires du design system (`docs/design-system/`) dans le `class:` des composants.
- Style inline dynamique : `style: "color: {couleur}"` ou attributs `background_color: "..."`.
- Voir le skill **responsive-mobile** pour le mobile-first.

## Organisation (déjà en place, à respecter)

```
rust-frontend/src/
├── main.rs / lib.rs
├── pages/        # une page par route
├── components/   # composants réutilisables (common/, layout/, home/)
├── hooks/        # hooks custom (use_xxx)
├── services/     # appels API
└── utils/        # fonctions pures testables
```

- La logique métier/testable va dans `utils/` et `rust-shared`, pas dans les composants.

## Tests des composants (voir aussi le skill rust-tdd)

- Rendu : `dioxus_ssr::render_element(rsx!{...})` + `pretty_assertions::assert_str_eq!`.
- Hooks/state : piloter un `VirtualDom` manuellement (pattern `test_hook`, doc officielle 0.7).
- Navigateur/WASM : `wasm-bindgen-test` (déjà en dev-dep) — `wasm-pack test --headless --chrome`.
- E2E : Playwright + `dx serve`.

## Références

- Doc 0.7 — testing : https://dioxuslabs.com/learn/0.7/guides/testing/web/
- Doc 0.7 — styling : https://dioxuslabs.com/learn/0.7/essentials/ui/styling/
- Migration 0.7 : https://dioxuslabs.com/learn/0.7/migration/to_07/
- Release 0.7 : https://dioxuslabs.com/blog/release-070/
