//! Main layout component for Palet Vendéen

use super::{Footer, StickyHeader};
use crate::components::home::{SiteHeader, TopBar};
use crate::hooks::use_auth::use_auth_guard;
use dioxus::prelude::*;

/// Main layout component
///
/// Toutes les pages (hors home, qui rend son propre `TopBar`/`SiteHeader`)
/// affichent le bandeau et le header de la home page via [`TopBar`] et
/// [`SiteHeader`]. [`StickyHeader`] reste présent pour la recherche, les
/// notifications et les actions de compte.
///
/// Le garde de route masque le contenu des pages protégées et redirige vers
/// la page fédération quand le visiteur n'est pas connecté.
#[component]
pub fn Layout(children: Element) -> Element {
    let allowed = use_auth_guard();
    rsx! {
        div { class: "min-h-screen bg-brand-green-900 text-brand-green-100 flex flex-col",
            TopBar {}
            SiteHeader {}
            StickyHeader {}
            main { class: "flex-1",
                if allowed {
                    {children}
                }
            }
            Footer {}
        }
    }
}
