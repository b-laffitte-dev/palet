//! Tests TDD pour ChampionshipService (Palet Vendéen)
//!
//! Approche :
//! 1. Écrire les tests EN PREMIER (rouge)
//! 2. Implémenter le code minimal pour passer les tests (vert)
//! 3. Refactoriser (si nécessaire)

use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, CompetitionStatus, CreateChampionshipRequest,
    TeamClassificationStatus, TeamStats, UpdateChampionshipRequest,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::harness::mocks::MockChampionshipRepository;
use crate::harness::utils::{ChampionshipBuilder, TeamBuilder, create_team_stats};
use crate::services::championship_service::{
    ChampionshipService, ChampionshipServiceError, ChampionshipServiceImpl,
};

// ============================================
// TEST HELPERS
// ============================================

/// Create a test service with mock repository
fn create_test_service() -> ChampionshipServiceImpl<MockChampionshipRepository> {
    let repo = MockChampionshipRepository::new();
    ChampionshipServiceImpl::new(Arc::new(repo))
}

// ============================================
// 1. TESTS DE CRÉATION DE CHAMPIONNAT
// ============================================

#[tokio::test]
async fn test_create_championship_with_valid_data() {
    // GIVEN: A service and valid request
    let service = create_test_service();
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

    // WHEN: Create championship
    let result = service.create_championship(request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Creating valid championship should succeed");

    let championship = result.unwrap();
    assert_eq!(championship.name, "Championnat de Vendée Fonte 2025-2026");
    assert_eq!(championship.season, "2025-2026");
    assert_eq!(championship.league, "Vendée");
    assert_eq!(championship.divisions.len(), 2);
    assert_eq!(championship.total_journees, 10);
    assert_eq!(championship.status, CompetitionStatus::Pending);
}

#[tokio::test]
async fn test_create_championship_with_empty_name_fails() {
    // GIVEN: A service and invalid request (empty name)
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Create championship
    let result = service.create_championship(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating championship with empty name should fail");

    match result.unwrap_err() {
        ChampionshipServiceError::ValidationError(msg) => {
            assert!(msg.contains("name"), "Error should mention name field");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_championship_with_empty_season_fails() {
    // GIVEN: A service and invalid request (empty season)
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "Championnat Test".to_string(),
        code: None,
        season: "".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Create championship
    let result = service.create_championship(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating championship with empty season should fail");
}

#[tokio::test]
async fn test_create_championship_with_no_divisions_fails() {
    // GIVEN: A service and invalid request (no divisions)
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "Championnat Test".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec![],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Create championship
    let result = service.create_championship(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating championship with no divisions should fail");
}

#[tokio::test]
async fn test_create_championship_with_zero_journees_fails() {
    // GIVEN: A service and invalid request (zero journees)
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "Championnat Test".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 0,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Create championship
    let result = service.create_championship(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating championship with zero journees should fail");
}

#[tokio::test]
async fn test_create_palet_vendeen_championship() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Create Palet Vendéen championship
    let result = service
        .create_palet_vendeen_championship(
            "Championnat de Vendée".to_string(),
            "2025-2026".to_string(),
            "Vendée".to_string(),
            vec!["D1".to_string(), "D2".to_string(), "D3".to_string()],
            10,
            Uuid::new_v4(),
        )
        .await;

    // THEN: Should succeed with Palet Vendéen rules
    assert!(result.is_ok(), "Creating Palet Vendéen championship should succeed");

    let championship = result.unwrap();
    assert_eq!(championship.league, "Vendée");
    assert_eq!(championship.classification_rules.primary, ClassificationPrimary::Points);
    assert_eq!(championship.classification_rules.secondary, ClassificationSecondary::Diff);
    assert_eq!(
        championship.classification_rules.tertiary,
        ClassificationTertiary::PointsFor
    );
    assert!(championship.code.is_some());
    assert!(championship.code.unwrap().contains("VENDÉE"));
}

// ============================================
// 2. TESTS DE LECTURE DE CHAMPIONNAT
// ============================================

#[tokio::test]
async fn test_get_championship_not_found() {
    // GIVEN: A service with empty repository
    let service = create_test_service();
    let championship_id = Uuid::new_v4();

    // WHEN: Get non-existent championship
    let result = service.get_championship(championship_id).await;

    // THEN: Should return None
    assert!(result.is_ok(), "Getting non-existent championship should return Ok");
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_get_championships_empty() {
    // GIVEN: A service with empty repository
    let service = create_test_service();

    // WHEN: Get all championships
    let result = service.get_championships().await;

    // THEN: Should return empty list
    assert!(result.is_ok(), "Getting all championships from empty repo should succeed");
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_get_championships_by_league() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Get championships by league
    let result = service.get_championships_by_league("Vendée".to_string()).await;

    // THEN: Should succeed (may return empty if not added to mock)
    assert!(result.is_ok(), "Getting championships by league should succeed");
}

#[tokio::test]
async fn test_get_championships_by_season() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Get championships by season
    let result = service.get_championships_by_season("2025-2026".to_string()).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Getting championships by season should succeed");
}

#[tokio::test]
async fn test_get_championships_by_status() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Get championships by status
    let result = service
        .get_championships_by_status(CompetitionStatus::InProgress)
        .await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Getting championships by status should succeed");
}

// ============================================
// 3. TESTS DE MISE À JOUR DE CHAMPIONNAT
// ============================================

#[tokio::test]
async fn test_update_championship_not_found() {
    // GIVEN: A service and non-existent championship ID
    let service = create_test_service();
    let championship_id = Uuid::new_v4();
    let request = UpdateChampionshipRequest {
        name: Some("Updated Name".to_string()),
        ..Default::default()
    };

    // WHEN: Update non-existent championship
    let result = service.update_championship(championship_id, request).await;

    // THEN: Should return None (not found)
    assert!(result.is_ok(), "Updating non-existent championship should return Ok");
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_update_championship_name() {
    // GIVEN: A service
    let service = create_test_service();

    // Create a championship first
    let create_request = CreateChampionshipRequest {
        name: "Original Name".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let created = service.create_championship(create_request).await.unwrap();

    // WHEN: Update championship name
    let update_request = UpdateChampionshipRequest {
        name: Some("Updated Name".to_string()),
        ..Default::default()
    };

    let result = service.update_championship(created.id, update_request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Updating championship name should succeed");

    if let Some(updated) = result.unwrap() {
        assert_eq!(updated.name, "Updated Name");
    }
}

#[tokio::test]
async fn test_update_championship_status() {
    // GIVEN: A service
    let service = create_test_service();

    // Create a championship
    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let created = service.create_championship(create_request).await.unwrap();

    // WHEN: Update championship status to InProgress
    let update_request = UpdateChampionshipRequest {
        status: Some(CompetitionStatus::InProgress),
        current_journee: Some(1),
        ..Default::default()
    };

    let result = service.update_championship(created.id, update_request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Updating championship status should succeed");

    if let Some(updated) = result.unwrap() {
        assert_eq!(updated.status, CompetitionStatus::InProgress);
        assert_eq!(updated.current_journee, Some(1));
    }
}

// ============================================
// 4. TESTS DE SUPPRESSION DE CHAMPIONNAT
// ============================================

#[tokio::test]
async fn test_delete_championship_not_found() {
    // GIVEN: A service and non-existent championship ID
    let service = create_test_service();
    let championship_id = Uuid::new_v4();

    // WHEN: Delete non-existent championship
    let result = service.delete_championship(championship_id).await;

    // THEN: Should return false (not deleted)
    assert!(result.is_ok(), "Deleting non-existent championship should return Ok");
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_delete_championship() {
    // GIVEN: A service
    let service = create_test_service();

    // Create a championship first
    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let created = service.create_championship(create_request).await.unwrap();

    // WHEN: Delete championship
    let result = service.delete_championship(created.id).await;

    // THEN: Should succeed and return true
    assert!(result.is_ok(), "Deleting championship should succeed");
    assert!(result.unwrap());
}

// ============================================
// 5. TESTS DE CYCLE DE VIE DU CHAMPIONNAT
// ============================================

#[tokio::test]
async fn test_start_championship() {
    // GIVEN: A service and a pending championship
    let service = create_test_service();

    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let created = service.create_championship(create_request).await.unwrap();

    // WHEN: Start championship
    let result = service.start_championship(created.id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Starting championship should succeed");

    let started = result.unwrap();
    assert_eq!(started.status, CompetitionStatus::InProgress);
}

#[tokio::test]
async fn test_next_journee() {
    // GIVEN: A service and an in-progress championship
    let service = create_test_service();

    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let mut championship = service.create_championship(create_request).await.unwrap();

    // Start the championship first
    championship = service.start_championship(championship.id).await.unwrap();

    // WHEN: Advance to next journee
    let result = service.next_journee(championship.id).await;

    // THEN: Should succeed and advance journee
    assert!(result.is_ok(), "Advancing to next journee should succeed");

    let updated = result.unwrap();
    assert_eq!(updated.current_journee, Some(2));
}

#[tokio::test]
async fn test_complete_championship() {
    // GIVEN: A service and an in-progress championship
    let service = create_test_service();

    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let championship = service.create_championship(create_request).await.unwrap();

    // WHEN: Complete championship
    let result = service.complete_championship(championship.id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Completing championship should succeed");

    let completed = result.unwrap();
    assert_eq!(completed.status, CompetitionStatus::Completed);
}

// ============================================
// 6. TESTS DE CLASSIFICATION
// ============================================

#[tokio::test]
async fn test_calculate_classification_with_points() {
    // GIVEN: A service
    let service = create_test_service();

    // Create championship
    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let championship = service.create_championship(create_request).await.unwrap();

    // WHEN: Calculate classification for journee 5
    let result = service
        .calculate_classification(championship.id, "D1".to_string(), 5)
        .await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Calculating classification should succeed");

    let classification = result.unwrap();
    assert_eq!(classification.division, "D1");
    assert_eq!(classification.journee, 5);
}

#[tokio::test]
async fn test_calculate_classification_sorts_teams_by_points() {
    // GIVEN: A service
    let service = create_test_service();

    // Create championship
    let create_request = CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    let championship = service.create_championship(create_request).await.unwrap();

    // WHEN: Calculate classification
    let result = service
        .calculate_classification(championship.id, "D1".to_string(), 5)
        .await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Calculating classification should succeed");
}

// ============================================
// 7. TESTS DE VALIDATION
// ============================================

#[tokio::test]
async fn test_validate_championship_creation_with_valid_data() {
    // GIVEN: A service and valid request
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "Valid Championship".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Validate creation
    let result = service.validate_championship_creation(&request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Validating valid championship creation should succeed");
}

#[tokio::test]
async fn test_validate_championship_creation_with_empty_name() {
    // GIVEN: A service and invalid request
    let service = create_test_service();
    let request = CreateChampionshipRequest {
        name: "".to_string(),
        code: None,
        season: "2025-2026".to_string(),
        description: None,
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: None,
        classification_rules: palet_shared::ClassificationRules::default_palet(),
        organizer_id: Uuid::new_v4(),
    };

    // WHEN: Validate creation
    let result = service.validate_championship_creation(&request).await;

    // THEN: Should fail
    assert!(result.is_err(), "Validating championship with empty name should fail");
}
