//! Espace arbitre — liste des matchs à arbitrer (programmés, en cours, contestés)

use crate::components::common::team_name;
use crate::components::layout::Layout;
use crate::hooks::use_auth::use_auth;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use crate::utils::labels::{is_match_official, match_status_label};
use dioxus::prelude::*;
use palet_shared::Team;

#[component]
pub fn Arbitrage() -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let user = auth.read().user.clone();
    let allowed = user.as_ref().is_some_and(|u| is_match_official(&u.role));

    let scheduled = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_matches_by_status("scheduled").await }
        }
    });
    let in_progress = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_matches_by_status("in_progress").await }
        }
    });
    let disputed = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_matches_by_status("disputed").await }
        }
    });
    let teams = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_equipes(None, None).await }
        }
    });
    // Les matchs référencent l'id du club : repli via les clubs pour les noms.
    let clubs = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_clubs().await }
        }
    });
    let teams_list: Vec<Team> = match &*teams.read() {
        Some(Ok(t)) => t.clone(),
        _ => Vec::new(),
    };
    let clubs_list = match &*clubs.read() {
        Some(Ok(c)) => c.clone(),
        _ => Vec::new(),
    };

    let not_allowed = rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Arbitrage" }
                    if user.is_none() {
                        p { class: "page-sub", "Vous devez être connecté avec un compte arbitre pour accéder à cet espace." }
                        Link { to: "/connexion", class: "badge", "Se connecter" }
                    } else {
                        p { class: "page-sub", "Cet espace est réservé aux arbitres et administrateurs." }
                    }
                }
            }
        }
    };

    if !allowed {
        return not_allowed;
    }

    let render_section = |title: &str,
                          res: &Resource<Result<Vec<palet_shared::Match>, String>>|
     -> Element {
        match &*res.read() {
            Some(Ok(list)) if !list.is_empty() => rsx! {
                section {
                    h2 { class: "page-title", "{title} ({list.len()})" }
                    div { class: "page-grid",
                        for m in list {
                            Link { to: "/arbitrage/{m.id}", key: "{m.id}", class: "page-card",
                                span { class: "badge", "{match_status_label(&m.status)}" }
                                h3 { "{team_name(&teams_list, &clubs_list, &m.team1_id)} — {team_name(&teams_list, &clubs_list, &m.team2_id)}" }
                                p { class: "muted", "{format_date_fr(&m.date)}{time_suffix(&m.time)} · {m.location.clone().unwrap_or_default()}" }
                                if let Some(j) = m.journee {
                                    p { class: "muted", "Journée {j}" }
                                }
                            }
                        }
                    }
                }
            },
            Some(Ok(_)) => rsx! {},
            Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
            None => rsx! { p { class: "page-empty", "Chargement…" } },
        }
    };

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Arbitrage" }
                    p { class: "page-sub", "Matchs programmés, en cours et contestés." }
                    div { class: "page-detail",
                        {render_section("En cours", &in_progress)}
                        {render_section("Programmés", &scheduled)}
                        {render_section("Contestés", &disputed)}
                    }
                }
            }
        }
    }
}

/// Suffixe d'heure pour l'affichage (" 15h00"), chaîne vide sinon.
fn time_suffix(time: &Option<String>) -> String {
    match time {
        Some(t) if !t.is_empty() => format!(" {t}"),
        _ => String::new(),
    }
}
