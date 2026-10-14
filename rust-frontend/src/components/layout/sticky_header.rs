//! Sticky header component for Palet Vendéen

use super::Header;
use dioxus::prelude::*;

/// Sticky header component for desktop
#[component]
pub fn StickyHeader() -> Element {
    rsx! {
        div { class: "hidden md:block", Header { } }
    }
}
