//! Page 404

use crate::components::layout::Layout;
use dioxus::prelude::*;

/// Page component
#[component]
pub fn NotFound() -> Element {
    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "404" }
                    p { class: "page-sub", "Cette page n'existe pas." }
                    Link { to: "/", class: "badge", "Retour à l'accueil" }
                }
            }
        }
    }
}
