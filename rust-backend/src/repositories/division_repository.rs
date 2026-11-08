//! Division Repository - Database operations for departmental divisions
//!
//! A division (D1, D2, D3…) belongs to a league (département) and a season.
//! Divisions are dynamic: the federal space creates/deletes them, and the
//! home page renders one tab per division.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{CreateDivisionRequest, Division};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// DIVISION REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for division repository operations
#[async_trait]
pub trait DivisionRepository {
    async fn create_division(
        &self,
        request: CreateDivisionRequest,
    ) -> Result<Division, DatabaseError>;
    async fn get_division(&self, id: Uuid) -> Result<Option<Division>, DatabaseError>;
    /// List divisions, optionally filtered by league, ordered by position.
    async fn get_divisions(&self, league: Option<String>) -> Result<Vec<Division>, DatabaseError>;
    async fn delete_division(&self, id: Uuid) -> Result<bool, DatabaseError>;
}

// ============================================
// POSTGRES DIVISION REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of DivisionRepository
#[derive(Clone)]
pub struct PostgresDivisionRepository {
    pool: Arc<PgPool>,
}

impl PostgresDivisionRepository {
    /// Create a new PostgresDivisionRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_division(row: sqlx::postgres::PgRow) -> Division {
        Division {
            id: row.get("id"),
            name: row.get("name"),
            league: row.get("league"),
            season: row.get("season"),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[async_trait]
impl DivisionRepository for PostgresDivisionRepository {
    async fn create_division(
        &self,
        request: CreateDivisionRequest,
    ) -> Result<Division, DatabaseError> {
        let (name, league, season) = request
            .validated()
            .map_err(DatabaseError::ValidationError)?;

        // New divisions are appended after the existing ones of the league+season.
        let position: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM divisions WHERE league = $1 AND season = $2",
        )
        .bind(&league)
        .bind(&season)
        .fetch_one(&*self.pool)
        .await?;

        let now = Utc::now();
        let row = sqlx::query(
            r#"
            INSERT INTO divisions (id, name, league, season, position, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&name)
        .bind(&league)
        .bind(&season)
        .bind(position)
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                DatabaseError::ValidationError(format!(
                    "La division {name} existe déjà pour {league} (saison {season})"
                ))
            }
            other => DatabaseError::from(other),
        })?;

        Ok(Self::row_to_division(row))
    }

    async fn get_division(&self, id: Uuid) -> Result<Option<Division>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM divisions WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(row.map(Self::row_to_division))
    }

    async fn get_divisions(&self, league: Option<String>) -> Result<Vec<Division>, DatabaseError> {
        let rows = match league {
            Some(league) => {
                sqlx::query(
                    "SELECT * FROM divisions WHERE league = $1 ORDER BY position ASC, name ASC",
                )
                .bind(league)
                .fetch_all(&*self.pool)
                .await?
            }
            None => {
                sqlx::query("SELECT * FROM divisions ORDER BY league ASC, position ASC, name ASC")
                    .fetch_all(&*self.pool)
                    .await?
            }
        };

        Ok(rows.into_iter().map(Self::row_to_division).collect())
    }

    async fn delete_division(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let row = sqlx::query("DELETE FROM divisions WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(row.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// In-memory mock mirroring the Postgres rules: validation, uniqueness per
    /// (league, season, name), auto-incremented position, ordered listing.
    struct MockDivisionRepository {
        divisions: Mutex<Vec<Division>>,
    }

    impl MockDivisionRepository {
        fn new() -> Self {
            Self {
                divisions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl DivisionRepository for MockDivisionRepository {
        async fn create_division(
            &self,
            request: CreateDivisionRequest,
        ) -> Result<Division, DatabaseError> {
            let (name, league, season) = request
                .validated()
                .map_err(DatabaseError::ValidationError)?;

            let mut divisions = self.divisions.lock().unwrap();
            if divisions
                .iter()
                .any(|d| d.league == league && d.season == season && d.name == name)
            {
                return Err(DatabaseError::ValidationError(format!(
                    "La division {name} existe déjà pour {league} (saison {season})"
                )));
            }

            let position = divisions
                .iter()
                .filter(|d| d.league == league && d.season == season)
                .map(|d| d.position)
                .max()
                .unwrap_or(0)
                + 1;
            let now = Utc::now();
            let division = Division {
                id: Uuid::new_v4(),
                name,
                league,
                season,
                position,
                created_at: now,
                updated_at: now,
            };
            divisions.push(division.clone());
            Ok(division)
        }

        async fn get_division(&self, id: Uuid) -> Result<Option<Division>, DatabaseError> {
            Ok(self
                .divisions
                .lock()
                .unwrap()
                .iter()
                .find(|d| d.id == id)
                .cloned())
        }

        async fn get_divisions(
            &self,
            league: Option<String>,
        ) -> Result<Vec<Division>, DatabaseError> {
            let mut divisions = self.divisions.lock().unwrap().clone();
            if let Some(league) = &league {
                divisions.retain(|d| &d.league == league);
            }
            divisions.sort_by(|a, b| {
                a.league
                    .cmp(&b.league)
                    .then(a.position.cmp(&b.position))
                    .then(a.name.cmp(&b.name))
            });
            Ok(divisions)
        }

        async fn delete_division(&self, id: Uuid) -> Result<bool, DatabaseError> {
            let mut divisions = self.divisions.lock().unwrap();
            let before = divisions.len();
            divisions.retain(|d| d.id != id);
            Ok(divisions.len() != before)
        }
    }

    fn request(name: &str) -> CreateDivisionRequest {
        CreateDivisionRequest {
            name: name.to_string(),
            league: "Deux-Sèvres".to_string(),
            season: "2026-2027".to_string(),
        }
    }

    #[tokio::test]
    async fn create_assigns_increasing_positions_per_league_and_season() {
        let repo = MockDivisionRepository::new();

        let d1 = repo.create_division(request("D1")).await.expect("d1");
        let d2 = repo.create_division(request("d2")).await.expect("d2");
        assert_eq!(d1.position, 1);
        assert_eq!(d2.position, 2);
        assert_eq!(d2.name, "D2");

        // Another season restarts at position 1.
        let mut other_season = request("D1");
        other_season.season = "2027-2028".to_string();
        let d1_next = repo.create_division(other_season).await.expect("d1 next");
        assert_eq!(d1_next.position, 1);
    }

    #[tokio::test]
    async fn create_rejects_duplicate_name_for_same_league_and_season() {
        let repo = MockDivisionRepository::new();
        repo.create_division(request("D1")).await.expect("d1");

        let err = repo.create_division(request("D1")).await.unwrap_err();

        assert!(matches!(err, DatabaseError::ValidationError(_)));
    }

    #[tokio::test]
    async fn create_rejects_invalid_name() {
        let repo = MockDivisionRepository::new();

        let err = repo.create_division(request("  ")).await.unwrap_err();

        assert!(matches!(err, DatabaseError::ValidationError(_)));
    }

    #[tokio::test]
    async fn get_divisions_filters_by_league_and_orders_by_position() {
        let repo = MockDivisionRepository::new();
        // Created D2 first on purpose: listing follows creation order
        // (position), not alphabetical order.
        repo.create_division(request("D2")).await.expect("d2");
        repo.create_division(request("D1")).await.expect("d1");
        let mut vendee = request("D1");
        vendee.league = "Vendée".to_string();
        repo.create_division(vendee).await.expect("v d1");

        let deux_sevres = repo
            .get_divisions(Some("Deux-Sèvres".to_string()))
            .await
            .expect("list");

        let names: Vec<&str> = deux_sevres.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, vec!["D2", "D1"]);
    }

    #[tokio::test]
    async fn delete_removes_division_and_reports_missing() {
        let repo = MockDivisionRepository::new();
        let d1 = repo.create_division(request("D1")).await.expect("d1");

        assert!(repo.delete_division(d1.id).await.expect("delete"));
        assert!(!repo.delete_division(d1.id).await.expect("delete again"));
        assert!(repo
            .get_divisions(Some("Deux-Sèvres".to_string()))
            .await
            .expect("list")
            .is_empty());
    }
}
