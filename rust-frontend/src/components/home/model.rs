//! View models for the home page, mapped from the API types.
//!
//! The layout follows `docs/maquettes/homepage.png`. Only endpoints that
//! actually exist on the backend are used (`/clubs`, `/matchs`,
//! `/championnats`), so this layer is a pure transformation of their data.

use palet_shared::{
    Championship, ChampionshipClassification, Club, CompetitionType, Match, MatchResult,
    MatchStatus, TeamClassificationStatus,
};
use uuid::Uuid;

use crate::services::api::ApiService;

// ============================================
// VIEW MODELS (plain data, easy to render)
// ============================================

#[derive(Clone, PartialEq, Default)]
pub struct TeamSide {
    pub name: String,
    pub abbr: String,
    pub subtitle: String,
    pub score: Option<i32>,
}

#[derive(Clone, PartialEq, Default)]
pub struct HeroMatch {
    pub kicker: String,
    pub subtitle: String,
    pub info: String,
    pub live: bool,
    pub finished: bool,
    pub badge: String,
    pub team1: TeamSide,
    pub team2: TeamSide,
}

#[derive(Clone, PartialEq, Default)]
pub struct LiveItem {
    pub team1: String,
    pub team2: String,
    pub score1: Option<i32>,
    pub score2: Option<i32>,
    pub status: String,
    pub live: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Qualif,
    Barrage,
    Releg,
    Safe,
}

#[derive(Clone, PartialEq)]
pub struct StandingRow {
    pub position: i32,
    pub club: String,
    pub abbr: String,
    pub played: i32,
    pub wins: i32,
    pub losses: i32,
    pub diff: i32,
    pub points: i32,
    pub form: Vec<MatchResult>,
    pub zone: Zone,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventTone {
    Open,
    Championship,
    Federal,
}

#[derive(Clone, PartialEq)]
pub struct EventCard {
    pub tone: EventTone,
    pub tag: String,
    pub title: String,
    pub pill: String,
    pub date: String,
    pub location: String,
    pub details: Vec<String>,
    pub action: String,
    pub href: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Accent {
    Gold,
    Green,
    Amber,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeKind {
    Mvp,
    TopScorer,
    Revelation,
}

#[derive(Clone, PartialEq)]
pub struct TopCard {
    pub accent: Accent,
    pub initials: String,
    pub name: String,
    pub club: String,
    pub badge: String,
    pub badge_kind: BadgeKind,
    pub stats: Vec<(String, String)>,
}

/// Everything the home page needs, loaded once from the API.
#[derive(Clone, Default)]
pub struct HomeData {
    pub hero: Option<HeroMatch>,
    pub live: Vec<LiveItem>,
    pub journee: Option<i32>,
    pub championship_name: String,
    pub division: String,
    pub subtitle: String,
    pub standings: Vec<StandingRow>,
    pub events: Vec<EventCard>,
    pub top: Vec<TopCard>,
}

// ============================================
// HELPERS
// ============================================

/// "La Roche-sur-Yon Palet Club" -> "LR"
fn initials(name: &str) -> String {
    name.split([' ', '-', '\''])
        .filter(|word| !word.is_empty())
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
        .chars()
        .take(2)
        .collect()
}

/// Resolve a team id against the club list (championship matches oppose
/// clubs; the backend exposes no dedicated `/teams` endpoint).
fn team_label(clubs: &[Club], team_id: &Uuid) -> (String, String) {
    match clubs.iter().find(|c| c.id == *team_id) {
        Some(club) => (club.name.clone(), initials(&club.name)),
        None => {
            let short = team_id.to_string();
            let head: String = short.chars().take(8).collect();
            let abbr: String = short.chars().take(2).collect::<String>().to_uppercase();
            (format!("Équipe {head}"), abbr)
        }
    }
}

fn format_score(diff: i32) -> String {
    if diff >= 0 {
        format!("+{diff}")
    } else {
        format!("{diff}")
    }
}

fn format_match_datetime(m: &Match) -> String {
    let date = m.date.format("%d/%m/%Y").to_string();
    match &m.time {
        Some(time) if !time.is_empty() => format!("{date} · {time}"),
        _ => date,
    }
}

// ============================================
// LOAD + MAP
// ============================================

impl HomeData {
    pub async fn load(api: &ApiService) -> Self {
        // Each call is independent: a failing endpoint must not break the
        // whole page, so failures degrade to empty data.
        let clubs = api.get_clubs().await.unwrap_or_default();
        let live = api
            .get_matches_by_status("in_progress")
            .await
            .unwrap_or_default();
        let completed = api
            .get_matches_by_status("completed")
            .await
            .unwrap_or_default();
        let scheduled = api
            .get_matches_by_status("scheduled")
            .await
            .unwrap_or_default();
        let championships = api.get_championships().await.unwrap_or_default();

        let championship = pick_championship(&championships);
        let classification = match &championship {
            Some(c) => api
                .get_championship_classification(c.id)
                .await
                .unwrap_or_default(),
            None => Vec::new(),
        };

        let hero = build_hero(
            live.first().or(scheduled.first()),
            &clubs,
            championship.as_ref(),
        );
        let (live_items, journee) = build_live(&live, &completed, &clubs);
        let (championship_name, division, subtitle, standings) =
            build_standings(&classification, &clubs, championship.as_ref());
        let events = build_events(&scheduled, &clubs);
        let top = build_top(&standings);

        HomeData {
            hero,
            live: live_items,
            journee,
            championship_name,
            division,
            subtitle,
            standings,
            events,
            top,
        }
    }
}

/// Prefer a championship in progress, then one with a D1 division, then any.
fn pick_championship(championships: &[Championship]) -> Option<Championship> {
    championships
        .iter()
        .find(|c| c.status == palet_shared::CompetitionStatus::InProgress)
        .or_else(|| {
            championships
                .iter()
                .find(|c| c.divisions.iter().any(|d| d == "D1"))
        })
        .or_else(|| championships.first())
        .cloned()
}

fn build_hero(
    m: Option<&Match>,
    clubs: &[Club],
    championship: Option<&Championship>,
) -> Option<HeroMatch> {
    let m = m?;
    let (name1, abbr1) = team_label(clubs, &m.team1_id);
    let (name2, abbr2) = team_label(clubs, &m.team2_id);

    let (score1, score2, manche) = match &m.current_scores {
        Some(cs) => (Some(cs.team1), Some(cs.team2), Some(cs.manche)),
        None => (m.team1_score, m.team2_score, m.current_manche),
    };

    let live = m.status == MatchStatus::InProgress;
    let finished = m.status == MatchStatus::Completed;
    let badge = if live {
        format!("LIVE — Manche {}", manche.unwrap_or(1))
    } else if finished {
        "TERMINÉ".to_string()
    } else {
        "À VENIR".to_string()
    };

    let kicker = if live {
        "MATCH EN DIRECT".to_string()
    } else if finished {
        "DERNIER RÉSULTAT".to_string()
    } else {
        "PROCHAIN MATCH".to_string()
    };
    let kicker = match (&m.phase, &m.round, m.journee) {
        (Some(phase), _, _) => phase.to_uppercase(),
        (_, Some(round), _) => round.to_uppercase(),
        (_, _, Some(j)) => format!("JOURNÉE {j}"),
        _ => kicker,
    };
    let kicker = match championship {
        Some(c) => format!("{} — {}", kicker, c.name.to_uppercase()),
        None => kicker,
    };

    let subtitle = format_match_datetime(m);
    let info = m
        .location
        .clone()
        .unwrap_or_else(|| "Terrain clos municipal".to_string());

    Some(HeroMatch {
        kicker,
        subtitle,
        info,
        live,
        finished,
        badge,
        team1: TeamSide {
            name: name1,
            abbr: abbr1,
            subtitle: "Équipe 1".to_string(),
            score: score1,
        },
        team2: TeamSide {
            name: name2,
            abbr: abbr2,
            subtitle: "Équipe 2".to_string(),
            score: score2,
        },
    })
}

fn build_live(live: &[Match], completed: &[Match], clubs: &[Club]) -> (Vec<LiveItem>, Option<i32>) {
    let mut items = Vec::new();
    let mut journee = live.first().and_then(|m| m.journee);

    for m in live.iter().take(3) {
        let (name1, _) = team_label(clubs, &m.team1_id);
        let (name2, _) = team_label(clubs, &m.team2_id);
        let manche = m
            .current_scores
            .as_ref()
            .map(|cs| cs.manche)
            .or(m.current_manche);
        journee = journee.or(m.journee);
        items.push(LiveItem {
            team1: name1,
            team2: name2,
            score1: m
                .current_scores
                .as_ref()
                .map(|cs| cs.team1)
                .or(m.team1_score),
            score2: m
                .current_scores
                .as_ref()
                .map(|cs| cs.team2)
                .or(m.team2_score),
            status: match manche {
                Some(n) => format!("Manche {n} · en cours"),
                None => "En cours".to_string(),
            },
            live: true,
        });
    }

    // Fill the band with the most recently completed matches.
    let mut done: Vec<&Match> = completed.iter().collect();
    done.sort_by_key(|m| std::cmp::Reverse(m.date));
    for m in done.into_iter().take(3 - items.len().min(3)) {
        let (name1, _) = team_label(clubs, &m.team1_id);
        let (name2, _) = team_label(clubs, &m.team2_id);
        journee = journee.or(m.journee);
        items.push(LiveItem {
            team1: name1,
            team2: name2,
            score1: m.team1_score,
            score2: m.team2_score,
            status: "Terminé".to_string(),
            live: false,
        });
    }

    (items, journee)
}

fn build_standings(
    classification: &[ChampionshipClassification],
    clubs: &[Club],
    championship: Option<&Championship>,
) -> (String, String, String, Vec<StandingRow>) {
    let division_name = classification
        .iter()
        .find(|c| c.division == "D1")
        .or_else(|| classification.first());

    let (division, journee, rows) = match division_name {
        Some(c) => (
            c.division.clone(),
            Some(c.journee),
            c.teams
                .iter()
                .take(6)
                .map(|t| {
                    let (club, abbr) = team_label(clubs, &t.team_id);
                    StandingRow {
                        position: t.position,
                        club,
                        abbr,
                        played: t.stats.matches,
                        wins: t.stats.wins,
                        losses: t.stats.losses,
                        diff: t.stats.diff,
                        points: t.stats.points,
                        form: t.form.clone(),
                        zone: match t.status {
                            TeamClassificationStatus::Qualified => Zone::Qualif,
                            TeamClassificationStatus::Barrage => Zone::Barrage,
                            TeamClassificationStatus::Relegated => Zone::Releg,
                            TeamClassificationStatus::Safe => Zone::Safe,
                        },
                    }
                })
                .collect(),
        ),
        None => (String::new(), None, Vec::new()),
    };

    let name = championship
        .map(|c| c.name.to_uppercase())
        .unwrap_or_else(|| "CHAMPIONNAT".to_string());
    let title = if division.is_empty() {
        name
    } else {
        format!("{name} — {division}")
    };
    let subtitle = match (journee, championship.and_then(|c| c.current_journee)) {
        (Some(j), _) => format!("Classement après {j} journées"),
        (None, Some(j)) => format!("Classement après {j} journées"),
        (None, None) => "Classement de la saison".to_string(),
    };

    (title, division, subtitle, rows)
}

fn build_events(scheduled: &[Match], clubs: &[Club]) -> Vec<EventCard> {
    scheduled
        .iter()
        .take(3)
        .map(|m| {
            let (name1, _) = team_label(clubs, &m.team1_id);
            let (name2, _) = team_label(clubs, &m.team2_id);
            let (tag, tone, action, href) = match m.competition_type {
                CompetitionType::Championship => (
                    "CHAMPIONNAT",
                    EventTone::Championship,
                    "Voir le championnat",
                    "/championnats",
                ),
                CompetitionType::Tournament => {
                    ("TOURNOI", EventTone::Open, "Voir les tournois", "/tournois")
                }
                CompetitionType::Cup => (
                    "COUPE",
                    EventTone::Federal,
                    "Voir les matchs",
                    "/championnats",
                ),
                CompetitionType::Friendly => (
                    "MATCH AMICAL",
                    EventTone::Federal,
                    "Voir les matchs",
                    "/championnats",
                ),
            };
            let mut details = Vec::new();
            if let Some(j) = m.journee {
                details.push(format!("Journée {j}"));
            }
            if let Some(best_of) = m.best_of {
                details.push(format!("{best_of} manches maximum"));
            }
            if let Some(round) = &m.round {
                details.push(round.clone());
            }
            EventCard {
                tone,
                tag: tag.to_string(),
                title: format!("{name1} vs {name2}"),
                pill: "À VENIR".to_string(),
                date: format_match_datetime(m),
                location: m
                    .location
                    .clone()
                    .unwrap_or_else(|| "Lieu à confirmer".to_string()),
                details,
                action: action.to_string(),
                href: href.to_string(),
            }
        })
        .collect()
}

fn build_top(standings: &[StandingRow]) -> Vec<TopCard> {
    let accents = [Accent::Gold, Accent::Green, Accent::Amber];
    let badges = ["Leader", "Dauphin", "Troisième"];
    let kinds = [BadgeKind::Mvp, BadgeKind::TopScorer, BadgeKind::Revelation];

    standings
        .iter()
        .take(3)
        .enumerate()
        .map(|(i, row)| {
            let played = row.played.max(1);
            TopCard {
                accent: accents[i.min(2)],
                initials: row.abbr.clone(),
                name: row.club.clone(),
                club: format!("{} points au classement", row.points),
                badge: badges[i.min(2)].to_string(),
                badge_kind: kinds[i.min(2)],
                stats: vec![
                    ("Matchs joués".to_string(), row.played.to_string()),
                    (
                        "Victoires".to_string(),
                        format!("{} ({} % de réussite)", row.wins, (row.wins * 100) / played),
                    ),
                    ("Différence de points".to_string(), format_score(row.diff)),
                ],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use palet_shared::{
        ChampionshipTeam, CompetitionStatus, CurrentScores, TeamClassificationStatus, TeamStats,
    };

    fn club(id: Uuid, name: &str, city: &str) -> Club {
        Club {
            id,
            name: name.to_string(),
            nickname: None,
            code: String::new(),
            logo: None,
            address: None,
            city: city.to_string(),
            postal_code: None,
            league: "Vendée".to_string(),
            division: "D1".to_string(),
            status: palet_shared::ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: None,
            contact_phone: None,
            website: None,
            social: Default::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn live_match(team1: Uuid, team2: Uuid) -> Match {
        Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championship,
            journee: Some(12),
            phase: Some("Finale".to_string()),
            round: None,
            team1_id: team1,
            team2_id: team2,
            team1_score: Some(72),
            team2_score: Some(68),
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: Some(24),
            current_scores: Some(CurrentScores {
                team1: 72,
                team2: 68,
                manche: 24,
            }),
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: Some("15h00".to_string()),
            location: Some("Terrain clos de La Roche-sur-Yon".to_string()),
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        }
    }

    #[test]
    fn initials_are_two_uppercase_letters() {
        assert_eq!(initials("La Roche-sur-Yon Palet Club"), "LR");
        assert_eq!(initials("Clos Fontenois"), "CF");
        assert_eq!(initials("Luçon"), "L");
    }

    #[test]
    fn hero_uses_live_match_and_resolves_club_names() {
        let lr = Uuid::new_v4();
        let cf = Uuid::new_v4();
        let clubs = vec![
            club(lr, "La Roche-sur-Yon Palet Club", "La Roche-sur-Yon"),
            club(cf, "Clos Fontenois", "Fontenay"),
        ];
        let m = live_match(lr, cf);

        let hero = build_hero(Some(&m), &clubs, None).expect("hero");

        assert!(hero.live);
        assert_eq!(hero.badge, "LIVE — Manche 24");
        assert_eq!(hero.team1.name, "La Roche-sur-Yon Palet Club");
        assert_eq!(hero.team1.abbr, "LR");
        assert_eq!(hero.team1.score, Some(72));
        assert_eq!(hero.team2.score, Some(68));
        assert!(hero.kicker.contains("FINALE"));
        assert_eq!(hero.info, "Terrain clos de La Roche-sur-Yon");
    }

    #[test]
    fn standings_map_zones_and_stats() {
        let team_id = Uuid::new_v4();
        let clubs = vec![club(team_id, "Luçon Palet Club", "Luçon")];
        let mut team = ChampionshipTeam {
            position: 1,
            team_id,
            stats: TeamStats {
                matches: 12,
                wins: 10,
                draws: 0,
                losses: 2,
                points_for: 500,
                points_against: 416,
                diff: 84,
                points: 30,
                win_rate: 83.3,
            },
            form: vec![MatchResult::Win, MatchResult::Loss],
            status: TeamClassificationStatus::Qualified,
        };
        team.position = 1;
        let classification = vec![ChampionshipClassification {
            division: "D1".to_string(),
            journee: 12,
            updated_at: Utc::now(),
            teams: vec![team],
        }];

        let (title, division, subtitle, rows) = build_standings(&classification, &clubs, None);

        assert_eq!(division, "D1");
        assert!(title.contains("CHAMPIONNAT"));
        assert!(subtitle.contains("12 journées"));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].club, "Luçon Palet Club");
        assert_eq!(rows[0].abbr, "LP");
        assert_eq!(rows[0].points, 30);
        assert_eq!(rows[0].diff, 84);
        assert_eq!(rows[0].zone, Zone::Qualif);
        assert_eq!(rows[0].form.len(), 2);
    }

    #[test]
    fn scheduled_matches_become_upcoming_events() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let clubs = vec![
            club(a, "Challans Sport Palet", "Challans"),
            club(b, "Montaigu PC", "Montaigu"),
        ];
        let mut m = live_match(a, b);
        m.status = MatchStatus::Scheduled;
        m.current_scores = None;

        let events = build_events(&[m], &clubs);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, "Challans Sport Palet vs Montaigu PC");
        assert_eq!(events[0].tag, "CHAMPIONNAT");
        assert_eq!(events[0].pill, "À VENIR");
    }

    #[test]
    fn championship_status_filter_keeps_in_progress_first() {
        let mut pending = Championship {
            id: Uuid::new_v4(),
            name: "D2".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D2".to_string()],
            total_journees: 18,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: palet_shared::ClassificationPrimary::Points,
                secondary: palet_shared::ClassificationSecondary::Diff,
                tertiary: palet_shared::ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let mut d1 = pending.clone();
        d1.name = "D1".to_string();
        d1.divisions = vec!["D1".to_string()];
        d1.status = CompetitionStatus::InProgress;
        pending.status = CompetitionStatus::Completed;

        let picked = pick_championship(&[pending, d1.clone()]).expect("championship");
        assert_eq!(picked.name, "D1");
        assert_eq!(picked.status, CompetitionStatus::InProgress);
    }
}
