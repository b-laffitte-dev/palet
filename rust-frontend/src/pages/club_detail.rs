//! Fiche club — GET /clubs/{id} + équipes (GET /equipes?club_id=) + joueurs (GET /joueurs?club_id=)

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn ClubDetail(id: String) -> Element {
    let api = ApiService::new();
    let club_id = Uuid::parse_str(&id).ok();
    let club = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_club(u.to_string()).await,
                    Err(_) => Err("Identifiant de club invalide".to_string()),
                }
            }
        }
    });
    let teams = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match club_id {
                    Some(c) => api.get_equipes(Some(c), None).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });
    let players = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match club_id {
                    Some(c) => api.get_joueurs(None, Some(c), None).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    match &*club.read() {
                        Some(Ok(c)) => {
                            let address = c.address.clone().unwrap_or_else(|| "—".to_string());
                            let email = c.contact_email.clone().unwrap_or_else(|| "—".to_string());
                            let phone = c.contact_phone.clone().unwrap_or_else(|| "—".to_string());
                            let website = c.website.clone().unwrap_or_else(|| "—".to_string());
                            rsx! {
                            h1 { class: "page-title", "{c.name}" }
                            p { class: "page-sub", "{c.code} — {c.city} {c.postal_code.clone().unwrap_or_default()} · {c.league} · {c.division}" }
                            div { class: "page-detail",
                                section { class: "page-card",
                                    h3 { "Informations" }
                                    dl { class: "kv",
                                        dt { "Adresse" } dd { "{address}" }
                                        dt { "Contact" } dd { "{email}" }
                                        dt { "Téléphone" } dd { "{phone}" }
                                        dt { "Site web" } dd { "{website}" }
                                        dt { "Statut" } dd { "{c.status:?}" }
                                    }
                                }
                                section {
                                    h2 { class: "page-title", "Équipes ({c.teams_count})" }
                                    match &*teams.read() {
                                        Some(Ok(list)) if !list.is_empty() => rsx! {
                                            div { class: "page-grid",
                                                for t in list {
                                                    div { key: "{t.id}", class: "page-card",
                                                        h3 { "{t.name}" }
                                                        p { class: "muted", "{t.status:?} — {t.stats.wins}V / {t.stats.losses}D, {t.stats.points} pts" }
                                                    }
                                                }
                                            }
                                        },
                                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucune équipe." } },
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur équipes : {e}" } },
                                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                                    }
                                }
                                section {
                                    h2 { class: "page-title", "Joueurs ({c.players_count})" }
                                    match &*players.read() {
                                        Some(Ok(list)) if !list.is_empty() => rsx! {
                                            div { class: "page-grid",
                                                for p in list {
                                                    Link { to: "/joueurs/{p.id}", key: "{p.id}", class: "page-card",
                                                        h3 { "{p.full_name}" }
                                                        p { class: "muted", "{p.category} · licence {p.license_number} ({p.license_status:?})" }
                                                        p { class: "muted", "inscrit le {format_date_fr(&p.registration_date)}" }
                                                    }
                                                }
                                            }
                                        },
                                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucun joueur." } },
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur joueurs : {e}" } },
                                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                                    }
                                }
                            }
                        }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Club introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
