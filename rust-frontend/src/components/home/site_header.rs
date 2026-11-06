//! Main header: gold palet logo, brand, navigation, "S'inscrire" CTA

use dioxus::prelude::*;

#[component]
pub fn SiteHeader() -> Element {
    let links = [
        ("Championnat", "/championnats", true),
        ("Tournois", "/tournois", false),
        ("Clubs", "/clubs", false),
        ("Joueurs", "/joueurs", false),
        ("Fédération", "/federation", false),
        ("Actualités", "/actualites", false),
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
                    for (label, href, active) in links {
                        Link {
                            to: href,
                            class: if active { "pv-nav-link active" } else { "pv-nav-link" },
                            "{label}"
                        }
                    }
                }
                Link { to: "/inscription", class: "pv-btn-gold", "S'inscrire" }
            }
        }
    }
}
