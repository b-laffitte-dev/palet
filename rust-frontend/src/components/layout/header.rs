//! Header component for Palet Vendéen

use crate::components::common::{Button, ButtonSize, ButtonVariant, Logo};
use crate::hooks::use_auth::{logout, use_auth};
use crate::utils::labels::{is_club_manager, is_match_official};
use dioxus::prelude::*;

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
                            {
                                let auth = use_auth();
                                let user = auth.read().user.clone();
                                if let Some(u) = &user {
                                    if is_match_official(&u.role) {
                                        let _ = rsx! { Link { to: "/arbitrage", class: "text-brand-orange-300 hover:text-white transition-colors", "Arbitrage" } };
                                    }
                                    if is_club_manager(&u.role) && u.club_id.is_some() {
                                        let _ = rsx! { Link { to: "/mon-club", class: "text-brand-orange-300 hover:text-white transition-colors", "Mon club" } };
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex items-center space-x-4",
                        {
                            let auth = use_auth();
                            let user = auth.read().user.clone();
                            match user {
                                Some(u) => rsx! {
                                    span { class: "text-brand-green-200 text-sm hidden sm:inline",
                                        "{u.first_name} {u.last_name}"
                                    }
                                    Link { to: "/mon-compte",
                                        Button { variant: ButtonVariant::Secondary, size: ButtonSize::S, "Mon compte" }
                                    }
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::S,
                                        onclick: move |_| {
                                            spawn(async move { logout().await });
                                        },
                                        "Déconnexion"
                                    }
                                },
                                None => rsx! {
                                    Link { to: "/inscription",
                                        Button { variant: ButtonVariant::Secondary, size: ButtonSize::S, "Inscription" }
                                    }
                                    Link { to: "/connexion",
                                        Button { variant: ButtonVariant::Primary, size: ButtonSize::S, "Connexion" }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
