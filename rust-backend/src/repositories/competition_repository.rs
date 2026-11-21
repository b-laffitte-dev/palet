//! Competition Repository - compétitions "événement" (tournois/coupe/open) et
//! inscriptions d'équipes (tables competitions + registrations).

use crate::db::DatabaseError;
use crate::repositories::enum_conv::{enum_from_db_string, enum_to_db_string};
use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{
    CompetitionEvent, CompetitionStatus, CompetitionType, CreateCompetitionRequest, Registration,
    UpdateCompetitionRequest,
};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait CompetitionRepository: Send + Sync {
    async fn create_competition(
        &self,
        request: CreateCompetitionRequest,
    ) -> Result<CompetitionEvent, DatabaseError>;
    async fn get_competition(&self, id: Uuid) -> Result<Option<CompetitionEvent>, DatabaseError>;
    async fn get_competitions(
        &self,
        competition_type: Option<CompetitionType>,
        status: Option<CompetitionStatus>,
        season: Option<String>,
    ) -> Result<Vec<CompetitionEvent>, DatabaseError>;
    async fn update_competition(
        &self,
        id: Uuid,
        request: UpdateCompetitionRequest,
    ) -> Result<Option<CompetitionEvent>, DatabaseError>;
    async fn delete_competition(&self, id: Uuid) -> Result<bool, DatabaseError>;

    async fn register_team(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        registered_by: Uuid,
    ) -> Result<Registration, DatabaseError>;
    async fn get_registrations(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Registration>, DatabaseError>;
    async fn get_confirmed_team_ids(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Uuid>, DatabaseError>;
    async fn confirm_registration(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        confirmed_by: Uuid,
    ) -> Result<Option<Registration>, DatabaseError>;
    async fn count_registrations(&self, competition_id: Uuid) -> Result<i32, DatabaseError>;
}

const EVENT_SELECT: &str = "c.id, c.name, c.code, c.competition_type, c.category, c.material, c.season, c.description, c.location, c.format, c.registration_start, c.registration_end, c.start_date, c.end_date, c.status, c.max_teams, c.fee, c.is_public, c.organizer_id, c.created_at, c.updated_at, (SELECT count(*)::int FROM registrations r WHERE r.competition_id = c.id) AS registered_teams";

#[derive(Clone)]
pub struct PostgresCompetitionRepository {
    pool: Arc<PgPool>,
}

impl PostgresCompetitionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_event(row: sqlx::postgres::PgRow) -> Result<CompetitionEvent, DatabaseError> {
        Ok(CompetitionEvent {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            competition_type: enum_from_db_string(&row.get::<String, _>("competition_type"))?,
            category: enum_from_db_string(&row.get::<String, _>("category"))?,
            material: enum_from_db_string(&row.get::<String, _>("material"))?,
            season: row.get("season"),
            description: row.get("description"),
            location: row.get("location"),
            format: enum_from_db_string(&row.get::<String, _>("format"))?,
            registration_start: row.get("registration_start"),
            registration_end: row.get("registration_end"),
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
            status: enum_from_db_string(&row.get::<String, _>("status"))?,
            max_teams: row.get("max_teams"),
            registered_teams: row.try_get("registered_teams").unwrap_or(0),
            fee: row.get("fee"),
            is_public: row.get("is_public"),
            organizer_id: row.get("organizer_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn row_to_registration(row: sqlx::postgres::PgRow) -> Result<Registration, DatabaseError> {
        Ok(Registration {
            competition_id: row.get("competition_id"),
            team_id: row.get("team_id"),
            status: enum_from_db_string(&row.get::<String, _>("status"))?,
            registered_by: row.get("registered_by"),
            paid: row.get("paid"),
            created_at: row.get("created_at"),
            confirmed_at: row.get("confirmed_at"),
            confirmed_by: row.get("confirmed_by"),
        })
    }
}

#[async_trait]
impl CompetitionRepository for PostgresCompetitionRepository {
    async fn create_competition(
        &self,
        request: CreateCompetitionRequest,
    ) -> Result<CompetitionEvent, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO competitions (id, name, code, competition_type, category, material, season,
                description, location, format, registration_start, registration_end, start_date, end_date,
                status, max_teams, fee, is_public, organizer_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 'pending', $15, $16, $17, $18, $19, $19)
            RETURNING id, name, code, competition_type, category, material, season,
                description, location, format, registration_start, registration_end, start_date, end_date,
                status, max_teams, fee, is_public, organizer_id, created_at, updated_at,
                0 AS registered_teams
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&request.name)
        .bind(&request.code)
        .bind(enum_to_db_string(&request.competition_type))
        .bind(enum_to_db_string(&request.category))
        .bind(enum_to_db_string(&request.material))
        .bind(&request.season)
        .bind(&request.description)
        .bind(&request.location)
        .bind(enum_to_db_string(&request.format))
        .bind(request.registration_start)
        .bind(request.registration_end)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.max_teams)
        .bind(request.fee)
        .bind(request.is_public)
        .bind(request.organizer_id)
        .bind(Utc::now())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("competitions_code_key") => {
                DatabaseError::ValidationError(format!(
                    "Le code {} est déjà utilisé par une autre compétition",
                    request.code.as_deref().unwrap_or("?")
                ))
            }
            other => DatabaseError::from(other),
        })?;
        Self::row_to_event(row)
    }

    async fn get_competition(&self, id: Uuid) -> Result<Option<CompetitionEvent>, DatabaseError> {
        let row = sqlx::query(sqlx::AssertSqlSafe(format!(
            "SELECT {EVENT_SELECT} FROM competitions c WHERE c.id = $1"
        )))
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;
        row.map(Self::row_to_event).transpose()
    }

    async fn get_competitions(
        &self,
        competition_type: Option<CompetitionType>,
        status: Option<CompetitionStatus>,
        season: Option<String>,
    ) -> Result<Vec<CompetitionEvent>, DatabaseError> {
        let mut query = format!("SELECT {EVENT_SELECT} FROM competitions c");
        let mut conditions: Vec<String> = Vec::new();
        if competition_type.is_some() {
            conditions.push("c.competition_type = $1".to_string());
        }
        if status.is_some() {
            conditions.push(format!("c.status = ${}", conditions.len() + 1));
        }
        if season.is_some() {
            conditions.push(format!("c.season = ${}", conditions.len() + 1));
        }
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }
        query.push_str(" ORDER BY c.start_date NULLS LAST, c.created_at DESC");

        let mut q = sqlx::query(sqlx::AssertSqlSafe(query));
        if let Some(t) = &competition_type {
            q = q.bind(enum_to_db_string(t));
        }
        if let Some(s) = &status {
            q = q.bind(enum_to_db_string(s));
        }
        if let Some(s) = &season {
            q = q.bind(s);
        }
        let rows = q.fetch_all(&*self.pool).await?;
        rows.into_iter().map(Self::row_to_event).collect()
    }

    async fn update_competition(
        &self,
        id: Uuid,
        request: UpdateCompetitionRequest,
    ) -> Result<Option<CompetitionEvent>, DatabaseError> {
        let now = Utc::now();
        let row = sqlx::query(
            r#"
            UPDATE competitions SET
                name = COALESCE($1, name),
                code = COALESCE($2, code),
                status = COALESCE($3, status),
                description = COALESCE($4, description),
                location = COALESCE($5, location),
                format = COALESCE($6, format),
                registration_start = COALESCE($7, registration_start),
                registration_end = COALESCE($8, registration_end),
                start_date = COALESCE($9, start_date),
                end_date = COALESCE($10, end_date),
                max_teams = COALESCE($11, max_teams),
                fee = COALESCE($12, fee),
                is_public = COALESCE($13, is_public),
                updated_at = $14
            WHERE id = $15
            RETURNING id, name, code, competition_type, category, material, season, description, location, format, registration_start, registration_end, start_date, end_date, status, max_teams, fee, is_public, organizer_id, created_at, updated_at, (SELECT count(*)::int FROM registrations r WHERE r.competition_id = competitions.id) AS registered_teams
            "#,
        )
        .bind(&request.name)
        .bind(&request.code)
        .bind(request.status.as_ref().map(enum_to_db_string))
        .bind(&request.description)
        .bind(&request.location)
        .bind(request.format.as_ref().map(enum_to_db_string))
        .bind(request.registration_start)
        .bind(request.registration_end)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.max_teams)
        .bind(request.fee)
        .bind(request.is_public)
        .bind(now)
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;
        row.map(Self::row_to_event).transpose()
    }

    async fn delete_competition(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query("DELETE FROM competitions WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn register_team(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        registered_by: Uuid,
    ) -> Result<Registration, DatabaseError> {
        let now = Utc::now();
        let row = sqlx::query(
            r#"
            INSERT INTO registrations (competition_id, team_id, status, registered_by, created_at)
            VALUES ($1, $2, 'pending', $3, $4)
            RETURNING *
            "#,
        )
        .bind(competition_id)
        .bind(team_id)
        .bind(registered_by)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("registrations_pkey") => {
                DatabaseError::ValidationError(
                    "Cette équipe est déjà inscrite à cette compétition".to_string(),
                )
            }
            sqlx::Error::Database(db_err)
                if db_err.constraint() == Some("registrations_competition_id_fkey") =>
            {
                DatabaseError::NotFound("Compétition introuvable".to_string())
            }
            sqlx::Error::Database(db_err)
                if db_err.constraint() == Some("registrations_team_id_fkey") =>
            {
                DatabaseError::NotFound("Équipe introuvable".to_string())
            }
            other => DatabaseError::from(other),
        })?;
        Self::row_to_registration(row)
    }

    async fn get_registrations(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Registration>, DatabaseError> {
        let rows = sqlx::query(
            "SELECT * FROM registrations WHERE competition_id = $1 ORDER BY created_at",
        )
        .bind(competition_id)
        .fetch_all(&*self.pool)
        .await?;
        rows.into_iter().map(Self::row_to_registration).collect()
    }

    async fn get_confirmed_team_ids(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Uuid>, DatabaseError> {
        let rows = sqlx::query(
            "SELECT team_id FROM registrations WHERE competition_id = $1 AND status = 'confirmed' ORDER BY created_at",
        )
        .bind(competition_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(rows.iter().map(|r| r.get::<Uuid, _>("team_id")).collect())
    }

    async fn confirm_registration(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        confirmed_by: Uuid,
    ) -> Result<Option<Registration>, DatabaseError> {
        let now = Utc::now();
        let row = sqlx::query(
            r#"
            UPDATE registrations SET status = 'confirmed', confirmed_at = $1, confirmed_by = $2
            WHERE competition_id = $3 AND team_id = $4
            RETURNING *
            "#,
        )
        .bind(now)
        .bind(confirmed_by)
        .bind(competition_id)
        .bind(team_id)
        .fetch_optional(&*self.pool)
        .await?;
        row.map(Self::row_to_registration).transpose()
    }

    async fn count_registrations(&self, competition_id: Uuid) -> Result<i32, DatabaseError> {
        let count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM registrations WHERE competition_id = $1")
                .bind(competition_id)
                .fetch_one(&*self.pool)
                .await?;
        Ok(count as i32)
    }
}
