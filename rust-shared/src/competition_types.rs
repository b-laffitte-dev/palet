//! Competition types for Palet Vendéen

use serde::{Deserialize, Serialize};

/// Type of competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionType {
    Championship,
    Tournament,
    Cup,
    Friendly,
}

/// Category of competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionCategory {
    Individual,
    Doublette,
    Triplette,
}

/// Material used in competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionMaterial {
    Fonte,
    Laiton,
    Bois,
    Mixte,
}

/// Status of a competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionStatus {
    Pending,
    Registration,
    InProgress,
    Completed,
    Cancelled,
}

/// Format of the competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionFormat {
    RoundRobin,
    League,
    Groups,
    Knockout,
}

/// Points type for competition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointsType {
    P13,
    P15,
}

/// Result of a match for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchResult {
    Win,
    Draw,
    Loss,
}

/// Classification status of a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamClassificationStatus {
    Qualified,
    Barrage,
    Relegated,
    Safe,
}

/// Statistics for a team
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TeamStats {
    pub matches: i32,
    pub wins: i32,
    pub draws: i32,
    pub losses: i32,
    pub points_for: i32,
    pub points_against: i32,
    pub diff: i32,
    pub points: i32,
    pub win_rate: f64,
}

impl TeamStats {
    pub fn new() -> Self {
        TeamStats {
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points_for: 0,
            points_against: 0,
            diff: 0,
            points: 0,
            win_rate: 0.0,
        }
    }

    pub fn update_from_match(&mut self, points_for: i32, points_against: i32, result: MatchResult) {
        self.matches += 1;
        self.points_for += points_for;
        self.points_against += points_against;
        self.diff = self.points_for - self.points_against;

        match result {
            MatchResult::Win => {
                self.wins += 1;
                self.points += 2;
            }
            MatchResult::Draw => {
                self.draws += 1;
                self.points += 1;
            }
            MatchResult::Loss => {
                self.losses += 1;
            }
        }

        if self.matches > 0 {
            self.win_rate = self.wins as f64 / self.matches as f64;
        }
    }
}
