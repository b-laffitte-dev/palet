//! Home page for Palet Vendéen

use dioxus::prelude::*;
use crate::components::layout::Layout;
use crate::components::common::{Button, ButtonVariant, ButtonSize};

/// Home page component
#[component]
pub fn Home() -> Element {
    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    // Hero section
                    section { class: "mb-12",
                        div { class: "bg-gradient-to-r from-brand-green-800 to-brand-green-600 rounded-2xl p-8 lg:p-12 text-center",
                            h1 { class: "text-heading-xl lg:text-heading-2xl font-bold text-white uppercase tracking-wide mb-4",
                                "Bienvenue sur Palet Vendéen"
                            }
                            p { class: "text-body-l text-brand-green-100 mb-8",
                                "La plateforme de fédération des joueurs, clubs et fédérations de palet vendéen"
                            }
                            Link { to: "/clubs", Button { variant: ButtonVariant::Primary, size: ButtonSize::L } }
                        }
                    }

                    // Featured sections
                    section { class: "mb-12",
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                            // Clubs section
                            div { class: "bg-brand-green-800/30 rounded-xl p-6",
                                h2 { class: "text-heading-m font-bold text-white mb-4", "Clubs" }
                                p { class: "text-brand-green-200 mb-4", "Gérez vos clubs et joueurs" }
                                Link { to: "/clubs", class: "text-brand-green-400 hover:text-brand-green-300 text-sm", "Voir tous les clubs" }
                            }

                            // Tournaments section
                            div { class: "bg-brand-green-800/30 rounded-xl p-6",
                                h2 { class: "text-heading-m font-bold text-white mb-4", "Tournois" }
                                p { class: "text-brand-green-200 mb-4", "Organisez vos tournois" }
                                Link { to: "/tournois", class: "text-brand-green-400 hover:text-brand-green-300 text-sm", "Voir tous les tournois" }
                            }

                            // Championships section
                            div { class: "bg-brand-green-800/30 rounded-xl p-6",
                                h2 { class: "text-heading-m font-bold text-white mb-4", "Championnats" }
                                p { class: "text-brand-green-200 mb-4", "Suivez les classements" }
                                Link { to: "/championnats", class: "text-brand-green-400 hover:text-brand-green-300 text-sm", "Voir tous les championnats" }
                            }
                        }
                    }
                }
            }
        }
    }
}
