//! Footer of the home page (maquette bottom strip)

use dioxus::prelude::*;

#[component]
pub fn HomeFooter() -> Element {
    rsx! {
        footer { class: "pv-footer",
            div { class: "pv-container pv-footer-inner",
                div { class: "pv-footer-links",
                    span { "Fédération de Palet Vendéen" }
                    a { href: "/mentions-legales", "Mentions légales" }
                    a { href: "/rgpd", "RGPD" }
                    a { href: "/contact", "Contact" }
                }
                span { class: "pv-footer-note", "© 2025 Fédération de Palet Vendéen" }
            }
        }
    }
}
