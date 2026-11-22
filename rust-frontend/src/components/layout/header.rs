//! Header component for Palet Vendéen

use crate::components::common::{Button, ButtonSize, ButtonVariant, Logo};
use crate::hooks::use_auth::{logout, use_auth};
use crate::services::api::{url_encode, ApiService};
use crate::utils::labels::{is_club_manager, is_match_official};
use crate::utils::nav::active_section;
use crate::Route;
use dioxus::prelude::*;

/// Header component
#[component]
pub fn Header() -> Element {
    // Section active déduite de la route courante (pages détail incluses).
    let section = active_section(&use_route::<Route>());
    let link_class = |href: &str| {
        if section == href {
            "text-white font-semibold underline"
        } else {
            "text-brand-green-200 hover:text-white transition-colors"
        }
    };
    rsx! {
        header { class: "bg-brand-green-900/80 backdrop-blur-sm border-b border-brand-green-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex items-center justify-between h-16",
                    div { class: "flex items-center space-x-8",
                        Link { to: "/", Logo { } }
                        nav { class: "hidden md:flex space-x-6",
                            Link { to: "/clubs", class: link_class("/clubs"), "Clubs" }
                            Link { to: "/joueurs", class: link_class("/joueurs"), "Joueurs" }
                            Link { to: "/tournois", class: link_class("/tournois"), "Tournois" }
                            Link { to: "/championnats", class: link_class("/championnats"), "Championnats" }
                            Link { to: "/actualites", class: link_class("/actualites"), "Actualités" }
                            Link { to: "/federation", class: link_class("/federation"), "Fédération" }
                            {
                                let auth = use_auth();
                                let user = auth.read().user.clone();
                                let arbitre = user.as_ref().is_some_and(|u| is_match_official(&u.role));
                                let club_mgr = user
                                    .as_ref()
                                    .is_some_and(|u| is_club_manager(&u.role) && u.club_id.is_some());
                                rsx! {
                                    if arbitre {
                                        Link { to: "/arbitrage", class: "text-brand-orange-300 hover:text-white transition-colors", "Arbitrage" }
                                    }
                                    if club_mgr {
                                        Link { to: "/mon-club", class: "text-brand-orange-300 hover:text-white transition-colors", "Mon club" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex items-center space-x-4",
                        {
                            let mut q = use_signal(String::new);
                            let navigator = use_navigator();
                            rsx! {
                                form {
                                    class: "hidden md:flex items-center",
                                    onsubmit: move |evt: FormEvent| {
                                        evt.prevent_default();
                                        let v = q.read().trim().to_string();
                                        if v.chars().count() >= 2 {
                                            navigator.push(format!("/recherche/{}", url_encode(&v)));
                                        }
                                    },
                                    input {
                                        class: "page-input",
                                        r#type: "search",
                                        placeholder: "Rechercher…",
                                        value: "{q}",
                                        oninput: move |evt: FormEvent| q.set(evt.value()),
                                    }
                                }
                            }
                        }
                        {
                            let unread = use_resource(|| {
                                let api = ApiService::new();
                                async move {
                                    if crate::services::api::stored_access_token().is_some() {
                                        api.get_unread_count().await.ok()
                                    } else {
                                        None
                                    }
                                }
                            });
                            let connected = use_auth().read().user.is_some();
                            let count = if connected {
                                unread.read().clone().flatten().map(|u| u.unread).unwrap_or(0)
                            } else {
                                0
                            };
                            rsx! {
                                if connected {
                                    Link { to: "/notifications", class: "relative text-brand-green-200 hover:text-white transition-colors",
                                        "🔔"
                                        if count > 0 {
                                            span { class: "badge orange", style: "margin-left: .25rem;", "{count}" }
                                        }
                                    }
                                }
                            }
                        }
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
