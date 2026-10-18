//! Match Repository - Database operations for matches
//!
//! This module provides the database operations for managing Palet Vendéen matches.
//! It implements the TDD harness requirements for match functionality.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    CreateMatchRequest, MancheScore, Match, MatchStatus, MatchWinner, PointsType,
    UpdateMatchRequest,
};
use serde_json;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// MATCH REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for match repository operations
#[async_trait]
pub trait MatchRepository {
    // CRUD Operations
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError>;
    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, DatabaseError>;
    async fn get_matches(&self) -> Result<Vec<Match>, DatabaseError>;
    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError>;
    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, DatabaseError>;
    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError>;
    async fn delete_match(&self, match_id: Uuid) -> Result<bool, DatabaseError>;

    // Match Lifecycle Operations
    async fn start_match(&self, match_id: Uuid) -> Result<Match, DatabaseError>;
    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: MatchWinner,
    ) -> Result<Match, DatabaseError>;

    // Manche Operations (Palet Vendéen specific)
    async fn add_manche(
        &self,
        match_id: Uuid,
        manche: MancheScore,
    ) -> Result<Match, DatabaseError>;
    async fn update_manche(
        &self,
        match_id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, DatabaseError>;

    // Verification Operations
    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, DatabaseError>;
    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, DatabaseError>;

    // Query Operations
    async fn get_matches_by_date_range(
        &self,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) -> Result<Vec<Match>, DatabaseError>;
    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, DatabaseError>;
    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError>;

    // Championship-specific operations
    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError>;
    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError>;
}

// ============================================
// POSTGRES MATCH REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of MatchRepository
#[derive(Clone)]
pub struct PostgresMatchRepository {
    pool: Arc<PgPool>,
}

impl PostgresMatchRepository {
    /// Create a new PostgresMatchRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper function to convert database row to Match
    async fn row_to_match(row: sqlx::postgres::PgRow) -> Result<Match, sqlx::Error> {
        let id: Uuid = row.get("id");
        let competition_id: Uuid = row.get("competition_id");
        let competition_type: palet_shared::CompetitionType = palet_shared::CompetitionType::Championship;
        let journee: Option<i32> = row.get("journee");
        let phase: Option<String> = row.get("phase");
        let round: Option<String> = row.get("round");
        let team1_id: Uuid = row.get("team1_id");
        let team2_id: Uuid = row.get("team2_id");
        let team1_score: Option<i32> = row.get("team1_score");
        let team2_score: Option<i32> = row.get("team2_score");
        let winner: Option<MatchWinner> = None;
        let manches: Option<Vec<MancheScore>> = None;
        let best_of: Option<i32> = row.get("best_of");
        let current_manche: Option<i32> = row.get("current_manche");
        let current_scores = row.get("current_scores");
        let current_scores: Option<palet_shared::CurrentScores> = match current_scores {
            Some(value) => serde_json::from_value(value).ok(),
            None => None,
        };
        let status: MatchStatus = MatchStatus::Scheduled;
        let verified: bool = row.get("verified");
        let verified_by: Option<Uuid> = row.get("verified_by");
        let verified_at: Option<chrono::DateTime<Utc>> = row.get("verified_at");
        let date: chrono::DateTime<Utc> = row.get("date");
        let time: Option<String> = row.get("time");
        let location: Option<String> = row.get("location");
        let duration: Option<String> = row.get("duration");
        let created_at: chrono::DateTime<Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get("updated_at");
        let started_at: Option<chrono::DateTime<Utc>> = row.get("started_at");
        let ended_at: Option<chrono::DateTime<Utc>> = row.get("ended_at");

        Ok(Match {
            id,
            competition_id,
            competition_type,
            journee,
            phase,
            round,
            team1_id,
            team2_id,
            team1_score,
            team2_score,
            winner,
            manches,
            best_of,
            current_manche,
            current_scores,
            status,
            verified,
            verified_by,
            verified_at,
            date,
            time,
            location,
            duration,
            created_at,
            updated_at,
            started_at,
            ended_at,
        })
    }
}

#[async_trait]
impl MatchRepository for PostgresMatchRepository {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError> {
        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            journee: request.journee,
            phase: request.phase,
            round: request.round,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: request.best_of,
            current_manche: Some(0),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 0,
                team2: 0,
                manche: 0,
            }),
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: request.date,
            time: None,
            location: request.location,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        // Insert into database
        let query = r#"
            INSERT INTO matches (
                id, competition_id, competition_type, journee, phase, round,
                team1_id, team2_id, team1_score, team2_score, winner,
                manches, best_of, current_manche, current_scores, status,
                verified, verified_by, verified_at, date, time, location,
                duration, created_at, updated_at, started_at, ended_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16,
                $17, $18, $19, $20, $21, $22,
                $23, $24, $25, $26, $27
            )
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_obj.id)
            .bind(match_obj.competition_id)
            .bind(serde_json::to_string(&match_obj.competition_type).unwrap())
            .bind(match_obj.journee)
            .bind(match_obj.phase)
            .bind(match_obj.round)
            .bind(match_obj.team1_id)
            .bind(match_obj.team2_id)
            .bind(match_obj.team1_score)
            .bind(match_obj.team2_score)
            .bind(serde_json::to_string(&match_obj.winner).unwrap())
            .bind(sqlx::types::Json(&match_obj.manches))
            .bind(match_obj.best_of)
            .bind(match_obj.current_manche)
            .bind(sqlx::types::Json(match_obj.current_scores))
            .bind(serde_json::to_string(&match_obj.status).unwrap())
            .bind(match_obj.verified)
            .bind(match_obj.verified_by)
            .bind(match_obj.verified_at)
            .bind(match_obj.date)
            .bind(match_obj.time)
            .bind(match_obj.location)
            .bind(match_obj.duration)
            .bind(match_obj.created_at)
            .bind(match_obj.updated_at)
            .bind(match_obj.started_at)
            .bind(match_obj.ended_at)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, DatabaseError> {
        let query = "SELECT * FROM matches WHERE id = $1";
        let row = sqlx::query(query)
            .bind(match_id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_match(row).await?)),
            None => Ok(None),
        }
    }

    async fn get_matches(&self) -> Result<Vec<Match>, DatabaseError> {
        let query = "SELECT * FROM matches ORDER BY created_at DESC";
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        let query = "SELECT * FROM matches WHERE competition_id = $1 ORDER BY date, journee";
        let rows = sqlx::query(query)
            .bind(competition_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, DatabaseError> {
        let query = "SELECT * FROM matches WHERE status = $1 ORDER BY date";
        let rows = sqlx::query(query)
            .bind(serde_json::to_string(&status).unwrap())
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError> {
        // Check if match exists
        let existing = self.get_match(match_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut match_obj = existing.unwrap();

        // Update fields from request
        if let Some(team1_score) = request.team1_score {
            match_obj.team1_score = Some(team1_score);
        }
        if let Some(team2_score) = request.team2_score {
            match_obj.team2_score = Some(team2_score);
        }
        if let Some(status) = request.status {
            match_obj.status = status;
        }
        if let Some(winner) = request.winner {
            match_obj.winner = Some(winner);
        }
        if let Some(verified) = request.verified {
            match_obj.verified = verified;
        }

        match_obj.updated_at = Utc::now();

        // Update in database
        let query = r#"
            UPDATE matches SET
                team1_score = COALESCE($1, team1_score),
                team2_score = COALESCE($2, team2_score),
                winner = COALESCE($3, winner),
                status = COALESCE($4::match_status, status),
                verified = COALESCE($5, verified),
                updated_at = $6
            WHERE id = $7
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_obj.team1_score)
            .bind(match_obj.team2_score)
            .bind(serde_json::to_string(&match_obj.winner).unwrap())
            .bind(serde_json::to_string(&match_obj.status).unwrap())
            .bind(match_obj.verified)
            .bind(match_obj.updated_at)
            .bind(match_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Some(Self::row_to_match(row).await?))
    }

    async fn delete_match(&self, match_id: Uuid) -> Result<bool, DatabaseError> {
        let query = "DELETE FROM matches WHERE id = $1";
        let result = sqlx::query(query)
            .bind(match_id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn start_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        let query = r#"
            UPDATE matches SET
                status = 'in_progress',
                started_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'scheduled'
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: MatchWinner,
    ) -> Result<Match, DatabaseError> {
        let query = r#"
            UPDATE matches SET
                status = 'completed',
                winner = $2,
                ended_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'in_progress'
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .bind(serde_json::to_string(&winner).unwrap())
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn add_manche(
        &self,
        match_id: Uuid,
        manche: MancheScore,
    ) -> Result<Match, DatabaseError> {
        // Get current match
        let mut match_obj = self.get_match(match_id).await?.ok_or_else(|| {
            DatabaseError::NotFound(format!("Match with id {} not found", match_id))
        })?;

        // Initialize manches if None
        let mut manches = match_obj.manches.clone().unwrap_or_default();
        
        // Check if manche number already exists
        if let Some(existing) = manches.iter_mut().find(|m| m.number == manche.number) {
            *existing = manche;
        } else {
            manches.push(manche);
        }
        manches.sort_by_key(|m| m.number);

        // Calculate current scores
        let team1_total: i32 = manches.iter().map(|m| m.team1).sum();
        let team2_total: i32 = manches.iter().map(|m| m.team2).sum();
        let current_manche = manche.number;

        let current_scores = palet_shared::CurrentScores {
            team1: team1_total,
            team2: team2_total,
            manche: current_manche,
        };

        // Update match
        let query = r#"
            UPDATE matches SET
                manches = $2,
                current_manche = $3,
                current_scores = $4,
                team1_score = $5,
                team2_score = $6,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .bind(sqlx::types::Json(manches))
            .bind(Some(current_manche))
            .bind(sqlx::types::Json(Some(current_scores)))
            .bind(Some(team1_total))
            .bind(Some(team2_total))
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn update_manche(
        &self,
        match_id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, DatabaseError> {
        // Get current match
        let mut match_obj = self.get_match(match_id).await?.ok_or_else(|| {
            DatabaseError::NotFound(format!("Match with id {} not found", match_id))
        })?;

        // Update the specific manche
        if let Some(manches) = &mut match_obj.manches {
            if let Some(manche) = manches.iter_mut().find(|m| m.number == manche_number) {
                manche.team1 = team1_score;
                manche.team2 = team2_score;
            }
        }

        // Recalculate scores
        let team1_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team1).sum());
        let team2_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team2).sum());

        // Update in database
        let query = r#"
            UPDATE matches SET
                manches = $2,
                team1_score = $3,
                team2_score = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .bind(sqlx::types::Json(match_obj.manches))
            .bind(Some(team1_total))
            .bind(Some(team2_total))
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, DatabaseError> {
        let query = r#"
            UPDATE matches SET
                verified = true,
                verified_by = $2,
                verified_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'completed'
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .bind(verifier_id)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        let query = r#"
            UPDATE matches SET
                verified = false,
                verified_by = NULL,
                verified_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(match_id)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_match(row).await.map_err(DatabaseError::from)
    }

    async fn get_matches_by_date_range(
        &self,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) -> Result<Vec<Match>, DatabaseError> {
        let query = "SELECT * FROM matches WHERE date BETWEEN $1 AND $2 ORDER BY date";
        let rows = sqlx::query(query)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        let query = r#"
            SELECT * FROM matches 
            WHERE team1_id = $1 OR team2_id = $1 
            ORDER BY date DESC
        "#;
        let rows = sqlx::query(query)
            .bind(team_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        let query = "SELECT * FROM matches WHERE journee = $1 AND competition_id = $2 ORDER BY date";
        let rows = sqlx::query(query)
            .bind(journee)
            .bind(competition_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }

    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        self.get_matches_by_competition(championship_id).await
    }

    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        let query = r#"
            SELECT * FROM matches 
            WHERE competition_id = $1 AND status = 'completed' 
            ORDER BY date, journee
        "#;
        let rows = sqlx::query(query)
            .bind(championship_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(Self::row_to_match(row).await?);
        }
        Ok(matches)
    }
}

// ============================================
// MOCK MATCH REPOSITORY (For Testing)
// ============================================

/// Mock implementation of MatchRepository for testing
#[derive(Clone, Default)]
pub struct MockMatchRepository {
    pub matches: Vec<Match>,
    pub should_fail: bool,
}

impl MockMatchRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_matches(matches: Vec<Match>) -> Self {
        Self {
            matches,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_match(&mut self, match_obj: Match) {
        self.matches.push(match_obj);
    }

    pub fn find_match_mut(&mut self, match_id: Uuid) -> Option<&mut Match> {
        self.matches.iter_mut().find(|m| m.id == match_id)
    }
}

#[async_trait]
impl MatchRepository for MockMatchRepository {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            journee: request.journee,
            phase: request.phase,
            round: request.round,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: request.best_of,
            current_manche: Some(0),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 0,
                team2: 0,
                manche: 0,
            }),
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: request.date,
            time: None,
            location: request.location,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        Ok(match_obj)
    }

    async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.matches.iter().find(|m| m.id == match_id).cloned())
    }

    async fn get_matches(&self) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.matches.clone())
    }

    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.competition_id == competition_id)
            .cloned()
            .collect())
    }

    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.status == status)
            .cloned()
            .collect())
    }

    async fn update_match(
        &self,
        match_id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        // Find and update the match
        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            if let Some(team1_score) = request.team1_score {
                match_obj.team1_score = Some(team1_score);
            }
            if let Some(team2_score) = request.team2_score {
                match_obj.team2_score = Some(team2_score);
            }
            if let Some(status) = request.status {
                match_obj.status = status;
            }
            if let Some(winner) = request.winner {
                match_obj.winner = Some(winner);
            }
            if let Some(verified) = request.verified {
                match_obj.verified = verified;
            }
            match_obj.updated_at = Utc::now();

            Ok(Some(match_obj.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_match(&self, match_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let index = self.matches.iter().position(|m| m.id == match_id);
        Ok(index.is_some())
    }

    async fn start_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.status = MatchStatus::InProgress;
            match_obj.started_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: MatchWinner,
    ) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.status = MatchStatus::Completed;
            match_obj.winner = Some(winner);
            match_obj.ended_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn add_manche(
        &self,
        match_id: Uuid,
        manche: MancheScore,
    ) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            let mut manches = match_obj.manches.clone().unwrap_or_default();
            
            // Check if manche number already exists
            if let Some(existing) = manches.iter_mut().find(|m| m.number == manche.number) {
                *existing = manche;
            } else {
                manches.push(manche);
            }
            manches.sort_by_key(|m| m.number);

            let team1_total: i32 = manches.iter().map(|m| m.team1).sum();
            let team2_total: i32 = manches.iter().map(|m| m.team2).sum();
            let current_manche = manche.number;

            match_obj.manches = Some(manches);
            match_obj.current_manche = Some(current_manche);
            match_obj.current_scores = Some(palet_shared::CurrentScores {
                team1: team1_total,
                team2: team2_total,
                manche: current_manche,
            });
            match_obj.team1_score = Some(team1_total);
            match_obj.team2_score = Some(team2_total);
            match_obj.updated_at = Utc::now();

            // Check if match is completed (13 or 15 points)
            let best_of = match_obj.best_of.unwrap_or(13);
            if team1_total >= best_of || team2_total >= best_of {
                match_obj.status = MatchStatus::Completed;
                match_obj.winner = if team1_total >= best_of {
                    Some(MatchWinner::Team1)
                } else {
                    Some(MatchWinner::Team2)
                };
                match_obj.ended_at = Some(Utc::now());
            }

            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn update_manche(
        &self,
        match_id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            if let Some(manches) = &mut match_obj.manches {
                if let Some(manche) = manches.iter_mut().find(|m| m.number == manche_number) {
                    manche.team1 = team1_score;
                    manche.team2 = team2_score;
                }
            }

            // Recalculate scores
            let team1_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team1).sum());
            let team2_total: i32 = match_obj.manches.as_ref().map_or(0, |m| m.iter().map(|m| m.team2).sum());

            match_obj.team1_score = Some(team1_total);
            match_obj.team2_score = Some(team2_total);
            match_obj.updated_at = Utc::now();

            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn verify_match(&self, match_id: Uuid, verifier_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.verified = true;
            match_obj.verified_by = Some(verifier_id);
            match_obj.verified_at = Some(Utc::now());
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn unverify_match(&self, match_id: Uuid) -> Result<Match, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(match_obj) = self.matches.iter_mut().find(|m| m.id == match_id) {
            match_obj.verified = false;
            match_obj.verified_by = None;
            match_obj.verified_at = None;
            match_obj.updated_at = Utc::now();
            Ok(match_obj.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Match with id {} not found",
                match_id
            )))
        }
    }

    async fn get_matches_by_date_range(
        &self,
        _start_date: chrono::DateTime<Utc>,
        _end_date: chrono::DateTime<Utc>,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.matches.clone())
    }

    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| m.team1_id == team_id || m.team2_id == team_id)
            .cloned()
            .collect())
    }

    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| {
                m.journee == Some(journee) && m.competition_id == competition_id
            })
            .cloned()
            .collect())
    }

    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        self.get_matches_by_competition(championship_id).await
    }

    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .matches
            .iter()
            .filter(|m| {
                m.competition_id == championship_id && m.status == MatchStatus::Completed
            })
            .cloned()
            .collect())
    }
}
