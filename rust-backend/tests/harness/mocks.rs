//! Mock implementations for Palet Vendéen TDD harness
//!
//! This module provides mock implementations of services and repositories for testing.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationRules, ClassificationSecondary, ClassificationTertiary, Club, ClubStatus,
    CompetitionCategory, CompetitionFormat, CompetitionMaterial, CompetitionStatus,
    CompetitionType, CreateChampionshipRequest, CreateClubRequest, CreateMatchRequest,
    CreateTournamentRequest, Federation, Federations, LicenseStatus, MancheScore, Match,
    MatchResult, MatchStatus, MatchWinner, Player, PlayerStatus, PointsType, SocialLinks,
    Team, TeamClassificationStatus, TeamStats, Tournament, TournamentEligibility,
    TournamentMatch, TournamentPhase, TournamentPhaseType, UpdateChampionshipRequest,
    UpdateClubRequest, UpdateMatchRequest, User, UserRole, UserStatus,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

use crate::repositories::{
    ChampionshipRepository, ClubRepository, MatchRepository, PlayerRepository,
    TeamRepository, TournamentRepository,
};
use crate::services::{ChampionshipService, MatchService, PaletRulesService};

// ============================================
// MOCK DATABASE ERROR
// ============================================

/// Error type for mock database operations
#[derive(Debug, Error, PartialEq)]
pub enum MockDatabaseError {
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Schema initialization failed")]
    SchemaInitializationFailed,
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

// ============================================
// MOCK DATABASE STATE
// ============================================

/// State for the mock database
#[derive(Debug, Default, Clone)]
pub struct MockDatabaseState {
    pub federations: HashMap<Uuid, Federation>,
    pub clubs: HashMap<Uuid, Club>,
    pub players: HashMap<Uuid, Player>,
    pub teams: HashMap<Uuid, Team>,
    pub championships: HashMap<Uuid, Championship>,
    pub championship_classifications: HashMap<String, ChampionshipClassification>,
    pub matches: HashMap<Uuid, Match>,
    pub tournament_matches: HashMap<Uuid, TournamentMatch>,
    pub tournaments: HashMap<Uuid, Tournament>,
    pub users: HashMap<Uuid, User>,
    pub is_connected: bool,
    pub schema_initialized: bool,
}

// ============================================
// MOCK DATABASE
// ============================================

/// Mock database implementation for testing
#[derive(Debug, Clone, Default)]
pub struct MockDatabase {
    state: Arc<RwLock<MockDatabaseState>>,
}

impl MockDatabase {
    /// Create a new mock database
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(MockDatabaseState {
                federations: HashMap::new(),
                clubs: HashMap::new(),
                players: HashMap::new(),
                teams: HashMap::new(),
                championships: HashMap::new(),
                championship_classifications: HashMap::new(),
                matches: HashMap::new(),
                tournament_matches: HashMap::new(),
                tournaments: HashMap::new(),
                users: HashMap::new(),
                is_connected: true,
                schema_initialized: false,
            })),
        }
    }

    /// Set connection status
    pub fn set_connected(&self, connected: bool) {
        let mut state = self.state.write().unwrap();
        state.is_connected = connected;
    }

    /// Set schema initialization status
    pub fn set_schema_initialized(&self, initialized: bool) {
        let mut state = self.state.write().unwrap();
        state.schema_initialized = initialized;
    }

    /// Add a federation
    pub fn add_federation(&self, federation: Federation) {
        let mut state = self.state.write().unwrap();
        state.federations.insert(federation.id, federation);
    }

    /// Add a club
    pub fn add_club(&self, club: Club) {
        let mut state = self.state.write().unwrap();
        state.clubs.insert(club.id, club);
    }

    /// Add a player
    pub fn add_player(&self, player: Player) {
        let mut state = self.state.write().unwrap();
        state.players.insert(player.id, player);
    }

    /// Add a team
    pub fn add_team(&self, team: Team) {
        let mut state = self.state.write().unwrap();
        state.teams.insert(team.id, team);
    }

    /// Add a championship
    pub fn add_championship(&self, championship: Championship) {
        let mut state = self.state.write().unwrap();
        state.championships.insert(championship.id, championship);
    }

    /// Add a championship classification
    pub fn add_championship_classification(&self, key: String, classification: ChampionshipClassification) {
        let mut state = self.state.write().unwrap();
        state
            .championship_classifications
            .insert(key, classification);
    }

    /// Add a match
    pub fn add_match(&self, match_obj: Match) {
        let mut state = self.state.write().unwrap();
        state.matches.insert(match_obj.id, match_obj);
    }

    /// Add a tournament match
    pub fn add_tournament_match(&self, match_obj: TournamentMatch) {
        let mut state = self.state.write().unwrap();
        state.tournament_matches.insert(match_obj.id, match_obj);
    }

    /// Add a tournament
    pub fn add_tournament(&self, tournament: Tournament) {
        let mut state = self.state.write().unwrap();
        state.tournaments.insert(tournament.id, tournament);
    }

    /// Add a user
    pub fn add_user(&self, user: User) {
        let mut state = self.state.write().unwrap();
        state.users.insert(user.id, user);
    }

    /// Check if the database connection is alive
    pub async fn check_connection(&self) -> bool {
        let state = self.state.read().unwrap();
        state.is_connected
    }

    /// Initialize database schema
    pub async fn initialize(&self) -> Result<(), MockDatabaseError> {
        let mut state = self.state.write().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        state.schema_initialized = true;
        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        if !state.schema_initialized {
            return Err(MockDatabaseError::SchemaInitializationFailed);
        }
        Ok(())
    }
}

// ============================================
// MOCK CHAMPIONSHIP REPOSITORY
// ============================================

/// Mock implementation of ChampionshipRepository
#[derive(Debug, Clone)]
pub struct MockChampionshipRepository {
    db: MockDatabase,
}

#[async_trait]
impl ChampionshipRepository for MockChampionshipRepository {
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        let championship = Championship {
            id: Uuid::new_v4(),
            name: request.name.clone(),
            code: request.code.clone(),
            season: request.season.clone(),
            description: request.description.clone(),
            league: request.league.clone(),
            divisions: request.divisions.clone(),
            total_journees: request.total_journees,
            journee_duration: request.journee_duration.clone(),
            classification_rules: request.classification_rules.clone(),
            status: CompetitionStatus::Pending,
            current_journee: Some(1),
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.db.add_championship(championship.clone());
        Ok(championship)
    }

    async fn get_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.championships.get(&championship_id).cloned())
    }

    async fn get_championships(&self) -> Result<Vec<Championship>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.championships.values().cloned().collect())
    }

    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        if let Some(championship) = state.championships.get_mut(&championship_id) {
            if let Some(name) = request.name {
                championship.name = name;
            }
            if let Some(code) = request.code {
                championship.code = Some(code);
            }
            if let Some(season) = request.season {
                championship.season = season;
            }
            if let Some(description) = request.description {
                championship.description = Some(description);
            }
            if let Some(league) = request.league {
                championship.league = league;
            }
            if let Some(divisions) = request.divisions {
                championship.divisions = divisions;
            }
            if let Some(total_journees) = request.total_journees {
                championship.total_journees = total_journees;
            }
            if let Some(journee_duration) = request.journee_duration {
                championship.journee_duration = Some(journee_duration);
            }
            if let Some(classification_rules) = request.classification_rules {
                championship.classification_rules = classification_rules;
            }
            if let Some(status) = request.status {
                championship.status = status;
            }
            if let Some(current_journee) = request.current_journee {
                championship.current_journee = Some(current_journee);
            }
            if let Some(current_phase) = request.current_phase {
                championship.current_phase = Some(current_phase);
            }
            championship.updated_at = Utc::now();

            Ok(Some(championship.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_championship(&self, championship_id: Uuid) -> Result<bool, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.championships.remove(&championship_id).is_some())
    }

    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .championships
            .values()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .championships
            .values()
            .filter(|c| c.season == season)
            .cloned()
            .collect())
    }

    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .championships
            .values()
            .filter(|c| c.status == status)
            .cloned()
            .collect())
    }

    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        journee: i32,
    ) -> Result<Option<ChampionshipClassification>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        let key = format!("{}-{}-{}", championship_id, division, journee);
        Ok(state.championship_classifications.get(&key).cloned())
    }

    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        journee: i32,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        let key = format!("{}-{}-{}", championship_id, division, journee);
        state
            .championship_classifications
            .insert(key, classification.clone());

        Ok(classification)
    }

    async fn advance_championship_journee(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        if let Some(championship) = state.championships.get_mut(&championship_id) {
            if let Some(current_journee) = championship.current_journee {
                if current_journee < championship.total_journees {
                    championship.current_journee = Some(current_journee + 1);
                    championship.updated_at = Utc::now();

                    // If this is the last journee, mark as completed
                    if championship.current_journee == Some(championship.total_journees) {
                        championship.status = CompetitionStatus::Completed;
                    }

                    Ok(Some(championship.clone()))
                } else {
                    Ok(None)
                }
            } else {
                championship.current_journee = Some(1);
                championship.updated_at = Utc::now();
                Ok(Some(championship.clone()))
            }
        } else {
            Ok(None)
        }
    }
}

// ============================================
// MOCK CLUB REPOSITORY
// ============================================

/// Mock implementation of ClubRepository
#[derive(Debug, Clone)]
pub struct MockClubRepository {
    db: MockDatabase,
}

#[async_trait]
impl ClubRepository for MockClubRepository {
    async fn create_club(
        &self,
        request: CreateClubRequest,
    ) -> Result<Club, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        let club = Club {
            id: Uuid::new_v4(),
            name: request.name.clone(),
            nickname: request.nickname.clone(),
            code: request.code.clone(),
            logo: request.logo.clone(),
            address: request.address.clone(),
            city: request.city.clone(),
            postal_code: request.postal_code.clone(),
            league: request.league.clone(),
            division: request.division.clone(),
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: request.contact_email.clone(),
            contact_phone: request.contact_phone.clone(),
            website: request.website.clone(),
            social: SocialLinks::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.db.add_club(club.clone());
        Ok(club)
    }

    async fn get_club(&self, club_id: Uuid) -> Result<Option<Club>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.clubs.get(&club_id).cloned())
    }

    async fn get_clubs(&self) -> Result<Vec<Club>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.clubs.values().cloned().collect())
    }

    async fn update_club(
        &self,
        club_id: Uuid,
        request: UpdateClubRequest,
    ) -> Result<Option<Club>, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        if let Some(club) = state.clubs.get_mut(&club_id) {
            if let Some(name) = request.name {
                club.name = name;
            }
            if let Some(nickname) = request.nickname {
                club.nickname = Some(nickname);
            }
            if let Some(code) = request.code {
                club.code = code;
            }
            if let Some(logo) = request.logo {
                club.logo = Some(logo);
            }
            if let Some(address) = request.address {
                club.address = Some(address);
            }
            if let Some(city) = request.city {
                club.city = city;
            }
            if let Some(postal_code) = request.postal_code {
                club.postal_code = Some(postal_code);
            }
            if let Some(league) = request.league {
                club.league = league;
            }
            if let Some(division) = request.division {
                club.division = division;
            }
            if let Some(status) = request.status {
                club.status = status;
            }
            if let Some(contact_email) = request.contact_email {
                club.contact_email = Some(contact_email);
            }
            if let Some(contact_phone) = request.contact_phone {
                club.contact_phone = Some(contact_phone);
            }
            if let Some(website) = request.website {
                club.website = Some(website);
            }
            club.updated_at = Utc::now();

            Ok(Some(club.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_club(&self, club_id: Uuid) -> Result<bool, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.clubs.remove(&club_id).is_some())
    }

    async fn get_clubs_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Club>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .clubs
            .values()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_division(
        &self,
        division: String,
    ) -> Result<Vec<Club>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .clubs
            .values()
            .filter(|c| c.division == division)
            .cloned()
            .collect())
    }
}

// ============================================
// MOCK MATCH REPOSITORY
// ============================================

/// Mock implementation of MatchRepository
#[derive(Debug, Clone)]
pub struct MockMatchRepository {
    db: MockDatabase,
}

#[async_trait]
impl MatchRepository for MockMatchRepository {
    async fn create_match(
        &self,
        request: CreateMatchRequest,
    ) -> Result<Match, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_name: None,
            team2_name: None,
            team1_score: 0,
            team2_score: 0,
            team1_manches: vec![],
            team2_manches: vec![],
            date: request.date,
            location: request.location.clone(),
            journee: request.journee,
            phase: request.phase.clone(),
            round: request.round.clone(),
            best_of: request.best_of,
            status: MatchStatus::Scheduled,
            winner: None,
            duration: None,
            referee_id: None,
            table_number: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.db.add_match(match_obj.clone());
        Ok(match_obj)
    }

    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.matches.get(&match_id).cloned())
    }

    async fn get_matches(&self) -> Result<Vec<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.matches.values().cloned().collect())
    }

    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        if let Some(match_obj) = state.matches.get_mut(&match_id) {
            if let Some(team1_score) = request.team1_score {
                match_obj.team1_score = team1_score;
            }
            if let Some(team2_score) = request.team2_score {
                match_obj.team2_score = team2_score;
            }
            if let Some(team1_manches) = request.team1_manches {
                match_obj.team1_manches = team1_manches;
            }
            if let Some(team2_manches) = request.team2_manches {
                match_obj.team2_manches = team2_manches;
            }
            if let Some(status) = request.status {
                match_obj.status = status;
            }
            if let Some(winner) = request.winner {
                match_obj.winner = Some(winner);
            }
            if let Some(location) = request.location {
                match_obj.location = Some(location);
            }
            if let Some(date) = request.date {
                match_obj.date = date;
            }
            if let Some(duration) = request.duration {
                match_obj.duration = Some(duration);
            }
            match_obj.updated_at = Utc::now();

            Ok(Some(match_obj.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_match(&self, match_id: Uuid) -> Result<bool, crate::db::DatabaseError> {
        let mut state = self.db.state.write().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state.matches.remove(&match_id).is_some())
    }

    async fn get_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .matches
            .values()
            .filter(|m| m.competition_id == championship_id && m.competition_type == CompetitionType::Championship)
            .cloned()
            .collect())
    }

    async fn get_matches_by_journee(
        &self,
        championship_id: Uuid,
        journee: i32,
    ) -> Result<Vec<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .matches
            .values()
            .filter(|m| {
                m.competition_id == championship_id
                    && m.journee == Some(journee)
                    && m.competition_type == CompetitionType::Championship
            })
            .cloned()
            .collect())
    }

    async fn get_matches_by_team(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .matches
            .values()
            .filter(|m| m.team1_id == team_id || m.team2_id == team_id)
            .cloned()
            .collect())
    }

    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, crate::db::DatabaseError> {
        let state = self.db.state.read().unwrap();
        if !state.is_connected {
            return Err(crate::db::DatabaseError::ConnectionFailed);
        }

        Ok(state
            .matches
            .values()
            .filter(|m| m.status == status)
            .cloned()
            .collect())
    }
}

// ============================================
// MOCK PALET RULES SERVICE
// ============================================

/// Mock implementation of PaletRulesService
#[derive(Debug, Clone, Default)]
pub struct MockPaletRulesService;

#[async_trait]
impl PaletRulesService for MockPaletRulesService {
    fn get_match_points_target(
        &self,
        competition_type: CompetitionType,
        phase: Option<String>,
    ) -> i32 {
        match competition_type {
            CompetitionType::Championship => 13,
            CompetitionType::Tournament => {
                if let Some(phase) = phase {
                    if phase.contains("final") || phase.contains("Finale") {
                        15
                    } else {
                        13
                    }
                } else {
                    13
                }
            }
            _ => 13,
        }
    }

    fn validate_manche_score(
        &self,
        team1_score: i32,
        team2_score: i32,
        _manche_number: i32,
    ) -> Result<(), palet_shared::PaletRulesError> {
        if team1_score < 0 || team2_score < 0 {
            return Err(palet_shared::PaletRulesError::InvalidScore(
                "Scores cannot be negative".to_string(),
            ));
        }
        if team1_score > 15 || team2_score > 15 {
            return Err(palet_shared::PaletRulesError::InvalidScore(
                "Score exceeds maximum of 15".to_string(),
            ));
        }
        Ok(())
    }

    fn calculate_manche_winner(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Option<MatchWinner> {
        if team1_score > team2_score {
            Some(MatchWinner::Team1)
        } else if team2_score > team1_score {
            Some(MatchWinner::Team2)
        } else {
            None
        }
    }

    fn is_match_completed(
        &self,
        team1_score: i32,
        team2_score: i32,
        best_of: i32,
    ) -> bool {
        let target = self.get_match_points_target(CompetitionType::Championship, None);
        team1_score >= target || team2_score >= target
    }

    fn determine_match_winner(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Option<MatchWinner> {
        if team1_score > team2_score {
            Some(MatchWinner::Team1)
        } else if team2_score > team1_score {
            Some(MatchWinner::Team2)
        } else {
            None
        }
    }

    fn get_championship_classification_rules(&self) -> ClassificationRules {
        ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        }
    }

    fn calculate_team_classification(
        &self,
        mut teams: Vec<ChampionshipTeam>,
    ) -> Vec<ChampionshipTeam> {
        let rules = self.get_championship_classification_rules();
        teams.sort_by(|a, b| rules.compare_teams(a, b).reverse());
        for (index, team) in teams.iter_mut().enumerate() {
            team.position = (index + 1) as i32;
        }
        teams
    }

    fn get_win_points(&self) -> i32 {
        3
    }

    fn get_draw_points(&self) -> i32 {
        1
    }

    fn get_loss_points(&self) -> i32 {
        0
    }

    fn calculate_team_stats(
        &self,
        matches: Vec<Match>,
        team_id: Uuid,
    ) -> TeamStats {
        let mut stats = TeamStats::new();

        for match_obj in matches {
            if match_obj.team1_id == team_id || match_obj.team2_id == team_id {
                stats.matches += 1;

                let is_team1 = match_obj.team1_id == team_id;
                let team_score = if is_team1 {
                    match_obj.team1_score
                } else {
                    match_obj.team2_score
                };
                let opponent_score = if is_team1 {
                    match_obj.team2_score
                } else {
                    match_obj.team1_score
                };

                stats.points_for += team_score;
                stats.points_against += opponent_score;
                stats.diff = stats.points_for - stats.points_against;

                match match_obj.winner {
                    Some(MatchWinner::Team1) if is_team1 => {
                        stats.wins += 1;
                        stats.points += self.get_win_points();
                    }
                    Some(MatchWinner::Team2) if !is_team1 => {
                        stats.wins += 1;
                        stats.points += self.get_win_points();
                    }
                    Some(MatchWinner::Team1) if !is_team1 => {
                        stats.losses += 1;
                        stats.points += self.get_loss_points();
                    }
                    Some(MatchWinner::Team2) if is_team1 => {
                        stats.losses += 1;
                        stats.points += self.get_loss_points();
                    }
                    None => {
                        stats.draws += 1;
                        stats.points += self.get_draw_points();
                    }
                    _ => {}
                }
            }
        }

        if stats.matches > 0 {
            stats.win_rate = stats.wins as f64 / stats.matches as f64;
        }

        stats
    }

    fn validate_championship_rules(
        &self,
        championship: &Championship,
    ) -> Result<(), palet_shared::PaletRulesError> {
        if championship.divisions.is_empty() {
            return Err(palet_shared::PaletRulesError::InvalidConfiguration(
                "Championship must have at least one division".to_string(),
            ));
        }
        if championship.total_journees <= 0 {
            return Err(palet_shared::PaletRulesError::InvalidConfiguration(
                "Championship must have at least one journee".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_match_rules(&self, _match_obj: &Match) -> Result<(), palet_shared::PaletRulesError> {
        // Mock validation always succeeds
        Ok(())
    }
}
