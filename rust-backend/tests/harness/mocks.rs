//! Mock implementations for Palet Vendéen TDD tests
//!
//! This module provides mock implementations of repositories and services for testing
//! without requiring a real PostgreSQL connection.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, ClubStatus, CompetitionStatus,
    CompetitionType, CreateChampionshipRequest, CreateClubRequest, CreateMatchRequest,
    CreatePlayerRequest, CreateTeamRequest, MancheScore, Match, MatchStatus, MatchWinner,
    Player, PlayerStatus, Team, TeamClassificationStatus, TeamStats, UpdateChampionshipRequest,
    UpdateClubRequest, UpdateMatchRequest, UpdatePlayerRequest, UpdateTeamRequest,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// MOCK CHAMPIONSHIP REPOSITORY
// ============================================

/// Mock implementation of ChampionshipRepository for testing
#[derive(Clone, Default)]
pub struct MockChampionshipRepository {
    pub championships: Vec<Championship>,
    pub classifications: Vec<ChampionshipClassification>,
    pub should_fail: bool,
}

impl MockChampionshipRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_championships(championships: Vec<Championship>) -> Self {
        Self {
            championships,
            ..Default::default()
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_championship(&mut self, championship: Championship) {
        self.championships.push(championship);
    }

    pub fn find_championship_mut(&mut self, championship_id: Uuid) -> Option<&mut Championship> {
        self.championships.iter_mut().find(|c| c.id == championship_id)
    }
}

#[async_trait]
impl super::super::repositories::championship_repository::ChampionshipRepository 
    for MockChampionshipRepository
{
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let championship = Championship {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code,
            season: request.season,
            description: request.description,
            league: request.league,
            divisions: request.divisions,
            total_journees: request.total_journees,
            journee_duration: request.journee_duration,
            classification_rules: request.classification_rules,
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(championship)
    }

    async fn get_championship(&self, championship_id: Uuid) -> Result<Option<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.championships.iter().find(|c| c.id == championship_id).cloned())
    }

    async fn get_championships(&self) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.championships.clone())
    }

    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
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

    async fn delete_championship(&self, championship_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let index = self.championships.iter().position(|c| c.id == championship_id);
        Ok(index.is_some())
    }

    async fn get_championships_by_league(&self, league: String) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_championships_by_season(&self, season: String) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.season == season)
            .cloned()
            .collect())
    }

    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.status == status)
            .cloned()
            .collect())
    }

    async fn get_championships_by_organizer(&self, organizer_id: Uuid) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.organizer_id == organizer_id)
            .cloned()
            .collect())
    }

    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            championship.status = CompetitionStatus::InProgress;
            championship.current_journee = Some(1);
            championship.updated_at = Utc::now();
            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            let current_journee = championship.current_journee.unwrap_or(0);
            let next_journee = current_journee + 1;

            if next_journee > championship.total_journees {
                return Err(DatabaseError::ValidationError(format!(
                    "Cannot proceed to journee {}: championship only has {} journees",
                    next_journee, championship.total_journees
                )));
            }

            championship.current_journee = Some(next_journee);
            championship.updated_at = Utc::now();

            // If this is the last journee, mark as completed
            if next_journee == championship.total_journees {
                championship.status = CompetitionStatus::Completed;
                championship.current_phase = Some("finished".to_string());
            }

            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn complete_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            championship.status = CompetitionStatus::Completed;
            championship.current_phase = Some("finished".to_string());
            championship.updated_at = Utc::now();
            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let classifications = if let Some(div) = division {
            self.classifications
                .iter()
                .filter(|c| c.division == div)
                .cloned()
                .collect()
        } else {
            self.classifications
                .iter()
                .filter(|c| c.championship_id == championship_id)
                .cloned()
                .collect()
        };

        Ok(classifications)
    }

    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        // Find and update existing classification or add new one
        if let Some(existing) = self
            .classifications
            .iter_mut()
            .find(|c| c.championship_id == championship_id && c.division == division && c.journee == classification.journee)
        {
            *existing = classification.clone();
        } else {
            self.classifications.push(classification.clone());
        }

        Ok(classification)
    }

    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(())
    }

    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(())
    }

    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        // For mock, return empty vector
        Ok(Vec::new())
    }

    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        // For mock, always return true
        Ok(true)
    }
}

// ============================================
// MOCK MATCH REPOSITORY
// ============================================

/// Mock implementation of MatchRepository for testing
#[derive(Clone, Default)]
pub struct MockMatchRepository {
    pub matches: Vec<Match>,
    pub should_fail: bool,
}

impl MockMatchRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_matches(matches: Vec<Match>) -> Self {
        Self {
            matches,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_match(&mut self, match_obj: Match) {
        self.matches.push(match_obj);
    }

    pub fn find_match_mut(&mut self, match_id: Uuid) -> Option<&mut Match> {
        self.matches.iter_mut().find(|m| m.id == match_id)
    }
}

#[async_trait]
impl super::super::repositories::match_repository::MatchRepository for MockMatchRepository {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            journee: request.journee,
            phase: request.phase,
            round: request.round,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: request.best_of,
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
            date: request.date,
            time: None,
            location: request.location,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        Ok(match_obj)
    }

    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.matches.iter().find(|m| m.id == match_id).cloned())
    }

    async fn get_matches(&self) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.matches.clone())
    }

    async fn get_matches_by_competition(&self, competition_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.competition_id == competition_id)
            .cloned()
            .collect())
    }

    async fn get_matches_by_status(&self, status: MatchStatus) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.status == status)
            .cloned()
            .collect())
    }

    async fn update_match(&self, match_id: Uuid, request: UpdateMatchRequest) -> Result<Option<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            if let Some(team1_score) = request.team1_score {
                match_obj.team1_score = Some(team1_score);
            }
            if let Some(team2_score) = request.team2_score {
                match_obj.team2_score = Some(team2_score);
            }
            if let Some(status) = request.status {
                match_obj.status = status;
            }
            if let Some(winner) = request.winner {
                match_obj.winner = Some(winner);
            }
            if let Some(verified) = request.verified {
                match_obj.verified = verified;
            }
            match_obj.updated_at = Utc::now();

            Ok(Some(match_obj.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_match(&self, match_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let index = self.matches.iter().position(|m| m.id == match_id);
        Ok(index.is_some())
    }

    async fn start_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.status = MatchStatus::InProgress;
            match_obj.started_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn complete_match(&self, match_id: Uuid, winner: MatchWinner) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.status = MatchStatus::Completed;
            match_obj.winner = Some(winner);
            match_obj.ended_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn add_manche(&self, match_id: Uuid, manche: MancheScore) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            let mut manches = match_obj.manches.clone().unwrap_or_default();

            // Check if manche number already exists
            if let Some(existing) = manches.iter_mut().find(|m| m.number == manche.number) {
                *existing = manche;
            } else {
                manches.push(manche);
            }
            manches.sort_by_key(|m| m.number);

            let team1_total: i32 = manches.iter().map(|m| m.team1).sum();
            let team2_total: i32 = manches.iter().map(|m| m.team2).sum();
            let current_manche = manche.number;

            match_obj.manches = Some(manches);
            match_obj.current_manche = Some(current_manche);
            match_obj.current_scores = Some(palet_shared::CurrentScores {
                team1: team1_total,
                team2: team2_total,
                manche: current_manche,
            });
            match_obj.team1_score = Some(team1_total);
            match_obj.team2_score = Some(team2_total);
            match_obj.updated_at = Utc::now();

            // Check if match is completed (13 or 15 points)
            let best_of = match_obj.best_of.unwrap_or(13);
            if team1_total >= best_of || team2_total >= best_of {
                match_obj.status = MatchStatus::Completed;
                match_obj.winner = if team1_total >= best_of {
                    Some(MatchWinner::Team1)
                } else {
                    Some(MatchWinner::Team2)
                };
                match_obj.ended_at = Some(Utc::now());
            }

            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn update_manche(
        &self,
        match_id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            if let Some(manches) = &mut match_obj.manches {
                if let Some(manche) = manches.iter_mut().find(|m| m.number == manche_number) {
                    manche.team1 = team1_score;
                    manche.team2 = team2_score;
                }
            }

            // Recalculate scores
            let team1_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team1).sum());
            let team2_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team2).sum());

            match_obj.team1_score = Some(team1_total);
            match_obj.team2_score = Some(team2_total);
            match_obj.updated_at = Utc::now();

            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.verified = true;
            match_obj.verified_by = Some(verifier_id);
            match_obj.verified_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.verified = false;
            match_obj.verified_by = None;
            match_obj.verified_at = None;
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn get_matches_by_date_range(
        &self,
        _start_date: chrono::DateTime<Utc>,
        _end_date: chrono::DateTime<Utc>,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.matches.clone())
    }

    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.team1_id == team_id || m.team2_id == team_id)
            .cloned()
            .collect())
    }

    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.journee == Some(journee) && m.competition_id == competition_id)
            .cloned()
            .collect())
    }

    async fn get_championship_matches(&self, championship_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        self.get_matches_by_competition(championship_id).await
    }

    async fn get_completed_matches_by_championship(&self, championship_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.competition_id == championship_id && m.status == MatchStatus::Completed)
            .cloned()
            .collect())
    }
}

// ============================================
// MOCK CLUB REPOSITORY
// ============================================

/// Mock implementation of ClubRepository for testing
#[derive(Clone, Default)]
pub struct MockClubRepository {
    pub clubs: Vec<Club>,
    pub should_fail: bool,
}

impl MockClubRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_clubs(clubs: Vec<Club>) -> Self {
        Self {
            clubs,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_club(&mut self, club: Club) {
        self.clubs.push(club);
    }

    pub fn find_club_mut(&mut self, club_id: Uuid) -> Option<&mut Club> {
        self.clubs.iter_mut().find(|c| c.id == club_id)
    }
}

#[async_trait]
impl super::super::repositories::club_repository::ClubRepository for MockClubRepository {
    async fn create_club(&self, request: CreateClubRequest) -> Result<Club, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let club = Club {
            id: Uuid::new_v4(),
            name: request.name,
            nickname: request.nickname,
            code: request.code,
            logo: None,
            address: None,
            city: request.city,
            postal_code: request.postal_code,
            league: request.league,
            division: request.division,
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: request.contact_email,
            contact_phone: request.contact_phone,
            website: request.website,
            social: palet_shared::SocialLinks::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(club)
    }

    async fn get_club(&self, club_id: Uuid) -> Result<Option<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.clubs.iter().find(|c| c.id == club_id).cloned())
    }

    async fn get_clubs(&self) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.clubs.clone())
    }

    async fn update_club(&self, club_id: Uuid, request: UpdateClubRequest) -> Result<Option<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            if let Some(name) = request.name {
                club.name = name;
            }
            if let Some(nickname) = request.nickname {
                club.nickname = Some(nickname);
            }
            if let Some(code) = request.code {
                club.code = code;
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

    async fn delete_club(&self, club_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let index = self.clubs.iter().position(|c| c.id == club_id);
        Ok(index.is_some())
    }

    async fn get_clubs_by_league(&self, league: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_division(&self, division: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.division == division)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_status(&self, status: ClubStatus) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.status == status)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_federation(&self, _federation_id: Uuid) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.clubs.clone())
    }

    async fn search_clubs(&self, query: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let query_lower = query.to_lowercase();
        Ok(self
            .clubs
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.code.to_lowercase().contains(&query_lower)
                    || c.city.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect())
    }
}

// ============================================
// MOCK TEAM REPOSITORY
// ============================================

/// Mock implementation of TeamRepository for testing
#[derive(Clone, Default)]
pub struct MockTeamRepository {
    pub teams: Vec<Team>,
    pub should_fail: bool,
}

impl MockTeamRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_teams(teams: Vec<Team>) -> Self {
        Self {
            teams,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    pub fn find_team_mut(&mut self, team_id: Uuid) -> Option<&mut Team> {
        self.teams.iter_mut().find(|t| t.id == team_id)
    }
}

#[async_trait]
impl super::super::repositories::team_repository::TeamRepository for MockTeamRepository {
    async fn create_team(&self, request: CreateTeamRequest) -> Result<Team, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let team = Team {
            id: Uuid::new_v4(),
            name: request.name,
            nickname: request.nickname,
            code: request.code,
            club_id: request.club_id,
            division: request.division,
            category: request.category,
            players: Vec::new(),
            captain_id: None,
            stats: palet_shared::TeamStats::default(),
            status: palet_shared::TeamStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(team)
    }

    async fn get_team(&self, team_id: Uuid) -> Result<Option<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.teams.iter().find(|t| t.id == team_id).cloned())
    }

    async fn get_teams(&self) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.teams.clone())
    }

    async fn update_team(&self, team_id: Uuid, request: UpdateTeamRequest) -> Result<Option<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(team) = self.teams.iter_mut().find(|t| t.id == team_id) {
            if let Some(name) = request.name {
                team.name = name;
            }
            if let Some(nickname) = request.nickname {
                team.nickname = Some(nickname);
            }
            if let Some(code) = request.code {
                team.code = code;
            }
            if let Some(division) = request.division {
                team.division = division;
            }
            if let Some(category) = request.category {
                team.category = category;
            }

            team.updated_at = Utc::now();

            Ok(Some(team.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_team(&self, team_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let index = self.teams.iter().position(|t| t.id == team_id);
        Ok(index.is_some())
    }

    async fn get_teams_by_club(&self, club_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .teams
            .iter()
            .filter(|t| t.club_id == club_id)
            .cloned()
            .collect())
    }

    async fn get_teams_by_division(&self, division: String) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .teams
            .iter()
            .filter(|t| t.division == division)
            .cloned()
            .collect())
    }

    async fn add_player_to_team(&self, team_id: Uuid, player_id: Uuid) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(())
    }

    async fn remove_player_from_team(&self, team_id: Uuid, player_id: Uuid) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(())
    }
}

// ============================================
// MOCK PLAYER REPOSITORY
// ============================================

/// Mock implementation of PlayerRepository for testing
#[derive(Clone, Default)]
pub struct MockPlayerRepository {
    pub players: Vec<Player>,
    pub should_fail: bool,
}

impl MockPlayerRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_players(players: Vec<Player>) -> Self {
        Self {
            players,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn find_player_mut(&mut self, player_id: Uuid) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == player_id)
    }
}

#[async_trait]
impl super::super::repositories::player_repository::PlayerRepository for MockPlayerRepository {
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let player = Player {
            id: Uuid::new_v4(),
            first_name: request.first_name,
            last_name: request.last_name,
            full_name: format!("{} {}", request.first_name, request.last_name),
            alias: request.alias,
            email: request.email,
            photo: None,
            status: PlayerStatus::Active,
            birth_date: request.birth_date,
            age: request.age,
            birth_place: request.birth_place,
            nationality: request.nationality,
            dominant_hand: request.dominant_hand,
            play_style: request.play_style,
            phone: request.phone,
            address: request.address,
            city: request.city,
            postal_code: request.postal_code,
            club_id: request.club_id,
            club_number: request.club_number,
            category: request.category,
            license_number: request.license_number,
            license_date: request.license_date,
            license_status: palet_shared::LicenseStatus::Valid,
            registration_date: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(player)
    }

    async fn get_player(&self, player_id: Uuid) -> Result<Option<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.players.iter().find(|p| p.id == player_id).cloned())
    }

    async fn get_players(&self) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self.players.clone())
    }

    async fn update_player(&self, player_id: Uuid, request: UpdatePlayerRequest) -> Result<Option<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            if let Some(first_name) = request.first_name {
                player.first_name = first_name;
                player.full_name = format!("{} {}", first_name, player.last_name);
            }
            if let Some(last_name) = request.last_name {
                player.last_name = last_name;
                player.full_name = format!("{} {}", player.first_name, last_name);
            }
            if let Some(alias) = request.alias {
                player.alias = Some(alias);
            }
            if let Some(email) = request.email {
                player.email = email;
            }
            if let Some(status) = request.status {
                player.status = status;
            }

            player.updated_at = Utc::now();

            Ok(Some(player.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_player(&self, player_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        let index = self.players.iter().position(|p| p.id == player_id);
        Ok(index.is_some())
    }

    async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.club_id == club_id)
            .cloned()
            .collect())
    }

    async fn get_players_by_team(&self, team_id: Uuid) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.team_id == Some(team_id))
            .cloned()
            .collect())
    }

    async fn get_players_by_status(&self, status: PlayerStatus) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError("Mock connection failed".to_string()));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.status == status)
            .cloned()
            .collect())
    }
}

// ============================================
// MOCK DATABASE
// ============================================

/// Mock database that combines all mock repositories
#[derive(Clone, Default)]
pub struct MockDatabase {
    pub championships: MockChampionshipRepository,
    pub matches: MockMatchRepository,
    pub clubs: MockClubRepository,
    pub teams: MockTeamRepository,
    pub players: MockPlayerRepository,
    pub should_fail: bool,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
        self.championships.should_fail = fail;
        self.matches.should_fail = fail;
        self.clubs.should_fail = fail;
        self.teams.should_fail = fail;
        self.players.should_fail = fail;
    }
}
