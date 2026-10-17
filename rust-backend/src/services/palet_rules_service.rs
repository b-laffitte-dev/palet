//! Palet Vendéen Rules Service - Business logic for Palet rules
//!
//! This module provides the specific business logic for Palet Vendéen rules.
//! It implements the TDD harness requirements for Palet-specific functionality.

use async_trait::async_trait;
use palet_shared::{
    Championship, CompetitionCategory, CompetitionMaterial, CompetitionType, MancheScore,
    Match, MatchStatus, MatchWinner, PointsType,
};
use std::sync::Arc;
use uuid::Uuid;

// ============================================
// PALET RULES SERVICE TRAIT (Interface)
// ============================================

/// Trait defining the interface for Palet Vendéen rules service
#[async_trait]
pub trait PaletRulesService {
    // Match Rules
    /// Check if a match should be played to 13 or 15 points
    fn get_match_points_target(
        &self,
        competition_type: CompetitionType,
        phase: Option<String>,
    ) -> i32;

    /// Validate a manche score according to Palet Vendéen rules
    fn validate_manche_score(
        &self,
        team1_score: i32,
        team2_score: i32,
        manche_number: i32,
    ) -> Result<(), PaletRulesError>;

    /// Calculate the winner of a manche
    fn calculate_manche_winner(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Option<MatchWinner>;

    /// Check if a match is completed based on scores
    fn is_match_completed(
        &self,
        team1_score: i32,
        team2_score: i32,
        best_of: i32,
    ) -> bool;

    /// Determine the winner of a completed match
    fn determine_match_winner(
        &self,
        team1_score: i32,
        team2_score: i32,
    ) -> Option<MatchWinner>;

    // Championship Rules
    /// Get the classification rules for a Palet Vendéen championship
    fn get_championship_classification_rules(
        &self,
    ) -> palet_shared::ClassificationRules;

    /// Calculate team classification based on Palet Vendéen rules
    fn calculate_team_classification(
        &self,
        teams: Vec<palet_shared::ChampionshipTeam>,
    ) -> Vec<palet_shared::ChampionshipTeam>;

    // Scoring Rules
    /// Calculate points for a win in championship
    fn get_win_points(&self) -> i32;

    /// Calculate points for a draw in championship
    fn get_draw_points(&self) -> i32;

    /// Calculate points for a loss in championship
    fn get_loss_points(&self) -> i32;

    /// Calculate team stats based on match results
    fn calculate_team_stats(
        &self,
        matches: Vec<Match>,
        team_id: Uuid,
    ) -> palet_shared::TeamStats;

    // Validation Rules
    /// Validate that a championship follows Palet Vendéen rules
    fn validate_championship_rules(
        &self,
        championship: &Championship,
    ) -> Result<(), PaletRulesError>;

    /// Validate that a match follows Palet Vendéen rules
    fn validate_match_rules(
        &self,
        match_obj: &Match,
    ) -> Result<(), PaletRulesError>;
}

// ============================================
// PALET RULES SERVICE ERROR
// ============================================

/// Error type for Palet rules service operations
#[derive(Debug, thiserror::Error)]
pub enum PaletRulesError {
    #[error("Rule violation: {0}")]
    RuleViolation(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid score: {0}")]
    InvalidScore(String),
}

// ============================================
// PALET RULES SERVICE (Implementation)
// ============================================

/// Service implementation for Palet Vendéen rules
#[derive(Clone, Default)]
pub struct PaletRulesServiceImpl;

impl PaletRulesServiceImpl {
    /// Create a new PaletRulesServiceImpl
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PaletRulesService for PaletRulesServiceImpl {
    fn get_match_points_target(
        &self,
        competition_type: CompetitionType,
        phase: Option<String>,
    ) -> i32 {
        // In Palet Vendéen:
        // - Normal matches: 13 points
        // - Finals (la balle): 15 points
        if let Some(phase) = phase {
            let phase_lower = phase.to_lowercase();
            if phase_lower.contains("final") || phase_lower.contains("balle") {
                return 15;
            }
        }

        // Default to 13 points
        13
    }

    fn validate_manche_score(
        &self,
        team1_score: i32,
        team2_score: i32,
        manche_number: i32,
    ) -> Result<(), PaletRulesError> {
        // Validate scores are non-negative
        if team1_score < 0 || team2_score < 0 {
            return Err(PaletRulesError::InvalidScore(
                "Scores cannot be negative".to_string(),
            ));
        }

        // Validate scores are within valid range
        // In Palet Vendéen, maximum score per manche depends on the format:
        // - Doublette (2 players): max 6 points per manche
        // - Triplette (3 players): max 9 points per manche
        // For now, we use 6 as the maximum (most common)
        if team1_score > 6 || team2_score > 6 {
            return Err(PaletRulesError::InvalidScore(format!(
                "Scores cannot exceed 6 per manche (got {} and {})",
                team1_score, team2_score
            )));
        }

        // In Palet Vendéen, a manche must have a winner (no draws)
        // However, we allow draws in the database for flexibility
        // The business logic will handle the winner determination

        // Validate total score doesn't exceed match target
        let total_score = team1_score + team2_score;
        if total_score > 6 {
            return Err(PaletRulesError::InvalidScore(format!(
                "Total score per manche cannot exceed 6 (got {})",
                total_score
            )));
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
            None // Draw (shouldn't happen in Palet, but possible in database)
        }
    }

    fn is_match_completed(
        &self,
        team1_score: i32,
        team2_score: i32,
        best_of: i32,
    ) -> bool {
        team1_score >= best_of || team2_score >= best_of
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
            None // Draw
        }
    }

    fn get_championship_classification_rules(
        &self,
    ) -> palet_shared::ClassificationRules {
        // Palet Vendéen standard classification rules:
        // 1. Points (descending)
        // 2. Diff (goal difference, descending)
        // 3. Points For (Bp - bons points, descending)
        palet_shared::ClassificationRules {
            primary: palet_shared::ClassificationPrimary::Points,
            secondary: palet_shared::ClassificationSecondary::Diff,
            tertiary: palet_shared::ClassificationTertiary::PointsFor,
        }
    }

    fn calculate_team_classification(
        &self,
        mut teams: Vec<palet_shared::ChampionshipTeam>,
    ) -> Vec<palet_shared::ChampionshipTeam> {
        // Sort teams based on Palet Vendéen classification rules
        teams.sort_by(|a, b| {
            // Primary: Points (descending)
            b.stats.points.cmp(&a.stats.points)
                // Secondary: Diff (descending)
                .then_with(|| b.stats.diff.cmp(&a.stats.diff))
                // Tertiary: Points For (descending)
                .then_with(|| b.stats.points_for.cmp(&a.stats.points_for))
        });

        // Update positions
        for (index, team) in teams.iter_mut().enumerate() {
            team.position = (index + 1) as i32;
        }

        teams
    }

    fn get_win_points(&self) -> i32 {
        // Standard Palet Vendéen: 3 points for a win
        3
    }

    fn get_draw_points(&self) -> i32 {
        // Standard Palet Vendéen: 1 point for a draw
        1
    }

    fn get_loss_points(&self) -> i32 {
        // Standard Palet Vendéen: 0 points for a loss
        0
    }

    fn calculate_team_stats(
        &self,
        matches: Vec<Match>,
        team_id: Uuid,
    ) -> palet_shared::TeamStats {
        let mut matches_played = 0;
        let mut wins = 0;
        let mut draws = 0;
        let mut losses = 0;
        let mut points_for = 0;
        let mut points_against = 0;

        for match_obj in matches {
            // Check if this team is in the match
            let is_team1 = match_obj.team1_id == team_id;
            let is_team2 = match_obj.team2_id == team_id;

            if !is_team1 && !is_team2 {
                continue; // Team not in this match
            }

            matches_played += 1;

            if let (Some(team1_score), Some(team2_score)) = (match_obj.team1_score, match_obj.team2_score) {
                if is_team1 {
                    points_for += team1_score;
                    points_against += team2_score;

                    if team1_score > team2_score {
                        wins += 1;
                    } else if team1_score < team2_score {
                        losses += 1;
                    } else {
                        draws += 1;
                    }
                } else if is_team2 {
                    points_for += team2_score;
                    points_against += team1_score;

                    if team2_score > team1_score {
                        wins += 1;
                    } else if team2_score < team1_score {
                        losses += 1;
                    } else {
                        draws += 1;
                    }
                }
            }
        }

        let diff = points_for - points_against;
        let points = wins * self.get_win_points() + draws * self.get_draw_points();
        let win_rate = if matches_played > 0 {
            wins as f64 / matches_played as f64
        } else {
            0.0
        };

        palet_shared::TeamStats {
            matches: matches_played,
            wins,
            draws,
            losses,
            points_for,
            points_against,
            diff,
            points,
            win_rate,
        }
    }

    fn validate_championship_rules(
        &self,
        championship: &Championship,
    ) -> Result<(), PaletRulesError> {
        // Validate classification rules
        let expected_rules = self.get_championship_classification_rules();

        if championship.classification_rules.primary != expected_rules.primary {
            return Err(PaletRulesError::RuleViolation(format!(
                "Invalid primary classification rule: expected {:?}, got {:?}",
                expected_rules.primary, championship.classification_rules.primary
            )));
        }

        if championship.classification_rules.secondary != expected_rules.secondary {
            return Err(PaletRulesError::RuleViolation(format!(
                "Invalid secondary classification rule: expected {:?}, got {:?}",
                expected_rules.secondary, championship.classification_rules.secondary
            )));
        }

        if championship.classification_rules.tertiary != expected_rules.tertiary {
            return Err(PaletRulesError::RuleViolation(format!(
                "Invalid tertiary classification rule: expected {:?}, got {:?}",
                expected_rules.tertiary, championship.classification_rules.tertiary
            )));
        }

        // Validate total journees
        if championship.total_journees <= 0 {
            return Err(PaletRulesError::ValidationError(
                "Championship must have at least one journee".to_string(),
            ));
        }

        // Validate divisions
        if championship.divisions.is_empty() {
            return Err(PaletRulesError::ValidationError(
                "Championship must have at least one division".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_match_rules(
        &self,
        match_obj: &Match,
    ) -> Result<(), PaletRulesError> {
        // Validate best_of value
        if let Some(best_of) = match_obj.best_of {
            if best_of != 13 && best_of != 15 {
                return Err(PaletRulesError::RuleViolation(format!(
                    "Invalid best_of value for Palet Vendéen: expected 13 or 15, got {}",
                    best_of
                )));
            }
        }

        // Validate scores
        if let (Some(team1_score), Some(team2_score)) = (match_obj.team1_score, match_obj.team2_score) {
            if team1_score < 0 || team2_score < 0 {
                return Err(PaletRulesError::InvalidScore(
                    "Scores cannot be negative".to_string(),
                ));
            }

            // Check if match is completed but scores don't match best_of
            if match_obj.status == MatchStatus::Completed {
                let best_of = match_obj.best_of.unwrap_or(13);
                if team1_score < best_of && team2_score < best_of {
                    return Err(PaletRulesError::RuleViolation(
                        "Completed match must have at least one team reaching the best_of score".to_string(),
                    ));
                }
            }
        }

        // Validate manches
        if let Some(manches) = &match_obj.manches {
            for (index, manche) in manches.iter().enumerate() {
                self.validate_manche_score(manche.team1, manche.team2, (index + 1) as i32)?;
            }
        }

        Ok(())
    }
}

// ============================================
// UNIT TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_match_points_target_normal() {
        // GIVEN: A normal match (not final)
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting points target
        let target = service.get_match_points_target(CompetitionType::Championnat, None);

        // THEN: Should be 13
        assert_eq!(target, 13);
    }

    #[test]
    fn test_get_match_points_target_final() {
        // GIVEN: A final match
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting points target for final
        let target = service.get_match_points_target(
            CompetitionType::Tournament,
            Some("Finale".to_string()),
        );

        // THEN: Should be 15
        assert_eq!(target, 15);
    }

    #[test]
    fn test_get_match_points_target_la_balle() {
        // GIVEN: A match in "la balle" phase (final)
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting points target for "la balle"
        let target = service.get_match_points_target(
            CompetitionType::Tournament,
            Some("La Balle".to_string()),
        );

        // THEN: Should be 15
        assert_eq!(target, 15);
    }

    #[test]
    fn test_validate_manche_score_valid() {
        // GIVEN: Valid manche scores
        let service = PaletRulesServiceImpl::new();

        // WHEN: Validating scores
        let result = service.validate_manche_score(3, 1, 1);

        // THEN: Should succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_manche_score_negative() {
        // GIVEN: Negative score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Validating negative score
        let result = service.validate_manche_score(-1, 3, 1);

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(PaletRulesError::InvalidScore(msg)) = result {
            assert!(msg.contains("negative"));
        } else {
            panic!("Expected InvalidScore error");
        }
    }

    #[test]
    fn test_validate_manche_score_too_high() {
        // GIVEN: Score exceeding maximum (6)
        let service = PaletRulesServiceImpl::new();

        // WHEN: Validating too high score
        let result = service.validate_manche_score(7, 0, 1);

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(PaletRulesError::InvalidScore(msg)) = result {
            assert!(msg.contains("cannot exceed 6"));
        } else {
            panic!("Expected InvalidScore error");
        }
    }

    #[test]
    fn test_validate_manche_score_total_too_high() {
        // GIVEN: Total score exceeding maximum (6)
        let service = PaletRulesServiceImpl::new();

        // WHEN: Validating total score too high
        let result = service.validate_manche_score(4, 3, 1);

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(PaletRulesError::InvalidScore(msg)) = result {
            assert!(msg.contains("Total score per manche cannot exceed 6"));
        } else {
            panic!("Expected InvalidScore error");
        }
    }

    #[test]
    fn test_calculate_manche_winner_team1() {
        // GIVEN: Team1 has higher score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Calculating winner
        let winner = service.calculate_manche_winner(3, 1);

        // THEN: Team1 should win
        assert_eq!(winner, Some(MatchWinner::Team1));
    }

    #[test]
    fn test_calculate_manche_winner_team2() {
        // GIVEN: Team2 has higher score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Calculating winner
        let winner = service.calculate_manche_winner(1, 3);

        // THEN: Team2 should win
        assert_eq!(winner, Some(MatchWinner::Team2));
    }

    #[test]
    fn test_calculate_manche_winner_draw() {
        // GIVEN: Both teams have same score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Calculating winner
        let winner = service.calculate_manche_winner(3, 3);

        // THEN: Should be None (draw)
        assert_eq!(winner, None);
    }

    #[test]
    fn test_is_match_completed_13_points() {
        // GIVEN: Team1 reached 13 points
        let service = PaletRulesServiceImpl::new();

        // WHEN: Checking completion
        let completed = service.is_match_completed(13, 8, 13);

        // THEN: Should be completed
        assert!(completed);
    }

    #[test]
    fn test_is_match_completed_15_points() {
        // GIVEN: Team2 reached 15 points in final
        let service = PaletRulesServiceImpl::new();

        // WHEN: Checking completion
        let completed = service.is_match_completed(12, 15, 15);

        // THEN: Should be completed
        assert!(completed);
    }

    #[test]
    fn test_is_match_completed_not_yet() {
        // GIVEN: Neither team reached target
        let service = PaletRulesServiceImpl::new();

        // WHEN: Checking completion
        let completed = service.is_match_completed(10, 8, 13);

        // THEN: Should not be completed
        assert!(!completed);
    }

    #[test]
    fn test_determine_match_winner_team1() {
        // GIVEN: Team1 has higher score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Determining winner
        let winner = service.determine_match_winner(13, 8);

        // THEN: Team1 should win
        assert_eq!(winner, Some(MatchWinner::Team1));
    }

    #[test]
    fn test_determine_match_winner_team2() {
        // GIVEN: Team2 has higher score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Determining winner
        let winner = service.determine_match_winner(8, 13);

        // THEN: Team2 should win
        assert_eq!(winner, Some(MatchWinner::Team2));
    }

    #[test]
    fn test_determine_match_winner_draw() {
        // GIVEN: Both teams have same score
        let service = PaletRulesServiceImpl::new();

        // WHEN: Determining winner
        let winner = service.determine_match_winner(13, 13);

        // THEN: Should be None (draw)
        assert_eq!(winner, None);
    }

    #[test]
    fn test_get_championship_classification_rules() {
        // GIVEN: Palet rules service
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting classification rules
        let rules = service.get_championship_classification_rules();

        // THEN: Should have correct Palet Vendéen rules
        assert_eq!(rules.primary, palet_shared::ClassificationPrimary::Points);
        assert_eq!(rules.secondary, palet_shared::ClassificationSecondary::Diff);
        assert_eq!(
            rules.tertiary,
            palet_shared::ClassificationTertiary::PointsFor
        );
    }

    #[test]
    fn test_calculate_team_classification_by_points() {
        // GIVEN: Teams with different points
        let service = PaletRulesServiceImpl::new();

        let team1 = palet_shared::ChampionshipTeam {
            position: 0,
            team_id: Uuid::new_v4(),
            stats: palet_shared::TeamStats {
                matches: 5,
                wins: 4,
                draws: 1,
                losses: 0,
                points_for: 45,
                points_against: 20,
                diff: 25,
                points: 13, // 4 wins * 3 + 1 draw * 1 = 13
                win_rate: 0.8,
            },
            form: vec![],
            status: palet_shared::TeamClassificationStatus::Qualified,
        };

        let team2 = palet_shared::ChampionshipTeam {
            position: 0,
            team_id: Uuid::new_v4(),
            stats: palet_shared::TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 40,
                points_against: 25,
                diff: 15,
                points: 10, // 3 wins * 3 + 1 draw * 1 = 10
                win_rate: 0.6,
            },
            form: vec![],
            status: palet_shared::TeamClassificationStatus::Qualified,
        };

        let team3 = palet_shared::ChampionshipTeam {
            position: 0,
            team_id: Uuid::new_v4(),
            stats: palet_shared::TeamStats {
                matches: 5,
                wins: 2,
                draws: 2,
                losses: 1,
                points_for: 35,
                points_against: 30,
                diff: 5,
                points: 8, // 2 wins * 3 + 2 draws * 1 = 8
                win_rate: 0.4,
            },
            form: vec![],
            status: palet_shared::TeamClassificationStatus::Safe,
        };

        let teams = vec![team1, team2, team3];

        // WHEN: Calculating classification
        let classified_teams = service.calculate_team_classification(teams);

        // THEN: Teams should be sorted by points
        assert_eq!(classified_teams.len(), 3);
        assert_eq!(classified_teams[0].stats.points, 13);
        assert_eq!(classified_teams[1].stats.points, 10);
        assert_eq!(classified_teams[2].stats.points, 8);
        assert_eq!(classified_teams[0].position, 1);
        assert_eq!(classified_teams[1].position, 2);
        assert_eq!(classified_teams[2].position, 3);
    }

    #[test]
    fn test_calculate_team_classification_by_diff() {
        // GIVEN: Teams with same points but different diffs
        let service = PaletRulesServiceImpl::new();

        let team1 = palet_shared::ChampionshipTeam {
            position: 0,
            team_id: Uuid::new_v4(),
            stats: palet_shared::TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 50,
                points_against: 20,
                diff: 30,
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: palet_shared::TeamClassificationStatus::Qualified,
        };

        let team2 = palet_shared::ChampionshipTeam {
            position: 0,
            team_id: Uuid::new_v4(),
            stats: palet_shared::TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 45,
                points_against: 25,
                diff: 20,
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: palet_shared::TeamClassificationStatus::Qualified,
        };

        let teams = vec![team1, team2];

        // WHEN: Calculating classification
        let classified_teams = service.calculate_team_classification(teams);

        // THEN: Teams should be sorted by diff (secondary rule)
        assert_eq!(classified_teams.len(), 2);
        assert_eq!(classified_teams[0].stats.diff, 30);
        assert_eq!(classified_teams[1].stats.diff, 20);
    }

    #[test]
    fn test_get_win_points() {
        // GIVEN: Palet rules service
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting win points
        let points = service.get_win_points();

        // THEN: Should be 3
        assert_eq!(points, 3);
    }

    #[test]
    fn test_get_draw_points() {
        // GIVEN: Palet rules service
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting draw points
        let points = service.get_draw_points();

        // THEN: Should be 1
        assert_eq!(points, 1);
    }

    #[test]
    fn test_get_loss_points() {
        // GIVEN: Palet rules service
        let service = PaletRulesServiceImpl::new();

        // WHEN: Getting loss points
        let points = service.get_loss_points();

        // THEN: Should be 0
        assert_eq!(points, 0);
    }

    #[test]
    fn test_validate_championship_rules_valid() {
        // GIVEN: A championship with valid Palet Vendéen rules
        let service = PaletRulesServiceImpl::new();

        let championship = palet_shared::Championship {
            id: Uuid::new_v4(),
            name: "Test Championship".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: palet_shared::ClassificationPrimary::Points,
                secondary: palet_shared::ClassificationSecondary::Diff,
                tertiary: palet_shared::ClassificationTertiary::PointsFor,
            },
            status: palet_shared::CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // WHEN: Validating rules
        let result = service.validate_championship_rules(&championship);

        // THEN: Should succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_championship_rules_invalid_primary() {
        // GIVEN: A championship with invalid primary rule
        let service = PaletRulesServiceImpl::new();

        let championship = palet_shared::Championship {
            id: Uuid::new_v4(),
            name: "Test Championship".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: palet_shared::ClassificationPrimary::WinRate, // Invalid for Palet
                secondary: palet_shared::ClassificationSecondary::Diff,
                tertiary: palet_shared::ClassificationTertiary::PointsFor,
            },
            status: palet_shared::CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // WHEN: Validating rules
        let result = service.validate_championship_rules(&championship);

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(PaletRulesError::RuleViolation(msg)) = result {
            assert!(msg.contains("Invalid primary classification rule"));
        } else {
            panic!("Expected RuleViolation error");
        }
    }

    #[test]
    fn test_validate_match_rules_valid() {
        // GIVEN: A match with valid Palet Vendéen rules
        let service = PaletRulesServiceImpl::new();

        let match_obj = palet_shared::Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(13),
            team2_score: Some(8),
            winner: Some(MatchWinner::Team1),
            manches: Some(vec![
                MancheScore {
                    number: 1,
                    team1: 3,
                    team2: 1,
                    winner: Some(MatchWinner::Team1),
                },
                MancheScore {
                    number: 2,
                    team1: 4,
                    team2: 2,
                    winner: Some(MatchWinner::Team1),
                },
            ]),
            best_of: Some(13),
            current_manche: Some(2),
            current_scores: None,
            status: MatchStatus::Completed,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: chrono::Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            started_at: Some(chrono::Utc::now()),
            ended_at: Some(chrono::Utc::now()),
        };

        // WHEN: Validating rules
        let result = service.validate_match_rules(&match_obj);

        // THEN: Should succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_match_rules_invalid_best_of() {
        // GIVEN: A match with invalid best_of value
        let service = PaletRulesServiceImpl::new();

        let match_obj = palet_shared::Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(13),
            team2_score: Some(8),
            winner: Some(MatchWinner::Team1),
            manches: None,
            best_of: Some(10), // Invalid: must be 13 or 15
            current_manche: None,
            current_scores: None,
            status: MatchStatus::Completed,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: chrono::Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            started_at: Some(chrono::Utc::now()),
            ended_at: Some(chrono::Utc::now()),
        };

        // WHEN: Validating rules
        let result = service.validate_match_rules(&match_obj);

        // THEN: Should fail
        assert!(result.is_err());
        if let Err(PaletRulesError::RuleViolation(msg)) = result {
            assert!(msg.contains("Invalid best_of value"));
        } else {
            panic!("Expected RuleViolation error");
        }
    }
}
