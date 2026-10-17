//! Match types for Palet Vendéen

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CompetitionType, MatchResult};

/// Status of a match
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Completed,
    Postponed,
    Cancelled,
    Disputed,
}

/// Winner of a match
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchWinner {
    Team1,
    Team2,
}

impl MatchWinner {
    pub fn opposite(&self) -> Self {
        match self {
            MatchWinner::Team1 => MatchWinner::Team2,
            MatchWinner::Team2 => MatchWinner::Team1,
        }
    }
}

/// Score for a single manche (round)
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MancheScore {
    pub number: i32,
    pub team1: i32,
    pub team2: i32,
    pub winner: Option<MatchWinner>,
}

impl MancheScore {
    pub fn new(number: i32, team1: i32, team2: i32) -> Self {
        let winner = if team1 > team2 {
            Some(MatchWinner::Team1)
        } else if team2 > team1 {
            Some(MatchWinner::Team2)
        } else {
            None
        };
        MancheScore {
            number,
            team1,
            team2,
            winner,
        }
    }
}

/// Current scores during a match
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CurrentScores {
    pub team1: i32,
    pub team2: i32,
    pub manche: i32,
}

impl CurrentScores {
    pub fn new() -> Self {
        CurrentScores {
            team1: 0,
            team2: 0,
            manche: 1,
        }
    }
}

/// A match between two teams
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Match {
    pub id: Uuid,
    pub competition_id: Uuid,
    pub competition_type: CompetitionType,
    pub journee: Option<i32>,
    pub phase: Option<String>,
    pub round: Option<String>,
    pub team1_id: Uuid,
    pub team2_id: Uuid,
    pub team1_score: Option<i32>,
    pub team2_score: Option<i32>,
    pub winner: Option<MatchWinner>,
    pub manches: Option<Vec<MancheScore>>,
    pub best_of: Option<i32>,
    pub current_manche: Option<i32>,
    pub current_scores: Option<CurrentScores>,
    pub status: MatchStatus,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub date: DateTime<Utc>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Match {
    pub fn new(
        competition_id: Uuid,
        competition_type: CompetitionType,
        team1_id: Uuid,
        team2_id: Uuid,
        date: DateTime<Utc>,
    ) -> Self {
        Match {
            id: Uuid::new_v4(),
            competition_id,
            competition_type,
            journee: None,
            phase: None,
            round: None,
            team1_id,
            team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: Some(1),
            current_scores: Some(CurrentScores::new()),
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date,
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        }
    }

    pub fn team1_result(&self) -> MatchResult {
        match self.winner {
            Some(MatchWinner::Team1) => MatchResult::Win,
            Some(MatchWinner::Team2) => MatchResult::Loss,
            None => MatchResult::Draw,
        }
    }

    pub fn team2_result(&self) -> MatchResult {
        match self.team1_result() {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }
}

/// Request to create a new match
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateMatchRequest {
    pub competition_id: Uuid,
    pub competition_type: CompetitionType,
    pub team1_id: Uuid,
    pub team2_id: Uuid,
    pub date: DateTime<Utc>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub journee: Option<i32>,
    pub phase: Option<String>,
    pub round: Option<String>,
    pub best_of: Option<i32>,
}

impl CreateMatchRequest {
    pub fn to_match(&self) -> Match {
        Match::new(
            self.competition_id,
            self.competition_type.clone(),
            self.team1_id,
            self.team2_id,
            self.date,
        )
    }
}

/// Request to update a match
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateMatchRequest {
    pub team1_score: Option<i32>,
    pub team2_score: Option<i32>,
    pub status: Option<MatchStatus>,
    pub winner: Option<MatchWinner>,
    pub verified: Option<bool>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub date: Option<DateTime<Utc>>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,
    pub current_manche: Option<i32>,
    pub manches: Option<Vec<MancheScore>>,
}

impl UpdateMatchRequest {
    pub fn apply_to(&self, match_obj: &mut Match) {
        if let Some(team1_score) = self.team1_score {
            match_obj.team1_score = Some(team1_score);
        }
        if let Some(team2_score) = self.team2_score {
            match_obj.team2_score = Some(team2_score);
        }
        if let Some(status) = self.status.clone() {
            match_obj.status = status;
        }
        if let Some(ref winner) = self.winner {
            match_obj.winner = Some(winner.clone());
        }
        if let Some(verified) = self.verified {
            match_obj.verified = verified;
        }
        if let Some(verified_by) = self.verified_by {
            match_obj.verified_by = Some(verified_by);
        }
        if let Some(verified_at) = self.verified_at {
            match_obj.verified_at = Some(verified_at);
        }
        if let Some(date) = self.date {
            match_obj.date = date;
        }
        if let Some(time) = self.time.clone() {
            match_obj.time = Some(time);
        }
        if let Some(location) = self.location.clone() {
            match_obj.location = Some(location);
        }
        if let Some(duration) = self.duration.clone() {
            match_obj.duration = Some(duration);
        }
        if let Some(current_manche) = self.current_manche {
            match_obj.current_manche = Some(current_manche);
        }
        if let Some(manches) = self.manches.clone() {
            match_obj.manches = Some(manches);
        }
        match_obj.updated_at = Utc::now();
    }
}
