//! Page notifications — GET /notifications + marquer lu / tout lu

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::use_auth;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Notifications() -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let mut version = use_signal(|| 0u32);
    let notifs = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let _v = version();
            async move { api.get_notifications(false).await }
        }
    });

    if !auth.read().is_authenticated() {
        return rsx! {
            Layout {
                main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                    div { class: "max-w-7xl mx-auto",
                        h1 { class: "page-title", "Notifications" }
                        p { class: "page-sub", "Connectez-vous pour voir vos notifications." }
                        Link { to: "/connexion", class: "badge", "Se connecter" }
                    }
                }
            }
        };
    }

    let api_all = api.clone();
    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-3xl mx-auto",
                    h1 { class: "page-title", "Notifications" }
                    div { class: "page-filters",
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::S,
                            onclick: move |_| {
                                let api = api_all.clone();
                                spawn(async move {
                                    let _ = api.mark_all_notifications_read().await;
                                    version.set(version() + 1);
                                });
                            },
                            "Tout marquer comme lu"
                        }
                    }
                    match &*notifs.read() {
                        Some(Ok(list)) if !list.is_empty() => rsx! {
                            div { class: "page-detail",
                                for n in list {
                                    {
                                        let nid = n.id;
                                        let link = n.link.clone();
                                        let api_item = api.clone();
                                        rsx! {
                                            section { key: "{nid}", class: "page-card",
                                                if !n.read {
                                                    span { class: "badge orange", "Non lu" }
                                                } else {
                                                    span { class: "badge", "Lu" }
                                                }
                                                h3 { "{n.title}" }
                                                p { class: "muted", "{n.message}" }
                                                p { class: "muted", "{format_date_fr(&n.created_at)}" }
                                                div { class: "page-filters",
                                                    if let Some(l) = &link {
                                                        Link { to: "{l}", class: "badge", "Ouvrir" }
                                                    }
                                                    if !n.read {
                                                        Button {
                                                            variant: ButtonVariant::Ghost,
                                                            size: ButtonSize::S,
                                                            onclick: move |_| {
                                                                let api = api_item.clone();
                                                                spawn(async move {
                                                                    let _ = api.mark_notification_read(nid).await;
                                                                    version.set(version() + 1);
                                                                });
                                                            },
                                                            "Marquer lu"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucune notification." } },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}

// utilisé pour forcer le typage du key dans la boucle
#[allow(dead_code)]
fn _uuid_marker(_: Uuid) {}
