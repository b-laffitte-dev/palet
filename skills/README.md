# Skills du projet Palet Vendéen

Compétences réutilisables pour les agents de code (et les humains) qui travaillent
sur ce dépôt. Chaque skill vit dans son propre dossier avec un fichier `SKILL.md`
au format standard (frontmatter `name` + `description`, puis les instructions).

| Skill | Quand l'utiliser |
|---|---|
| [rust-tdd](rust-tdd/SKILL.md) | Avant d'écrire une fonctionnalité : cycle TDD, organisation et outils de test Rust/Dioxus |
| [kiss](kiss/SKILL.md) | À chaque écriture/revue de code : garder le code simple, éviter la sur-ingénierie |
| [rust-idioms](rust-idioms/SKILL.md) | En écrivant du Rust : ownership, gestion d'erreurs, clippy, idiomes du langage |
| [dioxus-patterns](dioxus-patterns/SKILL.md) | En écrivant des composants Dioxus 0.7 : signals, props, hooks, rsx, routing, tests |
| [responsive-mobile](responsive-mobile/SKILL.md) | En stylisant une page : mobile-first, touch, safe areas, breakpoints, perfs mobile |

## Activation automatique (signaux faibles)

Chaque `SKILL.md` déclare un champ `triggers:` dans son frontmatter : la liste
des signaux faibles (mots-clés et intentions, en français) qui activent le
skill. Le `AGENTS.md` à la racine du dépôt ordonne à l'agent de scanner chaque
demande de l'utilisateur contre ces triggers et de charger automatiquement le
`SKILL.md` correspondant **avant** de commencer la tâche. Plusieurs skills
peuvent s'activer simultanément.

| Skill | S'active quand la demande évoque (exemples de signaux faibles) |
|---|---|
| [rust-tdd](rust-tdd/SKILL.md) | test, bug, régression, couverture, « ça ne marche pas », « corrige ce bug », cargo test, recette, QA |
| [kiss](kiss/SKILL.md) | simple, simplifier, « trop complexe », refactor, dette technique, KISS, « trop le bazar » |
| [rust-idioms](rust-idioms/SKILL.md) | rust, ownership, unwrap, panic, Result, clippy, « compile pas », itérateur, borrow checker |
| [dioxus-patterns](dioxus-patterns/SKILL.md) | composant, rsx, signal, props, hook, route, page, UI, frontend, bouton, formulaire, « ajoute une page » |
| [responsive-mobile](responsive-mobile/SKILL.md) | mobile, téléphone, css, style, responsive, breakpoint, tactile, encoche, iPhone, Android, « ça déborde », « trop petit » |

Conventions : un dossier par skill, un seul `SKILL.md` par dossier, frontmatter
`name` + `description` + `triggers`, le contenu est en français, les extraits
de code sont en Rust/CSS réels du projet.
