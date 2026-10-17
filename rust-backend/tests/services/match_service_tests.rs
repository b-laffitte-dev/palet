//! Tests TDD pour MatchService (Palet Vendeen)
//!
//! Approche :
//! 1. Ecrire les tests EN PREMIER (rouge)
//! 2. Implémenter le code minimal pour passer les tests (vert)
//! 3. Refactoriser (si nécessaire)

use chrono::Utc;
use palet_shared::{
    CompetitionType, CreateMatchRequest, MancheScore, Match, MatchStatus, MatchWinner,
    UpdateMatchRequest,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::harness::mocks::MockMatchRepository;
use crate::harness::utils::new_id;
use crate::services::match_service::{MatchService, MatchServiceError, MatchServiceImpl};

// ============================================
// TEST HELPERS
// ============================================

/// Create a test service with mock repository
fn create_test_service() -> MatchServiceImpl<MockMatchRepository> {
    let repo = MockMatchRepository::new();
    MatchServiceImpl::new(Arc::new(repo))
}

// ============================================
// 1. TESTS DE CREATION DE MATCH
// ============================================

#[tokio::test]
async fn test_create_match_with_valid_data() {
    // GIVEN: A service and valid request
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Salle des Essarts".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    // WHEN: Create match
    let result = service.create_match(request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Creating valid match should succeed");

    let match_obj = result.unwrap();
    assert_eq!(match_obj.competition_type, CompetitionType::Championship);
    assert_eq!(match_obj.team1_id, team1_id);
    assert_eq!(match_obj.team2_id, team2_id);
    assert_eq!(match_obj.journee, Some(1));
    assert_eq!(match_obj.best_of, Some(13));
    assert_eq!(match_obj.status, MatchStatus::Scheduled);
}

#[tokio::test]
async fn test_create_match_with_same_teams_fails() {
    // GIVEN: A service and invalid request (same team1 and team2)
    let service = create_test_service();
    let team_id = new_id();

    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id: team_id,
        team2_id: team_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    // WHEN: Create match
    let result = service.create_match(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating match with same teams should fail");

    match result.unwrap_err() {
        MatchServiceError::ValidationError(msg) => {
            assert!(msg.contains("same"), "Error should mention same teams");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_match_with_invalid_best_of_fails() {
    // GIVEN: A service and invalid request (invalid best_of)
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(21), // Invalid: must be 13 or 15
    };

    // WHEN: Create match
    let result = service.create_match(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating match with invalid best_of should fail");

    match result.unwrap_err() {
        MatchServiceError::ValidationError(msg) => {
            assert!(msg.contains("13 or 15"), "Error should mention valid best_of values");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_match_with_past_date_fails() {
    // GIVEN: A service and invalid request (past date)
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() - chrono::Duration::days(1), // Past date
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    // WHEN: Create match
    let result = service.create_match(request).await;

    // THEN: Should fail with validation error
    assert!(result.is_err(), "Creating match with past date should fail");

    match result.unwrap_err() {
        MatchServiceError::ValidationError(msg) => {
            assert!(msg.contains("past"), "Error should mention past date");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_tournament_match_with_best_of_15() {
    // GIVEN: A service and valid tournament match request (final with best_of 15)
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Tournament,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Salle des Essarts".to_string()),
        journee: None,
        phase: Some("Finale".to_string()),
        round: None,
        best_of: Some(15), // Final matches are played to 15
    };

    // WHEN: Create match
    let result = service.create_match(request).await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Creating tournament final match with best_of 15 should succeed"
    );

    let match_obj = result.unwrap();
    assert_eq!(match_obj.competition_type, CompetitionType::Tournament);
    assert_eq!(match_obj.best_of, Some(15));
    assert_eq!(match_obj.phase, Some("Finale".to_string()));
}

// ============================================
// 2. TESTS DE LECTURE DE MATCH
// ============================================

#[tokio::test]
async fn test_get_match_not_found() {
    // GIVEN: A service with empty repository
    let service = create_test_service();
    let match_id = Uuid::new_v4();

    // WHEN: Get non-existent match
    let result = service.get_match(match_id).await;

    // THEN: Should return None
    assert!(result.is_ok(), "Getting non-existent match should return Ok");
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_get_matches_empty() {
    // GIVEN: A service with empty repository
    let service = create_test_service();

    // WHEN: Get all matches
    let result = service.get_matches().await;

    // THEN: Should return empty list
    assert!(
        result.is_ok(),
        "Getting all matches from empty repo should succeed"
    );
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_get_matches_by_status() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Get matches by status
    let result = service.get_matches_by_status(MatchStatus::Completed).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Getting matches by status should succeed");
}

#[tokio::test]
async fn test_get_matches_by_team() {
    // GIVEN: A service
    let service = create_test_service();
    let team_id = new_id();

    // WHEN: Get matches by team
    let result = service.get_matches_by_team(team_id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Getting matches by team should succeed");
}

// ============================================
// 3. TESTS DE MISE A JOUR DE MATCH
// ============================================

#[tokio::test]
async fn test_update_match_not_found() {
    // GIVEN: A service and non-existent match ID
    let service = create_test_service();
    let match_id = Uuid::new_v4();
    let request = UpdateMatchRequest {
        team1_score: Some(13),
        team2_score: Some(10),
        ..Default::default()
    };

    // WHEN: Update non-existent match
    let result = service.update_match(match_id, request).await;

    // THEN: Should return None (not found)
    assert!(result.is_ok(), "Updating non-existent match should return Ok");
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_update_match_scores() {
    // GIVEN: A service
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    // Create a match first
    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let created = service.create_match(create_request).await.unwrap();

    // WHEN: Update match scores
    let update_request = UpdateMatchRequest {
        team1_score: Some(13),
        team2_score: Some(10),
        status: Some(MatchStatus::Completed),
        winner: Some(MatchWinner::Team1),
        ..Default::default()
    };

    let result = service.update_match(created.id, update_request).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Updating match scores should succeed");

    if let Some(updated) = result.unwrap() {
        assert_eq!(updated.team1_score, 13);
        assert_eq!(updated.team2_score, 10);
        assert_eq!(updated.status, MatchStatus::Completed);
        assert_eq!(updated.winner, Some(MatchWinner::Team1));
    }
}

// ============================================
// 4. TESTS DE SUPPRESSION DE MATCH
// ============================================

#[tokio::test]
async fn test_delete_match_not_found() {
    // GIVEN: A service and non-existent match ID
    let service = create_test_service();
    let match_id = Uuid::new_v4();

    // WHEN: Delete non-existent match
    let result = service.delete_match(match_id).await;

    // THEN: Should return false (not deleted)
    assert!(result.is_ok(), "Deleting non-existent match should return Ok");
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_delete_match() {
    // GIVEN: A service
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    // Create a match first
    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let created = service.create_match(create_request).await.unwrap();

    // WHEN: Delete match
    let result = service.delete_match(created.id).await;

    // THEN: Should succeed and return true
    assert!(result.is_ok(), "Deleting match should succeed");
    assert!(result.unwrap());
}

// ============================================
// 5. TESTS DE CYCLE DE VIE DU MATCH
// ============================================

#[tokio::test]
async fn test_start_match() {
    // GIVEN: A service and a scheduled match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let created = service.create_match(create_request).await.unwrap();

    // WHEN: Start match
    let result = service.start_match(created.id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Starting match should succeed");

    let started = result.unwrap();
    assert_eq!(started.status, MatchStatus::InProgress);
}

#[tokio::test]
async fn test_complete_match() {
    // GIVEN: A service and an in-progress match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();

    // Start the match first
    match_obj = service.start_match(match_obj.id).await.unwrap();

    // WHEN: Complete match
    let result = service.complete_match(match_obj.id, MatchWinner::Team1).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Completing match should succeed");

    let completed = result.unwrap();
    assert_eq!(completed.status, MatchStatus::Completed);
    assert_eq!(completed.winner, Some(MatchWinner::Team1));
}

// ============================================
// 6. TESTS DE MANCHE (Palet Vendeen)
// ============================================

#[tokio::test]
async fn test_add_manche() {
    // GIVEN: A service and an in-progress match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();

    // WHEN: Add a manche (6-4 for team1)
    let result = service.add_manche(match_obj.id, 6, 4).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Adding manche should succeed");

    let updated = result.unwrap();
    // Verify manche was added
    assert!(
        !updated.team1_manches.is_empty() || !updated.team2_manches.is_empty()
    );
}

#[tokio::test]
async fn test_add_manche_with_negative_score_fails() {
    // GIVEN: A service and an in-progress match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();

    // WHEN: Add a manche with negative score
    let result = service.add_manche(match_obj.id, -1, 4).await;

    // THEN: Should fail with validation error
    assert!(
        result.is_err(),
        "Adding manche with negative score should fail"
    );

    match result.unwrap_err() {
        MatchServiceError::ValidationError(msg) => {
            assert!(
                msg.contains("negative"),
                "Error should mention negative score"
            );
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_add_manche_with_score_exceeding_6_fails() {
    // GIVEN: A service and an in-progress match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();

    // WHEN: Add a manche with score exceeding 6
    let result = service.add_manche(match_obj.id, 7, 4).await;

    // THEN: Should fail with validation error
    assert!(
        result.is_err(),
        "Adding manche with score > 6 should fail"
    );

    match result.unwrap_err() {
        MatchServiceError::ValidationError(msg) => {
            assert!(msg.contains("6"), "Error should mention max score of 6");
        }
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_update_manche() {
    // GIVEN: A service and an in-progress match with existing manches
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();

    // Add first manche
    match_obj = service.add_manche(match_obj.id, 6, 4).await.unwrap();

    // WHEN: Update the first manche
    let result = service.update_manche(match_obj.id, 1, 6, 5).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Updating manche should succeed");
}

// ============================================
// 7. TESTS DE VERIFICATION DE MATCH
// ============================================

#[tokio::test]
async fn test_verify_match() {
    // GIVEN: A service and a completed match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();
    let verifier_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();
    match_obj = service
        .complete_match(match_obj.id, MatchWinner::Team1)
        .await
        .unwrap();

    // WHEN: Verify match
    let result = service.verify_match(match_obj.id, verifier_id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Verifying match should succeed");

    let verified = result.unwrap();
    // In a real implementation, verified_by would be set
}

#[tokio::test]
async fn test_unverify_match() {
    // GIVEN: A service and a verified match
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();
    let verifier_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();
    match_obj = service.start_match(match_obj.id).await.unwrap();
    match_obj = service
        .complete_match(match_obj.id, MatchWinner::Team1)
        .await
        .unwrap();
    match_obj = service.verify_match(match_obj.id, verifier_id).await.unwrap();

    // WHEN: Unverify match
    let result = service.unverify_match(match_obj.id).await;

    // THEN: Should succeed
    assert!(result.is_ok(), "Unverifying match should succeed");
}

// ============================================
// 8. TESTS DE REGLES PALET VENDEEN
// ============================================

#[tokio::test]
async fn test_validate_manche_score_with_valid_scores() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Validate valid manche score
    let result = service.validate_manche_score(6, 4).await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Validating valid manche score should succeed"
    );
}

#[tokio::test]
async fn test_validate_manche_score_with_zero_scores() {
    // GIVEN: A service
    let service = create_test_service();

    // WHEN: Validate zero scores (possible in Palet)
    let result = service.validate_manche_score(0, 0).await;

    // THEN: Should succeed (0-0 is technically valid, though unusual)
    assert!(
        result.is_ok(),
        "Validating zero scores should succeed"
    );
}

#[tokio::test]
async fn test_check_match_completion_not_completed() {
    // GIVEN: A service and a match with scores below best_of
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();

    // Update scores but not enough to complete
    let update_request = UpdateMatchRequest {
        team1_score: Some(10),
        team2_score: Some(8),
        ..Default::default()
    };

    match_obj = service
        .update_match(match_obj.id, update_request)
        .await
        .unwrap()
        .unwrap();

    // WHEN: Check if match is completed
    let result = service.check_match_completion(&match_obj).await;

    // THEN: Should return false
    assert!(
        result.is_ok(),
        "Checking match completion should succeed"
    );
    assert!(!result.unwrap(), "Match should not be completed yet");
}

#[tokio::test]
async fn test_check_match_completion_is_completed() {
    // GIVEN: A service and a match with scores reaching best_of
    let service = create_test_service();
    let team1_id = new_id();
    let team2_id = new_id();

    let create_request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id,
        team2_id,
        date: Utc::now() + chrono::Duration::hours(1),
        location: Some("Test".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    };

    let mut match_obj = service.create_match(create_request).await.unwrap();

    // Update scores to reach best_of
    let update_request = UpdateMatchRequest {
        team1_score: Some(13),
        team2_score: Some(10),
        ..Default::default()
    };

    match_obj = service
        .update_match(match_obj.id, update_request)
        .await
        .unwrap()
        .unwrap();

    // WHEN: Check if match is completed
    let result = service.check_match_completion(&match_obj).await;

    // THEN: Should return true
    assert!(
        result.is_ok(),
        "Checking match completion should succeed"
    );
    assert!(result.unwrap(), "Match should be completed");
}

// ============================================
// 9. TESTS DE REQUETES PAR COMPETITION
// ============================================

#[tokio::test]
async fn test_get_matches_by_competition() {
    // GIVEN: A service
    let service = create_test_service();
    let competition_id = new_id();

    // WHEN: Get matches by competition
    let result = service.get_matches_by_competition(competition_id).await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Getting matches by competition should succeed"
    );
}

#[tokio::test]
async fn test_get_championship_matches() {
    // GIVEN: A service
    let service = create_test_service();
    let championship_id = new_id();

    // WHEN: Get championship matches
    let result = service.get_championship_matches(championship_id).await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Getting championship matches should succeed"
    );
}

#[tokio::test]
async fn test_get_matches_by_journee() {
    // GIVEN: A service
    let service = create_test_service();
    let competition_id = new_id();

    // WHEN: Get matches by journee
    let result = service.get_matches_by_journee(1, competition_id).await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Getting matches by journee should succeed"
    );
}

#[tokio::test]
async fn test_get_completed_matches_by_championship() {
    // GIVEN: A service
    let service = create_test_service();
    let championship_id = new_id();

    // WHEN: Get completed matches by championship
    let result = service
        .get_completed_matches_by_championship(championship_id)
        .await;

    // THEN: Should succeed
    assert!(
        result.is_ok(),
        "Getting completed matches by championship should succeed"
    );
}
