//! Page "Mon compte" — profil courant (GET /auth/me) + déconnexion

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::{logout, use_auth};
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;

#[component]
pub fn MonCompte() -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let navigator = use_navigator();

    let me = use_resource(move || {
        let api = api.clone();
        async move { api.me().await }
    });

    if !auth.read().is_authenticated() {
        return rsx! {
            Layout {
                main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                    div { class: "max-w-7xl mx-auto",
                        h1 { class: "page-title", "Mon compte" }
                        p { class: "page-sub", "Vous devez être connecté pour voir cette page." }
                        Link { to: "/connexion",
                            Button { variant: ButtonVariant::Primary, size: ButtonSize::M, "Se connecter" }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Mon compte" }
                    match &*me.read() {
                        Some(Ok(user)) => {
                            let license = user.license_number.clone().unwrap_or_else(|| "—".to_string());
                            let member_since = format_date_fr(&user.created_at);
                            rsx! {
                            div { class: "page-detail",
                                section { class: "page-card",
                                    h3 { "{user.first_name} {user.last_name}" }
                                    dl { class: "kv",
                                        dt { "Email" } dd { "{user.email}" }
                                        dt { "Rôle" } dd { "{user.role:?}" }
                                        dt { "Licence" } dd { "{license}" }
                                        dt { "Catégorie" } dd { "{user.category}" }
                                        dt { "Membre depuis" } dd { "{member_since}" }
                                    }
                                }
                                Button {
                                    variant: ButtonVariant::Danger,
                                    size: ButtonSize::M,
                                    onclick: move |_| {
                                        let navigator = navigator;
                                        spawn(async move {
                                            logout().await;
                                            navigator.push("/");
                                        });
                                    },
                                    "Se déconnecter"
                                }
                            }
                        }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
