//! TDD Tests for Championship functionality in Palet Vendéen
//!
//! Business rules verified:
//! - Classification: Points (primary) > Diff (secondary) > PointsFor (tertiary)

use chrono::{DateTime, Utc};
use palet_shared::{Championship, ChampionshipClassification, ChampionshipTeam, ClassificationRules, ClassificationPrimary, ClassificationSecondary, ClassificationTertiary, CompetitionStatus, MatchResult, TeamStats, TeamClassificationStatus};
use uuid::Uuid;

// ============================================
// CHAMPIONSHIP CREATION TESTS
// ============================================

#[test]
fn test_create_championship_with_default_values() {
    let championship = Championship::new(
        "Championnat Vendée 2025".to_string(),
        "2025-2026".to_string(),
        "Vendée".to_string(),
        vec!["Division 1".to_string(), "Division 2".to_string()],
        10,
        Uuid::new_v4(),
    );

    assert_eq!(championship.name, "Championnat Vendée 2025");
    assert_eq!(championship.season, "2025-2026");
    assert_eq!(championship.league, "Vendée");
    assert_eq!(championship.divisions, vec!["Division 1", "Division 2"]);
    assert_eq!(championship.total_journees, 10);
    assert_eq!(championship.status, CompetitionStatus::Pending);
    assert_eq!(championship.current_journee, Some(1));
    assert_eq!(championship.classification_rules.primary, ClassificationPrimary::Points);
    assert_eq!(championship.classification_rules.secondary, ClassificationSecondary::Diff);
    assert_eq!(championship.classification_rules.tertiary, ClassificationTertiary::PointsFor);
}

// ============================================
// CLASSIFICATION RULES TESTS
// ============================================

#[test]
fn test_default_classification_rules() {
    let rules = ClassificationRules::default_palet();
    assert_eq!(rules.primary, ClassificationPrimary::Points);
    assert_eq!(rules.secondary, ClassificationSecondary::Diff);
    assert_eq!(rules.tertiary, ClassificationTertiary::PointsFor);
}

#[test]
fn test_classification_rules_compare_teams_by_points() {
    let rules = ClassificationRules::default_palet();

    let mut team1 = ChampionshipTeam::new(Uuid::new_v4());
    team1.stats.points = 10;
    team1.stats.diff = 5;
    team1.stats.points_for = 50;

    let mut team2 = ChampionshipTeam::new(Uuid::new_v4());
    team2.stats.points = 8;
    team2.stats.diff = 10;
    team2.stats.points_for = 60;

    let cmp = rules.compare_teams(&team1, &team2);
    assert_eq!(cmp, std::cmp::Ordering::Greater);
}

#[test]
fn test_classification_rules_compare_teams_by_diff() {
    let rules = ClassificationRules::default_palet();

    let mut team1 = ChampionshipTeam::new(Uuid::new_v4());
    team1.stats.points = 10;
    team1.stats.diff = 15;
    team1.stats.points_for = 50;

    let mut team2 = ChampionshipTeam::new(Uuid::new_v4());
    team2.stats.points = 10;
    team2.stats.diff = 10;
    team2.stats.points_for = 60;

    let cmp = rules.compare_teams(&team1, &team2);
    assert_eq!(cmp, std::cmp::Ordering::Greater);
}

#[test]
fn test_classification_rules_compare_teams_by_points_for() {
    let rules = ClassificationRules::default_palet();

    let mut team1 = ChampionshipTeam::new(Uuid::new_v4());
    team1.stats.points = 10;
    team1.stats.diff = 10;
    team1.stats.points_for = 60;

    let mut team2 = ChampionshipTeam::new(Uuid::new_v4());
    team2.stats.points = 10;
    team2.stats.diff = 10;
    team2.stats.points_for = 50;

    let cmp = rules.compare_teams(&team1, &team2);
    assert_eq!(cmp, std::cmp::Ordering::Greater);
}

#[test]
fn test_classification_rules_equal_teams() {
    let rules = ClassificationRules::default_palet();

    let mut team1 = ChampionshipTeam::new(Uuid::new_v4());
    team1.stats.points = 10;
    team1.stats.diff = 10;
    team1.stats.points_for = 50;

    let mut team2 = ChampionshipTeam::new(Uuid::new_v4());
    team2.stats.points = 10;
    team2.stats.diff = 10;
    team2.stats.points_for = 50;

    let cmp = rules.compare_teams(&team1, &team2);
    assert_eq!(cmp, std::cmp::Ordering::Equal);
}

// ============================================
// CHAMPIONSHIP TEAM TESTS
// ============================================

#[test]
fn test_championship_team_creation() {
    let team_id = Uuid::new_v4();
    let team = ChampionshipTeam::new(team_id);

    assert_eq!(team.team_id, team_id);
    assert_eq!(team.position, 0);
    assert_eq!(team.stats.matches, 0);
    assert_eq!(team.form.len(), 0);
    assert_eq!(team.status, TeamClassificationStatus::Safe);
}

#[test]
fn test_championship_team_set_position() {
    let mut team = ChampionshipTeam::new(Uuid::new_v4());
    team.set_position(3);
    assert_eq!(team.position, 3);
}

#[test]
fn test_championship_team_add_form_result() {
    let mut team = ChampionshipTeam::new(Uuid::new_v4());
    team.add_form_result(MatchResult::Win);
    team.add_form_result(MatchResult::Loss);
    assert_eq!(team.form.len(), 2);
    assert_eq!(team.form[0], MatchResult::Win);
    assert_eq!(team.form[1], MatchResult::Loss);
}

#[test]
fn test_championship_team_form_keeps_last_5() {
    let mut team = ChampionshipTeam::new(Uuid::new_v4());
    for _ in 0..6 {
        team.add_form_result(MatchResult::Win);
    }
    assert_eq!(team.form.len(), 5);
}

// ============================================
// CHAMPIONSHIP CLASSIFICATION TESTS
// ============================================

#[test]
fn test_championship_classification_creation() {
    let classification = ChampionshipClassification::new("Division 1".to_string(), 1);

    assert_eq!(classification.division, "Division 1");
    assert_eq!(classification.journee, 1);
    assert_eq!(classification.teams.len(), 0);
}

#[test]
fn test_championship_classification_add_team() {
    let mut classification = ChampionshipClassification::new("Division 1".to_string(), 1);
    let team = ChampionshipTeam::new(Uuid::new_v4());
    classification.add_team(team);

    assert_eq!(classification.teams.len(), 1);
}

#[test]
fn test_championship_classification_sort_teams() {
    let mut classification = ChampionshipClassification::new("Division 1".to_string(), 1);

    let mut team1 = ChampionshipTeam::new(Uuid::new_v4());
    team1.stats.points = 10;
    team1.stats.diff = 5;
    team1.stats.points_for = 50;

    let mut team2 = ChampionshipTeam::new(Uuid::new_v4());
    team2.stats.points = 12;
    team2.stats.diff = 3;
    team2.stats.points_for = 45;

    let mut team3 = ChampionshipTeam::new(Uuid::new_v4());
    team3.stats.points = 10;
    team3.stats.diff = 8;
    team3.stats.points_for = 55;

    classification.add_team(team1);
    classification.add_team(team2);
    classification.add_team(team3);

    classification.sort_teams(&ClassificationRules::default_palet());

    assert_eq!(classification.teams[0].stats.points, 12);
    assert_eq!(classification.teams[1].stats.points, 10);
    assert_eq!(classification.teams[1].stats.diff, 8);
    assert_eq!(classification.teams[2].stats.points, 10);
    assert_eq!(classification.teams[2].stats.diff, 5);

    assert_eq!(classification.teams[0].position, 1);
    assert_eq!(classification.teams[1].position, 2);
    assert_eq!(classification.teams[2].position, 3);
}

#[test]
fn test_championship_classification_get_team() {
    let mut classification = ChampionshipClassification::new("Division 1".to_string(), 1);
    let team_id = Uuid::new_v4();
    let team = ChampionshipTeam::new(team_id);
    classification.add_team(team);

    let retrieved = classification.get_team(team_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().team_id, team_id);
}

#[test]
fn test_championship_classification_get_team_not_found() {
    let classification = ChampionshipClassification::new("Division 1".to_string(), 1);
    let team_id = Uuid::new_v4();

    let retrieved = classification.get_team(team_id);
    assert!(retrieved.is_none());
}

// ============================================
// TEAM STATS TESTS
// ============================================

#[test]
fn test_team_stats_new() {
    let stats = TeamStats::new();
    assert_eq!(stats.matches, 0);
    assert_eq!(stats.wins, 0);
    assert_eq!(stats.draws, 0);
    assert_eq!(stats.losses, 0);
    assert_eq!(stats.points_for, 0);
    assert_eq!(stats.points_against, 0);
    assert_eq!(stats.diff, 0);
    assert_eq!(stats.points, 0);
    assert_eq!(stats.win_rate, 0.0);
}

#[test]
fn test_team_stats_update_win() {
    let mut stats = TeamStats::new();
    stats.update_from_match(13, 8, MatchResult::Win);

    assert_eq!(stats.matches, 1);
    assert_eq!(stats.wins, 1);
    assert_eq!(stats.losses, 0);
    assert_eq!(stats.draws, 0);
    assert_eq!(stats.points_for, 13);
    assert_eq!(stats.points_against, 8);
    assert_eq!(stats.diff, 5);
    assert_eq!(stats.points, 2);
    assert!((stats.win_rate - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_team_stats_update_loss() {
    let mut stats = TeamStats::new();
    stats.update_from_match(8, 13, MatchResult::Loss);

    assert_eq!(stats.matches, 1);
    assert_eq!(stats.wins, 0);
    assert_eq!(stats.losses, 1);
    assert_eq!(stats.draws, 0);
    assert_eq!(stats.points_for, 8);
    assert_eq!(stats.points_against, 13);
    assert_eq!(stats.diff, -5);
    assert_eq!(stats.points, 0);
    assert!((stats.win_rate - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_team_stats_update_draw() {
    let mut stats = TeamStats::new();
    stats.update_from_match(13, 13, MatchResult::Draw);

    assert_eq!(stats.matches, 1);
    assert_eq!(stats.wins, 0);
    assert_eq!(stats.losses, 0);
    assert_eq!(stats.draws, 1);
    assert_eq!(stats.points_for, 13);
    assert_eq!(stats.points_against, 13);
    assert_eq!(stats.diff, 0);
    assert_eq!(stats.points, 1);
    assert!((stats.win_rate - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_team_stats_multiple_matches() {
    let mut stats = TeamStats::new();
    stats.update_from_match(13, 8, MatchResult::Win);
    stats.update_from_match(10, 13, MatchResult::Loss);
    stats.update_from_match(13, 10, MatchResult::Win);

    assert_eq!(stats.matches, 3);
    assert_eq!(stats.wins, 2);
    assert_eq!(stats.losses, 1);
    assert_eq!(stats.draws, 0);
    assert_eq!(stats.points_for, 36);
    assert_eq!(stats.points_against, 31);
    assert_eq!(stats.diff, 5);
    assert_eq!(stats.points, 4);
    assert!((stats.win_rate - (2.0 / 3.0)).abs() < f64::EPSILON);
}

// ============================================
// CHAMPIONSHIP BUSINESS RULE TESTS
// ============================================

#[test]
fn test_championship_default_classification() {
    let championship = Championship::new(
        "Test".to_string(),
        "2025".to_string(),
        "Vendée".to_string(),
        vec!["D1".to_string()],
        10,
        Uuid::new_v4(),
    );

    assert_eq!(championship.classification_rules.primary, ClassificationPrimary::Points);
    assert_eq!(championship.classification_rules.secondary, ClassificationSecondary::Diff);
    assert_eq!(championship.classification_rules.tertiary, ClassificationTertiary::PointsFor);
}

#[test]
fn test_championship_current_journee_defaults_to_1() {
    let championship = Championship::new(
        "Test".to_string(),
        "2025".to_string(),
        "Vendée".to_string(),
        vec!["D1".to_string()],
        10,
        Uuid::new_v4(),
    );

    assert_eq!(championship.current_journee, Some(1));
}

#[test]
fn test_championship_status_defaults_to_pending() {
    let championship = Championship::new(
        "Test".to_string(),
        "2025".to_string(),
        "Vendée".to_string(),
        vec!["D1".to_string()],
        10,
        Uuid::new_v4(),
    );

    assert_eq!(championship.status, CompetitionStatus::Pending);
}
