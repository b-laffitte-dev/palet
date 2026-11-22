//! Espace arbitre — détail d'un match : démarrer, saisir les manches,
//! terminer (vainqueur) et vérifier le résultat.

use crate::components::common::{team_name, Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::use_auth;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use crate::utils::labels::{is_match_official, match_status_label};
use dioxus::prelude::*;
use palet_shared::{MatchStatus, Team};
use uuid::Uuid;

#[component]
pub fn ArbitrageMatch(id: String) -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let user = auth.read().user.clone();
    let allowed = user.as_ref().is_some_and(|u| is_match_official(&u.role));
    let match_id = Uuid::parse_str(&id).ok();

    // bump -> recharge le match après chaque action
    let mut version = use_signal(|| 0u32);
    let match_res = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            let _v = version();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_match(u).await,
                    Err(_) => Err("Identifiant de match invalide".to_string()),
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
    // disponibilités du match (rechargées avec le match via `version`)
    let dispos = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let _v = version();
            async move {
                match match_id {
                    Some(u) => api.get_dispos(u).await,
                    None => Ok(Default::default()),
                }
            }
        }
    });
    // joueurs pour résoudre les noms des dispo
    let all_players = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_joueurs(None, None, None).await }
        }
    });
    // joueurs du club de l'utilisateur connecté (pour déclarer une dispo)
    let user_club_id = user.as_ref().and_then(|u| u.club_id);
    let club_players = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match user_club_id {
                    Some(c) => api.get_joueurs(None, Some(c), None).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });
    let players_list: Vec<palet_shared::Player> = match &*all_players.read() {
        Some(Ok(p)) => p.clone(),
        _ => Vec::new(),
    };
    let club_players_list: Vec<palet_shared::Player> = match &*club_players.read() {
        Some(Ok(p)) => p.clone(),
        _ => Vec::new(),
    };

    // formulaire manche + fin de match
    let mut s1 = use_signal(String::new);
    let mut s2 = use_signal(String::new);
    let mut winner = use_signal(|| "team1".to_string());
    let mut busy = use_signal(|| false);
    let mut feedback = use_signal(|| Option::<(bool, String)>::None);
    let mut dispo_player = use_signal(|| Option::<Uuid>::None);
    let mut dispo_status = use_signal(|| "disponible".to_string());
    let mut dispo_comment = use_signal(String::new);

    let api_dispo = api.clone();
    let run_action = move |action: MatchAction| {
        let api = api.clone();
        busy.set(true);
        feedback.set(None);
        spawn(async move {
            let result: Result<String, String> = match action {
                MatchAction::Start(u) => api
                    .start_match(u)
                    .await
                    .map(|_| "Match démarré".to_string()),
                MatchAction::Manche(u, t1, t2, cur) => api
                    .add_manche(u, t1, t2, cur)
                    .await
                    .map(|_| format!("Manche enregistrée : {t1} - {t2}")),
                MatchAction::Complete(u, w) => api
                    .complete_match(u, &w)
                    .await
                    .map(|_| "Match terminé".to_string()),
                MatchAction::Verify(u) => api
                    .verify_match(u)
                    .await
                    .map(|_| "Résultat vérifié".to_string()),
                MatchAction::Unverify(u) => api
                    .unverify_match(u)
                    .await
                    .map(|_| "Vérification annulée".to_string()),
            };
            match result {
                Ok(msg) => feedback.set(Some((true, msg))),
                Err(e) => feedback.set(Some((false, e))),
            }
            version.set(version() + 1);
            busy.set(false);
        });
    };

    let mut ra_start = run_action.clone();
    let mut ra_manche = run_action.clone();
    let mut ra_complete = run_action.clone();
    let mut ra_verify = run_action.clone();
    let mut ra_unverify = run_action.clone();

    if !allowed {
        return rsx! {
            Layout {
                main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                    div { class: "max-w-7xl mx-auto",
                        h1 { class: "page-title", "Arbitrage" }
                        p { class: "page-sub", "Cet espace est réservé aux arbitres et administrateurs." }
                        Link { to: "/arbitrage", class: "badge", "Retour à la liste" }
                    }
                }
            }
        };
    }

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-3xl mx-auto",
                    Link { to: "/arbitrage", class: "page-sub", "← Tous les matchs" }
                    match &*match_res.read() {
                        Some(Ok(m)) => {
                            let name1 = team_name(&teams_list, &clubs_list, &m.team1_id);
                            let name2 = team_name(&teams_list, &clubs_list, &m.team2_id);
                            let manches = m.manches.clone().unwrap_or_default();
                            let current_manche = manches.len() as i32;
                            let score_line = match (&m.team1_score, &m.team2_score) {
                                (Some(a), Some(b)) => format!("{a} - {b}"),
                                _ => "—".to_string(),
                            };
                            rsx! {
                                h1 { class: "page-title", "{name1} — {name2}" }
                                p { class: "page-sub",
                                    span { class: "badge", "{match_status_label(&m.status)}" }
                                    if m.verified {
                                        span { class: "badge orange", "Vérifié" }
                                    }
                                    span { "  {format_date_fr(&m.date)}" }
                                    if let Some(t) = &m.time {
                                        span { " {t}" }
                                    }
                                    if let Some(j) = m.journee {
                                        span { " · Journée {j}" }
                                    }
                                }
                                if let Some(loc) = &m.location {
                                    p { class: "page-sub", "Lieu : {loc}" }
                                }
                                section { class: "page-card",
                                    h3 { "Score" }
                                    p { "{score_line}" }
                                    if !manches.is_empty() {
                                        div { class: "table-wrap",
                                            table { class: "table",
                                                thead { tr { th { "Manche" } th { "{name1}" } th { "{name2}" } } }
                                                tbody {
                                                    for mc in &manches {
                                                        tr { key: "{mc.number}",
                                                            td { "{mc.number}" }
                                                            td { "{mc.team1}" }
                                                            td { "{mc.team2}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                section { class: "page-card",
                                    h3 { "Disponibilités" }
                                    match &*dispos.read() {
                                        Some(Ok(d)) => {
                                            let pname = |pid: &Uuid| -> String {
                                                players_list.iter().find(|p| &p.id == pid).map(|p| p.full_name.clone()).unwrap_or_else(|| pid.to_string().chars().take(8).collect())
                                            };
                                            rsx! {
                                                p { class: "page-sub",
                                                    span { class: "badge", "✅ {d.disponibles}" }
                                                    span { class: "badge orange", "❌ {d.indisponibles}" }
                                                    span { class: "badge", "❓ {d.incertains}" }
                                                    span { "  ({d.total} déclaration(s))" }
                                                }
                                                if !d.disponibilites.is_empty() {
                                                    div { class: "table-wrap",
                                                        table { class: "table",
                                                            thead { tr { th { "Joueur" } th { "Statut" } th { "Commentaire" } } }
                                                            tbody {
                                                                for dd in &d.disponibilites {
                                                                    {
                                                                        let pid = dd.player_id;
                                                                        rsx! {
                                                                            tr { key: "{pid}",
                                                                                td { "{pname(&pid)}" }
                                                                                td { span { class: "badge", "{dd.status:?}" } }
                                                                                td { "{dd.commentaire.clone().unwrap_or_default()}" }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur dispo : {e}" } },
                                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                                    }
                                    if user.is_some() && !club_players_list.is_empty() {
                                        div { class: "page-form",
                                            label {
                                                "Ma disponibilité"
                                                select {
                                                    class: "page-input",
                                                    onchange: move |evt: FormEvent| {
                                                        let v = evt.value();
                                                        dispo_player.set(if v.is_empty() { None } else { Uuid::parse_str(&v).ok() });
                                                    },
                                                    option { value: "", "Choisir un joueur…" }
                                                    for p in &club_players_list {
                                                        option { value: "{p.id}", "{p.full_name}" }
                                                    }
                                                }
                                            }
                                            label {
                                                "Statut"
                                                select {
                                                    class: "page-input",
                                                    onchange: move |evt: FormEvent| dispo_status.set(evt.value()),
                                                    option { value: "disponible", "✅ Disponible" }
                                                    option { value: "incertain", "❓ Incertain" }
                                                    option { value: "indisponible", "❌ Indisponible" }
                                                }
                                            }
                                            label {
                                                "Commentaire"
                                                input {
                                                    class: "page-input",
                                                    r#type: "text",
                                                    placeholder: "Motif éventuel…",
                                                    value: "{dispo_comment}",
                                                    oninput: move |evt: FormEvent| dispo_comment.set(evt.value()),
                                                }
                                            }
                                            Button {
                                                variant: ButtonVariant::Secondary,
                                                size: ButtonSize::M,
                                                disabled: busy() || dispo_player.read().is_none(),
                                                onclick: move |_| {
                                                    let (Some(u), Some(pid)) = (match_id, *dispo_player.read()) else { return };
                                                    let api = api_dispo.clone();
                                                    let status = dispo_status.read().clone();
                                                    let comment = dispo_comment.read().clone();
                                                    busy.set(true);
                                                    spawn(async move {
                                                        let result = api.set_dispo(u, pid, &status, if comment.is_empty() { None } else { Some(comment) }).await;
                                                        match result {
                                                            Ok(_) => feedback.set(Some((true, "Disponibilité enregistrée".to_string()))),
                                                            Err(e) => feedback.set(Some((false, e))),
                                                        }
                                                        version.set(version() + 1);
                                                        busy.set(false);
                                                    });
                                                },
                                                "Déclarer ma disponibilité"
                                            }
                                        }
                                    }
                                }
                                // Actions selon le statut
                                if m.status == MatchStatus::Scheduled {
                                    section { class: "page-card",
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            size: ButtonSize::M,
                                            disabled: busy(),
                                            onclick: move |_| {
                                                if let Some(u) = match_id { ra_start(MatchAction::Start(u)); }
                                            },
                                            "Démarrer le match"
                                        }
                                    }
                                }
                                if m.status == MatchStatus::InProgress {
                                    section { class: "page-card",
                                        h3 { "Saisir une manche" }
                                        div { class: "page-form",
                                            label {
                                                "{name1}"
                                                input {
                                                    class: "page-input",
                                                    r#type: "number",
                                                    min: "0",
                                                    value: "{s1}",
                                                    oninput: move |evt: FormEvent| s1.set(evt.value()),
                                                }
                                            }
                                            label {
                                                "{name2}"
                                                input {
                                                    class: "page-input",
                                                    r#type: "number",
                                                    min: "0",
                                                    value: "{s2}",
                                                    oninput: move |evt: FormEvent| s2.set(evt.value()),
                                                }
                                            }
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                size: ButtonSize::M,
                                                disabled: busy() || s1.read().is_empty() || s2.read().is_empty(),
                                                onclick: move |_| {
                                                    let (Ok(t1), Ok(t2)) = (
                                                        s1.read().parse::<i32>(),
                                                        s2.read().parse::<i32>(),
                                                    ) else { return };
                                                    if let Some(u) = match_id {
                                                        ra_manche(MatchAction::Manche(u, t1, t2, current_manche));
                                                    }
                                                },
                                                "Valider la manche"
                                            }
                                        }
                                    }
                                    section { class: "page-card",
                                        h3 { "Terminer le match" }
                                        div { class: "page-form",
                                            label {
                                                "Vainqueur"
                                                select {
                                                    class: "page-input",
                                                    onchange: move |evt: FormEvent| winner.set(evt.value()),
                                                    option { value: "team1", "{name1}" }
                                                    option { value: "team2", "{name2}" }
                                                    option { value: "draw", "Match nul" }
                                                }
                                            }
                                            Button {
                                                variant: ButtonVariant::Danger,
                                                size: ButtonSize::M,
                                                disabled: busy(),
                                                onclick: move |_| {
                                                    if let Some(u) = match_id {
                                                        ra_complete(MatchAction::Complete(u, winner.read().clone()));
                                                    }
                                                },
                                                "Terminer le match"
                                            }
                                        }
                                    }
                                }
                                if m.status == MatchStatus::Completed {
                                    section { class: "page-card",
                                        h3 { "Vérification" }
                                        if m.verified {
                                            p { class: "page-sub", "Résultat vérifié" }
                                            if let Some(at) = &m.verified_at {
                                                p { class: "muted", "le {format_date_fr(at)}" }
                                            }
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                size: ButtonSize::S,
                                                disabled: busy(),
                                                onclick: move |_| {
                                                    if let Some(u) = match_id { ra_unverify(MatchAction::Unverify(u)); }
                                                },
                                                "Annuler la vérification"
                                            }
                                        } else {
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                size: ButtonSize::M,
                                                disabled: busy(),
                                                onclick: move |_| {
                                                    if let Some(u) = match_id { ra_verify(MatchAction::Verify(u)); }
                                                },
                                                "Vérifier le résultat"
                                            }
                                        }
                                    }
                                }
                                if let Some((ok, msg)) = &*feedback.read() {
                                    if *ok {
                                        p { class: "page-sub", "{msg}" }
                                    } else {
                                        p { class: "page-error", "{msg}" }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Match introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}

/// Action arbitre déclenchée depuis l'UI.
enum MatchAction {
    Start(Uuid),
    Manche(Uuid, i32, i32, i32),
    Complete(Uuid, String),
    Verify(Uuid),
    Unverify(Uuid),
}
