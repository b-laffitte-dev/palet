//! Header component for Palet Vendéen

use dioxus::prelude::*;
use crate::components::common::{Button, ButtonVariant, ButtonSize, Logo};

/// Header component
#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "bg-brand-green-900/80 backdrop-blur-sm border-b border-brand-green-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex items-center justify-between h-16",
                    div { class: "flex items-center space-x-8",
                        Link { to: "/", Logo { } }
                        nav { class: "hidden md:flex space-x-6",
                            Link { to: "/clubs", class: "text-brand-green-200 hover:text-white transition-colors", "Clubs" }
                            Link { to: "/joueurs", class: "text-brand-green-200 hover:text-white transition-colors", "Joueurs" }
                            Link { to: "/tournois", class: "text-brand-green-200 hover:text-white transition-colors", "Tournois" }
                            Link { to: "/championnats", class: "text-brand-green-200 hover:text-white transition-colors", "Championnats" }
                            Link { to: "/actualites", class: "text-brand-green-200 hover:text-white transition-colors", "Actualités" }
                            Link { to: "/federation", class: "text-brand-green-200 hover:text-white transition-colors", "Fédération" }
                        }
                    }
                    div { class: "flex items-center space-x-4",
                        Link { to: "/connexion", Button { variant: ButtonVariant::Ghost, size: ButtonSize::S } }
                    }
                }
            }
        }
    }
}
