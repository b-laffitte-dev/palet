//! Custom assertions for Palet Vendéen TDD tests
//!
//! This module provides custom assertion utilities for testing championships, matches, and related entities.

use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, CompetitionStatus, Match, MatchResult,
    MatchStatus, MatchWinner, Player, Team, TeamClassificationStatus, TeamStats,
};
use uuid::Uuid;

// ============================================
// CHAMPIONSHIP ASSERTIONS
// ============================================

/// Assert that a championship has the expected properties
#[macro_export]
macro_rules! assert_championship {
    ($championship:expr, $expected:expr) => {
        assert_eq!($championship.id, $expected.id);
        assert_eq!($championship.name, $expected.name);
        assert_eq!($championship.season, $expected.season);
        assert_eq!($championship.league, $expected.league);
        assert_eq!($championship.divisions, $expected.divisions);
        assert_eq!($championship.total_journees, $expected.total_journees);
        assert_eq!($championship.status, $expected.status);
        assert_eq!($championship.organizer_id, $expected.organizer_id);
    };
}

/// Assert that a championship is in the expected state
pub fn assert_championship_status(championship: &Championship, expected_status: CompetitionStatus) {
    assert_eq!(
        championship.status, expected_status,
        "Expected championship status to be {:?}, but was {:?}",
        expected_status, championship.status
    );
}

/// Assert that a championship has the expected journee
pub fn assert_championship_journee(championship: &Championship, expected_journee: Option<i32>) {
    assert_eq!(
        championship.current_journee, expected_journee,
        "Expected championship current_journee to be {:?}, but was {:?}",
        expected_journee, championship.current_journee
    );
}

/// Assert that a championship has the expected classification rules
pub fn assert_championship_classification_rules(
    championship: &Championship,
    primary: ClassificationPrimary,
    secondary: ClassificationSecondary,
    tertiary: ClassificationTertiary,
) {
    assert_eq!(
        championship.classification_rules.primary, primary,
        "Expected primary classification rule to be {:?}",
        primary
    );
    assert_eq!(
        championship.classification_rules.secondary, secondary,
        "Expected secondary classification rule to be {:?}",
        secondary
    );
    assert_eq!(
        championship.classification_rules.tertiary, tertiary,
        "Expected tertiary classification rule to be {:?}",
        tertiary
    );
}

// ============================================
// MATCH ASSERTIONS
// ============================================

/// Assert that a match has the expected properties
#[macro_export]
macro_rules! assert_match {
    ($match:expr, $expected:expr) => {
        assert_eq!($match.id, $expected.id);
        assert_eq!($match.competition_id, $expected.competition_id);
        assert_eq!($match.team1_id, $expected.team1_id);
        assert_eq!($match.team2_id, $expected.team2_id);
        assert_eq!($match.status, $expected.status);
        assert_eq!($match.best_of, $expected.best_of);
    };
}

/// Assert that a match is in the expected state
pub fn assert_match_status(match_obj: &Match, expected_status: MatchStatus) {
    assert_eq!(
        match_obj.status, expected_status,
        "Expected match status to be {:?}, but was {:?}",
        expected_status, match_obj.status
    );
}

/// Assert that a match has the expected winner
pub fn assert_match_winner(match_obj: &Match, expected_winner: Option<MatchWinner>) {
    assert_eq!(
        match_obj.winner, expected_winner,
        "Expected match winner to be {:?}, but was {:?}",
        expected_winner, match_obj.winner
    );
}

/// Assert that a match has the expected scores
pub fn assert_match_scores(
    match_obj: &Match,
    expected_team1_score: Option<i32>,
    expected_team2_score: Option<i32>,
) {
    assert_eq!(
        match_obj.team1_score, expected_team1_score,
        "Expected team1 score to be {:?}, but was {:?}",
        expected_team1_score, match_obj.team1_score
    );
    assert_eq!(
        match_obj.team2_score, expected_team2_score,
        "Expected team2 score to be {:?}, but was {:?}",
        expected_team2_score, match_obj.team2_score
    );
}

/// Assert that a match is completed when a team reaches the target score
pub fn assert_match_completed_at_target(
    match_obj: &Match,
    expected_winner: MatchWinner,
    target_score: i32,
) {
    assert_eq!(match_obj.status, MatchStatus::Completed);
    assert_eq!(match_obj.winner, Some(expected_winner));
    
    match expected_winner {
        MatchWinner::Team1 => {
            assert!(
                match_obj.team1_score.unwrap_or(0) >= target_score,
                "Team1 should have at least {} points",
                target_score
            );
        }
        MatchWinner::Team2 => {
            assert!(
                match_obj.team2_score.unwrap_or(0) >= target_score,
                "Team2 should have at least {} points",
                target_score
            );
        }
        MatchWinner::Draw => {
            // For draws, both teams should have equal scores
            assert_eq!(
                match_obj.team1_score, match_obj.team2_score,
                "Draw requires equal scores"
            );
        }
    }
}

// ============================================
// CLASSIFICATION ASSERTIONS
// ============================================

/// Assert that teams are sorted correctly according to Palet Vendéen rules
/// Rules: 1. Points, 2. Diff, 3. Points For
pub fn assert_teams_sorted_by_palet_rules(teams: &[ChampionshipTeam]) {
    for i in 0..teams.len() - 1 {
        let current = &teams[i];
        let next = &teams[i + 1];

        // First by points (descending)
        if current.stats.points != next.stats.points {
            assert!(
                current.stats.points >= next.stats.points,
                "Team at position {} should have more or equal points than team at position {}",
                i,
                i + 1
            );
            continue;
        }

        // If points are equal, by diff (descending)
        if current.stats.diff != next.stats.diff {
            assert!(
                current.stats.diff >= next.stats.diff,
                "Team at position {} should have better or equal diff than team at position {}",
                i,
                i + 1
            );
            continue;
        }

        // If diff is equal, by points for (descending)
        assert!(
            current.stats.points_for >= next.stats.points_for,
            "Team at position {} should have more or equal points_for than team at position {}",
            i,
            i + 1
        );
    }
}

/// Assert that a classification has the expected number of teams
pub fn assert_classification_team_count(
    classification: &ChampionshipClassification,
    expected_count: usize,
) {
    assert_eq!(
        classification.teams.len(),
        expected_count,
        "Expected {} teams in classification, but found {}",
        expected_count,
        classification.teams.len()
    );
}

// ============================================
// CLUB ASSERTIONS
// ============================================

/// Assert that a club has the expected properties
pub fn assert_club_properties(club: &Club, expected_name: &str, expected_league: &str) {
    assert_eq!(club.name, expected_name, "Club name mismatch");
    assert_eq!(club.league, expected_league, "Club league mismatch");
}

/// Assert that a club is active
pub fn assert_club_active(club: &Club) {
    assert_eq!(club.status, palet_shared::ClubStatus::Active, "Club should be active");
}

// ============================================
// PLAYER ASSERTIONS
// ============================================

/// Assert that a player has the expected properties
pub fn assert_player_properties(player: &Player, expected_first_name: &str, expected_last_name: &str) {
    assert_eq!(player.first_name, expected_first_name, "Player first name mismatch");
    assert_eq!(player.last_name, expected_last_name, "Player last name mismatch");
}

/// Assert that a player is active
pub fn assert_player_active(player: &Player) {
    assert_eq!(
        player.status,
        palet_shared::PlayerStatus::Active,
        "Player should be active"
    );
}

// ============================================
// TEAM ASSERTIONS
// ============================================

/// Assert that a team has the expected properties
pub fn assert_team_properties(team: &Team, expected_name: &str, expected_club_id: Uuid) {
    assert_eq!(team.name, expected_name, "Team name mismatch");
    assert_eq!(team.club_id, expected_club_id, "Team club_id mismatch");
}

/// Assert that a team has the expected number of players
pub fn assert_team_player_count(team: &Team, expected_count: usize) {
    assert_eq!(
        team.players.len(),
        expected_count,
        "Expected {} players in team, but found {}",
        expected_count,
        team.players.len()
    );
}

// ============================================
// PALET VENDÉEN RULES ASSERTIONS
// ============================================

/// Assert that a manche score is valid (0-6 for each team)
pub fn assert_valid_manche_score(team1_score: i32, team2_score: i32) {
    assert!(
        team1_score >= 0 && team1_score <= 6,
        "Team1 manche score must be between 0 and 6, got {}",
        team1_score
    );
    assert!(
        team2_score >= 0 && team2_score <= 6,
        "Team2 manche score must be between 0 and 6, got {}",
        team2_score
    );
    assert_ne!(
        team1_score, team2_score,
        "Manche scores cannot be equal (no draws in manche)"
    );
}

/// Assert that a championship match has the correct target score (13 points)
pub fn assert_championship_match_target(match_obj: &Match, expected_target: i32) {
    assert_eq!(
        match_obj.best_of,
        Some(expected_target),
        "Championship match should have best_of = {}",
        expected_target
    );
}

/// Assert that a tournament final match has the correct target score (15 points)
pub fn assert_tournament_final_match_target(match_obj: &Match, expected_target: i32) {
    if match_obj.phase == Some("Finale".to_string()) {
        assert_eq!(
            match_obj.best_of,
            Some(expected_target),
            "Tournament final match should have best_of = {}",
            expected_target
        );
    }
}

// ============================================
// ERROR ASSERTIONS
// ============================================

/// Assert that a result is an error
pub fn assert_is_error<T, E>(result: &Result<T, E>) {
    assert!(result.is_err(), "Expected an error, but got Ok");
}

/// Assert that a result is ok
pub fn assert_is_ok<T, E>(result: &Result<T, E>) {
    assert!(result.is_ok(), "Expected Ok, but got an error: {:?}", result);
}

/// Assert that a result is ok and contains the expected value
pub fn assert_ok_with<T: PartialEq, E>(result: &Result<T, E>, expected: &T) {
    assert!(result.is_ok(), "Expected Ok, but got an error");
    assert_eq!(result.as_ref().unwrap(), expected);
}
