//! Championship Repository - Database operations for championships
//!
//! This module provides the database operations for managing Palet Vendéen championships.
//! It implements the TDD harness requirements for championship functionality.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, CompetitionStatus, CreateChampionshipRequest,
    TeamClassificationStatus, TeamStats, UpdateChampionshipRequest,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// CHAMPIONSHIP REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for championship repository operations
#[async_trait]
pub trait ChampionshipRepository {
    // CRUD Operations
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, DatabaseError>;
    async fn get_championship(&self, championship_id: Uuid) -> Result<Option<Championship>, DatabaseError>;
    async fn get_championships(&self) -> Result<Vec<Championship>, DatabaseError>;
    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, DatabaseError>;
    async fn delete_championship(&self, championship_id: Uuid) -> Result<bool, DatabaseError>;

    // Query Operations
    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, DatabaseError>;
    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, DatabaseError>;
    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, DatabaseError>;
    async fn get_championships_by_organizer(
        &self,
        organizer_id: Uuid,
    ) -> Result<Vec<Championship>, DatabaseError>;

    // Lifecycle Operations
    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError>;
    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, DatabaseError>;
    async fn complete_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Championship, DatabaseError>;

    // Classification Operations
    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, DatabaseError>;
    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, DatabaseError>;

    // Team Operations
    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), DatabaseError>;
    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), DatabaseError>;
    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, DatabaseError>;

    // Validation Operations
    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, DatabaseError>;
}

// ============================================
// POSTGRES CHAMPIONSHIP REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of ChampionshipRepository
#[derive(Clone)]
pub struct PostgresChampionshipRepository {
    pool: Arc<PgPool>,
}

impl PostgresChampionshipRepository {
    /// Create a new PostgresChampionshipRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper function to convert database row to Championship
    async fn row_to_championship(
        row: sqlx::postgres::PgRow,
    ) -> Result<Championship, sqlx::Error> {
        let id: Uuid = row.get("id");
        let name: String = row.get("name");
        let code: Option<String> = row.get("code");
        let season: String = row.get("season");
        let description: Option<String> = row.get("description");
        let league: String = row.get("league");
        let divisions: Vec<String> = row.get("divisions");
        let total_journees: i32 = row.get("total_journees");
        let journee_duration: Option<String> = row.get("journee_duration");
        
        // Parse classification rules
        let primary: ClassificationPrimary = row.get("classification_primary");
        let secondary: ClassificationSecondary = row.get("classification_secondary");
        let tertiary: ClassificationTertiary = row.get("classification_tertiary");
        let classification_rules = palet_shared::ClassificationRules {
            primary,
            secondary,
            tertiary,
        };

        let status: CompetitionStatus = row.get("status");
        let current_journee: Option<i32> = row.get("current_journee");
        let current_phase: Option<String> = row.get("current_phase");
        let organizer_id: Uuid = row.get("organizer_id");
        let created_at: chrono::DateTime<Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get("updated_at");

        Ok(Championship {
            id,
            name,
            code,
            season,
            description,
            league,
            divisions,
            total_journees,
            journee_duration,
            classification_rules,
            status,
            current_journee,
            current_phase,
            organizer_id,
            created_at,
            updated_at,
        })
    }

    // Helper function to convert database row to ChampionshipClassification
    async fn row_to_classification(
        row: sqlx::postgres::PgRow,
    ) -> Result<ChampionshipClassification, sqlx::Error> {
        let division: String = row.get("division");
        let journee: i32 = row.get("journee");
        let updated_at: chrono::DateTime<Utc> = row.get("updated_at");
        
        // Parse teams JSON
        let teams_json: serde_json::Value = row.get("teams");
        let teams: Vec<ChampionshipTeam> = serde_json::from_value(teams_json)?;

        Ok(ChampionshipClassification {
            division,
            journee,
            updated_at,
            teams,
        })
    }

    // Helper function to convert database row to ChampionshipTeam
    async fn row_to_championship_team(
        row: sqlx::postgres::PgRow,
    ) -> Result<ChampionshipTeam, sqlx::Error> {
        let position: i32 = row.get("position");
        let team_id: Uuid = row.get("team_id");
        
        // Parse stats JSON
        let stats_json: serde_json::Value = row.get("stats");
        let stats: TeamStats = serde_json::from_value(stats_json)?;

        // Parse form JSON
        let form_json: serde_json::Value = row.get("form");
        let form: Vec<palet_shared::MatchResult> = serde_json::from_value(form_json).unwrap_or_default();

        let status: TeamClassificationStatus = row.get("status");

        Ok(ChampionshipTeam {
            position,
            team_id,
            stats,
            form,
            status,
        })
    }
}

#[async_trait]
impl ChampionshipRepository for PostgresChampionshipRepository {
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, DatabaseError> {
        let championship = Championship {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code,
            season: request.season,
            description: request.description,
            league: request.league,
            divisions: request.divisions,
            total_journees: request.total_journees,
            journee_duration: request.journee_duration,
            classification_rules: request.classification_rules,
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert into database
        let query = r#"
            INSERT INTO championships (
                id, name, code, season, description, league, divisions,
                total_journees, journee_duration, classification_primary, classification_secondary, classification_tertiary,
                status, current_journee, current_phase, organizer_id, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17, $18
            )
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship.id)
            .bind(&championship.name)
            .bind(championship.code)
            .bind(&championship.season)
            .bind(championship.description)
            .bind(&championship.league)
            .bind(&championship.divisions)
            .bind(championship.total_journees)
            .bind(championship.journee_duration)
            .bind(championship.classification_rules.primary)
            .bind(championship.classification_rules.secondary)
            .bind(championship.classification_rules.tertiary)
            .bind(championship.status)
            .bind(championship.current_journee)
            .bind(championship.current_phase)
            .bind(championship.organizer_id)
            .bind(championship.created_at)
            .bind(championship.updated_at)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_championship(row).await.map_err(DatabaseError::from)
    }

    async fn get_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships WHERE id = $1";
        let row = sqlx::query(query)
            .bind(championship_id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_championship(row).await?)),
            None => Ok(None),
        }
    }

    async fn get_championships(&self) -> Result<Vec<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships ORDER BY season DESC, name";
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;

        let mut championships = Vec::new();
        for row in rows {
            championships.push(Self::row_to_championship(row).await?);
        }
        Ok(championships)
    }

    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, DatabaseError> {
        // Check if championship exists
        let existing = self.get_championship(championship_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut championship = existing.unwrap();

        // Update fields from request
        if let Some(name) = request.name {
            championship.name = name;
        }
        if let Some(code) = request.code {
            championship.code = Some(code);
        }
        if let Some(season) = request.season {
            championship.season = season;
        }
        if let Some(description) = request.description {
            championship.description = Some(description);
        }
        if let Some(league) = request.league {
            championship.league = league;
        }
        if let Some(divisions) = request.divisions {
            championship.divisions = divisions;
        }
        if let Some(total_journees) = request.total_journees {
            championship.total_journees = total_journees;
        }
        if let Some(journee_duration) = request.journee_duration {
            championship.journee_duration = Some(journee_duration);
        }
        if let Some(classification_rules) = request.classification_rules {
            championship.classification_rules = classification_rules;
        }
        if let Some(status) = request.status {
            championship.status = status;
        }
        if let Some(current_journee) = request.current_journee {
            championship.current_journee = Some(current_journee);
        }
        if let Some(current_phase) = request.current_phase {
            championship.current_phase = Some(current_phase);
        }

        championship.updated_at = Utc::now();

        // Update in database
        let query = r#"
            UPDATE championships SET
                name = COALESCE($1, name),
                code = COALESCE($2, code),
                season = COALESCE($3, season),
                description = COALESCE($4, description),
                league = COALESCE($5, league),
                divisions = COALESCE($6, divisions),
                total_journees = COALESCE($7, total_journees),
                journee_duration = COALESCE($8, journee_duration),
                classification_primary = COALESCE($9::classification_primary, classification_primary),
                classification_secondary = COALESCE($10::classification_secondary, classification_secondary),
                classification_tertiary = COALESCE($11::classification_tertiary, classification_tertiary),
                status = COALESCE($12::competition_status, status),
                current_journee = COALESCE($13, current_journee),
                current_phase = COALESCE($14, current_phase),
                updated_at = $15
            WHERE id = $16
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship.name)
            .bind(championship.code)
            .bind(championship.season)
            .bind(championship.description)
            .bind(championship.league)
            .bind(championship.divisions)
            .bind(championship.total_journees)
            .bind(championship.journee_duration)
            .bind(championship.classification_rules.primary)
            .bind(championship.classification_rules.secondary)
            .bind(championship.classification_rules.tertiary)
            .bind(championship.status)
            .bind(championship.current_journee)
            .bind(championship.current_phase)
            .bind(championship.updated_at)
            .bind(championship_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Some(Self::row_to_championship(row).await?))
    }

    async fn delete_championship(&self, championship_id: Uuid) -> Result<bool, DatabaseError> {
        let query = "DELETE FROM championships WHERE id = $1";
        let result = sqlx::query(query)
            .bind(championship_id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships WHERE league = $1 ORDER BY season DESC";
        let rows = sqlx::query(query)
            .bind(league)
            .fetch_all(&*self.pool)
            .await?;

        let mut championships = Vec::new();
        for row in rows {
            championships.push(Self::row_to_championship(row).await?);
        }
        Ok(championships)
    }

    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships WHERE season = $1 ORDER BY league, name";
        let rows = sqlx::query(query)
            .bind(season)
            .fetch_all(&*self.pool)
            .await?;

        let mut championships = Vec::new();
        for row in rows {
            championships.push(Self::row_to_championship(row).await?);
        }
        Ok(championships)
    }

    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships WHERE status = $1 ORDER BY season DESC, name";
        let rows = sqlx::query(query)
            .bind(status)
            .fetch_all(&*self.pool)
            .await?;

        let mut championships = Vec::new();
        for row in rows {
            championships.push(Self::row_to_championship(row).await?);
        }
        Ok(championships)
    }

    async fn get_championships_by_organizer(
        &self,
        organizer_id: Uuid,
    ) -> Result<Vec<Championship>, DatabaseError> {
        let query = "SELECT * FROM championships WHERE organizer_id = $1 ORDER BY season DESC";
        let rows = sqlx::query(query)
            .bind(organizer_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut championships = Vec::new();
        for row in rows {
            championships.push(Self::row_to_championship(row).await?);
        }
        Ok(championships)
    }

    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        let query = r#"
            UPDATE championships SET
                status = 'in_progress',
                current_journee = 1,
                updated_at = NOW()
            WHERE id = $1 AND status = 'pending'
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship_id)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_championship(row).await.map_err(DatabaseError::from)
    }

    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        // Get current championship
        let championship = self.get_championship(championship_id).await?.ok_or_else(|| {
            DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            ))
        })?;

        let current_journee = championship.current_journee.unwrap_or(0);
        let next_journee = current_journee + 1;

        // Check if we can proceed to next journee
        if next_journee > championship.total_journees {
            return Err(DatabaseError::ValidationError(format!(
                "Cannot proceed to journee {}: championship only has {} journees",
                next_journee, championship.total_journees
            )));
        }

        let query = r#"
            UPDATE championships SET
                current_journee = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship_id)
            .bind(next_journee)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_championship(row).await.map_err(DatabaseError::from)
    }

    async fn complete_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        let query = r#"
            UPDATE championships SET
                status = 'completed',
                current_phase = 'finished',
                updated_at = NOW()
            WHERE id = $1 AND status != 'completed'
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship_id)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_championship(row).await.map_err(DatabaseError::from)
    }

    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, DatabaseError> {
        let query = match division {
            Some(div) => "SELECT * FROM championship_classifications WHERE championship_id = $1 AND division = $2",
            None => "SELECT * FROM championship_classifications WHERE championship_id = $1",
        };

        let mut query_builder = sqlx::query(query);
        query_builder = query_builder.bind(championship_id);
        
        if let Some(div) = division {
            query_builder = query_builder.bind(div);
        }

        let rows = query_builder.fetch_all(&*self.pool).await?;

        let mut classifications = Vec::new();
        for row in rows {
            classifications.push(Self::row_to_classification(row).await?);
        }
        Ok(classifications)
    }

    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, DatabaseError> {
        let query = r#"
            INSERT INTO championship_classifications (championship_id, division, journee, updated_at, teams)
            VALUES ($1, $2, $3, NOW(), $4)
            ON CONFLICT (championship_id, division, journee)
            DO UPDATE SET
                teams = EXCLUDED.teams,
                updated_at = NOW()
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(championship_id)
            .bind(division)
            .bind(classification.journee)
            .bind(sqlx::types::Json(classification.teams))
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_classification(row).await.map_err(DatabaseError::from)
    }

    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), DatabaseError> {
        // Check if team is already registered
        let query = "SELECT 1 FROM championship_teams WHERE championship_id = $1 AND team_id = $2";
        let exists = sqlx::query_scalar::<_, i32>(query)
            .bind(championship_id)
            .bind(team_id)
            .fetch_optional(&*self.pool)
            .await?;

        if exists.is_some() {
            return Ok(()); // Team already registered
        }

        // Add team to championship
        let query = r#"
            INSERT INTO championship_teams (championship_id, team_id, division, joined_at)
            VALUES ($1, $2, $3, NOW())
        "#;
        sqlx::query(query)
            .bind(championship_id)
            .bind(team_id)
            .bind(division)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), DatabaseError> {
        let query = "DELETE FROM championship_teams WHERE championship_id = $1 AND team_id = $2";
        sqlx::query(query)
            .bind(championship_id)
            .bind(team_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, DatabaseError> {
        let query = match division {
            Some(div) => "SELECT * FROM championship_teams WHERE championship_id = $1 AND division = $2 ORDER BY position",
            None => "SELECT * FROM championship_teams WHERE championship_id = $1 ORDER BY position",
        };

        let mut query_builder = sqlx::query(query);
        query_builder = query_builder.bind(championship_id);
        
        if let Some(div) = division {
            query_builder = query_builder.bind(div);
        }

        let rows = query_builder.fetch_all(&*self.pool).await?;

        let mut teams = Vec::new();
        for row in rows {
            teams.push(Self::row_to_championship_team(row).await?);
        }
        Ok(teams)
    }

    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, DatabaseError> {
        // Check if championship exists and is in registration phase
        let championship = self.get_championship(championship_id).await?.ok_or_else(|| {
            DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            ))
        })?;

        // Check if team is already registered
        let query = "SELECT 1 FROM championship_teams WHERE championship_id = $1 AND team_id = $2";
        let exists = sqlx::query_scalar::<_, i32>(query)
            .bind(championship_id)
            .bind(team_id)
            .fetch_optional(&*self.pool)
            .await?;

        // Check if registration is open
        let registration_open = championship.status == CompetitionStatus::Registration
            || championship.status == CompetitionStatus::Pending;

        Ok(exists.is_none() && registration_open)
    }
}

// ============================================
// MOCK CHAMPIONSHIP REPOSITORY (For Testing)
// ============================================

/// Mock implementation of ChampionshipRepository for testing
#[derive(Clone, Default)]
pub struct MockChampionshipRepository {
    pub championships: Vec<Championship>,
    pub classifications: Vec<ChampionshipClassification>,
    pub should_fail: bool,
}

impl MockChampionshipRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_championships(championships: Vec<Championship>) -> Self {
        Self {
            championships,
            ..Default::default()
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_championship(&mut self, championship: Championship) {
        self.championships.push(championship);
    }

    pub fn find_championship_mut(&mut self, championship_id: Uuid) -> Option<&mut Championship> {
        self.championships
            .iter_mut()
            .find(|c| c.id == championship_id)
    }
}

#[async_trait]
impl ChampionshipRepository for MockChampionshipRepository {
    async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let championship = Championship {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code,
            season: request.season,
            description: request.description,
            league: request.league,
            divisions: request.divisions,
            total_journees: request.total_journees,
            journee_duration: request.journee_duration,
            classification_rules: request.classification_rules,
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(championship)
    }

    async fn get_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Option<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .championships
            .iter()
            .find(|c| c.id == championship_id)
            .cloned())
    }

    async fn get_championships(&self) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.championships.clone())
    }

    async fn update_championship(
        &self,
        championship_id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            if let Some(name) = request.name {
                championship.name = name;
            }
            if let Some(code) = request.code {
                championship.code = Some(code);
            }
            if let Some(season) = request.season {
                championship.season = season;
            }
            if let Some(description) = request.description {
                championship.description = Some(description);
            }
            if let Some(league) = request.league {
                championship.league = league;
            }
            if let Some(divisions) = request.divisions {
                championship.divisions = divisions;
            }
            if let Some(total_journees) = request.total_journees {
                championship.total_journees = total_journees;
            }
            if let Some(journee_duration) = request.journee_duration {
                championship.journee_duration = Some(journee_duration);
            }
            if let Some(classification_rules) = request.classification_rules {
                championship.classification_rules = classification_rules;
            }
            if let Some(status) = request.status {
                championship.status = status;
            }
            if let Some(current_journee) = request.current_journee {
                championship.current_journee = Some(current_journee);
            }
            if let Some(current_phase) = request.current_phase {
                championship.current_phase = Some(current_phase);
            }

            championship.updated_at = Utc::now();

            Ok(Some(championship.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_championship(&self, championship_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let index = self
            .championships
            .iter()
            .position(|c| c.id == championship_id);
        Ok(index.is_some())
    }

    async fn get_championships_by_league(
        &self,
        league: String,
    ) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_championships_by_season(
        &self,
        season: String,
    ) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.season == season)
            .cloned()
            .collect())
    }

    async fn get_championships_by_status(
        &self,
        status: CompetitionStatus,
    ) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.status == status)
            .cloned()
            .collect())
    }

    async fn get_championships_by_organizer(
        &self,
        organizer_id: Uuid,
    ) -> Result<Vec<Championship>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .championships
            .iter()
            .filter(|c| c.organizer_id == organizer_id)
            .cloned()
            .collect())
    }

    async fn start_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            championship.status = CompetitionStatus::InProgress;
            championship.current_journee = Some(1);
            championship.updated_at = Utc::now();
            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn next_journee(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            let current_journee = championship.current_journee.unwrap_or(0);
            let next_journee = current_journee + 1;

            if next_journee > championship.total_journees {
                return Err(DatabaseError::ValidationError(format!(
                    "Cannot proceed to journee {}: championship only has {} journees",
                    next_journee, championship.total_journees
                )));
            }

            championship.current_journee = Some(next_journee);
            championship.updated_at = Utc::now();

            // If this is the last journee, mark as completed
            if next_journee == championship.total_journees {
                championship.status = CompetitionStatus::Completed;
                championship.current_phase = Some("finished".to_string());
            }

            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn complete_championship(&self, championship_id: Uuid) -> Result<Championship, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(championship) = self.championships.iter_mut().find(|c| c.id == championship_id) {
            championship.status = CompetitionStatus::Completed;
            championship.current_phase = Some("finished".to_string());
            championship.updated_at = Utc::now();
            Ok(championship.clone())
        } else {
            Err(DatabaseError::NotFound(format!(
                "Championship with id {} not found",
                championship_id
            )))
        }
    }

    async fn get_championship_classification(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipClassification>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let classifications = if let Some(div) = division {
            self.classifications
                .iter()
                .filter(|c| c.championship_id == championship_id && c.division == div)
                .cloned()
                .collect()
        } else {
            self.classifications
                .iter()
                .filter(|c| c.championship_id == championship_id)
                .cloned()
                .collect()
        };

        Ok(classifications)
    }

    async fn update_championship_classification(
        &self,
        championship_id: Uuid,
        division: String,
        classification: ChampionshipClassification,
    ) -> Result<ChampionshipClassification, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        // Find and update existing classification or add new one
        if let Some(existing) = self
            .classifications
            .iter_mut()
            .find(|c| c.championship_id == championship_id && c.division == division && c.journee == classification.journee)
        {
            *existing = classification.clone();
        } else {
            self.classifications.push(classification.clone());
        }

        Ok(classification)
    }

    async fn add_team_to_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
        division: String,
    ) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(())
    }

    async fn remove_team_from_championship(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(())
    }

    async fn get_championship_teams(
        &self,
        championship_id: Uuid,
        division: Option<String>,
    ) -> Result<Vec<ChampionshipTeam>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        // For mock, return empty vector
        Ok(Vec::new())
    }

    async fn validate_championship_registration(
        &self,
        championship_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        // For mock, always return true
        Ok(true)
    }
}
