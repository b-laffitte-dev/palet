//! Tableau des matchs d'une compétition, groupés par journée.
//!
//! Utilisé par les pages détail championnat et tournoi.

use crate::utils::format::format_date_fr;
use crate::utils::labels::match_status_label;
use dioxus::prelude::*;
use palet_shared::{Match, Team};
use uuid::Uuid;

/// Nom d'affichage d'une équipe à partir de la liste des équipes.
///
/// En Vague 1 l'id d'une équipe est l'id de son club ; si l'équipe est
/// introuvable on affiche un extrait de l'UUID.
pub fn team_name(teams: &[Team], team_id: &Uuid) -> String {
    teams
        .iter()
        .find(|t| t.id == *team_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| team_id.to_string().chars().take(8).collect())
}

/// Ligne de match aplatie, prête à être rendue.
struct MatchRow {
    id: Uuid,
    when: String,
    label: String,
    score: String,
    location: String,
    status: &'static str,
}

fn build_rows(matches: &[Match], teams: &[Team]) -> Vec<(String, Vec<MatchRow>)> {
    let mut groups: Vec<(Option<i32>, Vec<&Match>)> = Vec::new();
    for m in matches {
        match groups.iter_mut().find(|(j, _)| *j == m.journee) {
            Some((_, group)) => group.push(m),
            None => groups.push((m.journee, vec![m])),
        }
    }
    groups.sort_by_key(|(j, _)| j.unwrap_or(i32::MAX));

    groups
        .into_iter()
        .map(|(journee, mut group)| {
            group.sort_by_key(|m| m.date);
            let title = match journee {
                Some(j) => format!("Journée {j}"),
                None => "Matchs".to_string(),
            };
            let rows = group
                .iter()
                .map(|m| {
                    let when = match &m.time {
                        Some(t) => format!("{} {t}", format_date_fr(&m.date)),
                        None => format_date_fr(&m.date),
                    };
                    let score = match (m.team1_score, m.team2_score) {
                        (Some(s1), Some(s2)) => format!("{s1} - {s2}"),
                        _ => "—".to_string(),
                    };
                    MatchRow {
                        id: m.id,
                        when,
                        label: format!(
                            "{} — {}",
                            team_name(teams, &m.team1_id),
                            team_name(teams, &m.team2_id)
                        ),
                        score,
                        location: m.location.clone().unwrap_or_default(),
                        status: match_status_label(&m.status),
                    }
                })
                .collect();
            (title, rows)
        })
        .collect()
}

/// Tableau des matchs d'une compétition, groupé par journée.
#[component]
pub fn MatchTable(matches: Vec<Match>, teams: Vec<Team>) -> Element {
    let groups = build_rows(&matches, &teams);

    rsx! {
        if groups.is_empty() {
            p { class: "page-empty", "Aucun match pour le moment." }
        }
        for (title, rows) in groups {
            section { key: "{title}",
                h2 { class: "page-title", "{title}" }
                div { class: "table-wrap page-card",
                    table { class: "table",
                        thead {
                            tr {
                                th { "Date" }
                                th { "Match" }
                                th { "Score" }
                                th { "Lieu" }
                                th { "Statut" }
                            }
                        }
                        tbody {
                            for row in rows {
                                tr { key: "{row.id}",
                                    td { "{row.when}" }
                                    td { "{row.label}" }
                                    td { "{row.score}" }
                                    td { "{row.location}" }
                                    td { span { class: "badge", "{row.status}" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
