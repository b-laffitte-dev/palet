//! Match Service - Business logic for matches
//!
//! This module provides the business logic for managing Palet Vendéen matches.
//! It implements the TDD harness requirements and integrates with the repository.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    CompetitionType, CreateMatchRequest, MancheScore, Match, MatchStatus, MatchWinner,
    UpdateMatchRequest,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::match_repository::{MatchRepository, MockMatchRepository};

// ============================================
// MATCH SERVICE TRAIT (Interface)
// ============================================

/// Trait defining the interface for match service operations
#[async_trait]
pub trait MatchService {
    // CRUD Operations
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, MatchServiceError>;
    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, MatchServiceError>;
    async fn get_matches(&self) -> Result<Vec<Match>, MatchServiceError>;
    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, MatchServiceError>;
    async fn delete_match(&self, match_id: Uuid) -> Result<bool, MatchServiceError>;

    // Match Lifecycle Operations
    async fn start_match(&self, match_id: Uuid) -> Result<Match, MatchServiceError>;
    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: MatchWinner,
    ) -> Result<Match, MatchServiceError>;

    // Manche Operations (Palet Vendéen specific)
    async fn add_manche(
        &self,
        match_id: Uuid,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, MatchServiceError>;
    async fn update_manche(
        &self,
        match_id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, MatchServiceError>;

    // Verification Operations
    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, MatchServiceError>;
    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, MatchServiceError>;

    // Query Operations
    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError>;
    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, MatchServiceError>;
    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, MatchServiceError>;
    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError>;

    // Championship-specific operations
    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError>;
    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError>;

    // Palet Vendéen Rules Validation
    async fn validate_match_creation(&self, request: &CreateMatchRequest) -> Result<(), MatchServiceError>;
    async fn validate_manche_score(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<(), MatchServiceError>;
    async fn check_match_completion(&self, match_obj: &Match) -> Result<bool, MatchServiceError>;
}

// ============================================
// MATCH SERVICE ERROR
// ============================================

/// Error type for match service operations
#[derive(Debug, thiserror::Error)]
pub enum MatchServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Match not found: {0}")]
    NotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Rule violation: {0}")]
    RuleViolation(String),
}

impl From<crate::db::DatabaseError> for MatchServiceError {
    fn from(error: crate::db::DatabaseError) -> Self {
        match error {
            crate::db::DatabaseError::NotFound(msg) => MatchServiceError::NotFound(msg),
            crate::db::DatabaseError::ValidationError(msg) => {
                MatchServiceError::ValidationError(msg)
            }
            other => MatchServiceError::RepositoryError(other.to_string()),
        }
    }
}

// ============================================
// MATCH SERVICE (Implementation)
// ============================================

/// Service implementation for match operations
#[derive(Clone)]
pub struct MatchServiceImpl<T: MatchRepository> {
    repository: Arc<T>,
}

impl<T: MatchRepository> MatchServiceImpl<T> {
    /// Create a new MatchServiceImpl
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: MatchRepository + Send + Sync> MatchService for MatchServiceImpl<T> {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, MatchServiceError> {
        // Validate the request
        self.validate_match_creation(&request).await?;

        // Create the match
        let match_obj = self.repository.create_match(request).await?;

        Ok(match_obj)
    }

    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, MatchServiceError> {
        let match_obj = self.repository.get_match(match_id).await?;
        Ok(match_obj)
    }

    async fn get_matches(&self) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self.repository.get_matches().await?;
        Ok(matches)
    }

    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, MatchServiceError> {
        let match_obj = self.repository.update_match(match_id, request).await?;
        Ok(match_obj)
    }

    async fn delete_match(&self, match_id: Uuid) -> Result<bool, MatchServiceError> {
        let deleted = self.repository.delete_match(match_id).await?;
        Ok(deleted)
    }

    async fn start_match(&self, match_id: Uuid) -> Result<Match, MatchServiceError> {
        // Check if match exists and is in scheduled state
        let match_obj = self.repository.get_match(match_id).await?;
        if let Some(match_obj) = match_obj {
            if match_obj.status != MatchStatus::Scheduled {
                return Err(MatchServiceError::InvalidOperation(format!(
                    "Cannot start match in state: {:?}",
                    match_obj.status
                )));
            }
        }

        let match_obj = self.repository.start_match(match_id).await?;
        Ok(match_obj)
    }

    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: MatchWinner,
    ) -> Result<Match, MatchServiceError> {
        // Check if match exists and is in progress
        let match_obj = self.repository.get_match(match_id).await?;
        if let Some(match_obj) = match_obj {
            if match_obj.status != MatchStatus::InProgress {
                return Err(MatchServiceError::InvalidOperation(format!(
                    "Cannot complete match in state: {:?}",
                    match_obj.status
                )));
            }
        }

        let match_obj = self.repository.complete_match(match_id, winner).await?;
        Ok(match_obj)
    }

    async fn add_manche(
        &self,
        match_id: Uuid,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, MatchServiceError> {
        // Validate manche score
        self.validate_manche_score(team1_score, team2_score)
            .await?;

        // Get current match to determine next manche number
        let match_obj = self.repository.get_match(match_id).await?;
        if let Some(match_obj) = match_obj {
            let next_manche_number = match_obj.current_manche.map_or(1, |n| n + 1);

            let manche = MancheScore {
                number: next_manche_number,
                team1: team1_score,
                team2: team2_score,
                winner: if team1_score > team2_score {
                    Some(MatchWinner::Team1)
                } else if team2_score > team1_score {
                    Some(MatchWinner::Team2)
                } else {
                    None // Draw in manche (shouldn't happen in Palet)
                },
            };

            let match_obj = self.repository.add_manche(match_id, manche).await?;
            Ok(match_obj)
        } else {
            Err(MatchServiceError::NotFound(format!(
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
    ) -> Result<Match, MatchServiceError> {
        // Validate manche score
        self.validate_manche_score(team1_score, team2_score)
            .await?;

        let match_obj = self
            .repository
            .update_manche(match_id, manche_number, team1_score, team2_score)
            .await?;
        Ok(match_obj)
    }

    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, MatchServiceError> {
        let match_obj = self.repository.verify_match(match_id, verifier_id).await?;
        Ok(match_obj)
    }

    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, MatchServiceError> {
        let match_obj = self.repository.unverify_match(match_id).await?;
        Ok(match_obj)
    }

    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self
            .repository
            .get_matches_by_competition(competition_id)
            .await?;
        Ok(matches)
    }

    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self.repository.get_matches_by_status(status).await?;
        Ok(matches)
    }

    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self.repository.get_matches_by_team(team_id).await?;
        Ok(matches)
    }

    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self
            .repository
            .get_matches_by_journee(journee, competition_id)
            .await?;
        Ok(matches)
    }

    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self
            .repository
            .get_championship_matches(championship_id)
            .await?;
        Ok(matches)
    }

    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, MatchServiceError> {
        let matches = self
            .repository
            .get_completed_matches_by_championship(championship_id)
            .await?;
        Ok(matches)
    }

    async fn validate_match_creation(&self, request: &CreateMatchRequest) -> Result<(), MatchServiceError> {
        // Validate required fields
        if request.team1_id == request.team2_id {
            return Err(MatchServiceError::ValidationError(
                "Team1 and Team2 cannot be the same".to_string(),
            ));
        }

        // Validate best_of value (13 or 15 for Palet Vendéen)
        if let Some(best_of) = request.best_of {
            if best_of != 13 && best_of != 15 {
                return Err(MatchServiceError::ValidationError(
                    format!("best_of must be 13 or 15 for Palet Vendéen, got {}", best_of),
                ));
            }
        }

        // Validate date is in the future or today (with a small tolerance
        // for the elapsed time between the caller capturing `now` and this check)
        if request.date < Utc::now() - chrono::Duration::seconds(1) {
            return Err(MatchServiceError::ValidationError(
                "Match date cannot be in the past".to_string(),
            ));
        }

        Ok(())
    }

    async fn validate_manche_score(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<(), MatchServiceError> {
        // Validate scores are non-negative
        if team1_score < 0 || team2_score < 0 {
            return Err(MatchServiceError::ValidationError(
                "Scores cannot be negative".to_string(),
            ));
        }

        // Validate scores are within valid range (0-6 for doublette)
        if team1_score > 6 || team2_score > 6 {
            return Err(MatchServiceError::ValidationError(
                format!(
                    "Scores cannot exceed 6 per manche (got {} and {})",
                    team1_score, team2_score
                ),
            ));
        }

        // In Palet Vendéen, a manche cannot be a draw (one team must have more points)
        // However, we allow it for now as the database might store it
        // The business logic will handle the winner determination

        Ok(())
    }

    async fn check_match_completion(&self, match_obj: &Match) -> Result<bool, MatchServiceError> {
        let team1_score = match_obj.team1_score.unwrap_or(0);
        let team2_score = match_obj.team2_score.unwrap_or(0);
        let best_of = match_obj.best_of.unwrap_or(13);

        // Check if either team has reached the required points
        if team1_score >= best_of || team2_score >= best_of {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// ============================================
// DEFAULT MATCH SERVICE (with Mock Repository)
// ============================================

/// Default match service with mock repository for testing
pub type DefaultMatchService = MatchServiceImpl<MockMatchRepository>;

impl Default for DefaultMatchService {
    fn default() -> Self {
        Self::new(Arc::new(MockMatchRepository::new()))
    }
}

// ============================================
// TEST HELPER FUNCTIONS
// ============================================

/// Helper function to create a test match request
pub fn create_test_match_request(
    competition_type: CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
) -> CreateMatchRequest {
    CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type,
        team1_id,
        team2_id,
        date: Utc::now(),
        location: Some("Test Location".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    }
}

/// Helper function to create a test match
pub fn create_test_match(
    competition_type: CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id,
        team2_id,
        team1_score: Some(0),
        team2_score: Some(0),
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
        location: Some("Test Location".to_string()),
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

// ============================================
// UNIT TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use palet_shared::CompetitionType;

    #[tokio::test]
    async fn test_create_match_with_valid_data() {
        // GIVEN: A valid match request
        let request = CreateMatchRequest {
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            date: Utc::now(),
            location: Some("Salle des Essarts".to_string()),
            journee: Some(1),
            phase: None,
            round: None,
            best_of: Some(13),
        };

        // WHEN: Creating a match with mock service
        let service = DefaultMatchService::default();
        let result = service.create_match(request).await;

        // THEN: Should succeed
        assert!(result.is_ok());
        let match_obj = result.unwrap();
        assert_eq!(match_obj.competition_type, CompetitionType::Tournament);
        assert_eq!(match_obj.best_of, Some(13));
        assert_eq!(match_obj.status, MatchStatus::Scheduled);
    }

    #[tokio::test]
    async fn test_create_match_with_invalid_teams() {
        // GIVEN: A request with same team1 and team2
        let team_id = Uuid::new_v4();
        let request = CreateMatchRequest {
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            team1_id: team_id,
            team2_id: team_id, // Same team - invalid
            date: Utc::now(),
            location: Some("Salle des Essarts".to_string()),
            journee: Some(1),
            phase: None,
            round: None,
            best_of: Some(13),
        };

        // WHEN: Creating a match
        let service = DefaultMatchService::default();
        let result = service.create_match(request).await;

        // THEN: Should fail with validation error
        assert!(result.is_err());
        if let Err(MatchServiceError::ValidationError(msg)) = result {
            assert!(msg.contains("Team1 and Team2 cannot be the same"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[tokio::test]
    async fn test_validate_manche_score_valid() {
        // GIVEN: Valid manche scores
        let service = DefaultMatchService::default();

        // WHEN: Validating scores
        let result = service.validate_manche_score(3, 1).await;

        // THEN: Should succeed
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_manche_score_negative() {
        // GIVEN: Negative score
        let service = DefaultMatchService::default();

        // WHEN: Validating negative score
        let result = service.validate_manche_score(-1, 3).await;

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(MatchServiceError::ValidationError(msg)) = result {
            assert!(msg.contains("negative"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[tokio::test]
    async fn test_validate_manche_score_too_high() {
        // GIVEN: Score exceeding maximum (6)
        let service = DefaultMatchService::default();

        // WHEN: Validating too high score
        let result = service.validate_manche_score(7, 0).await;

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(MatchServiceError::ValidationError(msg)) = result {
            assert!(msg.contains("cannot exceed 6"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[tokio::test]
    async fn test_add_manche_to_match() {
        // GIVEN: A match in progress
        let service = DefaultMatchService::default();
        let team1_id = Uuid::new_v4();
        let team2_id = Uuid::new_v4();

        // Create a match
        let request = CreateMatchRequest {
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championship,
            team1_id,
            team2_id,
            date: Utc::now(),
            location: Some("Test Location".to_string()),
            journee: Some(1),
            phase: None,
            round: None,
            best_of: Some(13),
        };

        let match_obj = service.create_match(request).await.unwrap();
        let match_id = match_obj.id;

        // Start the match
        let _ = service.start_match(match_id).await.unwrap();

        // WHEN: Adding a manche
        let result = service.add_manche(match_id, 3, 1).await;

        // THEN: Should succeed and update match
        assert!(result.is_ok());
        let updated_match = result.unwrap();
        assert_eq!(updated_match.current_manche, Some(1));
        assert_eq!(updated_match.team1_score, Some(3));
        assert_eq!(updated_match.team2_score, Some(1));
    }

    #[tokio::test]
    async fn test_check_match_completion_not_completed() {
        // GIVEN: A match with scores below best_of
        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championship,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(10),
            team2_score: Some(8),
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: Some(5),
            current_scores: None,
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: Checking completion
        let service = DefaultMatchService::default();
        let result = service.check_match_completion(&match_obj).await;

        // THEN: Should not be completed
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_check_match_completion_completed() {
        // GIVEN: A match with team1 reaching 13 points
        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championship,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(13),
            team2_score: Some(8),
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: Some(5),
            current_scores: None,
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: Checking completion
        let service = DefaultMatchService::default();
        let result = service.check_match_completion(&match_obj).await;

        // THEN: Should be completed
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_check_match_completion_final_15_points() {
        // GIVEN: A final match requiring 15 points
        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            journee: None,
            phase: Some("Finale".to_string()),
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(15),
            team2_score: Some(12),
            winner: None,
            manches: None,
            best_of: Some(15),
            current_manche: Some(10),
            current_scores: None,
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: Checking completion
        let service = DefaultMatchService::default();
        let result = service.check_match_completion(&match_obj).await;

        // THEN: Should be completed (15 points reached)
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
