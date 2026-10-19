//! Logo component for Palet Vendéen

use dioxus::prelude::*;

/// Logo component
#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex items-center space-x-2",
            div { class: "w-8 h-8 bg-brand-green-500 rounded-lg flex items-center justify-center",
                span { class: "text-white font-bold text-lg", "P" }
            }
            span { class: "text-xl font-bold text-white", "Palet Vendéen" }
        }
    }
}
