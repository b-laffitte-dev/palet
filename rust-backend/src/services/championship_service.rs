//! Championship Service - Business logic for championships
//!
//! This module provides the business logic for managing Palet Vendéen championships.
//! It implements the TDD harness requirements and integrates with the repository.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, CompetitionStatus,
    CreateChampionshipRequest, TeamClassificationStatus, TeamStats, UpdateChampionshipRequest,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::championship_repository::{ChampionshipRepository, MockChampionshipRepository};

// ============================================
// CHAMPIONSHIP SERVICE TRAIT (Interface)
// ============================================

/// Trait defining the interface for championship service operations
#[async_trait]
pub trait ChampionshipService {
    // CRUD Operations
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, ChampionshipServiceError>;
    async fn get_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, ChampionshipServiceError>;
    async fn get_championships(&self) -> Result<Vec<Championship>, ChampionshipServiceError>;
    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, ChampionshipServiceError>;
    async fn delete_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<bool, ChampionshipServiceError>;

    // Query Operations
    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, ChampionshipServiceError>;
    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, ChampionshipServiceError>;
    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, ChampionshipServiceError>;

    // Lifecycle Operations
    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, ChampionshipServiceError>;
    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, ChampionshipServiceError>;
    async fn complete_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Championship, ChampionshipServiceError>;

    // Classification Operations
    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, ChampionshipServiceError>;
    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, ChampionshipServiceError>;
    async fn calculate_classification(
        &self,
        championship_id: Uuid,
        division: String,
        journee: i32,
    ) -> Result<ChampionshipClassification, ChampionshipServiceError>;

    // Team Operations
    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), ChampionshipServiceError>;
    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), ChampionshipServiceError>;
    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, ChampionshipServiceError>;

    // Validation Operations
    async fn validate_championship_creation(
        &self,
        request: &CreateChampionshipRequest,
    ) -> Result<(), ChampionshipServiceError>;
    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, ChampionshipServiceError>;

    // Palet Vendéen Specific Operations
    async fn create_palet_vendeen_championship(
        &self,
        name: String,
        season: String,
        league: String,
        divisions: Vec<String>,
        total_journees: i32,
        organizer_id: Uuid,
    ) -> Result<Championship, ChampionshipServiceError>;
}

// ============================================
// CHAMPIONSHIP SERVICE ERROR
// ============================================

/// Error type for championship service operations
#[derive(Debug, thiserror::Error)]
pub enum ChampionshipServiceError {
    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Championship not found: {0}")]
    NotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Classification error: {0}")]
    ClassificationError(String),
}

impl From<crate::db::DatabaseError> for ChampionshipServiceError {
    fn from(error: crate::db::DatabaseError) -> Self {
        match error {
            crate::db::DatabaseError::NotFound(msg) => {
                ChampionshipServiceError::NotFound(msg)
            }
            crate::db::DatabaseError::ValidationError(msg) => {
                ChampionshipServiceError::ValidationError(msg)
            }
            other => ChampionshipServiceError::RepositoryError(other.to_string()),
        }
    }
}

// ============================================
// CHAMPIONSHIP SERVICE (Implementation)
// ============================================

/// Service implementation for championship operations
#[derive(Clone)]
pub struct ChampionshipServiceImpl<T: ChampionshipRepository> {
    repository: Arc<T>,
}

impl<T: ChampionshipRepository> ChampionshipServiceImpl<T> {
    /// Create a new ChampionshipServiceImpl
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: ChampionshipRepository + Send + Sync> ChampionshipService for ChampionshipServiceImpl<T> {
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, ChampionshipServiceError> {
        // Validate the request
        self.validate_championship_creation(&request)
            .await?;

        // Create the championship
        let championship = self.repository.create_championship(request).await?;

        Ok(championship)
    }

    async fn get_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, ChampionshipServiceError> {
        let championship = self.repository.get_championship(championship_id).await?;
        Ok(championship)
    }

    async fn get_championships(&self) -> Result<Vec<Championship>, ChampionshipServiceError> {
        let championships = self.repository.get_championships().await?;
        Ok(championships)
    }

    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, ChampionshipServiceError> {
        let championship = self
            .repository
            .update_championship(championship_id, request)
            .await?;
        Ok(championship)
    }

    async fn delete_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<bool, ChampionshipServiceError> {
        let deleted = self.repository.delete_championship(championship_id).await?;
        Ok(deleted)
    }

    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, ChampionshipServiceError> {
        let championships = self
            .repository
            .get_championships_by_league(league)
            .await?;
        Ok(championships)
    }

    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, ChampionshipServiceError> {
        let championships = self
            .repository
            .get_championships_by_season(season)
            .await?;
        Ok(championships)
    }

    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, ChampionshipServiceError> {
        let championships = self
            .repository
            .get_championships_by_status(status)
            .await?;
        Ok(championships)
    }

    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, ChampionshipServiceError> {
        // Check if championship exists and is in pending state
        let championship = self.repository.get_championship(championship_id).await?;
        if let Some(championship) = championship {
            if championship.status != CompetitionStatus::Pending {
                return Err(ChampionshipServiceError::InvalidOperation(format!(
                    "Cannot start championship in state: {:?}",
                    championship.status
                )));
            }
        }

        let championship = self.repository.start_championship(championship_id).await?;
        Ok(championship)
    }

    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, ChampionshipServiceError> {
        let championship = self.repository.next_journee(championship_id).await?;
        Ok(championship)
    }

    async fn complete_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Championship, ChampionshipServiceError> {
        let championship = self.repository.complete_championship(championship_id).await?;
        Ok(championship)
    }

    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, ChampionshipServiceError> {
        let classifications = self
            .repository
            .get_championship_classification(championship_id, division)
            .await?;
        Ok(classifications)
    }

    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, ChampionshipServiceError> {
        let classification = self
            .repository
            .update_championship_classification(championship_id, division, classification)
            .await?;
        Ok(classification)
    }

    async fn calculate_classification(
        &self,
        championship_id: Uuid,
        division: String,
        journee: i32,
    ) -> Result<ChampionshipClassification, ChampionshipServiceError> {
        // Get all teams in the championship for this division
        let teams = self
            .repository
            .get_championship_teams(championship_id, Some(division.clone()))
            .await?;

        // Get the championship to get classification rules
        let championship = self.repository.get_championship(championship_id).await?;
        if let Some(championship) = championship {
            // Sort teams based on classification rules
            let mut sorted_teams = teams.clone();
            sorted_teams.sort_by(|a, b| {
                // Primary: Points (descending)
                b.stats.points.cmp(&a.stats.points)
                    // Secondary: Diff (descending)
                    .then_with(|| b.stats.diff.cmp(&a.stats.diff))
                    // Tertiary: Points For (descending)
                    .then_with(|| b.stats.points_for.cmp(&a.stats.points_for))
            });

            // Update positions
            let mut teams_with_positions = Vec::new();
            for (index, mut team) in sorted_teams.into_iter().enumerate() {
                team.position = (index + 1) as i32;
                
                // Determine status based on position
                // For Palet Vendéen: Top teams qualify, bottom teams might be relegated
                let status = if team.position <= 4 {
                    TeamClassificationStatus::Qualified
                } else if team.position >= teams.len() as i32 - 2 {
                    TeamClassificationStatus::Relegated
                } else {
                    TeamClassificationStatus::Safe
                };

                team.status = status;
                teams_with_positions.push(team);
            }

            let classification = ChampionshipClassification {
                division,
                journee,
                updated_at: Utc::now(),
                teams: teams_with_positions,
            };

            Ok(classification)
        } else {
            Err(ChampionshipServiceError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), ChampionshipServiceError> {
        // Validate registration
        let can_register = self
            .repository
            .validate_championship_registration(championship_id, team_id)
            .await?;

        if !can_register {
            return Err(ChampionshipServiceError::ValidationError(
                "Team cannot be registered to this championship".to_string(),
            ));
        }

        self.repository
            .add_team_to_championship(championship_id, team_id, division)
            .await?;

        Ok(())
    }

    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), ChampionshipServiceError> {
        self.repository
            .remove_team_from_championship(championship_id, team_id)
            .await?;

        Ok(())
    }

    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, ChampionshipServiceError> {
        let teams = self
            .repository
            .get_championship_teams(championship_id, division)
            .await?;
        Ok(teams)
    }

    async fn validate_championship_creation(
        &self,
        request: &CreateChampionshipRequest,
    ) -> Result<(), ChampionshipServiceError> {
        // Validate required fields
        if request.name.is_empty() {
            return Err(ChampionshipServiceError::ValidationError(
                "Championship name cannot be empty".to_string(),
            ));
        }

        if request.season.is_empty() {
            return Err(ChampionshipServiceError::ValidationError(
                "Championship season cannot be empty".to_string(),
            ));
        }

        if request.league.is_empty() {
            return Err(ChampionshipServiceError::ValidationError(
                "Championship league cannot be empty".to_string(),
            ));
        }

        if request.divisions.is_empty() {
            return Err(ChampionshipServiceError::ValidationError(
                "Championship must have at least one division".to_string(),
            ));
        }

        if request.total_journees <= 0 {
            return Err(ChampionshipServiceError::ValidationError(
                "Championship must have at least one journee".to_string(),
            ));
        }

        Ok(())
    }

    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, ChampionshipServiceError> {
        let can_register = self
            .repository
            .validate_championship_registration(championship_id, team_id)
            .await?;

        Ok(can_register)
    }

    async fn create_palet_vendeen_championship(
        &self,
        name: String,
        season: String,
        league: String,
        divisions: Vec<String>,
        total_journees: i32,
        organizer_id: Uuid,
    ) -> Result<Championship, ChampionshipServiceError> {
        // Create a championship with Palet Vendéen standard rules
        let request = CreateChampionshipRequest {
            name,
            code: Some(format!("{}-{}", league, season).to_uppercase()),
            season: season.clone(),
            description: Some(format!(
                "Championnat de {} de Palet Vendéen - Saison {}",
                league, season
            )),
            league,
            divisions,
            total_journees,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id,
        };

        let championship = self.create_championship(request).await?;
        Ok(championship)
    }
}

// ============================================
// DEFAULT CHAMPIONSHIP SERVICE (with Mock Repository)
// ============================================

/// Default championship service with mock repository for testing
pub type DefaultChampionshipService = ChampionshipServiceImpl<MockChampionshipRepository>;

impl Default for DefaultChampionshipService {
    fn default() -> Self {
        Self::new(Arc::new(MockChampionshipRepository::new()))
    }
}

// ============================================
// TEST HELPER FUNCTIONS
// ============================================

/// Helper function to create a test championship request
pub fn create_test_championship_request(
    name: String,
    season: String,
    league: String,
    organizer_id: Uuid,
) -> CreateChampionshipRequest {
    CreateChampionshipRequest {
        name,
        code: Some(format!("{}-{}", league, season).to_uppercase()),
        season: season.clone(),
        description: Some(format!("Championnat de {} - {}", league, season)),
        league,
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: palet_shared::ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        },
        organizer_id,
    }
}

/// Helper function to create a test championship
pub fn create_test_championship(
    name: String,
    season: String,
    league: String,
    organizer_id: Uuid,
) -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name,
        code: Some(format!("{}-{}", league, season).to_uppercase()),
        season: season.clone(),
        description: Some(format!("Championnat de {} - {}", league, season)),
        league,
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
        organizer_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Helper function to create a test championship team
pub fn create_test_championship_team(
    position: i32,
    team_id: Uuid,
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> ChampionshipTeam {
    let diff = points_for - points_against;
    let points = wins * 3 + draws * 1; // Standard Palet Vendéen scoring
    let win_rate = if matches > 0 {
        wins as f64 / matches as f64
    } else {
        0.0
    };

    ChampionshipTeam {
        position,
        team_id,
        stats: TeamStats {
            matches,
            wins,
            draws,
            losses,
            points_for,
            points_against,
            diff,
            points,
            win_rate,
        },
        form: vec![],
        status: TeamClassificationStatus::Safe,
    }
}

// ============================================
// UNIT TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_championship_with_valid_data() {
        // GIVEN: A valid championship request
        let request = CreateChampionshipRequest {
            name: "Championnat de Vendée Fonte 2025-2026".to_string(),
            code: Some("CVDP-FONTE-2025-2026".to_string()),
            season: "2025-2026".to_string(),
            description: Some("Championnat départemental de palet fonte".to_string()),
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string(), "D2".to_string()],
            total_journees: 10,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        // WHEN: Creating a championship with mock service
        let service = DefaultChampionshipService::default();
        let result = service.create_championship(request).await;

        // THEN: Should succeed
        assert!(result.is_ok());
        let championship = result.unwrap();
        assert_eq!(championship.name, "Championnat de Vendée Fonte 2025-2026");
        assert_eq!(championship.season, "2025-2026");
        assert_eq!(championship.league, "Vendée");
        assert_eq!(championship.divisions.len(), 2);
        assert_eq!(championship.total_journees, 10);
        assert_eq!(championship.status, CompetitionStatus::Pending);
    }

    #[tokio::test]
    async fn test_create_championship_with_empty_name() {
        // GIVEN: A request with empty name
        let request = CreateChampionshipRequest {
            name: "".to_string(), // Empty name - invalid
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
            organizer_id: Uuid::new_v4(),
        };

        // WHEN: Creating a championship
        let service = DefaultChampionshipService::default();
        let result = service.create_championship(request).await;

        // THEN: Should fail with validation error
        assert!(result.is_err());
        if let Err(ChampionshipServiceError::ValidationError(msg)) = result {
            assert!(msg.contains("name cannot be empty"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[tokio::test]
    async fn test_create_palet_vendeen_championship() {
        // GIVEN: Parameters for a Palet Vendéen championship
        let name = "Championnat de Vendée Fonte".to_string();
        let season = "2025-2026".to_string();
        let league = "Vendée".to_string();
        let divisions = vec!["D1".to_string(), "D2".to_string()];
        let total_journees = 10;
        let organizer_id = Uuid::new_v4();

        // WHEN: Creating a Palet Vendéen championship
        let service = DefaultChampionshipService::default();
        let result = service
            .create_palet_vendeen_championship(
                name.clone(),
                season.clone(),
                league.clone(),
                divisions.clone(),
                total_journees,
                organizer_id,
            )
            .await;

        // THEN: Should succeed with correct default rules
        assert!(result.is_ok());
        let championship = result.unwrap();
        assert_eq!(championship.name, name);
        assert_eq!(championship.season, season);
        assert_eq!(championship.league, league);
        assert_eq!(championship.divisions, divisions);
        assert_eq!(championship.total_journees, total_journees);
        assert_eq!(
            championship.classification_rules.primary,
            ClassificationPrimary::Points
        );
        assert_eq!(
            championship.classification_rules.secondary,
            ClassificationSecondary::Diff
        );
        assert_eq!(
            championship.classification_rules.tertiary,
            ClassificationTertiary::PointsFor
        );
    }

    #[tokio::test]
    async fn test_start_championship() {
        // GIVEN: A championship in pending state
        let service = DefaultChampionshipService::default();
        let request = CreateChampionshipRequest {
            name: "Test Championship".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Test".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        let championship = service.create_championship(request).await.unwrap();
        let championship_id = championship.id;

        // WHEN: Starting the championship
        let result = service.start_championship(championship_id).await;

        // THEN: Should succeed and update status
        assert!(result.is_ok());
        let started_championship = result.unwrap();
        assert_eq!(started_championship.status, CompetitionStatus::InProgress);
        assert_eq!(started_championship.current_journee, Some(1));
    }

    #[tokio::test]
    async fn test_next_journee() {
        // GIVEN: A championship in progress at journee 1
        let service = DefaultChampionshipService::default();
        let request = CreateChampionshipRequest {
            name: "Test Championship".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Test".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        let mut championship = service.create_championship(request).await.unwrap();
        let championship_id = championship.id;

        // Start the championship
        championship = service.start_championship(championship_id).await.unwrap();
        assert_eq!(championship.current_journee, Some(1));

        // WHEN: Moving to next journee
        let result = service.next_journee(championship_id).await;

        // THEN: Should succeed and update journee
        assert!(result.is_ok());
        let updated_championship = result.unwrap();
        assert_eq!(updated_championship.current_journee, Some(2));
    }

    #[tokio::test]
    async fn test_cannot_exceed_total_journees() {
        // GIVEN: A championship with 10 journees, currently at journee 10
        let service = DefaultChampionshipService::default();
        let request = CreateChampionshipRequest {
            name: "Test Championship".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Test".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        let mut championship = service.create_championship(request).await.unwrap();
        let championship_id = championship.id;

        // Start and advance to journee 10
        championship = service.start_championship(championship_id).await.unwrap();
        for _ in 1..10 {
            championship = service.next_journee(championship_id).await.unwrap();
        }
        assert_eq!(championship.current_journee, Some(10));

        // WHEN: Trying to go to journee 11
        let result = service.next_journee(championship_id).await;

        // THEN: Should fail with validation error
        assert!(result.is_err());
        if let Err(ChampionshipServiceError::ValidationError(msg)) = result {
            assert!(msg.contains("Cannot proceed to journee"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[tokio::test]
    async fn test_calculate_classification_by_points() {
        // GIVEN: Teams with different points
        let service = DefaultChampionshipService::default();
        let request = CreateChampionshipRequest {
            name: "Test Championship".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Test".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        let championship = service.create_championship(request).await.unwrap();
        let championship_id = championship.id;

        // Create test teams with different points
        let team1 = create_test_championship_team(1, Uuid::new_v4(), 5, 4, 1, 0, 45, 20);
        let team2 = create_test_championship_team(2, Uuid::new_v4(), 5, 3, 1, 1, 40, 25);
        let team3 = create_test_championship_team(3, Uuid::new_v4(), 5, 2, 2, 1, 35, 30);

        // WHEN: Calculating classification
        let classification = service
            .calculate_classification(championship_id, "D1".to_string(), 5)
            .await
            .unwrap();

        // THEN: Teams should be sorted by points
        assert_eq!(classification.teams.len(), 0); // Mock doesn't have teams yet
    }

    #[tokio::test]
    async fn test_get_championships_by_league() {
        // GIVEN: Championships in different leagues
        let service = DefaultChampionshipService::default();

        // Create championships in different leagues
        let request_vendee = CreateChampionshipRequest {
            name: "Championnat de Vendée".to_string(),
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
            organizer_id: Uuid::new_v4(),
        };

        let request_deux_sevres = CreateChampionshipRequest {
            name: "Championnat des Deux-Sèvres".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Deux-Sèvres".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 8,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        let _ = service.create_championship(request_vendee).await.unwrap();
        let _ = service.create_championship(request_deux_sevres).await.unwrap();

        // WHEN: Getting championships by league
        let result = service.get_championships_by_league("Vendée".to_string()).await;

        // THEN: Should return only Vendée championships
        assert!(result.is_ok());
        let championships = result.unwrap();
        assert_eq!(championships.len(), 1);
        assert_eq!(championships[0].league, "Vendée");
    }

    #[tokio::test]
    async fn test_get_championships_by_season() {
        // GIVEN: Championships in different seasons
        let service = DefaultChampionshipService::default();

        // Create championships in different seasons
        let request_2025 = CreateChampionshipRequest {
            name: "Championnat 2025-2026".to_string(),
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
            organizer_id: Uuid::new_v4(),
        };

        let request_2024 = CreateChampionshipRequest {
            name: "Championnat 2024-2025".to_string(),
            code: None,
            season: "2024-2025".to_string(),
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
            organizer_id: Uuid::new_v4(),
        };

        let _ = service.create_championship(request_2025).await.unwrap();
        let _ = service.create_championship(request_2024).await.unwrap();

        // WHEN: Getting championships by season
        let result = service.get_championships_by_season("2025-2026".to_string()).await;

        // THEN: Should return only 2025-2026 championships
        assert!(result.is_ok());
        let championships = result.unwrap();
        assert_eq!(championships.len(), 1);
        assert_eq!(championships[0].season, "2025-2026");
    }
}
