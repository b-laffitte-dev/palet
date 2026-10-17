//! Utility functions for Palet Vendéen TDD harness
//!
//! This module provides utility functions for testing Palet Vendéen functionality.

use chrono::{DateTime, Utc};
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationRules,
    Club, CompetitionStatus, Match, MatchStatus, Team, TeamStats,
};
use uuid::Uuid;

// ============================================
// CHAMPIONSHIP UTILITIES
// ============================================

/// Create a championship with default Palet Vendéen rules
pub fn create_championship_with_palet_rules(
    name: &str,
    season: &str,
    league: &str,
    divisions: Vec<String>,
    total_journees: i32,
    organizer_id: Uuid,
) -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: name.to_string(),
        code: Some(format!("{}-{}", name.to_uppercase().replace(' ', "-"), season.replace('/', "-"))),
        season: season.to_string(),
        description: Some(format!("Championnat {} - Saison {}", name, season)),
        league: league.to_string(),
        divisions,
        total_journees,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: ClassificationRules::default_palet(),
        status: CompetitionStatus::Pending,
        current_journee: Some(1),
        current_phase: None,
        organizer_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a championship classification with teams
pub fn create_classification(
    division: &str,
    journee: i32,
    teams: Vec<ChampionshipTeam>,
) -> ChampionshipClassification {
    let mut classification = ChampionshipClassification::new(division.to_string(), journee);
    
    for team in teams {
        classification.add_team(team);
    }
    
    classification
}

/// Sort teams according to Palet Vendéen classification rules
pub fn sort_teams_by_palet_rules(
    mut teams: Vec<ChampionshipTeam>,
    rules: &ClassificationRules,
) -> Vec<ChampionshipTeam> {
    teams.sort_by(|a, b| rules.compare_teams(a, b).reverse());
    
    for (index, team) in teams.iter_mut().enumerate() {
        team.position = (index + 1) as i32;
    }
    
    teams
}

// ============================================
// CLUB UTILITIES
// ============================================

/// Create a club with default values
pub fn create_club(
    name: &str,
    code: &str,
    league: &str,
    division: &str,
) -> Club {
    Club {
        id: Uuid::new_v4(),
        name: name.to_string(),
        nickname: Some(name.chars().take(3).collect()),
        code: code.to_string(),
        logo: None,
        address: Some(format!("Rue de {}, {}", name, league)),
        city: league.to_string(),
        postal_code: None,
        league: league.to_string(),
        division: division.to_string(),
        status: palet_shared::ClubStatus::Active,
        players_count: 0,
        teams_count: 0,
        contact_email: Some(format!("contact@{}.fr", code.to_lowercase())),
        contact_phone: None,
        website: None,
        social: palet_shared::SocialLinks::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ============================================
// TEAM UTILITIES
// ============================================

/// Create a team with default values
pub fn create_team(
    name: &str,
    club_id: Uuid,
    category: &str,
) -> Team {
    Team {
        id: Uuid::new_v4(),
        name: name.to_string(),
        nickname: Some(name.chars().take(3).collect()),
        club_id,
        club_name: None,
        category: category.to_string(),
        division: "D1".to_string(),
        players: vec![],
        captain_id: None,
        coach_id: None,
        status: palet_shared::TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a championship team with stats
pub fn create_championship_team(
    team_id: Uuid,
    position: i32,
    stats: TeamStats,
) -> ChampionshipTeam {
    ChampionshipTeam {
        position,
        team_id,
        stats,
        form: vec![],
        status: palet_shared::TeamClassificationStatus::Safe,
    }
}

// ============================================
// MATCH UTILITIES
// ============================================

/// Create a match with default values
pub fn create_match(
    competition_id: Uuid,
    competition_type: palet_shared::CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
    date: DateTime<Utc>,
    journee: Option<i32>,
) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id,
        competition_type,
        team1_id,
        team2_id,
        team1_name: None,
        team2_name: None,
        team1_score: 0,
        team2_score: 0,
        team1_manches: vec![],
        team2_manches: vec![],
        date,
        location: None,
        journee,
        phase: None,
        round: None,
        best_of: Some(13),
        status: MatchStatus::Scheduled,
        winner: None,
        duration: None,
        referee_id: None,
        table_number: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a completed match with scores
pub fn create_completed_match(
    competition_id: Uuid,
    competition_type: palet_shared::CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
    team1_score: i32,
    team2_score: i32,
    winner: Option<palet_shared::MatchWinner>,
) -> Match {
    let status = if team1_score > 0 || team2_score > 0 {
        MatchStatus::Completed
    } else {
        MatchStatus::Scheduled
    };

    Match {
        id: Uuid::new_v4(),
        competition_id,
        competition_type,
        team1_id,
        team2_id,
        team1_name: None,
        team2_name: None,
        team1_score,
        team2_score,
        team1_manches: vec![],
        team2_manches: vec![],
        date: Utc::now(),
        location: None,
        journee: None,
        phase: None,
        round: None,
        best_of: Some(13),
        status,
        winner,
        duration: None,
        referee_id: None,
        table_number: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ============================================
// TEAM STATS UTILITIES
// ============================================

/// Create team stats with default values
pub fn create_team_stats(
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> TeamStats {
    let diff = points_for - points_against;
    let points = wins * 3 + draws * 1;
    let win_rate = if matches > 0 {
        wins as f64 / matches as f64
    } else {
        0.0
    };

    TeamStats {
        matches,
        wins,
        draws,
        losses,
        points_for,
        points_against,
        diff,
        points,
        win_rate,
    }
}

// ============================================
// TIME UTILITIES
// ============================================

/// Get the current time as DateTime<Utc>
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Get a date in the future
pub fn future_date(days: i64) -> DateTime<Utc> {
    Utc::now() + chrono::Duration::days(days)
}

/// Get a date in the past
pub fn past_date(days: i64) -> DateTime<Utc> {
    Utc::now() - chrono::Duration::days(days)
}

// ============================================
// ID GENERATION UTILITIES
// ============================================

/// Generate a new unique ID
pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

/// Generate multiple unique IDs
pub fn new_ids(count: usize) -> Vec<Uuid> {
    (0..count).map(|_| Uuid::new_v4()).collect()
}

// ============================================
// TEST DATA BUILDERS
// ============================================

/// Builder for creating test championships
pub struct ChampionshipBuilder {
    name: String,
    season: String,
    league: String,
    divisions: Vec<String>,
    total_journees: i32,
    organizer_id: Uuid,
    status: CompetitionStatus,
    current_journee: Option<i32>,
}

impl ChampionshipBuilder {
    pub fn new() -> Self {
        Self {
            name: "Test Championship".to_string(),
            season: "2025-2026".to_string(),
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            organizer_id: Uuid::new_v4(),
            status: CompetitionStatus::Pending,
            current_journee: Some(1),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn season(mut self, season: &str) -> Self {
        self.season = season.to_string();
        self
    }

    pub fn league(mut self, league: &str) -> Self {
        self.league = league.to_string();
        self
    }

    pub fn divisions(mut self, divisions: Vec<String>) -> Self {
        self.divisions = divisions;
        self
    }

    pub fn total_journees(mut self, total_journees: i32) -> Self {
        self.total_journees = total_journees;
        self
    }

    pub fn organizer_id(mut self, organizer_id: Uuid) -> Self {
        self.organizer_id = organizer_id;
        self
    }

    pub fn status(mut self, status: CompetitionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn current_journee(mut self, current_journee: Option<i32>) -> Self {
        self.current_journee = current_journee;
        self
    }

    pub fn build(self) -> Championship {
        Championship {
            id: Uuid::new_v4(),
            name: self.name,
            code: Some(format!("{}-{}", self.name.to_uppercase().replace(' ', "-"), self.season.replace('/', "-"))),
            season: self.season,
            description: Some(format!("Championnat {} - Saison {}", self.name, self.season)),
            league: self.league,
            divisions: self.divisions,
            total_journees: self.total_journees,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: ClassificationRules::default_palet(),
            status: self.status,
            current_journee: self.current_journee,
            current_phase: None,
            organizer_id: self.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Builder for creating test teams
pub struct TeamBuilder {
    name: String,
    club_id: Uuid,
    category: String,
    division: String,
    status: palet_shared::TeamStatus,
}

impl TeamBuilder {
    pub fn new(club_id: Uuid) -> Self {
        Self {
            name: "Test Team".to_string(),
            club_id,
            category: "Sénior".to_string(),
            division: "D1".to_string(),
            status: palet_shared::TeamStatus::Active,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn division(mut self, division: &str) -> Self {
        self.division = division.to_string();
        self
    }

    pub fn status(mut self, status: palet_shared::TeamStatus) -> Self {
        self.status = status;
        self
    }

    pub fn build(self) -> Team {
        Team {
            id: Uuid::new_v4(),
            name: self.name,
            nickname: Some(self.name.chars().take(3).collect()),
            club_id: self.club_id,
            club_name: None,
            category: self.category,
            division: self.division,
            players: vec![],
            captain_id: None,
            coach_id: None,
            status: self.status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
