//! Fiche joueur — GET /joueurs/{id} + club associé

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn PlayerDetail(id: String) -> Element {
    let api = ApiService::new();
    let player = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_joueur(u).await,
                    Err(_) => Err("Identifiant invalide".to_string()),
                }
            }
        }
    });
    let stats = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_player_stats(u).await,
                    Err(_) => Err("Identifiant invalide".to_string()),
                }
            }
        }
    });
    let club = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => match api.get_joueur(u).await {
                        Ok(p) => api.get_club(p.club_id.to_string()).await,
                        Err(e) => Err(e),
                    },
                    Err(_) => Err("Identifiant invalide".to_string()),
                }
            }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    match &*player.read() {
                        Some(Ok(p)) => {
                            let alias = p.alias.clone().unwrap_or_else(|| "—".to_string());
                            rsx! {
                            h1 { class: "page-title", "{p.full_name}" }
                            p { class: "page-sub",
                                span { class: "badge", "{p.category}" }
                                "  "
                                span { class: "badge orange", "{p.license_status:?}" }
                            }
                            div { class: "page-detail",
                                section { class: "page-card",
                                    h3 { "Profil" }
                                    dl { class: "kv",
                                        dt { "Email" } dd { "{p.email}" }
                                        dt { "Âge" } dd { "{p.age} ans (né le {format_date_fr(&p.birth_date)})" }
                                        dt { "Nationalité" } dd { "{p.nationality}" }
                                        dt { "Main dominante" } dd { "{p.dominant_hand:?}" }
                                        dt { "Surnom" } dd { "{alias}" }
                                        dt { "Licence" } dd { "{p.license_number} — émise le {format_date_fr(&p.license_date)}" }
                                        dt { "Inscrit le" } dd { "{format_date_fr(&p.registration_date)}" }
                                    }
                                }
                                section { class: "page-card",
                                    h3 { "Statistiques" }
                                    match &*stats.read() {
                                        Some(Ok(st)) => rsx! {
                                            dl { class: "kv",
                                                dt { "Matchs joués" } dd { "{st.matches_played}" }
                                                dt { "Victoires" } dd { "{st.wins}" }
                                                dt { "Défaites" } dd { "{st.losses}" }
                                                dt { "Nuls" } dd { "{st.draws}" }
                                                dt { "Points marqués" } dd { "{st.points_for}" }
                                                dt { "Points encaissés" } dd { "{st.points_against}" }
                                                dt { "Moyenne/match" } dd { "{st.avg_points_for:.1}" }
                                                dt { "Taux de victoire" } dd { "{(st.win_rate * 100.0):.0} %" }
                                                dt { "Dispo déclarées" } dd { "{st.disponibilites_declarees} ({(st.taux_disponible * 100.0):.0} % disponible)" }
                                            }
                                        },
                                        Some(Err(_)) => rsx! { p { class: "muted", "Stats indisponibles" } },
                                        None => rsx! { p { class: "muted", "Chargement…" } },
                                    }
                                }
                                section { class: "page-card",
                                    h3 { "Club" }
                                    match &*club.read() {
                                        Some(Ok(c)) => rsx! {
                                            p { "{c.name} — {c.city} ({c.league}, {c.division})" }
                                            Link { to: "/clubs/{c.id}", class: "text-brand-orange-300 underline", "Voir la fiche club" }
                                        },
                                        Some(Err(_)) => rsx! { p { class: "muted", "Club introuvable" } },
                                        None => rsx! { p { class: "muted", "Chargement…" } },
                                    }
                                }
                            }
                        }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Joueur introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
