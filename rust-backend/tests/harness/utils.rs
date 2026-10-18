//! Utility functions for Palet Vendéen TDD tests
//!
//! This module provides utility functions and builders for creating test data.

use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, ClubStatus, CompetitionStatus,
    CompetitionType, CreateChampionshipRequest, CreateClubRequest, CreateMatchRequest,
    CreatePlayerRequest, CreateTeamRequest, MancheScore, Match, MatchResult, MatchStatus,
    MatchWinner, Player, PlayerStatus, SocialLinks, Team, TeamClassificationStatus, TeamStats,
};
use uuid::Uuid;

// ============================================
// CHAMPIONSHIP BUILDER
// ============================================

/// Builder for creating test championships
#[derive(Debug, Clone)]
pub struct ChampionshipBuilder {
    id: Option<Uuid>,
    name: String,
    code: Option<String>,
    season: String,
    description: Option<String>,
    league: String,
    divisions: Vec<String>,
    total_journees: i32,
    journee_duration: Option<String>,
    classification_rules: palet_shared::ClassificationRules,
    status: CompetitionStatus,
    current_journee: Option<i32>,
    current_phase: Option<String>,
    organizer_id: Uuid,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl Default for ChampionshipBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: "Test Championship".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ChampionshipBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn with_season(mut self, season: &str) -> Self {
        self.season = season.to_string();
        self
    }

    pub fn with_league(mut self, league: &str) -> Self {
        self.league = league.to_string();
        self
    }

    pub fn with_divisions(mut self, divisions: Vec<&str>) -> Self {
        self.divisions = divisions.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_total_journees(mut self, total_journees: i32) -> Self {
        self.total_journees = total_journees;
        self
    }

    pub fn with_status(mut self, status: CompetitionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_current_journee(mut self, journee: i32) -> Self {
        self.current_journee = Some(journee);
        self
    }

    pub fn with_classification_rules(
        mut self,
        primary: ClassificationPrimary,
        secondary: ClassificationSecondary,
        tertiary: ClassificationTertiary,
    ) -> Self {
        self.classification_rules = palet_shared::ClassificationRules {
            primary,
            secondary,
            tertiary,
        };
        self
    }

    pub fn build(self) -> Championship {
        Championship {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            name: self.name,
            code: self.code,
            season: self.season,
            description: self.description,
            league: self.league,
            divisions: self.divisions,
            total_journees: self.total_journees,
            journee_duration: self.journee_duration,
            classification_rules: self.classification_rules,
            status: self.status,
            current_journee: self.current_journee,
            current_phase: self.current_phase,
            organizer_id: self.organizer_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn create_request(self) -> CreateChampionshipRequest {
        CreateChampionshipRequest {
            name: self.name,
            code: self.code,
            season: self.season,
            description: self.description,
            league: self.league,
            divisions: self.divisions,
            total_journees: self.total_journees,
            journee_duration: self.journee_duration,
            classification_rules: self.classification_rules.clone(),
            organizer_id: self.organizer_id,
        }
    }
}

// ============================================
// TEAM BUILDER
// ============================================

/// Builder for creating test teams
#[derive(Debug, Clone)]
pub struct TeamBuilder {
    id: Option<Uuid>,
    name: String,
    nickname: Option<String>,
    code: String,
    club_id: Uuid,
    division: String,
    category: String,
    players: Vec<Uuid>,
    captain_id: Option<Uuid>,
    stats: TeamStats,
    status: palet_shared::TeamStatus,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl Default for TeamBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: "Test Team".to_string(),
            nickname: None,
            code: "TT001".to_string(),
            club_id: Uuid::new_v4(),
            division: "D1".to_string(),
            category: "Senior".to_string(),
            players: Vec::new(),
            captain_id: None,
            stats: TeamStats::default(),
            status: palet_shared::TeamStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl TeamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.code = code.to_string();
        self
    }

    pub fn with_club_id(mut self, club_id: Uuid) -> Self {
        self.club_id = club_id;
        self
    }

    pub fn with_division(mut self, division: &str) -> Self {
        self.division = division.to_string();
        self
    }

    pub fn with_players(mut self, player_ids: Vec<Uuid>) -> Self {
        self.players = player_ids;
        self
    }

    pub fn with_captain(mut self, captain_id: Uuid) -> Self {
        self.captain_id = Some(captain_id);
        self
    }

    pub fn build(self) -> Team {
        Team {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            name: self.name,
            nickname: self.nickname,
            code: self.code,
            club_id: self.club_id,
            division: self.division,
            category: self.category,
            players: self.players,
            captain_id: self.captain_id,
            stats: self.stats,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn create_request(self) -> CreateTeamRequest {
        CreateTeamRequest {
            name: self.name,
            nickname: self.nickname,
            code: self.code,
            club_id: self.club_id,
            division: self.division,
            category: self.category,
        }
    }
}

// ============================================
// MATCH BUILDER
// ============================================

/// Builder for creating test matches
#[derive(Debug, Clone)]
pub struct MatchBuilder {
    id: Option<Uuid>,
    competition_id: Uuid,
    competition_type: CompetitionType,
    journee: Option<i32>,
    phase: Option<String>,
    round: Option<String>,
    team1_id: Uuid,
    team2_id: Uuid,
    team1_score: Option<i32>,
    team2_score: Option<i32>,
    winner: Option<MatchWinner>,
    manches: Option<Vec<MancheScore>>,
    best_of: Option<i32>,
    current_manche: Option<i32>,
    current_scores: Option<palet_shared::CurrentScores>,
    status: MatchStatus,
    verified: bool,
    verified_by: Option<Uuid>,
    verified_at: Option<chrono::DateTime<Utc>>,
    date: chrono::DateTime<Utc>,
    time: Option<String>,
    location: Option<String>,
    duration: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
    started_at: Option<chrono::DateTime<Utc>>,
    ended_at: Option<chrono::DateTime<Utc>>,
}

impl Default for MatchBuilder {
    fn default() -> Self {
        Self {
            id: None,
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: None,
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: Some(13), // Default to 13 points for championship
            current_manche: Some(0),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 0,
                team2: 0,
                manche: 0,
            }),
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        }
    }
}

impl MatchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_competition_id(mut self, competition_id: Uuid) -> Self {
        self.competition_id = competition_id;
        self
    }

    pub fn with_competition_type(mut self, competition_type: CompetitionType) -> Self {
        self.competition_type = competition_type;
        self
    }

    pub fn with_journee(mut self, journee: i32) -> Self {
        self.journee = Some(journee);
        self
    }

    pub fn with_phase(mut self, phase: &str) -> Self {
        self.phase = Some(phase.to_string());
        self
    }

    pub fn with_teams(mut self, team1_id: Uuid, team2_id: Uuid) -> Self {
        self.team1_id = team1_id;
        self.team2_id = team2_id;
        self
    }

    pub fn with_scores(mut self, team1_score: i32, team2_score: i32) -> Self {
        self.team1_score = Some(team1_score);
        self.team2_score = Some(team2_score);
        self
    }

    pub fn with_winner(mut self, winner: MatchWinner) -> Self {
        self.winner = Some(winner);
        self
    }

    pub fn with_best_of(mut self, best_of: i32) -> Self {
        self.best_of = Some(best_of);
        self
    }

    pub fn with_status(mut self, status: MatchStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_manches(mut self, manches: Vec<MancheScore>) -> Self {
        self.manches = Some(manches);
        self
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }

    pub fn build(self) -> Match {
        Match {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            competition_id: self.competition_id,
            competition_type: self.competition_type,
            journee: self.journee,
            phase: self.phase,
            round: self.round,
            team1_id: self.team1_id,
            team2_id: self.team2_id,
            team1_score: self.team1_score,
            team2_score: self.team2_score,
            winner: self.winner,
            manches: self.manches,
            best_of: self.best_of,
            current_manche: self.current_manche,
            current_scores: self.current_scores,
            status: self.status,
            verified: self.verified,
            verified_by: self.verified_by,
            verified_at: self.verified_at,
            date: self.date,
            time: self.time,
            location: self.location,
            duration: self.duration,
            created_at: self.created_at,
            updated_at: self.updated_at,
            started_at: self.started_at,
            ended_at: self.ended_at,
        }
    }

    pub fn create_request(self) -> CreateMatchRequest {
        CreateMatchRequest {
            competition_id: self.competition_id,
            competition_type: self.competition_type,
            team1_id: self.team1_id,
            team2_id: self.team2_id,
            date: self.date,
            location: self.location,
            journee: self.journee,
            phase: self.phase,
            round: self.round,
            best_of: self.best_of,
        }
    }
}

// ============================================
// CHAMPIONSHIP TEAM BUILDER
// ============================================

/// Builder for creating test championship teams
pub struct ChampionshipTeamBuilder {
    position: i32,
    team_id: Uuid,
    stats: TeamStats,
    form: Vec<MatchResult>,
    status: TeamClassificationStatus,
}

impl Default for ChampionshipTeamBuilder {
    fn default() -> Self {
        Self {
            position: 1,
            team_id: Uuid::new_v4(),
            stats: TeamStats::default(),
            form: Vec::new(),
            status: TeamClassificationStatus::Qualified,
        }
    }
}

impl ChampionshipTeamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, position: i32) -> Self {
        self.position = position;
        self
    }

    pub fn with_team_id(mut self, team_id: Uuid) -> Self {
        self.team_id = team_id;
        self
    }

    pub fn with_stats(mut self, stats: TeamStats) -> Self {
        self.stats = stats;
        self
    }

    pub fn with_form(mut self, form: Vec<MatchResult>) -> Self {
        self.form = form;
        self
    }

    pub fn with_status(mut self, status: TeamClassificationStatus) -> Self {
        self.status = status;
        self
    }

    pub fn build(self) -> ChampionshipTeam {
        ChampionshipTeam {
            position: self.position,
            team_id: self.team_id,
            stats: self.stats,
            form: self.form,
            status: self.status,
        }
    }
}

// ============================================
// CLASSIFICATION BUILDER
// ============================================

/// Builder for creating test championship classifications
pub struct ClassificationBuilder {
    championship_id: Uuid,
    division: String,
    journee: i32,
    teams: Vec<ChampionshipTeam>,
    updated_at: chrono::DateTime<Utc>,
}

impl Default for ClassificationBuilder {
    fn default() -> Self {
        Self {
            championship_id: Uuid::new_v4(),
            division: "D1".to_string(),
            journee: 1,
            teams: Vec::new(),
            updated_at: Utc::now(),
        }
    }
}

impl ClassificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_championship_id(mut self, championship_id: Uuid) -> Self {
        self.championship_id = championship_id;
        self
    }

    pub fn with_division(mut self, division: &str) -> Self {
        self.division = division.to_string();
        self
    }

    pub fn with_journee(mut self, journee: i32) -> Self {
        self.journee = journee;
        self
    }

    pub fn with_teams(mut self, teams: Vec<ChampionshipTeam>) -> Self {
        self.teams = teams;
        self
    }

    pub fn build(self) -> ChampionshipClassification {
        ChampionshipClassification {
            championship_id: self.championship_id,
            division: self.division,
            journee: self.journee,
            teams: self.teams,
            updated_at: self.updated_at,
        }
    }
}

// ============================================
// TEST DATA FACTORIES
// ============================================

/// Create a test club with default values
pub fn create_test_club() -> Club {
    Club {
        id: Uuid::new_v4(),
        name: "Test Club".to_string(),
        nickname: Some("TC".to_string()),
        code: "TC001".to_string(),
        logo: None,
        address: Some("123 Test Street".to_string()),
        city: "Test City".to_string(),
        postal_code: Some("12345".to_string()),
        league: "Vendée".to_string(),
        division: "D1".to_string(),
        status: ClubStatus::Active,
        players_count: 10,
        teams_count: 2,
        contact_email: Some("contact@testclub.com".to_string()),
        contact_phone: Some("0123456789".to_string()),
        website: Some("https://testclub.com".to_string()),
        social: SocialLinks::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test player with default values
pub fn create_test_player() -> Player {
    Player {
        id: Uuid::new_v4(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        full_name: "John Doe".to_string(),
        alias: Some("JD".to_string()),
        email: "john.doe@test.com".to_string(),
        photo: None,
        status: PlayerStatus::Active,
        birth_date: Utc::now() - chrono::Duration::days(365 * 25),
        age: 25,
        birth_place: Some("Test City".to_string()),
        nationality: "Français".to_string(),
        dominant_hand: palet_shared::DominantHand::Right,
        play_style: Some("Aggressif".to_string()),
        phone: Some("0123456789".to_string()),
        address: Some("123 Test Street".to_string()),
        city: Some("Test City".to_string()),
        postal_code: Some("12345".to_string()),
        club_id: Uuid::new_v4(),
        club_number: Some("TC001".to_string()),
        category: "Senior".to_string(),
        license_number: "LIC001".to_string(),
        license_date: Utc::now(),
        license_status: palet_shared::LicenseStatus::Valid,
        registration_date: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        team_id: None,
    }
}

/// Create a test team with default values
pub fn create_test_team() -> Team {
    Team {
        id: Uuid::new_v4(),
        name: "Test Team".to_string(),
        nickname: Some("TT".to_string()),
        code: "TT001".to_string(),
        club_id: Uuid::new_v4(),
        division: "D1".to_string(),
        category: "Senior".to_string(),
        players: Vec::new(),
        captain_id: None,
        stats: TeamStats::default(),
        status: palet_shared::TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test match with default values
pub fn create_test_match() -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championnat,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id: Uuid::new_v4(),
        team2_id: Uuid::new_v4(),
        team1_score: None,
        team2_score: None,
        winner: None,
        manches: None,
        best_of: Some(13),
        current_manche: Some(0),
        current_scores: Some(palet_shared::CurrentScores {
            team1: 0,
            team2: 0,
            manche: 0,
        }),
        status: MatchStatus::Scheduled,
        verified: false,
        verified_by: None,
        verified_at: None,
        date: Utc::now(),
        time: None,
        location: None,
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create a test championship with default values
pub fn create_test_championship() -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: "Test Championship".to_string(),
        code: Some("TC2025".to_string()),
        season: "2025-2026".to_string(),
        description: Some("Test championship for TDD".to_string()),
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: palet_shared::ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        },
        status: CompetitionStatus::Pending,
        current_journee: None,
        current_phase: None,
        organizer_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test manche score
pub fn create_test_manche(number: i32, team1_score: i32, team2_score: i32) -> MancheScore {
    MancheScore {
        number,
        team1: team1_score,
        team2: team2_score,
        winner: if team1_score > team2_score {
            Some(MatchWinner::Team1)
        } else {
            Some(MatchWinner::Team2)
        },
    }
}

/// Create team stats with specified values
pub fn create_test_team_stats(
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> TeamStats {
    TeamStats {
        matches,
        wins,
        draws,
        losses,
        points_for,
        points_against,
        diff: points_for - points_against,
        points: wins * 3 + draws * 1,
        win_rate: if matches > 0 {
            wins as f64 / matches as f64
        } else {
            0.0
        },
    }
}
