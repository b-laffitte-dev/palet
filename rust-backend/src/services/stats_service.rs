//! Statistiques (analytics) calculées à partir des matchs et des dispo.
//!
//! Fonctions pures sur des slices : testables sans base de données.
//! Les handlers (`stats_handler`) se chargent des requêtes SQL.

use palet_shared::{
    AvailabilityStatus, ClubStats, CompetitionStats, Match, MatchAvailability, MatchStatus,
    MatchWinner, PlayerStats,
};
use uuid::Uuid;

/// Vrai si le match concerne l'une des équipes données.
fn match_involves_team(m: &Match, team_ids: &[Uuid]) -> bool {
    team_ids.contains(&m.team1_id) || team_ids.contains(&m.team2_id)
}

/// Vrai si l'équipe donnée a gagné le match.
fn team_won(m: &Match, team_id: &Uuid) -> bool {
    match m.winner {
        Some(MatchWinner::Team1) => m.team1_id == *team_id,
        Some(MatchWinner::Team2) => m.team2_id == *team_id,
        _ => false,
    }
}

/// Score marqué par l'équipe donnée dans ce match.
fn team_points(m: &Match, team_id: &Uuid) -> i64 {
    if m.team1_id == *team_id {
        m.team1_score.unwrap_or(0) as i64
    } else {
        m.team2_score.unwrap_or(0) as i64
    }
}

/// Stats d'un joueur : matchs terminés de ses équipes + ses dispo déclarées.
pub fn compute_player_stats(
    player_id: Uuid,
    team_ids: &[Uuid],
    matches: &[Match],
    disponibilites: &[MatchAvailability],
) -> PlayerStats {
    let played: Vec<&Match> = matches
        .iter()
        .filter(|m| m.status == MatchStatus::Completed && match_involves_team(m, team_ids))
        .collect();

    let mut wins = 0i64;
    let mut losses = 0i64;
    let mut draws = 0i64;
    let mut points_for = 0i64;
    let mut points_against = 0i64;
    for m in &played {
        // côté du joueur = l'équipe qui le contient
        let my_team = if team_ids.contains(&m.team1_id) {
            m.team1_id
        } else {
            m.team2_id
        };
        let opp_points = if my_team == m.team1_id {
            m.team2_score.unwrap_or(0) as i64
        } else {
            m.team1_score.unwrap_or(0) as i64
        };
        points_for += team_points(m, &my_team);
        points_against += opp_points;
        if team_won(m, &my_team) {
            wins += 1;
        } else if m.winner == Some(MatchWinner::Draw) {
            draws += 1;
        } else {
            losses += 1;
        }
    }

    let n = played.len() as i64;
    let dispo_declarees = disponibilites.len() as i64;
    let dispo_ok = disponibilites
        .iter()
        .filter(|d| d.status == AvailabilityStatus::Disponible)
        .count() as i64;

    PlayerStats {
        player_id,
        matches_played: n,
        wins,
        losses,
        draws,
        points_for,
        points_against,
        avg_points_for: if n > 0 {
            points_for as f64 / n as f64
        } else {
            0.0
        },
        win_rate: if n > 0 { wins as f64 / n as f64 } else { 0.0 },
        disponibilites_declarees: dispo_declarees,
        taux_disponible: if dispo_declarees > 0 {
            dispo_ok as f64 / dispo_declarees as f64
        } else {
            0.0
        },
    }
}

/// Stats d'un club : matchs terminés de toutes ses équipes.
pub fn compute_club_stats(
    club_id: Uuid,
    team_ids: &[Uuid],
    teams_count: i64,
    players_count: i64,
    matches: &[Match],
) -> ClubStats {
    let played: Vec<&Match> = matches
        .iter()
        .filter(|m| m.status == MatchStatus::Completed && match_involves_team(m, team_ids))
        .collect();

    let mut wins = 0i64;
    let mut losses = 0i64;
    let mut draws = 0i64;
    let mut points_for = 0i64;
    let mut points_against = 0i64;
    for m in &played {
        let my_team = if team_ids.contains(&m.team1_id) {
            m.team1_id
        } else {
            m.team2_id
        };
        let opp_points = if my_team == m.team1_id {
            m.team2_score.unwrap_or(0) as i64
        } else {
            m.team1_score.unwrap_or(0) as i64
        };
        points_for += team_points(m, &my_team);
        points_against += opp_points;
        if team_won(m, &my_team) {
            wins += 1;
        } else if m.winner == Some(MatchWinner::Draw) {
            draws += 1;
        } else {
            losses += 1;
        }
    }

    let n = played.len() as i64;
    ClubStats {
        club_id,
        teams_count,
        players_count,
        matches_played: n,
        wins,
        losses,
        draws,
        points_for,
        points_against,
        win_rate: if n > 0 { wins as f64 / n as f64 } else { 0.0 },
    }
}

/// Stats d'une compétition : répartition des statuts + scoring.
pub fn compute_competition_stats(competition_id: Uuid, matches: &[Match]) -> CompetitionStats {
    let mut total = 0i64;
    let mut completed = 0i64;
    let mut in_progress = 0i64;
    let mut scheduled = 0i64;
    let mut verified = 0i64;
    let mut sum_points = 0i64;
    let mut best = 0i64;
    for m in matches {
        total += 1;
        match m.status {
            MatchStatus::Completed => {
                completed += 1;
                if m.verified {
                    verified += 1;
                }
                let total_points =
                    m.team1_score.unwrap_or(0) as i64 + m.team2_score.unwrap_or(0) as i64;
                sum_points += total_points;
                if total_points > best {
                    best = total_points;
                }
            }
            MatchStatus::InProgress => in_progress += 1,
            _ => scheduled += 1,
        }
    }
    CompetitionStats {
        competition_id,
        total_matches: total,
        completed,
        in_progress,
        scheduled,
        verified,
        avg_total_points: if completed > 0 {
            sum_points as f64 / completed as f64
        } else {
            0.0
        },
        best_total_points: best,
    }
}

/// Compte les dispo par statut (récap d'un match).
pub fn summarize_availabilities(disponibilites: &[MatchAvailability]) -> (i64, i64, i64, i64) {
    let mut dispo = 0i64;
    let mut indispo = 0i64;
    let mut incertain = 0i64;
    for d in disponibilites {
        match d.status {
            AvailabilityStatus::Disponible => dispo += 1,
            AvailabilityStatus::Indisponible => indispo += 1,
            AvailabilityStatus::Incertain => incertain += 1,
        }
    }
    (disponibilites.len() as i64, dispo, indispo, incertain)
}

/// Filtre insensible à la casse d'une liste de chaînes candidates.
pub fn matches_query(haystack: &str, query: &str) -> bool {
    haystack.to_lowercase().contains(&query.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use palet_shared::{CompetitionType, MancheScore};
    use time::OffsetDateTime;

    #[allow(clippy::too_many_arguments)]
    fn mk_match(
        id: Uuid,
        t1: Uuid,
        t2: Uuid,
        s1: i32,
        s2: i32,
        winner: Option<MatchWinner>,
        status: MatchStatus,
        verified: bool,
    ) -> Match {
        let now = OffsetDateTime::now_utc();
        Match {
            id,
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championship,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: t1,
            team2_id: t2,
            team1_score: Some(s1),
            team2_score: Some(s2),
            winner,
            manches: Some(vec![MancheScore {
                number: 1,
                team1: s1,
                team2: s2,
                winner: None,
            }]),
            best_of: Some(13),
            current_manche: Some(1),
            current_scores: None,
            status,
            verified,
            verified_by: None,
            verified_at: None,
            date: now,
            time: None,
            location: None,
            duration: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            ended_at: None,
        }
    }

    #[test]
    fn player_stats_count_only_completed_and_wins() {
        let team_a = Uuid::new_v4();
        let team_b = Uuid::new_v4();
        let player = Uuid::new_v4();
        let matches = vec![
            mk_match(
                Uuid::new_v4(),
                team_a,
                team_b,
                13,
                9,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                true,
            ),
            mk_match(
                Uuid::new_v4(),
                team_b,
                team_a,
                13,
                7,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                true,
            ), // team_b gagne -> défaite pour A
            mk_match(
                Uuid::new_v4(),
                team_a,
                team_b,
                0,
                0,
                None,
                MatchStatus::Scheduled,
                false,
            ), // pas joué
        ];
        let stats = compute_player_stats(player, &[team_a], &matches, &[]);
        assert_eq!(stats.matches_played, 2);
        assert_eq!(stats.wins, 1);
        assert_eq!(stats.losses, 1);
        assert_eq!(stats.points_for, 20); // 13 + 7
        assert_eq!(stats.points_against, 22); // 9 + 13
        assert!((stats.win_rate - 0.5).abs() < 1e-9);
        assert!((stats.avg_points_for - 10.0).abs() < 1e-9);
    }

    #[test]
    fn player_stats_taux_disponible() {
        let player = Uuid::new_v4();
        let m = Uuid::new_v4();
        let dispo = vec![
            MatchAvailability {
                match_id: m,
                player_id: player,
                status: AvailabilityStatus::Disponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
            MatchAvailability {
                match_id: m,
                player_id: player,
                status: AvailabilityStatus::Incertain,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
            MatchAvailability {
                match_id: m,
                player_id: player,
                status: AvailabilityStatus::Indisponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
            MatchAvailability {
                match_id: m,
                player_id: player,
                status: AvailabilityStatus::Disponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
        ];
        let stats = compute_player_stats(player, &[], &[], &dispo);
        assert_eq!(stats.disponibilites_declarees, 4);
        assert!((stats.taux_disponible - 0.5).abs() < 1e-9);
    }

    #[test]
    fn club_stats_aggregate_teams() {
        let team_a = Uuid::new_v4();
        let team_b = Uuid::new_v4();
        let team_x = Uuid::new_v4(); // autre club
        let club = Uuid::new_v4();
        let matches = vec![
            mk_match(
                Uuid::new_v4(),
                team_a,
                team_x,
                13,
                5,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                true,
            ),
            mk_match(
                Uuid::new_v4(),
                team_x,
                team_b,
                13,
                11,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                false,
            ),
            mk_match(
                Uuid::new_v4(),
                team_x,
                Uuid::new_v4(),
                13,
                0,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                false,
            ), // hors club
        ];
        let stats = compute_club_stats(club, &[team_a, team_b], 2, 6, &matches);
        assert_eq!(stats.matches_played, 2);
        assert_eq!(stats.wins, 1);
        assert_eq!(stats.losses, 1);
        assert_eq!(stats.points_for, 24);
        assert_eq!(stats.teams_count, 2);
        assert_eq!(stats.players_count, 6);
    }

    #[test]
    fn competition_stats_status_and_scoring() {
        let comp = Uuid::new_v4();
        let t1 = Uuid::new_v4();
        let t2 = Uuid::new_v4();
        let matches = vec![
            mk_match(
                Uuid::new_v4(),
                t1,
                t2,
                13,
                9,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                true,
            ),
            mk_match(
                Uuid::new_v4(),
                t1,
                t2,
                15,
                12,
                Some(MatchWinner::Team1),
                MatchStatus::Completed,
                false,
            ),
            mk_match(
                Uuid::new_v4(),
                t1,
                t2,
                0,
                0,
                None,
                MatchStatus::InProgress,
                false,
            ),
            mk_match(
                Uuid::new_v4(),
                t1,
                t2,
                0,
                0,
                None,
                MatchStatus::Scheduled,
                false,
            ),
        ];
        let stats = compute_competition_stats(comp, &matches);
        assert_eq!(stats.total_matches, 4);
        assert_eq!(stats.completed, 2);
        assert_eq!(stats.in_progress, 1);
        assert_eq!(stats.scheduled, 1);
        assert_eq!(stats.verified, 1);
        assert!((stats.avg_total_points - 24.5).abs() < 1e-9); // (22 + 27) / 2
        assert_eq!(stats.best_total_points, 27);
    }

    #[test]
    fn summarize_counts_by_status() {
        let m = Uuid::new_v4();
        let p = Uuid::new_v4();
        let list = vec![
            MatchAvailability {
                match_id: m,
                player_id: p,
                status: AvailabilityStatus::Disponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
            MatchAvailability {
                match_id: m,
                player_id: p,
                status: AvailabilityStatus::Disponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
            MatchAvailability {
                match_id: m,
                player_id: p,
                status: AvailabilityStatus::Indisponible,
                commentaire: None,
                updated_at: OffsetDateTime::now_utc(),
            },
        ];
        assert_eq!(summarize_availabilities(&list), (3, 2, 1, 0));
        assert_eq!(summarize_availabilities(&[]), (0, 0, 0, 0));
    }

    #[test]
    fn matches_query_is_case_insensitive() {
        assert!(matches_query("La Roche-sur-Yon Palet Club", "roche"));
        assert!(matches_query("LRPC", "lrpc"));
        assert!(!matches_query("LRPC", "challans"));
    }
}
