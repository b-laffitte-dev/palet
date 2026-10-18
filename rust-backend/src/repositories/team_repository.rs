//! Team Repository - Database operations for teams
//!
//! This module provides the database operations for managing teams.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{Team, TeamStatus, TeamStats};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// TEAM REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for team repository operations
#[async_trait]
pub trait TeamRepository {
    // CRUD Operations
    async fn create_team(
        &self,
        name: String,
        club_id: Uuid,
        captain_id: Option<Uuid>,
    ) -> Result<Team, DatabaseError>;
    async fn get_team(&self, team_id: Uuid) -> Result<Option<Team>, DatabaseError>;
    async fn get_teams(&self) -> Result<Vec<Team>, DatabaseError>;
    async fn update_team(
        &self,
        team_id: Uuid,
        name: Option<String>,
        captain_id: Option<Uuid>,
        stats: Option<TeamStats>,
        status: Option<TeamStatus>,
    ) -> Result<Option<Team>, DatabaseError>;
    async fn delete_team(&self, team_id: Uuid) -> Result<bool, DatabaseError>;

    // Query Operations
    async fn get_teams_by_club(&self, club_id: Uuid) -> Result<Vec<Team>, DatabaseError>;
    async fn get_teams_by_status(&self, status: TeamStatus) -> Result<Vec<Team>, DatabaseError>;
    async fn get_teams_by_captain(&self, captain_id: Uuid) -> Result<Vec<Team>, DatabaseError>;

    // Search Operations
    async fn search_teams(&self, query: String) -> Result<Vec<Team>, DatabaseError>;
}

// ============================================
// POSTGRES TEAM REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of TeamRepository
#[derive(Clone)]
pub struct PostgresTeamRepository {
    pool: Arc<PgPool>,
}

impl PostgresTeamRepository {
    /// Create a new PostgresTeamRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper function to convert database row to Team
    async fn row_to_team(row: sqlx::postgres::PgRow) -> Result<Team, sqlx::Error> {
        let id: Uuid = row.get::<_, Uuid>("id");
        let name: String = row.get::<_, String>("name");
        let club_id: Uuid = row.get::<_, Uuid>("club_id");
        let captain_id: Option<Uuid> = row.get::<_, Option<Uuid>>("captain_id");
        let logo: Option<String> = row.get::<_, Option<String>>("logo");

        // Parse stats JSON
        let stats_json: serde_json::Value = row.get::<_, serde_json::Value>("stats");
        let stats: TeamStats = serde_json::from_value(stats_json).unwrap_or_default();

        let status: TeamStatus = row.get::<_, TeamStatus>("status");
        let created_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("updated_at");

        Ok(Team {
            id,
            name,
            club_id,
            captain_id,
            logo,
            stats,
            status,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl TeamRepository for PostgresTeamRepository {
    async fn create_team(
        &self,
        name: String,
        club_id: Uuid,
        captain_id: Option<Uuid>,
    ) -> Result<Team, DatabaseError> {
        let team = Team {
            id: Uuid::new_v4(),
            name,
            club_id,
            captain_id,
            logo: None,
            stats: TeamStats::default(),
            status: TeamStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert into database
        let query = r#"
            INSERT INTO teams (
                id, name, club_id, captain_id, logo, stats, status, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(team.id)
            .bind(&team.name)
            .bind(team.club_id)
            .bind(team.captain_id)
            .bind(team.logo)
            .bind(sqlx::types::Json(team.stats))
            .bind(team.status)
            .bind(team.created_at)
            .bind(team.updated_at)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_team(row).await.map_err(DatabaseError::from)
    }

    async fn get_team(&self, team_id: Uuid) -> Result<Option<Team>, DatabaseError> {
        let query = "SELECT * FROM teams WHERE id = $1";
        let row = sqlx::query(query)
            .bind(team_id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_team(row).await?)),
            None => Ok(None),
        }
    }

    async fn get_teams(&self) -> Result<Vec<Team>, DatabaseError> {
        let query = "SELECT * FROM teams ORDER BY name";
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row).await?);
        }
        Ok(teams)
    }

    async fn update_team(
        &self,
        team_id: Uuid,
        name: Option<String>,
        captain_id: Option<Uuid>,
        stats: Option<TeamStats>,
        status: Option<TeamStatus>,
    ) -> Result<Option<Team>, DatabaseError> {
        // Check if team exists
        let existing = self.get_team(team_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut team = existing.unwrap();

        // Update fields
        if let Some(name) = name {
            team.name = name;
        }
        if let Some(captain_id) = captain_id {
            team.captain_id = Some(captain_id);
        }
        if let Some(stats) = stats {
            team.stats = stats;
        }
        if let Some(status) = status {
            team.status = status;
        }

        team.updated_at = Utc::now();

        // Update in database
        let query = r#"
            UPDATE teams SET
                name = COALESCE($1, name),
                captain_id = COALESCE($2, captain_id),
                stats = COALESCE($3, stats),
                status = COALESCE($4::team_status, status),
                updated_at = $5
            WHERE id = $6
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(team.name)
            .bind(team.captain_id)
            .bind(sqlx::types::Json(team.stats))
            .bind(team.status)
            .bind(team.updated_at)
            .bind(team_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Some(Self::row_to_team(row).await?))
    }

    async fn delete_team(&self, team_id: Uuid) -> Result<bool, DatabaseError> {
        let query = "DELETE FROM teams WHERE id = $1";
        let result = sqlx::query(query)
            .bind(team_id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_teams_by_club(&self, club_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        let query = "SELECT * FROM teams WHERE club_id = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(club_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row).await?);
        }
        Ok(teams)
    }

    async fn get_teams_by_status(&self, status: TeamStatus) -> Result<Vec<Team>, DatabaseError> {
        let query = "SELECT * FROM teams WHERE status = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(status)
            .fetch_all(&*self.pool)
            .await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row).await?);
        }
        Ok(teams)
    }

    async fn get_teams_by_captain(&self, captain_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        let query = "SELECT * FROM teams WHERE captain_id = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(captain_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row).await?);
        }
        Ok(teams)
    }

    async fn search_teams(&self, query: String) -> Result<Vec<Team>, DatabaseError> {
        let search_pattern = format!("%{}%", query);
        let sql_query = "SELECT * FROM teams WHERE name ILIKE $1 ORDER BY name";
        let rows = sqlx::query(sql_query)
            .bind(search_pattern)
            .fetch_all(&*self.pool)
            .await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_team(row).await?);
        }
        Ok(teams)
    }
}

// ============================================
// MOCK TEAM REPOSITORY (For Testing)
// ============================================

/// Mock implementation of TeamRepository for testing
#[derive(Clone, Default)]
pub struct MockTeamRepository {
    pub teams: Vec<Team>,
    pub should_fail: bool,
}

impl MockTeamRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_teams(teams: Vec<Team>) -> Self {
        Self {
            teams,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    pub fn find_team_mut(&mut self, team_id: Uuid) -> Option<&mut Team> {
        self.teams.iter_mut().find(|t| t.id == team_id)
    }
}

#[async_trait]
impl TeamRepository for MockTeamRepository {
    async fn create_team(
        &self,
        name: String,
        club_id: Uuid,
        captain_id: Option<Uuid>,
    ) -> Result<Team, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let team = Team {
            id: Uuid::new_v4(),
            name,
            club_id,
            captain_id,
            logo: None,
            stats: TeamStats::default(),
            status: TeamStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(team)
    }

    async fn get_team(&self, team_id: Uuid) -> Result<Option<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.teams.iter().find(|t| t.id == team_id).cloned())
    }

    async fn get_teams(&self) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.teams.clone())
    }

    async fn update_team(
        &self,
        team_id: Uuid,
        name: Option<String>,
        captain_id: Option<Uuid>,
        stats: Option<TeamStats>,
        status: Option<TeamStatus>,
    ) -> Result<Option<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(team) = self.teams.iter_mut().find(|t| t.id == team_id) {
            if let Some(name) = name {
                team.name = name;
            }
            if let Some(captain_id) = captain_id {
                team.captain_id = Some(captain_id);
            }
            if let Some(stats) = stats {
                team.stats = stats;
            }
            if let Some(status) = status {
                team.status = status;
            }

            team.updated_at = Utc::now();

            Ok(Some(team.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_team(&self, team_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let index = self.teams.iter().position(|t| t.id == team_id);
        Ok(index.is_some())
    }

    async fn get_teams_by_club(&self, club_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .teams
            .iter()
            .filter(|t| t.club_id == club_id)
            .cloned()
            .collect())
    }

    async fn get_teams_by_status(&self, status: TeamStatus) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .teams
            .iter()
            .filter(|t| t.status == status)
            .cloned()
            .collect())
    }

    async fn get_teams_by_captain(&self, captain_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .teams
            .iter()
            .filter(|t| t.captain_id == Some(captain_id))
            .cloned()
            .collect())
    }

    async fn search_teams(&self, query: String) -> Result<Vec<Team>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let query_lower = query.to_lowercase();
        Ok(self
            .teams
            .iter()
            .filter(|t| t.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect())
    }
}
