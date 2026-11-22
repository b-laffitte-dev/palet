//! Page détail d'une compétition ponctuelle — GET /competitions/{id} + inscriptions + matchs
//!
//! Un responsable de club connecté (club_admin) peut inscrire une de ses
//! équipes : POST /competitions/{id}/inscrire.

use crate::components::common::{team_name, Button, ButtonSize, ButtonVariant, MatchTable};
use crate::components::layout::Layout;
use crate::hooks::use_auth::use_auth;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use crate::utils::labels::{
    competition_category_label, competition_format_label, competition_material_label,
    competition_status_label, competition_type_label, registration_status_label,
};
use dioxus::prelude::*;
use palet_shared::{RegistrationStatus, UserRole};
use uuid::Uuid;

#[component]
pub fn TournamentDetail(id: String) -> Element {
    let api = ApiService::new();
    let auth = use_auth();
    let comp_id = Uuid::parse_str(&id).ok();

    let user = auth.read().user.clone();
    let club_id = user.as_ref().and_then(|u| u.club_id);
    let can_register = matches!(
        user.as_ref().map(|u| &u.role),
        Some(UserRole::ClubAdmin) | Some(UserRole::CommissionAdmin) | Some(UserRole::SuperAdmin)
    ) && club_id.is_some();

    let competition = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_competition(u).await,
                    Err(_) => Err("Identifiant de compétition invalide".to_string()),
                }
            }
        }
    });
    // Compteur pour recharger les inscriptions après une inscription.
    let mut version = use_signal(|| 0u32);
    let inscriptions = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let v = version();
            async move {
                let _ = v;
                match comp_id {
                    Some(u) => api.get_inscriptions(u).await,
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
                match comp_id {
                    Some(u) => api.get_matches_by_competition(u).await,
                    None => Ok(Vec::new()),
                }
            }
        }
    });
    let stats = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match comp_id {
                    Some(u) => api.get_competition_stats(u).await,
                    None => Err("Identifiant invalide".to_string()),
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
    let my_teams = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move {
                match (can_register, club_id) {
                    (true, Some(c)) => api.get_equipes(Some(c), None).await,
                    _ => Ok(Vec::new()),
                }
            }
        }
    });

    // Formulaire d'inscription
    let mut selected_team = use_signal(|| Option::<Uuid>::None);
    let mut sending = use_signal(|| false);
    let mut feedback = use_signal(|| Option::<(bool, String)>::None);

    // Les matchs de la compétition référencent l'id du club : les clubs
    // servent de repli pour résoudre les noms d'équipes.
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
    let my_teams_list = match &*my_teams.read() {
        Some(Ok(t)) => t.clone(),
        _ => Vec::new(),
    };
    let my_team_ids: Vec<Uuid> = my_teams_list.iter().map(|t| t.id).collect();
    let already_registered = match &*inscriptions.read() {
        Some(Ok(list)) => list
            .iter()
            .any(|r| my_team_ids.contains(&r.team_id) && r.status != RegistrationStatus::Cancelled),
        _ => false,
    };

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    match &*competition.read() {
                        Some(Ok(c)) => {
                            let teams_count = match c.max_teams {
                                Some(max) => format!("{} / {max}", c.registered_teams),
                                None => format!("{} (illimité)", c.registered_teams),
                            };
                            rsx! {
                            h1 { class: "page-title", "{c.name}" }
                            p { class: "page-sub",
                                span { class: "badge", "{competition_status_label(&c.status)}" }
                                span { "  {competition_type_label(&c.competition_type)} · {competition_category_label(&c.category)} · palets {competition_material_label(&c.material)} · {competition_format_label(&c.format)}" }
                            }
                            div { class: "page-detail",
                                section { class: "page-card",
                                    h3 { "Statistiques" }
                                    match &*stats.read() {
                                        Some(Ok(st)) => rsx! {
                                            dl { class: "kv",
                                                dt { "Matchs au total" } dd { "{st.total_matches}" }
                                                dt { "Terminés" } dd { "{st.completed} ({st.verified} vérifiés)" }
                                                dt { "En cours" } dd { "{st.in_progress}" }
                                                dt { "Programmés" } dd { "{st.scheduled}" }
                                                dt { "Moyenne de points/match" } dd { "{st.avg_total_points:.1}" }
                                                dt { "Meilleur total" } dd { "{st.best_total_points}" }
                                            }
                                        },
                                        Some(Err(_)) => rsx! { p { class: "muted", "Stats indisponibles" } },
                                        None => rsx! { p { class: "muted", "Chargement…" } },
                                    }
                                }
                                section { class: "page-card",
                                    h3 { "Informations" }
                                    dl { class: "kv",
                                        if let Some(desc) = &c.description {
                                            dt { "Description" } dd { "{desc}" }
                                        }
                                        if let Some(loc) = &c.location {
                                            dt { "Lieu" } dd { "{loc}" }
                                        }
                                        dt { "Saison" } dd { "{c.season}" }
                                        if let Some(s) = &c.start_date {
                                            dt { "Début" } dd { "{format_date_fr(s)}" }
                                        }
                                        if let Some(e) = &c.end_date {
                                            dt { "Fin" } dd { "{format_date_fr(e)}" }
                                        }
                                        if let Some(rs) = &c.registration_start {
                                            dt { "Inscriptions du" } dd { "{format_date_fr(rs)}" }
                                        }
                                        if let Some(re) = &c.registration_end {
                                            dt { "Inscriptions jusqu'au" } dd { "{format_date_fr(re)}" }
                                        }
                                        dt { "Équipes" }
                                        dd { "{teams_count}" }
                                        if let Some(fee) = c.fee {
                                            dt { "Engagement" } dd { "{fee} €" }
                                        }
                                    }
                                }
                                if can_register {
                                    section { class: "page-card",
                                        h3 { "Inscrire mon équipe" }
                                        if already_registered {
                                            p { class: "page-sub", "Une de vos équipes est déjà inscrite à cette compétition." }
                                        } else {
                                                div { class: "page-form",
                                                    label {
                                                        "Équipe"
                                                        select {
                                                            class: "page-input",
                                                            onchange: move |evt: FormEvent| {
                                                                let v = evt.value();
                                                                selected_team.set(if v.is_empty() { None } else { Uuid::parse_str(&v).ok() });
                                                            },
                                                            option { value: "", "Choisir une équipe…" }
                                                            for t in &my_teams_list {
                                                                option { value: "{t.id}", "{t.name}" }
                                                            }
                                                        }
                                                    }
                                                    Button {
                                                        variant: ButtonVariant::Primary,
                                                        size: ButtonSize::M,
                                                        disabled: sending() || selected_team.read().is_none(),
                                                        onclick: move |_| {
                                                            let Some(team_id) = *selected_team.read() else { return };
                                                            let api = api.clone();
                                                            sending.set(true);
                                                            feedback.set(None);
                                                            spawn(async move {
                                                                let result = match comp_id {
                                                                    Some(u) => api.inscrire(u, team_id).await.map(|r| {
                                                                        format!("Inscription enregistrée — statut : {}", registration_status_label(&r.status))
                                                                    }),
                                                                    None => Err("Compétition invalide".to_string()),
                                                                };
                                                                match result {
                                                                    Ok(msg) => {
                                                                        feedback.set(Some((true, msg)));
                                                                        version.set(version() + 1);
                                                                    }
                                                                    Err(e) => feedback.set(Some((false, e))),
                                                                }
                                                                sending.set(false);
                                                            });
                                                        },
                                                        "Inscrire mon équipe"
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
                                section {
                                    h2 { class: "page-title", "Équipes inscrites" }
                                    match &*inscriptions.read() {
                                        Some(Ok(list)) if !list.is_empty() => rsx! {
                                            div { class: "table-wrap page-card",
                                                table { class: "table",
                                                    thead {
                                                        tr {
                                                            th { "Équipe" } th { "Statut" } th { "Payé" } th { "Inscrite le" }
                                                        }
                                                    }
                                                    tbody {
                                                        for r in list {
                                                            tr { key: "{r.team_id}",
                                                                td { "{team_name(&teams_list, &clubs_list, &r.team_id)}" }
                                                                td { span { class: "badge", "{registration_status_label(&r.status)}" } }
                                                                td { if r.paid { "Oui" } else { "Non" } }
                                                                td { "{format_date_fr(&r.created_at)}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucune équipe inscrite pour le moment." } },
                                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur inscriptions : {e}" } },
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
                            }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Compétition introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
