//! Tableau des matchs d'une compétition, groupés par journée.
//!
//! Utilisé par les pages détail championnat et tournoi.

use crate::utils::format::format_date_fr;
use crate::utils::labels::match_status_label;
use dioxus::prelude::*;
use palet_shared::{Club, Match, Team};
use uuid::Uuid;

/// Nom d'affichage d'une équipe.
///
/// Les matchs et classements référencent l'id du **club** (Vague 1), tandis
/// que les inscriptions de tournoi référencent l'id de l'équipe : on cherche
/// donc d'abord dans les équipes, puis dans les clubs. Si rien ne correspond,
/// on affiche un extrait de l'UUID plutôt que de planter.
pub fn team_name(teams: &[Team], clubs: &[Club], team_id: &Uuid) -> String {
    teams
        .iter()
        .find(|t| t.id == *team_id)
        .map(|t| t.name.clone())
        .or_else(|| {
            clubs
                .iter()
                .find(|c| c.id == *team_id)
                .map(|c| c.name.clone())
        })
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

fn build_rows(matches: &[Match], teams: &[Team], clubs: &[Club]) -> Vec<(String, Vec<MatchRow>)> {
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
                            team_name(teams, clubs, &m.team1_id),
                            team_name(teams, clubs, &m.team2_id)
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
pub fn MatchTable(matches: Vec<Match>, teams: Vec<Team>, clubs: Vec<Club>) -> Element {
    let groups = build_rows(&matches, &teams, &clubs);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn team(id: &str, name: &str) -> Team {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "name": name,
            "club_id": "00000000-0000-0000-0000-000000000000",
            "captain_id": null,
            "logo": null,
            "stats": {
                "matches": 0, "wins": 0, "draws": 0, "losses": 0,
                "points_for": 0, "points_against": 0, "diff": 0, "points": 0, "win_rate": 0.0
            },
            "status": "active",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        }))
        .expect("team de test valide")
    }

    fn club(id: &str, name: &str) -> Club {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "name": name,
            "nickname": null,
            "code": "TST",
            "logo": null,
            "address": null,
            "city": "Testville",
            "postal_code": null,
            "league": "Deux-Sèvres",
            "division": "D2",
            "status": "active",
            "players_count": 0,
            "teams_count": 0,
            "contact_email": null,
            "contact_phone": null,
            "website": null,
            "social": { "facebook": null, "twitter": null, "instagram": null },
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        }))
        .expect("club de test valide")
    }

    #[test]
    fn team_name_trouve_le_nom_dans_les_equipes() {
        let teams = vec![team("11111111-1111-1111-1111-111111111111", "Airvault A")];
        let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        assert_eq!(team_name(&teams, &[], &id), "Airvault A");
    }

    #[test]
    fn team_name_retombe_sur_le_club_quand_pas_d_equipe() {
        // Vague 1 : les matchs/classements référencent l'id du club.
        let clubs = vec![club(
            "38c14d11-aa95-4f48-aff9-c2a2422669e1",
            "Niort Palet Club",
        )];
        let id = Uuid::parse_str("38c14d11-aa95-4f48-aff9-c2a2422669e1").unwrap();
        assert_eq!(team_name(&[], &clubs, &id), "Niort Palet Club");
    }

    #[test]
    fn team_name_equipe_gagne_sur_le_club() {
        let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let teams = vec![team("11111111-1111-1111-1111-111111111111", "Airvault A")];
        let clubs = vec![club(
            "11111111-1111-1111-1111-111111111111",
            "Airvault Palet Club",
        )];
        assert_eq!(team_name(&teams, &clubs, &id), "Airvault A");
    }

    #[test]
    fn team_name_inconnu_affiche_un_extrait_d_uuid() {
        let id = Uuid::parse_str("38c14d11-aa95-4f48-aff9-c2a2422669e1").unwrap();
        assert_eq!(team_name(&[], &[], &id), "38c14d11");
    }
}
