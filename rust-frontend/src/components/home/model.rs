//! View models for the home page, mapped from the API types.
//!
//! The layout follows `docs/maquettes/homepage.png`. Only endpoints that
//! actually exist on the backend are used (`/clubs`, `/matchs`,
//! `/championnats`), so this layer is a pure transformation of their data.

use palet_shared::{
    Championship, ChampionshipClassification, Club, CompetitionType, Division, Match, MatchResult,
    MatchStatus, MatchWinner, TeamClassificationStatus,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::services::api::ApiService;

/// League (département) whose divisions are shown as tabs on the home page.
pub const HOME_LEAGUE: &str = "Deux-Sèvres";

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

/// Une manche jouée, affichée dans le détail du match.
#[derive(Debug, Clone, PartialEq)]
pub struct CarouselManche {
    pub number: i32,
    pub team1: i32,
    pub team2: i32,
}

/// Une carte match dans le carrousel des journées en cours.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CarouselMatch {
    pub id: Uuid,
    pub team1: String,
    pub team2: String,
    pub abbr1: String,
    pub abbr2: String,
    pub score1: Option<i32>,
    pub score2: Option<i32>,
    pub status_label: String,
    pub live: bool,
    pub finished: bool,
    /// Ligne méta de la carte : « Manche 2 · Lieu » ou « 14h30 · Lieu ».
    pub meta: String,
    /// Heure ou manche en cours seule ("14h30", "Manche 2").
    pub when: String,
    pub location: String,
    /// Date du match ("08/11/2026").
    pub date_label: String,
    pub best_of: Option<i32>,
    pub manches: Vec<CarouselManche>,
    /// Nom du vainqueur quand le match est terminé.
    pub winner_name: Option<String>,
}

/// Match sélectionné dans le carrousel, remonté vers la home (pour le hero).
#[derive(Debug, Clone, PartialEq)]
pub struct CarouselSelection {
    pub division: String,
    pub journee: Option<i32>,
    pub m: CarouselMatch,
}

/// Une slide du carrousel : une division (dans une ligue) et les matchs de
/// sa journée en cours.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DivisionSlide {
    pub name: String,
    pub league: String,
    pub label: String,
    pub journee: Option<i32>,
    pub matches: Vec<CarouselMatch>,
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

/// One standings tab: a division (any league) and its classification rows.
#[derive(Clone, PartialEq)]
pub struct DivisionTab {
    /// Division name, e.g. "D1"
    pub name: String,
    /// League (département) of the division, e.g. "Deux-Sèvres"
    pub league: String,
    /// Label shown on the tab: `name`, or `name (league)` when several
    /// leagues share the same division name.
    pub label: String,
    pub journee: Option<i32>,
    pub rows: Vec<StandingRow>,
}

/// Everything the home page needs, loaded once from the API.
#[derive(Clone, Default)]
pub struct HomeData {
    pub hero: Option<HeroMatch>,
    /// Matchs des journées en cours, regroupés par division (carrousel).
    pub slides: Vec<DivisionSlide>,
    pub championship_name: String,
    pub subtitle: String,
    /// One tab per division, all leagues (dynamic, federal space).
    pub tabs: Vec<DivisionTab>,
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
        // Home standings show the department league (Deux-Sèvres); fall back
        // to all championships while the league has none yet. The full list
        // is kept too: the carousel needs the current journee of every league.
        let all_championships = api.get_championships().await.unwrap_or_default();
        let mut championships = api
            .get_championships_by_league(HOME_LEAGUE)
            .await
            .unwrap_or_default();
        if championships.is_empty() {
            championships = all_championships.clone();
        }

        let championship = pick_championship(&championships);
        let classification = match &championship {
            Some(c) => api
                .get_championship_classification(c.id)
                .await
                .unwrap_or_default(),
            None => Vec::new(),
        };
        // Division tabs are dynamic (managed from the federal space) and show
        // ALL divisions, every league; fall back to the championship's own
        // divisions when none are registered.
        let db_divisions = api.get_divisions(None).await.unwrap_or_default();
        let divisions = resolve_divisions(&db_divisions, championship.as_ref(), &classification);

        // One classification per league appearing in the tabs.
        let mut classification_by_league = HashMap::new();
        if let Some(c) = &championship {
            classification_by_league.insert(c.league.clone(), classification.clone());
        }
        for (_, league) in &divisions {
            if !classification_by_league.contains_key(league) {
                let champs = api
                    .get_championships_by_league(league)
                    .await
                    .unwrap_or_default();
                if let Some(c) = pick_championship(&champs) {
                    let classif = api
                        .get_championship_classification(c.id)
                        .await
                        .unwrap_or_default();
                    classification_by_league.insert(league.clone(), classif);
                }
            }
        }

        let hero = build_hero(
            live.first().or(scheduled.first()),
            &clubs,
            championship.as_ref(),
        );
        let mut all_matches: Vec<Match> = Vec::new();
        all_matches.extend(live.iter().cloned());
        all_matches.extend(scheduled.iter().cloned());
        all_matches.extend(completed.iter().cloned());
        let slides = build_carousel(&all_matches, &clubs, &all_championships, &divisions);
        let (championship_name, subtitle, tabs) = build_tabs(
            &divisions,
            &classification_by_league,
            &clubs,
            championship.as_ref(),
        );
        let events = build_events(&scheduled, &clubs);
        let top = build_top(&tabs);

        HomeData {
            hero,
            slides,
            championship_name,
            subtitle,
            tabs,
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

fn hero_scores(m: &Match) -> (Option<i32>, Option<i32>, Option<i32>) {
    match &m.current_scores {
        Some(cs) => (Some(cs.team1), Some(cs.team2), Some(cs.manche)),
        None => (m.team1_score, m.team2_score, m.current_manche),
    }
}

fn hero_badge(live: bool, finished: bool, manche: Option<i32>) -> String {
    if live {
        format!("LIVE — Manche {}", manche.unwrap_or(1))
    } else if finished {
        "TERMINÉ".to_string()
    } else {
        "À VENIR".to_string()
    }
}

fn hero_kicker(m: &Match, championship: Option<&Championship>) -> String {
    let base = if m.status == MatchStatus::InProgress {
        "MATCH EN DIRECT".to_string()
    } else if m.status == MatchStatus::Completed {
        "DERNIER RÉSULTAT".to_string()
    } else {
        "PROCHAIN MATCH".to_string()
    };
    let kicker = match (&m.phase, &m.round, m.journee) {
        (Some(phase), _, _) => phase.to_uppercase(),
        (_, Some(round), _) => round.to_uppercase(),
        (_, _, Some(j)) => format!("JOURNÉE {j}"),
        _ => base,
    };
    match championship {
        Some(c) => format!("{} — {}", kicker, c.name.to_uppercase()),
        None => kicker,
    }
}

fn team_side(clubs: &[Club], team_id: &Uuid, subtitle: &str, score: Option<i32>) -> TeamSide {
    let (name, abbr) = team_label(clubs, team_id);
    TeamSide {
        name,
        abbr,
        subtitle: subtitle.to_string(),
        score,
    }
}

fn build_hero(
    m: Option<&Match>,
    clubs: &[Club],
    championship: Option<&Championship>,
) -> Option<HeroMatch> {
    let m = m?;
    let (score1, score2, manche) = hero_scores(m);
    let live = m.status == MatchStatus::InProgress;
    let finished = m.status == MatchStatus::Completed;

    Some(HeroMatch {
        kicker: hero_kicker(m, championship),
        subtitle: format_match_datetime(m),
        info: m
            .location
            .clone()
            .unwrap_or_else(|| "Terrain clos municipal".to_string()),
        live,
        finished,
        badge: hero_badge(live, finished, manche),
        team1: team_side(clubs, &m.team1_id, "Équipe 1", score1),
        team2: team_side(clubs, &m.team2_id, "Équipe 2", score2),
    })
}

// ============================================
// CAROUSEL — matchs des journées en cours, par division
// ============================================

/// Journée en cours par ligue : `current_journee` de chaque championnat en
/// cours (plusieurs ligues peuvent avoir une journée en cours en même temps).
fn current_journees_by_league(championships: &[Championship]) -> HashMap<String, i32> {
    let mut by_league = HashMap::new();
    for c in championships {
        if c.status == palet_shared::CompetitionStatus::InProgress {
            if let Some(j) = c.current_journee {
                by_league.entry(c.league.clone()).or_insert(j);
            }
        }
    }
    by_league
}

/// Fallback quand aucun championnat n'est en cours : la plus grande journée
/// ayant encore des matchs non terminés.
fn fallback_journee(matches: &[&Match]) -> Option<i32> {
    matches
        .iter()
        .filter(|m| !matches!(m.status, MatchStatus::Completed | MatchStatus::Cancelled))
        .filter_map(|m| m.journee)
        .max()
}

/// Ordre d'affichage dans une slide : live d'abord, puis à venir, puis terminés.
fn match_rank(m: &Match) -> u8 {
    match m.status {
        MatchStatus::InProgress => 0,
        MatchStatus::Scheduled => 1,
        MatchStatus::Completed => 2,
        MatchStatus::Disputed => 3,
        MatchStatus::Postponed => 4,
        MatchStatus::Cancelled => 5,
    }
}

fn carousel_status_label(m: &Match) -> (String, bool) {
    let (label, live) = match m.status {
        MatchStatus::InProgress => ("EN DIRECT", true),
        MatchStatus::Scheduled => ("À VENIR", false),
        MatchStatus::Completed => ("TERMINÉ", false),
        MatchStatus::Postponed => ("REPORTÉ", false),
        MatchStatus::Cancelled => ("ANNULÉ", false),
        MatchStatus::Disputed => ("CONTESTÉ", false),
    };
    (label.to_string(), live)
}

/// Scores affichés sur la carte : le score live (`current_scores`) uniquement
/// quand le match est en cours — l'API renvoie `current_scores = 0-0` même
/// pour les matchs planifiés/terminés, ce qui masquerait le vrai score.
fn carousel_scores(m: &Match) -> (Option<i32>, Option<i32>) {
    match (&m.current_scores, &m.status) {
        (Some(cs), MatchStatus::InProgress) => (Some(cs.team1), Some(cs.team2)),
        _ => (m.team1_score, m.team2_score),
    }
}

/// Partie « quand » (manche en cours, ou heure) et lieu d'un match.
fn carousel_when_location(m: &Match) -> (String, String) {
    let when = if m.status == MatchStatus::InProgress {
        m.current_scores
            .as_ref()
            .map(|cs| cs.manche)
            .or(m.current_manche)
            .filter(|n| *n > 0)
            .map(|n| format!("Manche {n}"))
            .unwrap_or_default()
    } else {
        // `time` n'est pas toujours renseigné : retombe sur l'heure de `date`.
        m.time
            .as_ref()
            .filter(|t| !t.is_empty())
            .cloned()
            .unwrap_or_else(|| m.date.format("%Hh%M").to_string())
    };
    let location = m
        .location
        .clone()
        .filter(|l| !l.is_empty())
        .unwrap_or_default();
    (when, location)
}

fn carousel_match(m: &Match, clubs: &[Club]) -> CarouselMatch {
    let (name1, abbr1) = team_label(clubs, &m.team1_id);
    let (name2, abbr2) = team_label(clubs, &m.team2_id);
    let (status_label, live) = carousel_status_label(m);
    let (score1, score2) = carousel_scores(m);
    let (when, location) = carousel_when_location(m);
    let meta = match (when.is_empty(), location.is_empty()) {
        (true, true) => String::new(),
        (false, true) => when.clone(),
        (true, false) => location.clone(),
        (false, false) => format!("{when} · {location}"),
    };
    let winner_name = match (&m.winner, &m.status) {
        (Some(MatchWinner::Team1), MatchStatus::Completed) => Some(name1.clone()),
        (Some(MatchWinner::Team2), MatchStatus::Completed) => Some(name2.clone()),
        (Some(MatchWinner::Draw), MatchStatus::Completed) => Some("Match nul".to_string()),
        _ => None,
    };
    CarouselMatch {
        id: m.id,
        team1: name1,
        team2: name2,
        abbr1,
        abbr2,
        score1,
        score2,
        status_label,
        live,
        finished: m.status == MatchStatus::Completed,
        meta,
        when,
        location,
        date_label: m.date.format("%d/%m/%Y").to_string(),
        best_of: m.best_of,
        manches: m
            .manches
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|s| CarouselManche {
                number: s.number,
                team1: s.team1,
                team2: s.team2,
            })
            .collect(),
        winner_name,
    }
}

/// Kicker du hero pour un match sélectionné dans le carrousel.
fn selection_kicker(sel: &CarouselSelection) -> String {
    match sel.journee {
        Some(j) => format!("{} — JOURNÉE {j}", sel.division.to_uppercase()),
        None => sel.division.to_uppercase(),
    }
}

/// Construit le [`HeroMatch`] affiché dans le hero quand on clique sur une
/// carte du carrousel (même forme que le hero alimenté par l'API).
pub fn hero_from_carousel(sel: &CarouselSelection) -> HeroMatch {
    let m = &sel.m;
    let subtitle = if m.when.is_empty() {
        m.date_label.clone()
    } else {
        format!("{} · {}", m.date_label, m.when)
    };
    HeroMatch {
        kicker: selection_kicker(sel),
        subtitle,
        info: if m.location.is_empty() {
            "Terrain clos municipal".to_string()
        } else {
            m.location.clone()
        },
        live: m.live,
        finished: m.finished,
        badge: m.status_label.clone(),
        team1: TeamSide {
            name: m.team1.clone(),
            abbr: m.abbr1.clone(),
            subtitle: "Équipe 1".to_string(),
            score: m.score1,
        },
        team2: TeamSide {
            name: m.team2.clone(),
            abbr: m.abbr2.clone(),
            subtitle: "Équipe 2".to_string(),
            score: m.score2,
        },
    }
}

/// Division et ligue d'un match, déduites du club de l'équipe 1 (les matchs de
/// championnat opposent deux clubs d'une même division).
fn match_division(clubs: &[Club], m: &Match) -> (String, String) {
    match clubs.iter().find(|c| c.id == m.team1_id) {
        Some(c) => (c.division.clone(), c.league.clone()),
        None => ("Divers".to_string(), String::new()),
    }
}

/// Construit les slides du carrousel : une par division ayant des matchs sur
/// les journées en cours (toutes ligues confondues), ordonnée comme les
/// divisions enregistrées dans l'espace fédéral.
fn build_carousel(
    matches: &[Match],
    clubs: &[Club],
    championships: &[Championship],
    divisions_order: &[(String, String)],
) -> Vec<DivisionSlide> {
    let journees = current_journees_by_league(championships);
    let fallback = fallback_journee(&matches.iter().collect::<Vec<_>>());
    let active: HashSet<i32> = if journees.is_empty() {
        fallback.into_iter().collect()
    } else {
        journees.values().copied().collect()
    };
    if active.is_empty() {
        return Vec::new();
    }

    // Grouper les matchs des journées en cours par (division, ligue).
    let mut groups: HashMap<(String, String), Vec<&Match>> = HashMap::new();
    for m in matches {
        if m.journee.is_some_and(|j| active.contains(&j)) {
            groups.entry(match_division(clubs, m)).or_default().push(m);
        }
    }
    if groups.is_empty() {
        return Vec::new();
    }

    let keys: Vec<(String, String)> = groups.keys().cloned().collect();
    let labels = division_labels(&keys);

    let mut slides: Vec<DivisionSlide> = keys
        .into_iter()
        .zip(labels)
        .map(|(key, label)| {
            let mut ms = groups[&key].clone();
            ms.sort_by(|a, b| {
                match_rank(a)
                    .cmp(&match_rank(b))
                    .then_with(|| a.date.cmp(&b.date))
            });
            DivisionSlide {
                journee: journees.get(&key.1).copied().or(fallback),
                name: key.0.clone(),
                league: key.1.clone(),
                label,
                matches: ms.iter().map(|m| carousel_match(m, clubs)).collect(),
            }
        })
        .collect();

    // Ordre des slides : position des divisions enregistrées, puis le reste
    // (ex. "Divers") par ligue puis nom.
    slides.sort_by(|a, b| {
        let pos = |s: &DivisionSlide| {
            divisions_order
                .iter()
                .position(|(n, l)| *n == s.name && *l == s.league)
        };
        match (pos(a), pos(b)) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => (&a.league, &a.name).cmp(&(&b.league, &b.name)),
        }
    });
    slides
}

fn standing_zone(status: &TeamClassificationStatus) -> Zone {
    match status {
        TeamClassificationStatus::Qualified => Zone::Qualif,
        TeamClassificationStatus::Barrage => Zone::Barrage,
        TeamClassificationStatus::Relegated => Zone::Releg,
        TeamClassificationStatus::Safe => Zone::Safe,
    }
}

fn standing_row(t: &palet_shared::ChampionshipTeam, clubs: &[Club]) -> StandingRow {
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
        zone: standing_zone(&t.status),
    }
}

/// Divisions for the home tabs as `(name, league)` pairs, in tab order.
///
/// All dynamic divisions registered in the federal space win (they are
/// already ordered by `position`, across every league); otherwise fall back
/// to the championship's own divisions, then to whatever divisions exist in
/// the classification.
fn resolve_divisions(
    db_divisions: &[Division],
    championship: Option<&Championship>,
    classification: &[ChampionshipClassification],
) -> Vec<(String, String)> {
    if !db_divisions.is_empty() {
        return db_divisions
            .iter()
            .map(|d| (d.name.clone(), d.league.clone()))
            .collect();
    }
    let league = championship.map(|c| c.league.clone()).unwrap_or_default();
    if let Some(c) = championship {
        if !c.divisions.is_empty() {
            return c
                .divisions
                .iter()
                .map(|n| (n.clone(), league.clone()))
                .collect();
        }
    }
    classification
        .iter()
        .map(|c| (c.division.clone(), league.clone()))
        .collect()
}

/// Tab label per division: the name alone, or `name (league)` when several
/// leagues share the same division name (keeps tabs distinguishable).
fn division_labels(divisions: &[(String, String)]) -> Vec<String> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for (name, _) in divisions {
        *counts.entry(name.as_str()).or_default() += 1;
    }
    divisions
        .iter()
        .map(|(name, league)| {
            if counts[name.as_str()] > 1 {
                format!("{name} ({league})")
            } else {
                name.clone()
            }
        })
        .collect()
}

/// Build one tab per division (all leagues); the classification of a division
/// is looked up in its own league. A division without classification yet gets
/// an empty `rows` (the UI shows a placeholder).
fn build_tabs(
    divisions: &[(String, String)],
    classification_by_league: &HashMap<String, Vec<ChampionshipClassification>>,
    clubs: &[Club],
    championship: Option<&Championship>,
) -> (String, String, Vec<DivisionTab>) {
    let labels = division_labels(divisions);
    let tabs: Vec<DivisionTab> = divisions
        .iter()
        .zip(labels)
        .map(|((name, league), label)| {
            let classif = classification_by_league
                .get(league)
                .and_then(|cs| cs.iter().find(|c| &c.division == name));
            DivisionTab {
                name: name.clone(),
                league: league.clone(),
                label,
                journee: classif.map(|c| c.journee),
                rows: classif
                    .map(|c| {
                        c.teams
                            .iter()
                            .take(6)
                            .map(|t| standing_row(t, clubs))
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect();

    let title = championship
        .map(|c| c.name.to_uppercase())
        .unwrap_or_else(|| "CHAMPIONNAT".to_string());
    let journee = tabs
        .iter()
        .find_map(|t| t.journee)
        .or_else(|| championship.and_then(|c| c.current_journee));
    let subtitle = match journee {
        Some(j) => format!("Classement après {j} journées"),
        None => "Classement de la saison".to_string(),
    };

    (title, subtitle, tabs)
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

fn build_top(tabs: &[DivisionTab]) -> Vec<TopCard> {
    let accents = [Accent::Gold, Accent::Green, Accent::Amber];
    let badges = ["Leader", "Dauphin", "Troisième"];
    let kinds = [BadgeKind::Mvp, BadgeKind::TopScorer, BadgeKind::Revelation];

    // Podium of the first division that has a classification.
    let rows: &[StandingRow] = tabs
        .iter()
        .find(|t| !t.rows.is_empty())
        .or(tabs.first())
        .map(|t| t.rows.as_slice())
        .unwrap_or(&[]);

    rows.iter()
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
        ChampionshipTeam, CompetitionStatus, CurrentScores, MancheScore, MatchWinner,
        TeamClassificationStatus, TeamStats,
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

    fn division(name: &str, position: i32) -> Division {
        division_in(name, position, HOME_LEAGUE)
    }

    fn division_in(name: &str, position: i32, league: &str) -> Division {
        Division {
            id: Uuid::new_v4(),
            name: name.to_string(),
            league: league.to_string(),
            season: "2026-2027".to_string(),
            position,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn classification_of(
        division_name: &str,
        journee: i32,
        team_id: Uuid,
    ) -> ChampionshipClassification {
        ChampionshipClassification {
            division: division_name.to_string(),
            journee,
            updated_at: Utc::now(),
            teams: vec![ChampionshipTeam {
                position: 1,
                team_id,
                stats: TeamStats {
                    matches: 2,
                    wins: 2,
                    draws: 0,
                    losses: 0,
                    points_for: 26,
                    points_against: 10,
                    diff: 16,
                    points: 6,
                    win_rate: 100.0,
                },
                form: vec![MatchResult::Win],
                status: TeamClassificationStatus::Qualified,
            }],
        }
    }

    fn championship_fixture(divisions: Vec<String>) -> Championship {
        Championship {
            id: Uuid::new_v4(),
            name: "Championnat Deux-Sèvres".to_string(),
            code: None,
            season: "2026-2027".to_string(),
            description: None,
            league: HOME_LEAGUE.to_string(),
            divisions,
            total_journees: 14,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: palet_shared::ClassificationPrimary::Points,
                secondary: palet_shared::ClassificationSecondary::Diff,
                tertiary: palet_shared::ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::InProgress,
            current_journee: Some(3),
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn standings_map_zones_and_stats() {
        let team_id = Uuid::new_v4();
        let clubs = vec![club(team_id, "Luçon Palet Club", "Luçon")];
        let team = ChampionshipTeam {
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
        let classification = vec![ChampionshipClassification {
            division: "D1".to_string(),
            journee: 12,
            updated_at: Utc::now(),
            teams: vec![team],
        }];

        let divisions = resolve_divisions(&[], None, &classification);
        let mut by_league = HashMap::new();
        by_league.insert(String::new(), classification.clone());
        let (title, subtitle, tabs) = build_tabs(&divisions, &by_league, &clubs, None);

        assert!(title.contains("CHAMPIONNAT"));
        assert!(subtitle.contains("12 journées"));
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs[0].name, "D1");
        assert_eq!(tabs[0].label, "D1");
        assert_eq!(tabs[0].journee, Some(12));
        let rows = &tabs[0].rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].club, "Luçon Palet Club");
        assert_eq!(rows[0].abbr, "LP");
        assert_eq!(rows[0].points, 30);
        assert_eq!(rows[0].diff, 84);
        assert_eq!(rows[0].zone, Zone::Qualif);
        assert_eq!(rows[0].form.len(), 2);
    }

    #[test]
    fn tabs_follow_dynamic_divisions_and_allow_empty_classification() {
        let team_id = Uuid::new_v4();
        let clubs = vec![club(team_id, "Luçon Palet Club", "Luçon")];
        let classification = vec![ChampionshipClassification {
            division: "D1".to_string(),
            journee: 3,
            updated_at: Utc::now(),
            teams: vec![ChampionshipTeam {
                position: 1,
                team_id,
                stats: TeamStats {
                    matches: 2,
                    wins: 2,
                    draws: 0,
                    losses: 0,
                    points_for: 26,
                    points_against: 10,
                    diff: 16,
                    points: 6,
                    win_rate: 100.0,
                },
                form: vec![MatchResult::Win, MatchResult::Win],
                status: TeamClassificationStatus::Qualified,
            }],
        }];
        let db = vec![division("D1", 1), division("D2", 2), division("D3", 3)];

        let divisions = resolve_divisions(&db, None, &classification);
        assert_eq!(
            divisions,
            vec![
                ("D1".to_string(), HOME_LEAGUE.to_string()),
                ("D2".to_string(), HOME_LEAGUE.to_string()),
                ("D3".to_string(), HOME_LEAGUE.to_string()),
            ]
        );

        let mut by_league = HashMap::new();
        by_league.insert(HOME_LEAGUE.to_string(), classification.clone());
        let (_, _, tabs) = build_tabs(&divisions, &by_league, &clubs, None);

        assert_eq!(tabs.len(), 3);
        assert_eq!(tabs[0].name, "D1");
        assert_eq!(tabs[0].rows.len(), 1);
        assert!(tabs[1].rows.is_empty());
        assert_eq!(tabs[1].journee, None);
        assert_eq!(tabs[2].name, "D3");
    }

    #[test]
    fn tabs_look_up_classification_in_their_own_league() {
        let lucon = Uuid::new_v4();
        let challans = Uuid::new_v4();
        let clubs = vec![
            club(lucon, "Luçon Palet Club", "Luçon"),
            club(challans, "Challans Sport Palet", "Challans"),
        ];
        let db = vec![
            division_in("D1", 1, HOME_LEAGUE),
            division_in("D1", 1, "Vendée"),
        ];
        let mut by_league: HashMap<String, Vec<ChampionshipClassification>> = HashMap::new();
        by_league.insert(
            HOME_LEAGUE.to_string(),
            vec![classification_of("D1", 7, lucon)],
        );
        by_league.insert(
            "Vendée".to_string(),
            vec![classification_of("D1", 9, challans)],
        );

        let divisions = resolve_divisions(&db, None, &[]);
        let (_, _, tabs) = build_tabs(&divisions, &by_league, &clubs, None);

        assert_eq!(tabs.len(), 2);
        // Same division name in two leagues: the label disambiguates.
        assert_eq!(tabs[0].label, format!("D1 ({HOME_LEAGUE})"));
        assert_eq!(tabs[1].label, "D1 (Vendée)");
        assert_eq!(tabs[0].rows[0].club, "Luçon Palet Club");
        assert_eq!(tabs[0].journee, Some(7));
        assert_eq!(tabs[1].rows[0].club, "Challans Sport Palet");
        assert_eq!(tabs[1].journee, Some(9));
    }

    #[test]
    fn division_labels_add_league_only_on_name_collision() {
        let divisions = vec![
            ("D1".to_string(), HOME_LEAGUE.to_string()),
            ("D1".to_string(), "Vendée".to_string()),
            ("D2".to_string(), HOME_LEAGUE.to_string()),
        ];

        assert_eq!(
            division_labels(&divisions),
            vec![
                format!("D1 ({HOME_LEAGUE})"),
                "D1 (Vendée)".to_string(),
                "D2".to_string(),
            ]
        );
    }

    #[test]
    fn resolve_divisions_prefers_db_then_championship_then_classification() {
        let classification = vec![ChampionshipClassification {
            division: "D9".to_string(),
            journee: 1,
            updated_at: Utc::now(),
            teams: vec![],
        }];
        let championship = championship_fixture(vec!["D1".to_string(), "D2".to_string()]);

        assert_eq!(
            resolve_divisions(&[], None, &classification),
            vec![("D9".to_string(), String::new())]
        );
        assert_eq!(
            resolve_divisions(&[], Some(&championship), &classification),
            vec![
                ("D1".to_string(), HOME_LEAGUE.to_string()),
                ("D2".to_string(), HOME_LEAGUE.to_string()),
            ]
        );
        let db = vec![division("D5", 1)];
        assert_eq!(
            resolve_divisions(&db, Some(&championship), &classification),
            vec![("D5".to_string(), HOME_LEAGUE.to_string())]
        );
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

    // ===== Carrousel des journées en cours =====

    fn club_in(id: Uuid, name: &str, division: &str, league: &str) -> Club {
        let mut c = club(id, name, "ville");
        c.division = division.to_string();
        c.league = league.to_string();
        c
    }

    fn match_fixture(team1: Uuid, team2: Uuid, journee: i32, status: MatchStatus) -> Match {
        let mut m = live_match(team1, team2);
        m.journee = Some(journee);
        m.status = status.clone();
        // Les vrais matchs ont `time = None` : l'heure est dans `date`.
        m.time = None;
        m.date = chrono::DateTime::parse_from_rfc3339("2026-11-08T14:30:00Z")
            .expect("date")
            .with_timezone(&Utc);
        match &status {
            MatchStatus::InProgress => {
                m.manches = Some(vec![
                    MancheScore {
                        number: 1,
                        team1: 6,
                        team2: 4,
                        winner: None,
                    },
                    MancheScore {
                        number: 2,
                        team1: 72,
                        team2: 68,
                        winner: None,
                    },
                ]);
            }
            MatchStatus::Scheduled => {
                // L'API renvoie current_scores à 0 même sans manche jouée.
                m.current_scores = Some(CurrentScores {
                    team1: 0,
                    team2: 0,
                    manche: 0,
                });
                m.current_manche = Some(0);
                m.team1_score = None;
                m.team2_score = None;
            }
            MatchStatus::Completed => {
                m.current_scores = Some(CurrentScores {
                    team1: 0,
                    team2: 0,
                    manche: 0,
                });
                m.current_manche = Some(0);
                m.team1_score = Some(13);
                m.team2_score = Some(8);
                m.winner = Some(MatchWinner::Team1);
            }
            _ => {
                m.current_scores = None;
                m.current_manche = None;
            }
        }
        m
    }

    fn championship_in(league: &str, journee: i32) -> Championship {
        let mut c = championship_fixture(vec!["D1".to_string()]);
        c.league = league.to_string();
        c.current_journee = Some(journee);
        c.status = CompetitionStatus::InProgress;
        c
    }

    struct CarouselFixture {
        clubs: Vec<Club>,
        matchs: Vec<Match>,
        championships: Vec<Championship>,
        order: Vec<(String, String)>,
    }

    fn carousel_fixture() -> CarouselFixture {
        let herbiers = Uuid::new_v4();
        let challans = Uuid::new_v4();
        let lucon = Uuid::new_v4();
        let airvault = Uuid::new_v4();
        let cerizay = Uuid::new_v4();
        let niort = Uuid::new_v4();
        let clubs = vec![
            club_in(herbiers, "Les Herbiers Palet", "D1", "Vendée"),
            club_in(challans, "Challans Sport Palet", "D1", "Vendée"),
            club_in(lucon, "Luçon Palet Club", "D1", "Vendée"),
            club_in(airvault, "Airvault Palet Club", "D2", HOME_LEAGUE),
            club_in(cerizay, "Cerizay Palet Club", "D2", HOME_LEAGUE),
            club_in(niort, "Niort Palet Club", "D1", HOME_LEAGUE),
        ];
        let matchs = vec![
            // Journée 7 en cours en Vendée : 1 live + 1 à venir.
            match_fixture(challans, herbiers, 7, MatchStatus::InProgress),
            match_fixture(lucon, herbiers, 7, MatchStatus::Scheduled),
            // Journée 3 en cours dans les Deux-Sèvres.
            match_fixture(airvault, cerizay, 3, MatchStatus::Scheduled),
            // Hors journées en cours : J8 (à venir) et J6 (terminée) en Vendée.
            match_fixture(herbiers, challans, 8, MatchStatus::Scheduled),
            match_fixture(herbiers, lucon, 6, MatchStatus::Completed),
        ];
        let championships = vec![
            championship_in("Vendée", 7),
            championship_in(HOME_LEAGUE, 3),
        ];
        let order = vec![
            ("D1".to_string(), HOME_LEAGUE.to_string()),
            ("D2".to_string(), HOME_LEAGUE.to_string()),
            ("D1".to_string(), "Vendée".to_string()),
        ];
        CarouselFixture {
            clubs,
            matchs,
            championships,
            order,
        }
    }

    #[test]
    fn carousel_groupe_par_division_et_filtre_les_journees_en_cours() {
        let f = carousel_fixture();

        let slides = build_carousel(&f.matchs, &f.clubs, &f.championships, &f.order);

        // J8 et J6 sont exclus : seules les journées en cours (7 Vendée, 3 79).
        assert_eq!(slides.len(), 2, "slides: {slides:?}");
        // Ordre imposé par `order` : D2 Deux-Sèvres (position 1) avant D1 Vendée (position 2).
        assert_eq!(slides[0].name, "D2");
        assert_eq!(slides[0].league, HOME_LEAGUE);
        assert_eq!(slides[0].journee, Some(3));
        assert_eq!(slides[0].matches.len(), 1);
        assert_eq!(slides[1].name, "D1");
        assert_eq!(slides[1].league, "Vendée");
        assert_eq!(slides[1].journee, Some(7));
        assert_eq!(slides[1].matches.len(), 2);
        // Labels sans collision de nom.
        assert_eq!(slides[0].label, "D2");
        assert_eq!(slides[1].label, "D1");
    }

    #[test]
    fn carousel_trie_live_puis_scheduled_puis_termine_avec_labels() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let challans = f.clubs[1].id;
        let lucon = f.clubs[2].id;
        let done = match_fixture(herbiers, lucon, 7, MatchStatus::Completed);
        let live = match_fixture(challans, herbiers, 7, MatchStatus::InProgress);
        let id_live = live.id;
        let matchs = vec![
            done,
            match_fixture(lucon, herbiers, 7, MatchStatus::Scheduled),
            live,
        ];

        let slides = build_carousel(&matchs, &f.clubs, &f.championships, &f.order);

        assert_eq!(slides.len(), 1);
        let matches = &slides[0].matches;
        assert_eq!(matches.len(), 3);
        assert!(matches[0].live);
        assert_eq!(matches[0].status_label, "EN DIRECT");
        assert!(
            matches[0].meta.contains("Manche 24"),
            "meta: {}",
            matches[0].meta
        );
        assert_eq!(matches[0].score1, Some(72));
        assert_eq!(matches[0].score2, Some(68));
        assert_eq!(matches[0].abbr1, "CS");
        // Détail du match sélectionné : id, manches, date, best_of.
        assert_eq!(matches[0].id, id_live);
        assert_eq!(matches[0].manches.len(), 2);
        assert_eq!(matches[0].manches[0].number, 1);
        assert_eq!(matches[0].manches[0].team1, 6);
        assert_eq!(matches[0].date_label, "08/11/2026");
        assert_eq!(matches[0].best_of, Some(13));
        assert_eq!(matches[1].status_label, "À VENIR");
        assert!(!matches[1].live);
        // current_scores à 0 ne doit pas masquer l'absence de score.
        assert_eq!(matches[1].score1, None);
        assert_eq!(matches[1].score2, None);
        // Heure retombe sur `date` quand `time` est absent.
        assert!(
            matches[1].meta.contains("14h30"),
            "meta: {}",
            matches[1].meta
        );
        assert_eq!(matches[2].status_label, "TERMINÉ");
        assert_eq!(matches[2].score1, Some(13));
        assert_eq!(
            matches[2].winner_name.as_deref(),
            Some("Les Herbiers Palet")
        );
    }

    #[test]
    fn carousel_fallback_sur_journee_max_quand_aucun_championnat_en_cours() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let challans = f.clubs[1].id;
        let lucon = f.clubs[2].id;
        let matchs = vec![
            match_fixture(herbiers, lucon, 5, MatchStatus::Scheduled),
            match_fixture(challans, herbiers, 8, MatchStatus::InProgress),
            match_fixture(herbiers, challans, 6, MatchStatus::Completed),
        ];
        let mut pending = championship_fixture(vec!["D1".to_string()]);
        pending.status = CompetitionStatus::Pending;
        pending.current_journee = None;

        let slides = build_carousel(&matchs, &f.clubs, &[pending], &f.order);

        // Fallback : plus grande journée avec des matchs non terminés = 8.
        assert_eq!(slides.len(), 1);
        assert_eq!(slides[0].journee, Some(8));
        assert_eq!(slides[0].matches.len(), 1);
        assert!(slides[0].matches[0].live);
    }

    #[test]
    fn carousel_desambigue_les_labels_en_cas_de_collision_de_nom() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let niort = f.clubs[5].id;
        let airvault = f.clubs[3].id;
        // Même journée en cours (7) dans les deux ligues, et même nom de
        // division (D1) des deux côtés.
        let matchs = vec![
            match_fixture(herbiers, f.clubs[2].id, 7, MatchStatus::Scheduled),
            match_fixture(niort, airvault, 7, MatchStatus::Scheduled),
        ];
        let championships = vec![
            championship_in("Vendée", 7),
            championship_in(HOME_LEAGUE, 7),
        ];

        let slides = build_carousel(&matchs, &f.clubs, &championships, &f.order);

        assert_eq!(slides.len(), 2);
        let labels: Vec<&str> = slides.iter().map(|s| s.label.as_str()).collect();
        assert!(labels.contains(&"D1 (Vendée)"), "labels: {labels:?}");
        assert!(
            labels.contains(&format!("D1 ({HOME_LEAGUE})").as_str()),
            "labels: {labels:?}"
        );
    }

    #[test]
    fn carousel_est_vide_quand_aucun_match_sur_les_journees_en_cours() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let challans = f.clubs[1].id;
        let matchs = vec![match_fixture(herbiers, challans, 6, MatchStatus::Completed)];

        let slides = build_carousel(&matchs, &f.clubs, &f.championships, &f.order);

        assert!(slides.is_empty(), "slides: {slides:?}");
    }

    #[test]
    fn carousel_match_expose_when_location_et_finished() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let challans = f.clubs[1].id;
        let lucon = f.clubs[2].id;
        let matchs = vec![
            match_fixture(challans, herbiers, 7, MatchStatus::InProgress),
            match_fixture(lucon, herbiers, 7, MatchStatus::Scheduled),
            match_fixture(herbiers, lucon, 7, MatchStatus::Completed),
        ];

        let slides = build_carousel(&matchs, &f.clubs, &f.championships, &f.order);
        let matches = &slides[0].matches;

        assert_eq!(matches[0].when, "Manche 24");
        assert_eq!(matches[0].location, "Terrain clos de La Roche-sur-Yon");
        assert!(matches[0].meta.contains("Manche 24"));
        assert!(!matches[0].finished);
        assert_eq!(matches[1].when, "14h30");
        assert!(!matches[1].finished);
        assert!(matches[2].finished);
    }

    #[test]
    fn hero_from_carousel_reprend_la_forme_du_hero() {
        let f = carousel_fixture();
        let challans = f.clubs[1].id;
        let herbiers = f.clubs[0].id;
        let matchs = vec![match_fixture(
            challans,
            herbiers,
            7,
            MatchStatus::InProgress,
        )];
        let slides = build_carousel(&matchs, &f.clubs, &f.championships, &f.order);
        let m = slides[0].matches[0].clone();
        let sel = CarouselSelection {
            division: slides[0].label.clone(),
            journee: slides[0].journee,
            m,
        };

        let hero = hero_from_carousel(&sel);

        assert_eq!(hero.kicker, "D1 — JOURNÉE 7");
        assert_eq!(hero.subtitle, "08/11/2026 · Manche 24");
        assert_eq!(hero.info, "Terrain clos de La Roche-sur-Yon");
        assert!(hero.live);
        assert!(!hero.finished);
        assert_eq!(hero.badge, "EN DIRECT");
        assert_eq!(hero.team1.name, "Challans Sport Palet");
        assert_eq!(hero.team1.abbr, "CS");
        assert_eq!(hero.team1.score, Some(72));
        assert_eq!(hero.team2.name, "Les Herbiers Palet");
        assert_eq!(hero.team2.score, Some(68));
    }

    #[test]
    fn hero_from_carousel_match_termine_sans_lieu() {
        let f = carousel_fixture();
        let herbiers = f.clubs[0].id;
        let lucon = f.clubs[2].id;
        let mut m = match_fixture(herbiers, lucon, 7, MatchStatus::Completed);
        m.location = None;
        let matchs = vec![m];
        let slides = build_carousel(&matchs, &f.clubs, &f.championships, &f.order);
        let sel = CarouselSelection {
            division: "D1".to_string(),
            journee: None,
            m: slides[0].matches[0].clone(),
        };

        let hero = hero_from_carousel(&sel);

        assert_eq!(hero.kicker, "D1");
        assert!(hero.finished);
        assert_eq!(hero.badge, "TERMINÉ");
        assert_eq!(hero.info, "Terrain clos municipal");
        assert_eq!(hero.team1.score, Some(13));
    }
}
