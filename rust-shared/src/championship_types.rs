//! Championship types for Palet Vendéen

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

use super::{CompetitionStatus, MatchResult, TeamStats, TeamClassificationStatus};

/// Primary classification rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationPrimary {
    Points,
    WinRate,
    Diff,
}

/// Secondary classification rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationSecondary {
    Diff,
    PointsFor,
    HeadToHead,
}

/// Tertiary classification rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationTertiary {
    PointsFor,
    PointsAgainst,
}

/// Classification rules
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ClassificationRules {
    pub primary: ClassificationPrimary,
    pub secondary: ClassificationSecondary,
    pub tertiary: ClassificationTertiary,
}

impl ClassificationRules {
    pub fn default_palet() -> Self {
        ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        }
    }

    pub fn compare_teams(&self, team1: &ChampionshipTeam, team2: &ChampionshipTeam) -> Ordering {
        let primary_cmp = match self.primary {
            ClassificationPrimary::Points => team1.stats.points.cmp(&team2.stats.points),
            ClassificationPrimary::WinRate => {
                team1.stats.win_rate.partial_cmp(&team2.stats.win_rate).unwrap_or(Ordering::Equal)
            }
            ClassificationPrimary::Diff => team1.stats.diff.cmp(&team2.stats.diff),
        };
        if primary_cmp != Ordering::Equal {
            return primary_cmp;
        }
        let secondary_cmp = match self.secondary {
            ClassificationSecondary::Diff => team1.stats.diff.cmp(&team2.stats.diff),
            ClassificationSecondary::PointsFor => team1.stats.points_for.cmp(&team2.stats.points_for),
            ClassificationSecondary::HeadToHead => Ordering::Equal,
        };
        if secondary_cmp != Ordering::Equal {
            return secondary_cmp;
        }
        match self.tertiary {
            ClassificationTertiary::PointsFor => team1.stats.points_for.cmp(&team2.stats.points_for),
            ClassificationTertiary::PointsAgainst => {
                team1.stats.points_against.cmp(&team2.stats.points_against)
            }
        }
    }
}

impl Default for ClassificationRules {
    fn default() -> Self {
        Self::default_palet()
    }
}

/// A championship
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Championship {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub season: String,
    pub description: Option<String>,
    pub league: String,
    pub divisions: Vec<String>,
    pub total_journees: i32,
    pub journee_duration: Option<String>,
    pub classification_rules: ClassificationRules,
    pub status: CompetitionStatus,
    pub current_journee: Option<i32>,
    pub current_phase: Option<String>,
    pub organizer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Championship {
    pub fn new(
        name: String,
        season: String,
        league: String,
        divisions: Vec<String>,
        total_journees: i32,
        organizer_id: Uuid,
    ) -> Self {
        Championship {
            id: Uuid::new_v4(),
            name,
            code: None,
            season,
            description: None,
            league,
            divisions,
            total_journees,
            journee_duration: None,
            classification_rules: ClassificationRules::default_palet(),
            status: CompetitionStatus::Pending,
            current_journee: Some(1),
            current_phase: None,
            organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Team in championship classification
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChampionshipTeam {
    pub position: i32,
    pub team_id: Uuid,
    pub stats: TeamStats,
    pub form: Vec<MatchResult>,
    pub status: TeamClassificationStatus,
}

impl ChampionshipTeam {
    pub fn new(team_id: Uuid) -> Self {
        ChampionshipTeam {
            position: 0,
            team_id,
            stats: TeamStats::new(),
            form: Vec::new(),
            status: TeamClassificationStatus::Safe,
        }
    }
    pub fn set_position(&mut self, position: i32) {
        self.position = position;
    }
    pub fn add_form_result(&mut self, result: MatchResult) {
        self.form.push(result);
        if self.form.len() > 5 {
            self.form.remove(0);
        }
    }
}

/// Championship classification for a division and journee
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChampionshipClassification {
    pub division: String,
    pub journee: i32,
    pub updated_at: DateTime<Utc>,
    pub teams: Vec<ChampionshipTeam>,
}

impl ChampionshipClassification {
    pub fn new(division: String, journee: i32) -> Self {
        ChampionshipClassification {
            division,
            journee,
            updated_at: Utc::now(),
            teams: Vec::new(),
        }
    }
    pub fn add_team(&mut self, team: ChampionshipTeam) {
        self.teams.push(team);
    }
    pub fn sort_teams(&mut self, rules: &ClassificationRules) {
        self.teams.sort_by(|a, b| rules.compare_teams(a, b).reverse());
        for (index, team) in self.teams.iter_mut().enumerate() {
            team.position = (index + 1) as i32;
        }
    }
    pub fn get_team(&self, team_id: Uuid) -> Option<&ChampionshipTeam> {
        self.teams.iter().find(|t| t.team_id == team_id)
    }
}

/// Request to create championship
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateChampionshipRequest {
    pub name: String,
    pub code: Option<String>,
    pub season: String,
    pub description: Option<String>,
    pub league: String,
    pub divisions: Vec<String>,
    pub total_journees: i32,
    pub journee_duration: Option<String>,
    pub classification_rules: Option<ClassificationRules>,
    pub organizer_id: Uuid,
}

/// Request to update championship
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateChampionshipRequest {
    pub name: Option<String>,
    pub code: Option<String>,
    pub season: Option<String>,
    pub description: Option<String>,
    pub league: Option<String>,
    pub divisions: Option<Vec<String>>,
    pub total_journees: Option<i32>,
    pub journee_duration: Option<String>,
    pub classification_rules: Option<ClassificationRules>,
    pub status: Option<CompetitionStatus>,
    pub current_journee: Option<i32>,
    pub current_phase: Option<String>,
}
