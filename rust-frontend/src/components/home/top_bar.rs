//! Top bar: federation name, live indicator, account links (maquette top strip)

use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    rsx! {
        div { class: "pv-topbar",
            div { class: "pv-container pv-topbar-inner",
                span { class: "pv-topbar-left", "Fédération de Palet Vendéen" }
                div { class: "pv-topbar-right",
                    span { class: "pv-live-label",
                        span { class: "pv-live-dot" }
                        "EN DIRECT"
                    }
                    Link { to: "/connexion", class: "pv-topbar-link", "Connexion" }
                    span { class: "pv-sep", "|" }
                    Link { to: "/connexion", class: "pv-topbar-link", "Mon espace licencié" }
                }
            }
        }
    }
}
