//! Sticky header component for Palet Vendéen

use dioxus::prelude::*;
use super::Header;

/// Sticky header component for desktop
#[component]
pub fn StickyHeader() -> Element {
    rsx! {
        div { class: "hidden md:block", Header { } }
    }
}
