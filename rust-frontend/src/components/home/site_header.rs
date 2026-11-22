//! Main header: gold palet logo, brand, navigation, "S'inscrire" CTA

use crate::utils::nav::active_section;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteHeader() -> Element {
    // Section active déduite de la route courante (pages détail incluses).
    let section = active_section(&use_route::<Route>());
    let links = [
        ("Championnat", "/championnats"),
        ("Tournois", "/tournois"),
        ("Clubs", "/clubs"),
        ("Joueurs", "/joueurs"),
        ("Fédération", "/federation"),
        ("Actualités", "/actualites"),
    ];

    rsx! {
        header { class: "pv-header",
            div { class: "pv-container pv-header-inner",
                Link { to: "/", class: "pv-brand",
                    div { class: "pv-logo",
                        div { class: "pv-logo-inner",
                            div { class: "pv-logo-dot" }
                        }
                    }
                    div { class: "pv-brand-text",
                        span { class: "pv-brand-name", "PALET VENDÉEN" }
                        span { class: "pv-brand-sub", "LE SITE OFFICIEL — CHAMPIONNATS, CLUBS & TOURNOIS" }
                    }
                }
                nav { class: "pv-nav",
                    for (label, href) in links {
                        Link {
                            to: href,
                            class: if section == href { "pv-nav-link active" } else { "pv-nav-link" },
                            "{label}"
                        }
                    }
                }
                Link { to: "/inscription", class: "pv-btn-gold", "S'inscrire" }
            }
        }
    }
}
