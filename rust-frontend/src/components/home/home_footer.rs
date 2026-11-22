//! Footer of the home page (maquette bottom strip)

use crate::utils::format::year_of;
use dioxus::prelude::*;
use time::OffsetDateTime;

#[component]
pub fn HomeFooter() -> Element {
    let year = year_of(&OffsetDateTime::now_utc());
    rsx! {
        footer { class: "pv-footer",
            div { class: "pv-container pv-footer-inner",
                div { class: "pv-footer-links",
                    span { "Fédération de Palet Vendéen" }
                    a { href: "/mentions-legales", "Mentions légales" }
                    a { href: "/rgpd", "RGPD" }
                    a { href: "/contact", "Contact" }
                }
                span { class: "pv-footer-note", "© {year} Fédération de Palet Vendéen" }
            }
        }
    }
}
