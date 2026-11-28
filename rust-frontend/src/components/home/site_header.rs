//! Main header: gold palet logo, brand, navigation, "S'inscrire" CTA

use crate::hooks::use_auth::use_auth;
use crate::utils::nav::{active_section, nav_links};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteHeader() -> Element {
    // Section active déduite de la route courante (pages détail incluses).
    let section = active_section(&use_route::<Route>());
    // Un visiteur non connecté ne voit que la fédération et les actualités.
    let connected = use_auth().read().is_authenticated();
    let links = nav_links(connected);

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
                    for &(label, href) in links {
                        Link {
                            to: href,
                            class: if section == href { "pv-nav-link active" } else { "pv-nav-link" },
                            "{label}"
                        }
                    }
                }
                if !connected {
                    Link { to: "/inscription", class: "pv-btn-gold", "S'inscrire" }
                }
            }
        }
    }
}
