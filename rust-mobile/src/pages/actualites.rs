//! Page for Palet Vendéen

use dioxus::prelude::*;
use crate::components::layout::Layout;

/// Page component
#[component]
pub fn Actualites() -> Element {
    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "text-heading-xl font-bold text-white uppercase tracking-wide mb-8", "Actualites" }
                    p { class: "text-brand-green-200", "Page en construction..." }
                }
            }
        }
    }
}
