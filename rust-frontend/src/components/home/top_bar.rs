//! Top bar: federation name, live indicator, account links (maquette top strip)

use crate::hooks::use_auth::use_auth;
use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    let connected = use_auth().read().is_authenticated();
    rsx! {
        div { class: "pv-topbar",
            div { class: "pv-container pv-topbar-inner",
                span { class: "pv-topbar-left", "CDSP 79" }
                div { class: "pv-topbar-right",
                    span { class: "pv-live-label",
                        span { class: "pv-live-dot" }
                        "EN DIRECT"
                    }
                    if connected {
                        Link { to: "/mon-compte", class: "pv-topbar-link", "Mon espace licencié" }
                    } else {
                        Link { to: "/connexion", class: "pv-topbar-link", "Connexion" }
                        span { class: "pv-sep", "|" }
                        Link { to: "/connexion", class: "pv-topbar-link", "Mon espace licencié" }
                    }
                }
            }
        }
    }
}
