//! Main layout component for Palet Vendéen

use dioxus::prelude::*;
use super::{Footer, StickyHeader};

/// Main layout component
#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div { class: "min-h-screen bg-brand-green-900 text-brand-green-100 flex flex-col",
            StickyHeader {}
            main { class: "flex-1", {children} }
            Footer {}
        }
    }
}
