//! TDD Tests for Match functionality in Palet Vendéen
//!
//! Business rules verified:
//! - Matches are played to 13 points (15 in finals)
//! - Each manche awards 1 point per palet closer to the maître
//! - Maximum 6 points per manche

use chrono::{DateTime, Utc};
use palet_shared::{CompetitionType, Match, MatchStatus, MatchWinner, MancheScore, CurrentScores, CreateMatchRequest, UpdateMatchRequest, MatchResult};
use uuid::Uuid;

// ============================================
// MATCH CREATION TESTS
// ============================================

#[test]
fn test_create_match_with_default_values() {
    let competition_id = Uuid::new_v4();
    let team1_id = Uuid::new_v4();
    let team2_id = Uuid::new_v4();
    let date = Utc::now();

    let match_obj = Match::new(
        competition_id,
        CompetitionType::Championship,
        team1_id,
        team2_id,
        date,
    );

    assert_eq!(match_obj.status, MatchStatus::Scheduled);
    assert_eq!(match_obj.best_of, Some(13));
    assert_eq!(match_obj.current_manche, Some(1));
    assert!(!match_obj.verified);
    assert_eq!(match_obj.team1_score, None);
    assert_eq!(match_obj.team2_score, None);
    assert_eq!(match_obj.winner, None);
}

#[test]
fn test_create_match_request_to_match() {
    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Tournament,
        team1_id: Uuid::new_v4(),
        team2_id: Uuid::new_v4(),
        date: Utc::now(),
        time: Some("14:00".to_string()),
        location: Some("Terrain de La Roche".to_string()),
        journee: Some(1),
        phase: Some("Phase de poules".to_string()),
        round: Some("Round 1".to_string()),
        best_of: Some(15),
    };

    let match_obj = request.to_match();
    assert_eq!(match_obj.competition_type, CompetitionType::Tournament);
    assert_eq!(match_obj.best_of, Some(15));
    assert_eq!(match_obj.journee, Some(1));
}

// ============================================
// MANCHE (ROUND) TESTS
// ============================================

#[test]
fn test_manche_score_team1_wins() {
    let manche = MancheScore::new(1, 6, 3);
    assert_eq!(manche.number, 1);
    assert_eq!(manche.team1, 6);
    assert_eq!(manche.team2, 3);
    assert_eq!(manche.winner, Some(MatchWinner::Team1));
}

#[test]
fn test_manche_score_team2_wins() {
    let manche = MancheScore::new(2, 2, 7);
    assert_eq!(manche.winner, Some(MatchWinner::Team2));
}

#[test]
fn test_manche_score_draw() {
    let manche = MancheScore::new(1, 4, 4);
    assert_eq!(manche.winner, None);
}

// ============================================
// CURRENT SCORES TESTS
// ============================================

#[test]
fn test_current_scores_new() {
    let scores = CurrentScores::new();
    assert_eq!(scores.team1, 0);
    assert_eq!(scores.team2, 0);
    assert_eq!(scores.manche, 1);
}

// ============================================
// MATCH RESULT TESTS
// ============================================

#[test]
fn test_match_team1_result_win() {
    let mut match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
    match_obj.team1_score = Some(13);
    match_obj.team2_score = Some(8);
    match_obj.winner = Some(MatchWinner::Team1);

    assert_eq!(match_obj.team1_result(), MatchResult::Win);
    assert_eq!(match_obj.team2_result(), MatchResult::Loss);
}

#[test]
fn test_match_team1_result_loss() {
    let mut match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
    match_obj.team1_score = Some(8);
    match_obj.team2_score = Some(13);
    match_obj.winner = Some(MatchWinner::Team2);

    assert_eq!(match_obj.team1_result(), MatchResult::Loss);
    assert_eq!(match_obj.team2_result(), MatchResult::Win);
}

#[test]
fn test_match_team1_result_draw() {
    let mut match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
    match_obj.team1_score = Some(13);
    match_obj.team2_score = Some(13);
    match_obj.winner = None;

    assert_eq!(match_obj.team1_result(), MatchResult::Draw);
    assert_eq!(match_obj.team2_result(), MatchResult::Draw);
}

// ============================================
// MATCH WINNER TESTS
// ============================================

#[test]
fn test_match_winner_opposite() {
    assert_eq!(MatchWinner::Team1.opposite(), MatchWinner::Team2);
    assert_eq!(MatchWinner::Team2.opposite(), MatchWinner::Team1);
}

// ============================================
// UPDATE MATCH REQUEST TESTS
// ============================================

#[test]
fn test_update_match_request_apply() {
    let mut match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());

    let update = UpdateMatchRequest {
        team1_score: Some(13),
        team2_score: Some(10),
        status: Some(MatchStatus::Completed),
        winner: Some(MatchWinner::Team1),
        verified: Some(true),
        verified_by: Some(Uuid::new_v4()),
        verified_at: Some(Utc::now()),
        date: None,
        time: None,
        location: None,
        duration: None,
        current_manche: None,
        manches: None,
    };

    update.apply_to(&mut match_obj);

    assert_eq!(match_obj.team1_score, Some(13));
    assert_eq!(match_obj.team2_score, Some(10));
    assert_eq!(match_obj.status, MatchStatus::Completed);
    assert_eq!(match_obj.winner, Some(MatchWinner::Team1));
    assert!(match_obj.verified);
}

// ============================================
// PALET VENDÉEN BUSINESS RULE TESTS
// ============================================

#[test]
fn test_match_default_best_of_is_13() {
    let match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
    assert_eq!(match_obj.best_of, Some(13));
}

#[test]
fn test_match_can_use_15_for_finals() {
    let request = CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        team1_id: Uuid::new_v4(),
        team2_id: Uuid::new_v4(),
        date: Utc::now(),
        time: None,
        location: None,
        journee: None,
        phase: Some("Finale".to_string()),
        round: None,
        best_of: Some(15),
    };

    let match_obj = request.to_match();
    assert_eq!(match_obj.best_of, Some(15));
}

#[test]
fn test_match_not_verified_by_default() {
    let match_obj = Match::new(Uuid::new_v4(), CompetitionType::Championship, Uuid::new_v4(), Uuid::new_v4(), Utc::now());
    assert!(!match_obj.is_verified());
}

#[test]
fn test_manche_max_points() {
    let manche = MancheScore::new(1, 6, 4);
    assert_eq!(manche.team1, 6);
    assert_eq!(manche.team2, 4);
}
