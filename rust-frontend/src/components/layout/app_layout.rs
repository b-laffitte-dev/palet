//! Main layout component for Palet Vendéen

use super::{Footer, StickyHeader};
use crate::components::home::{SiteHeader, TopBar};
use dioxus::prelude::*;

/// Main layout component
///
/// Toutes les pages (hors home, qui rend son propre `TopBar`/`SiteHeader`)
/// affichent le bandeau et le header de la home page via [`TopBar`] et
/// [`SiteHeader`]. [`StickyHeader`] reste présent pour la recherche, les
/// notifications et les actions de compte.
#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div { class: "min-h-screen bg-brand-green-900 text-brand-green-100 flex flex-col",
            TopBar {}
            SiteHeader {}
            StickyHeader {}
            main { class: "flex-1", {children} }
            Footer {}
        }
    }
}
