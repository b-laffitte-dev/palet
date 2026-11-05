//! "Nos espaces" quick-access band (maquette dark section)

use dioxus::prelude::*;

#[component]
pub fn Spaces() -> Element {
    let spaces = [
        (
            "Espace Fédération",
            "Licences, règles, arbitres, sélections",
            "/federation",
        ),
        (
            "Espace Club",
            "Effectifs, résultats, gestion des équipes",
            "/clubs",
        ),
        (
            "Espace Joueur",
            "Profil, stats, palmarès personnel",
            "/joueurs",
        ),
        (
            "Espace Supporter",
            "Pronostics, votes, notifications",
            "/actualites",
        ),
    ];

    rsx! {
        section { class: "pv-spaces",
            div { class: "pv-container",
                h2 { class: "pv-spaces-title", "NOS ESPACES" }
                div { class: "pv-spaces-grid",
                    for (title, desc, href) in spaces {
                        Link { to: href, class: "pv-space",
                            p { class: "pv-space-title", "{title}" }
                            p { class: "pv-space-desc", "{desc}" }
                        }
                    }
                }
            }
        }
    }
}
