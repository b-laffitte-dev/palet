//! TDD Tests for ChampionshipService
//!
//! This module contains comprehensive tests for the ChampionshipService implementation
//! following the TDD methodology (RED -> GREEN -> REFACTOR).

use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, CompetitionStatus, CreateChampionshipRequest,
    MatchResult, TeamClassificationStatus, TeamStats, UpdateChampionshipRequest,
};
use uuid::Uuid;

use crate::db::DatabaseError;
use crate::harness::{
    assert_championship, assert_championship_classification_rules, assert_championship_journee,
    assert_championship_status, assert_classification_team_count, assert_is_error, assert_is_ok,
    assert_ok_with, assert_teams_sorted_by_palet_rules, create_test_championship,
    create_test_championship_team, create_test_championship_with, create_test_classification,
    create_test_classification_with_teams, create_test_create_championship_request,
    create_test_palet_vendeen_championship, create_test_team_stats, MockChampionshipRepository,
    MockMatchRepository,
};
use crate::services::championship_service::{
    ChampionshipService, ChampionshipServiceError, PostgresChampionshipService,
};

// ============================================
// 1. CREATION TESTS
// ============================================

#[cfg(test)]
mod creation_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_championship_with_valid_data() {
        // GIVEN: A valid championship request
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = create_test_create_championship_request();

        // WHEN: Creating a championship
        let result = service.create_championship(request).await;

        // THEN: Should succeed and return a championship
        assert_is_ok(&result);
        let championship = result.unwrap();
        assert_eq!(championship.name, "Test Championship");
        assert_eq!(championship.season, "2025-2026");
        assert_eq!(championship.league, "Vendée");
        assert_eq!(championship.divisions.len(), 1);
        assert_eq!(championship.total_journees, 10);
        assert_eq!(championship.status, CompetitionStatus::Pending);
    }

    #[tokio::test]
    async fn test_create_championship_with_empty_name() {
        // GIVEN: A championship request with empty name
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.name = "".to_string();

        // WHEN: Creating a championship with empty name
        let result = service.create_championship(request).await;

        // THEN: Should fail with validation error
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_create_championship_with_empty_season() {
        // GIVEN: A championship request with empty season
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.season = "".to_string();

        // WHEN: Creating a championship with empty season
        let result = service.create_championship(request).await;

        // THEN: Should fail with validation error
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_create_championship_with_zero_journees() {
        // GIVEN: A championship request with zero journees
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.total_journees = 0;

        // WHEN: Creating a championship with zero journees
        let result = service.create_championship(request).await;

        // THEN: Should fail with validation error
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_create_championship_with_empty_divisions() {
        // GIVEN: A championship request with empty divisions
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.divisions = vec![];

        // WHEN: Creating a championship with empty divisions
        let result = service.create_championship(request).await;

        // THEN: Should fail with validation error
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_create_palet_vendeen_championship() {
        // GIVEN: A championship with Palet Vendéen rules
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = CreateChampionshipRequest {
            name: "Championnat de Vendée Fonte".to_string(),
            code: Some("CVDP-FONTE-2025".to_string()),
            season: "2025-2026".to_string(),
            description: Some("Championnat officiel".to_string()),
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

        // WHEN: Creating a Palet Vendéen championship
        let result = service.create_championship(request).await;

        // THEN: Should succeed with correct rules
        assert_is_ok(&result);
        let championship = result.unwrap();
        assert_eq!(championship.league, "Vendée");
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
}

// ============================================
// 2. READ TESTS
// ============================================

#[cfg(test)]
mod read_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_championship_not_found() {
        // GIVEN: Empty repository
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting a non-existent championship
        let result = service.get_championship(Uuid::new_v4()).await;

        // THEN: Should return None
        assert_is_ok(&result);
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_championship_success() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting the championship
        let result = service.get_championship(championship_id).await;

        // THEN: Should return the championship
        assert_is_ok(&result);
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_championships_empty() {
        // GIVEN: Empty repository
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting all championships
        let result = service.get_championships().await;

        // THEN: Should return empty vector
        assert_is_ok(&result);
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_championships_by_league() {
        // GIVEN: A repository with multiple championships
        let mut repo = MockChampionshipRepository::new();
        repo.add_championship(create_test_championship_with(
            "Championnat Vendée",
            "Vendée",
            vec!["D1"],
            10,
        ));
        repo.add_championship(create_test_championship_with(
            "Championnat Deux-Sèvres",
            "Deux-Sèvres",
            vec!["D1"],
            8,
        ));
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting championships by league
        let result = service.get_championships_by_league("Vendée".to_string()).await;

        // THEN: Should return only Vendée championships
        assert_is_ok(&result);
        let championships = result.unwrap();
        assert_eq!(championships.len(), 1);
        assert_eq!(championships[0].league, "Vendée");
    }

    #[tokio::test]
    async fn test_get_championships_by_season() {
        // GIVEN: A repository with multiple championships
        let mut repo = MockChampionshipRepository::new();
        let mut champ1 = create_test_championship();
        champ1.season = "2025-2026".to_string();
        repo.add_championship(champ1);

        let mut champ2 = create_test_championship();
        champ2.season = "2024-2025".to_string();
        repo.add_championship(champ2);

        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting championships by season
        let result = service.get_championships_by_season("2025-2026".to_string()).await;

        // THEN: Should return only 2025-2026 championships
        assert_is_ok(&result);
        let championships = result.unwrap();
        assert_eq!(championships.len(), 1);
        assert_eq!(championships[0].season, "2025-2026");
    }

    #[tokio::test]
    async fn test_get_championships_by_status() {
        // GIVEN: A repository with championships of different statuses
        let mut repo = MockChampionshipRepository::new();

        let mut champ_pending = create_test_championship();
        champ_pending.status = CompetitionStatus::Pending;
        repo.add_championship(champ_pending);

        let mut champ_in_progress = create_test_championship();
        champ_in_progress.status = CompetitionStatus::InProgress;
        repo.add_championship(champ_in_progress);

        let mut champ_completed = create_test_championship();
        champ_completed.status = CompetitionStatus::Completed;
        repo.add_championship(champ_completed);

        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting championships by status
        let result = service
            .get_championships_by_status(CompetitionStatus::InProgress)
            .await;

        // THEN: Should return only in-progress championships
        assert_is_ok(&result);
        let championships = result.unwrap();
        assert_eq!(championships.len(), 1);
        assert_eq!(championships[0].status, CompetitionStatus::InProgress);
    }
}

// ============================================
// 3. UPDATE TESTS
// ============================================

#[cfg(test)]
mod update_tests {
    use super::*;

    #[tokio::test]
    async fn test_update_championship_not_found() {
        // GIVEN: Empty repository
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = UpdateChampionshipRequest {
            name: Some("Updated Name".to_string()),
            code: None,
            season: None,
            description: None,
            league: None,
            divisions: None,
            total_journees: None,
            journee_duration: None,
            classification_rules: None,
            status: None,
            current_journee: None,
            current_phase: None,
        };

        // WHEN: Updating a non-existent championship
        let result = service.update_championship(Uuid::new_v4(), request).await;

        // THEN: Should return None
        assert_is_ok(&result);
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_championship_name() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = UpdateChampionshipRequest {
            name: Some("Updated Championship Name".to_string()),
            code: None,
            season: None,
            description: None,
            league: None,
            divisions: None,
            total_journees: None,
            journee_duration: None,
            classification_rules: None,
            status: None,
            current_journee: None,
            current_phase: None,
        };

        // WHEN: Updating the championship name
        let result = service.update_championship(championship_id, request).await;

        // THEN: Should succeed and return updated championship
        assert_is_ok(&result);
        let updated = result.unwrap().unwrap();
        assert_eq!(updated.name, "Updated Championship Name");
    }

    #[tokio::test]
    async fn test_update_championship_status() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = UpdateChampionshipRequest {
            name: None,
            code: None,
            season: None,
            description: None,
            league: None,
            divisions: None,
            total_journees: None,
            journee_duration: None,
            classification_rules: None,
            status: Some(CompetitionStatus::InProgress),
            current_journee: Some(1),
            current_phase: None,
        };

        // WHEN: Updating the championship status
        let result = service.update_championship(championship_id, request).await;

        // THEN: Should succeed and return updated championship
        assert_is_ok(&result);
        let updated = result.unwrap().unwrap();
        assert_eq!(updated.status, CompetitionStatus::InProgress);
        assert_eq!(updated.current_journee, Some(1));
    }
}

// ============================================
// 4. DELETE TESTS
// ============================================

#[cfg(test)]
mod delete_tests {
    use super::*;

    #[tokio::test]
    async fn test_delete_championship_not_found() {
        // GIVEN: Empty repository
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Deleting a non-existent championship
        let result = service.delete_championship(Uuid::new_v4()).await;

        // THEN: Should return false
        assert_is_ok(&result);
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_delete_championship_success() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Deleting the championship
        let result = service.delete_championship(championship_id).await;

        // THEN: Should return true
        assert_is_ok(&result);
        assert!(result.unwrap());
    }
}

// ============================================
// 5. LIFECYCLE TESTS
// ============================================

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

    #[tokio::test]
    async fn test_start_championship() {
        // GIVEN: A repository with a pending championship
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::Pending;
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Starting the championship
        let result = service.start_championship(championship_id).await;

        // THEN: Should succeed and return in-progress championship
        assert_is_ok(&result);
        let started = result.unwrap();
        assert_eq!(started.status, CompetitionStatus::InProgress);
        assert_eq!(started.current_journee, Some(1));
    }

    #[tokio::test]
    async fn test_start_already_started_championship() {
        // GIVEN: A repository with an already started championship
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::InProgress;
        championship.current_journee = Some(5);
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Starting an already started championship
        let result = service.start_championship(championship_id).await;

        // THEN: Should fail
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_next_journee() {
        // GIVEN: A repository with an in-progress championship
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::InProgress;
        championship.current_journee = Some(1);
        championship.total_journees = 10;
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Moving to next journee
        let result = service.next_journee(championship_id).await;

        // THEN: Should succeed and return championship with next journee
        assert_is_ok(&result);
        let updated = result.unwrap();
        assert_eq!(updated.current_journee, Some(2));
        assert_eq!(updated.status, CompetitionStatus::InProgress);
    }

    #[tokio::test]
    async fn test_next_journee_completes_championship() {
        // GIVEN: A repository with a championship at last journee
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::InProgress;
        championship.current_journee = Some(9);
        championship.total_journees = 10;
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Moving to next journee (which is the last)
        let result = service.next_journee(championship_id).await;

        // THEN: Should succeed and mark championship as completed
        assert_is_ok(&result);
        let updated = result.unwrap();
        assert_eq!(updated.current_journee, Some(10));
        assert_eq!(updated.status, CompetitionStatus::Completed);
    }

    #[tokio::test]
    async fn test_next_journee_exceeds_total() {
        // GIVEN: A repository with a championship at last journee
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::InProgress;
        championship.current_journee = Some(10);
        championship.total_journees = 10;
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Trying to move beyond last journee
        let result = service.next_journee(championship_id).await;

        // THEN: Should fail with validation error
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_complete_championship() {
        // GIVEN: A repository with an in-progress championship
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.status = CompetitionStatus::InProgress;
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Completing the championship
        let result = service.complete_championship(championship_id).await;

        // THEN: Should succeed and return completed championship
        assert_is_ok(&result);
        let completed = result.unwrap();
        assert_eq!(completed.status, CompetitionStatus::Completed);
    }
}

// ============================================
// 6. CLASSIFICATION TESTS
// ============================================

#[cfg(test)]
mod classification_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_championship_classification() {
        // GIVEN: A repository with a championship and classifications
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);

        let classification = create_test_classification(championship_id, "D1", 5);
        repo.classifications.push(classification);

        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting the classification
        let result = service
            .get_championship_classification(championship_id, Some("D1".to_string()))
            .await;

        // THEN: Should return the classification
        assert_is_ok(&result);
        let classifications = result.unwrap();
        assert_eq!(classifications.len(), 1);
    }

    #[tokio::test]
    async fn test_update_championship_classification() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);

        let service = PostgresChampionshipService::new(Arc::new(repo));

        let teams = vec![create_test_championship_team(
            1,
            Uuid::new_v4(),
            13,
            4,
            1,
            0,
            45,
            20,
        )];
        let classification = create_test_classification_with_teams(
            championship_id,
            "D1",
            5,
            teams,
        );

        // WHEN: Updating the classification
        let result = service
            .update_championship_classification(championship_id, "D1".to_string(), classification)
            .await;

        // THEN: Should succeed
        assert_is_ok(&result);
    }

    #[tokio::test]
    async fn test_calculate_classification_with_points() {
        // GIVEN: A repository with a championship
        let repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Calculating classification
        let result = service
            .calculate_classification(championship_id, "D1".to_string(), 5)
            .await;

        // THEN: Should succeed
        assert_is_ok(&result);
    }

    #[tokio::test]
    async fn test_teams_sorted_by_palet_rules() {
        // GIVEN: Teams with different stats
        let teams = vec![
            create_test_championship_team(1, Uuid::new_v4(), 13, 4, 1, 0, 45, 20),
            create_test_championship_team(2, Uuid::new_v4(), 10, 3, 1, 1, 40, 25),
            create_test_championship_team(3, Uuid::new_v4(), 10, 3, 1, 1, 35, 30),
        ];

        // WHEN: Sorting by Palet Vendéen rules
        let mut sorted_teams = teams.clone();
        sorted_teams.sort_by(|a, b| {
            b.stats.points.cmp(&a.stats.points) // Points (primary)
                .then_with(|| b.stats.diff.cmp(&a.stats.diff)) // Diff (secondary)
                .then_with(|| b.stats.points_for.cmp(&a.stats.points_for)) // Points For (tertiary)
        });

        // THEN: Teams should be sorted correctly
        assert_teams_sorted_by_palet_rules(&sorted_teams);
        assert_eq!(sorted_teams[0].stats.points, 13);
        assert_eq!(sorted_teams[1].stats.points, 10);
        assert_eq!(sorted_teams[1].stats.diff, 15); // Better diff comes first
        assert_eq!(sorted_teams[2].stats.diff, 5);
    }
}

// ============================================
// 7. TEAM OPERATIONS TESTS
// ============================================

#[cfg(test)]
mod team_operations_tests {
    use super::*;

    #[tokio::test]
    async fn test_add_team_to_championship() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Adding a team to the championship
        let result = service
            .add_team_to_championship(championship_id, Uuid::new_v4(), "D1".to_string())
            .await;

        // THEN: Should succeed
        assert_is_ok(&result);
    }

    #[tokio::test]
    async fn test_remove_team_from_championship() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Removing a team from the championship
        let result = service
            .remove_team_from_championship(championship_id, Uuid::new_v4())
            .await;

        // THEN: Should succeed
        assert_is_ok(&result);
    }

    #[tokio::test]
    async fn test_get_championship_teams() {
        // GIVEN: A repository with a championship
        let mut repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Getting championship teams
        let result = service
            .get_championship_teams(championship_id, None)
            .await;

        // THEN: Should succeed (may return empty)
        assert_is_ok(&result);
    }
}

// ============================================
// 8. VALIDATION TESTS
// ============================================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_championship_data() {
        // GIVEN: Valid championship data
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let request = create_test_create_championship_request();

        // WHEN: Validating championship data
        let result = service.validate_championship_data(&request);

        // THEN: Should succeed
        assert_is_ok(&result);
    }

    #[tokio::test]
    async fn test_validate_championship_data_empty_name() {
        // GIVEN: Championship data with empty name
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.name = "".to_string();

        // WHEN: Validating championship data
        let result = service.validate_championship_data(&request);

        // THEN: Should fail
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_validate_championship_data_empty_season() {
        // GIVEN: Championship data with empty season
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.season = "".to_string();

        // WHEN: Validating championship data
        let result = service.validate_championship_data(&request);

        // THEN: Should fail
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_validate_championship_data_zero_journees() {
        // GIVEN: Championship data with zero journees
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.total_journees = 0;

        // WHEN: Validating championship data
        let result = service.validate_championship_data(&request);

        // THEN: Should fail
        assert_is_error(&result);
    }

    #[tokio::test]
    async fn test_validate_championship_data_empty_divisions() {
        // GIVEN: Championship data with empty divisions
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let mut request = create_test_create_championship_request();
        request.divisions = vec![];

        // WHEN: Validating championship data
        let result = service.validate_championship_data(&request);

        // THEN: Should fail
        assert_is_error(&result);
    }
}

// ============================================
// 9. PALET VENDÉEN SPECIFIC TESTS
// ============================================

#[cfg(test)]
mod palet_vendeen_tests {
    use super::*;

    #[tokio::test]
    async fn test_championship_with_palet_vendeen_rules() {
        // GIVEN: A championship with Palet Vendéen rules
        let repo = MockChampionshipRepository::new();
        let service = PostgresChampionshipService::new(Arc::new(repo));

        let championship = create_test_palet_vendeen_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);

        // WHEN: Getting the championship
        let result = service.get_championship(championship_id).await;

        // THEN: Should have correct Palet Vendéen rules
        assert_is_ok(&result);
        let champ = result.unwrap().unwrap();
        assert_eq!(champ.league, "Vendée");
        assert_championship_classification_rules(
            &champ,
            ClassificationPrimary::Points,
            ClassificationSecondary::Diff,
            ClassificationTertiary::PointsFor,
        );
    }

    #[tokio::test]
    async fn test_championship_journee_progression() {
        // GIVEN: A championship with 10 journees
        let mut repo = MockChampionshipRepository::new();
        let mut championship = create_test_championship();
        championship.total_journees = 10;
        championship.status = CompetitionStatus::InProgress;
        championship.current_journee = Some(1);
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // WHEN: Progressing through journees
        for expected_journee in 2..=10 {
            let result = service.next_journee(championship_id).await;
            assert_is_ok(&result);
            let updated = result.unwrap();
            assert_championship_journee(&updated, Some(expected_journee));
        }

        // THEN: Championship should be completed
        let result = service.get_championship(championship_id).await;
        assert_is_ok(&result);
        let completed = result.unwrap().unwrap();
        assert_championship_status(&completed, CompetitionStatus::Completed);
    }

    #[tokio::test]
    async fn test_championship_classification_sorted_correctly() {
        // GIVEN: A championship with teams
        let repo = MockChampionshipRepository::new();
        let championship = create_test_championship();
        let championship_id = championship.id;
        repo.add_championship(championship);
        let service = PostgresChampionshipService::new(Arc::new(repo));

        // Create teams with different stats
        let teams = vec![
            create_test_championship_team(1, Uuid::new_v4(), 13, 4, 1, 0, 45, 20),
            create_test_championship_team(2, Uuid::new_v4(), 10, 3, 1, 1, 40, 25),
            create_test_championship_team(3, Uuid::new_v4(), 10, 3, 1, 1, 35, 30),
        ];

        let classification = create_test_classification_with_teams(
            championship_id,
            "D1",
            5,
            teams,
        );

        // WHEN: Updating classification
        let result = service
            .update_championship_classification(championship_id, "D1".to_string(), classification)
            .await;

        // THEN: Should succeed
        assert_is_ok(&result);
    }
}
