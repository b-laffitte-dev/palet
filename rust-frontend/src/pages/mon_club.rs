//! Espace club — le responsable gère les effectifs de ses équipes :
//! ajouter / retirer des joueurs (POST/DELETE /equipes/{id}/joueurs...).

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::use_auth;
use crate::services::api::ApiService;
use crate::utils::labels::{is_club_manager, team_member_role_label};
use dioxus::prelude::*;
use palet_shared::{Player, Team, TeamMemberWithPlayer};
use uuid::Uuid;

#[component]
pub fn MonClub() -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let user = auth.read().user.clone();
    let club_id = user.as_ref().and_then(|u| u.club_id);
    let allowed = user
        .as_ref()
        .is_some_and(|u| is_club_manager(&u.role) && u.club_id.is_some());

    let my_teams = use_resource({
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
    let club_players = use_resource({
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

    let mut selected_team = use_signal(|| Option::<Uuid>::None);
    let mut version = use_signal(|| 0u32);
    let effectif = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let team = *selected_team.read();
            let _v = version();
            async move {
                match team {
                    Some(t) => api.get_effectif(t).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });

    let mut selected_player = use_signal(|| Option::<Uuid>::None);
    let mut busy = use_signal(|| false);
    let mut feedback = use_signal(|| Option::<(bool, String)>::None);

    let teams_list: Vec<Team> = match &*my_teams.read() {
        Some(Ok(t)) => t.clone(),
        _ => Vec::new(),
    };
    let players_list: Vec<Player> = match &*club_players.read() {
        Some(Ok(p)) => p.clone(),
        _ => Vec::new(),
    };
    let effectif_list: Vec<TeamMemberWithPlayer> = match &*effectif.read() {
        Some(Ok(e)) => e.clone(),
        _ => Vec::new(),
    };
    let member_ids: Vec<Uuid> = effectif_list.iter().map(|m| m.player_id).collect();
    let available: Vec<&Player> = players_list
        .iter()
        .filter(|p| !member_ids.contains(&p.id))
        .collect();

    // auto-sélection de la première équipe
    let current_team = *selected_team.read();
    let team_choice = match (current_team, teams_list.first()) {
        (Some(t), _) => Some(t),
        (None, Some(t)) => {
            let id = t.id;
            selected_team.set(Some(id));
            Some(id)
        }
        (None, None) => None,
    };

    if !allowed {
        return rsx! {
            Layout {
                main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                    div { class: "max-w-7xl mx-auto",
                        h1 { class: "page-title", "Mon club" }
                        if user.is_none() {
                            p { class: "page-sub", "Vous devez être connecté avec un compte responsable de club." }
                            Link { to: "/connexion", class: "badge", "Se connecter" }
                        } else {
                            p { class: "page-sub", "Cet espace est réservé aux responsables de club." }
                        }
                    }
                }
            }
        };
    }

    let add_player = {
        let api = api.clone();
        move |team: Uuid, player: Uuid| {
            let api = api.clone();
            busy.set(true);
            feedback.set(None);
            spawn(async move {
                let result = api
                    .add_team_member(team, player, None)
                    .await
                    .map(|m| format!("{} ajouté à l'effectif", m.player.full_name));
                match result {
                    Ok(msg) => feedback.set(Some((true, msg))),
                    Err(e) => feedback.set(Some((false, e))),
                }
                version.set(version() + 1);
                busy.set(false);
            });
        }
    };
    let remove_player = {
        let api = api.clone();
        move |team: Uuid, player: Uuid| {
            let api = api.clone();
            busy.set(true);
            feedback.set(None);
            spawn(async move {
                let result = api
                    .remove_team_member(team, player)
                    .await
                    .map(|_| "Joueur retiré de l'effectif".to_string());
                match result {
                    Ok(msg) => feedback.set(Some((true, msg))),
                    Err(e) => feedback.set(Some((false, e))),
                }
                version.set(version() + 1);
                busy.set(false);
            });
        }
    };
    let mut ra_add = add_player.clone();
    let ra_remove = remove_player.clone();

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Mon club" }
                    p { class: "page-sub", "Gérez les effectifs de vos équipes." }
                    div { class: "page-filters",
                        label {
                            "Équipe"
                            select {
                                class: "page-input",
                                onchange: move |evt: FormEvent| {
                                    let v = evt.value();
                                    selected_team.set(if v.is_empty() { None } else { Uuid::parse_str(&v).ok() });
                                },
                                option { value: "", "Choisir une équipe…" }
                                for t in &teams_list {
                                    if Some(t.id) == team_choice {
                                        option { value: "{t.id}", selected: true, "{t.name}" }
                                    } else {
                                        option { value: "{t.id}", "{t.name}" }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(team_id) = team_choice {
                        section { class: "page-card",
                            h3 { "Effectif ({effectif_list.len()})" }
                            if effectif_list.is_empty() {
                                p { class: "page-empty", "Aucun joueur dans cette équipe." }
                            } else {
                                div { class: "table-wrap",
                                    table { class: "table",
                                        thead {
                                            tr {
                                                th { "Joueur" } th { "Rôle" } th { "Poste" } th { "" }
                                            }
                                        }
                                        tbody {
                                            for m in &effectif_list {
                                                tr { key: "{m.player_id}",
                                                    td {
                                                        Link { to: "/joueurs/{m.player_id}", "{m.player.full_name}" }
                                                    }
                                                    td { span { class: "badge", "{team_member_role_label(&m.role)}" } }
                                                    td { "{m.position.clone().unwrap_or_default()}" }
                                                    td {
                                                        Button {
                                                            variant: ButtonVariant::Danger,
                                                            size: ButtonSize::S,
                                                            disabled: busy(),
                                                            onclick: {
                                                                let mut ra = ra_remove.clone();
                                                                let pid = m.player_id;
                                                                move |_| ra(team_id, pid)
                                                            },
                                                            "Retirer"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        section { class: "page-card",
                            h3 { "Ajouter un joueur" }
                            if available.is_empty() {
                                p { class: "page-empty", "Tous les joueurs du club sont déjà dans cette équipe." }
                            } else {
                                div { class: "page-form",
                                    label {
                                        "Joueur"
                                        select {
                                            class: "page-input",
                                            onchange: move |evt: FormEvent| {
                                                let v = evt.value();
                                                selected_player.set(if v.is_empty() { None } else { Uuid::parse_str(&v).ok() });
                                            },
                                            option { value: "", "Choisir un joueur…" }
                                            for p in &available {
                                                option { value: "{p.id}", "{p.full_name} ({p.license_number})" }
                                            }
                                        }
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        size: ButtonSize::M,
                                        disabled: busy() || selected_player.read().is_none(),
                                        onclick: move |_| {
                                            if let (Some(t), Some(p)) = (team_choice, *selected_player.read()) {
                                                ra_add(t, p);
                                            }
                                        },
                                        "Ajouter à l'effectif"
                                    }
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
            }
        }
    }
}
