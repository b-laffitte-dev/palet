//! Page détail d'un championnat — GET /championnats/{id} + /classification + matchs

use crate::components::common::{team_name, MatchTable};
use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::labels::{
    classification_status_label, competition_status_label, match_result_letter,
};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn ChampionnatDetail(id: String) -> Element {
    let api = ApiService::new();
    let champ_id = Uuid::parse_str(&id).ok();

    let championship = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_championship(u).await,
                    Err(_) => Err("Identifiant de championnat invalide".to_string()),
                }
            }
        }
    });
    let classification = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match champ_id {
                    Some(u) => api.get_championship_classification(u).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });
    let matches = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match champ_id {
                    Some(u) => api.get_matches_by_competition(u).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });
    let teams = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_equipes(None, None).await }
        }
    });
    // Les matchs/classements référencent l'id du club : les clubs servent
    // de repli pour résoudre les noms d'équipes.
    let clubs = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_clubs().await }
        }
    });

    let teams_list = match &*teams.read() {
        Some(Ok(t)) => t.clone(),
        _ => Vec::new(),
    };
    let clubs_list = match &*clubs.read() {
        Some(Ok(c)) => c.clone(),
        _ => Vec::new(),
    };

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    match &*championship.read() {
                        Some(Ok(c)) => rsx! {
                            h1 { class: "page-title", "{c.name}" }
                            p { class: "page-sub",
                                span { class: "badge", "{competition_status_label(&c.status)}" }
                                span { "  {c.league} · saison {c.season}" }
                            }
                            if let Some(desc) = &c.description {
                                p { class: "page-sub", "{desc}" }
                            }
                            div { class: "page-detail",
                                section {
                                    h2 { class: "page-title", "Classement" }
                                    match &*classification.read() {
                                        Some(Ok(list)) if !list.is_empty() => rsx! {
                                            for div in list {
                                                h3 { class: "page-sub", "Division {div.division} — journée {div.journee}" }
                                                div { class: "table-wrap page-card",
                                                    table { class: "table",
                                                        thead {
                                                            tr {
                                                                th { "#" } th { "Équipe" } th { "MJ" } th { "V" } th { "N" } th { "D" }
                                                                th { "Diff" } th { "Pts" } th { "Forme" } th { "Statut" }
                                                            }
                                                        }
                                                        tbody {
                                                            for t in &div.teams {
                                                                tr { key: "{t.team_id}",
                                                                    td { "{t.position}" }
                                                                    td { "{team_name(&teams_list, &clubs_list, &t.team_id)}" }
                                                                    td { "{t.stats.matches}" }
                                                                    td { "{t.stats.wins}" }
                                                                    td { "{t.stats.draws}" }
                                                                    td { "{t.stats.losses}" }
                                                                    td { "{t.stats.diff}" }
                                                                    td { "{t.stats.points}" }
                                                                    td { "{t.form.iter().map(match_result_letter).collect::<String>()}" }
                                                                    td { span { class: "badge", "{classification_status_label(&t.status)}" } }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Classement pas encore disponible." } },
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur classement : {e}" } },
                                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                                    }
                                }
                                section {
                                    h2 { class: "page-title", "Matchs" }
                                    match &*matches.read() {
                                        Some(Ok(list)) => rsx! { MatchTable { matches: list.clone(), teams: teams_list.clone(), clubs: clubs_list.clone() } },
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur matchs : {e}" } },
                                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Championnat introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
