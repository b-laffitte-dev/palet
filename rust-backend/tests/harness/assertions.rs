//! Assertions for Palet Vendéen TDD harness
//!
//! This module provides custom assertion utilities for testing Palet Vendéen functionality.

use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationRules,
    Club, CompetitionStatus, Match, MatchStatus, TeamStats, Tournament,
};
use std::fmt;
use uuid::Uuid;

// ============================================
// CHAMPIONSHIP ASSERTIONS
// ============================================

/// Assert that a championship has the expected properties
pub fn assert_championship_properties(
    championship: &Championship,
    expected_name: &str,
    expected_season: &str,
    expected_league: &str,
    expected_divisions: &[String],
    expected_total_journees: i32,
) {
    assert_eq!(
        championship.name, expected_name,
        "Championship name mismatch: expected '{}', got '{}'",
        expected_name, championship.name
    );
    assert_eq!(
        championship.season, expected_season,
        "Championship season mismatch: expected '{}', got '{}'",
        expected_season, championship.season
    );
    assert_eq!(
        championship.league, expected_league,
        "Championship league mismatch: expected '{}', got '{}'",
        expected_league, championship.league
    );
    assert_eq!(
        championship.divisions, expected_divisions,
        "Championship divisions mismatch"
    );
    assert_eq!(
        championship.total_journees, expected_total_journees,
        "Championship total_journees mismatch"
    );
}

/// Assert that a championship has the expected classification rules
pub fn assert_championship_classification_rules(
    championship: &Championship,
    expected_rules: &ClassificationRules,
) {
    assert_eq!(
        championship.classification_rules.primary,
        expected_rules.primary,
        "Classification primary rule mismatch"
    );
    assert_eq!(
        championship.classification_rules.secondary,
        expected_rules.secondary,
        "Classification secondary rule mismatch"
    );
    assert_eq!(
        championship.classification_rules.tertiary,
        expected_rules.tertiary,
        "Classification tertiary rule mismatch"
    );
}

/// Assert that a championship is in the expected status
pub fn assert_championship_status(
    championship: &Championship,
    expected_status: CompetitionStatus,
) {
    assert_eq!(
        championship.status, expected_status,
        "Championship status mismatch: expected {:?}, got {:?}",
        expected_status, championship.status
    );
}

/// Assert that a championship is at the expected journee
pub fn assert_championship_journee(
    championship: &Championship,
    expected_journee: Option<i32>,
) {
    assert_eq!(
        championship.current_journee, expected_journee,
        "Championship current_journee mismatch"
    );
}

// ============================================
// CLASSIFICATION ASSERTIONS
// ============================================

/// Assert that teams are sorted correctly according to classification rules
pub fn assert_teams_sorted(
    classification: &ChampionshipClassification,
    rules: &ClassificationRules,
) {
    let teams = &classification.teams;
    
    for i in 0..teams.len() - 1 {
        let team1 = &teams[i];
        let team2 = &teams[i + 1];
        
        // Team at position i should be better than team at position i+1
        let cmp = rules.compare_teams(team1, team2);
        assert!(
            cmp != std::cmp::Ordering::Less,
            "Teams not sorted correctly: team at position {} should be better than team at position {}",
            i, i + 1
        );
    }
    
    // Verify positions are sequential
    for (index, team) in teams.iter().enumerate() {
        assert_eq!(
            team.position, (index + 1) as i32,
            "Team position mismatch: expected {}, got {}",
            index + 1, team.position
        );
    }
}

/// Assert that a team has the expected position in classification
pub fn assert_team_position(
    classification: &ChampionshipClassification,
    team_id: Uuid,
    expected_position: i32,
) {
    let team = classification
        .teams
        .iter()
        .find(|t| t.team_id == team_id)
        .expect("Team not found in classification");
    
    assert_eq!(
        team.position, expected_position,
        "Team position mismatch: expected {}, got {}",
        expected_position, team.position
    );
}

/// Assert that a team has the expected stats
pub fn assert_team_stats(
    team: &ChampionshipTeam,
    expected_stats: &TeamStats,
) {
    assert_eq!(
        team.stats.matches, expected_stats.matches,
        "Team matches count mismatch"
    );
    assert_eq!(
        team.stats.wins, expected_stats.wins,
        "Team wins count mismatch"
    );
    assert_eq!(
        team.stats.draws, expected_stats.draws,
        "Team draws count mismatch"
    );
    assert_eq!(
        team.stats.losses, expected_stats.losses,
        "Team losses count mismatch"
    );
    assert_eq!(
        team.stats.points, expected_stats.points,
        "Team points mismatch"
    );
    assert_eq!(
        team.stats.points_for, expected_stats.points_for,
        "Team points_for mismatch"
    );
    assert_eq!(
        team.stats.points_against, expected_stats.points_against,
        "Team points_against mismatch"
    );
    assert_eq!(
        team.stats.diff, expected_stats.diff,
        "Team diff mismatch"
    );
}

// ============================================
// CLUB ASSERTIONS
// ============================================

/// Assert that a club has the expected properties
pub fn assert_club_properties(
    club: &Club,
    expected_name: &str,
    expected_code: &str,
    expected_league: &str,
    expected_division: &str,
) {
    assert_eq!(club.name, expected_name, "Club name mismatch");
    assert_eq!(club.code, expected_code, "Club code mismatch");
    assert_eq!(club.league, expected_league, "Club league mismatch");
    assert_eq!(club.division, expected_division, "Club division mismatch");
}

// ============================================
// MATCH ASSERTIONS
// ============================================

/// Assert that a match has the expected properties
pub fn assert_match_properties(
    match_obj: &Match,
    expected_team1_id: Uuid,
    expected_team2_id: Uuid,
    expected_status: MatchStatus,
) {
    assert_eq!(match_obj.team1_id, expected_team1_id, "Match team1_id mismatch");
    assert_eq!(match_obj.team2_id, expected_team2_id, "Match team2_id mismatch");
    assert_eq!(match_obj.status, expected_status, "Match status mismatch");
}

/// Assert that a match has the expected score
pub fn assert_match_score(
    match_obj: &Match,
    expected_team1_score: i32,
    expected_team2_score: i32,
) {
    assert_eq!(
        match_obj.team1_score, expected_team1_score,
        "Match team1_score mismatch"
    );
    assert_eq!(
        match_obj.team2_score, expected_team2_score,
        "Match team2_score mismatch"
    );
}

// ============================================
// TOURNAMENT ASSERTIONS
// ============================================

/// Assert that a tournament has the expected properties
pub fn assert_tournament_properties(
    tournament: &Tournament,
    expected_name: &str,
    expected_category: &str,
    expected_phase: &str,
) {
    assert_eq!(tournament.name, expected_name, "Tournament name mismatch");
    assert_eq!(
        tournament.category, expected_category,
        "Tournament category mismatch"
    );
    assert_eq!(
        tournament.current_phase, expected_phase,
        "Tournament current_phase mismatch"
    );
}

// ============================================
// CUSTOM ERROR TYPES FOR BETTER ERROR MESSAGES
// ============================================

/// Custom error type for assertion failures
#[derive(Debug)]
pub struct AssertionError {
    pub message: String,
    pub expected: String,
    pub actual: String,
}

impl AssertionError {
    pub fn new(message: &str, expected: &str, actual: &str) -> Self {
        Self {
            message: message.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }
}

impl fmt::Display for AssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: expected '{}', got '{}'",
            self.message, self.expected, self.actual
        )
    }
}

impl std::error::Error for AssertionError {}

// ============================================
// MACROS FOR CONCISE ASSERTIONS
// ============================================

/// Macro to assert that two values are equal with a custom message
#[macro_export]
macro_rules! assert_eq_with_msg {
    ($left:expr, $right:expr, $msg:expr) => {
        assert_eq!($left, $right, "{}", $msg);
    };
}

/// Macro to assert that a result is Ok with a custom message
#[macro_export]
macro_rules! assert_ok_with_msg {
    ($result:expr, $msg:expr) => {
        assert!($result.is_ok(), "{}: {:?}", $msg, $result);
    };
}

/// Macro to assert that a result is Err with a custom message
#[macro_export]
macro_rules! assert_err_with_msg {
    ($result:expr, $msg:expr) => {
        assert!($result.is_err(), "{}: {:?}", $msg, $result);
    };
}
