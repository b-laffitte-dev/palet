//! Repository des disponibilités des joueurs pour un match (Module 8, E3).

use async_trait::async_trait;
use palet_shared::{AvailabilityStatus, MatchAvailability};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::DatabaseError;
use crate::repositories::enum_conv::{enum_from_db_string, enum_to_db_string};

#[async_trait]
pub trait AvailabilityRepository {
    /// Déclare/met à jour la dispo d'un joueur pour un match (upsert).
    async fn upsert(
        &self,
        match_id: Uuid,
        player_id: Uuid,
        status: AvailabilityStatus,
        commentaire: Option<String>,
    ) -> Result<MatchAvailability, DatabaseError>;

    /// Toutes les dispo déclarées pour un match.
    async fn get_for_match(&self, match_id: Uuid) -> Result<Vec<MatchAvailability>, DatabaseError>;

    /// Toutes les dispo déclarées par un joueur.
    async fn get_for_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<MatchAvailability>, DatabaseError>;
}

pub struct PostgresAvailabilityRepository {
    pool: Arc<PgPool>,
}

impl PostgresAvailabilityRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_availability(row: sqlx::postgres::PgRow) -> Result<MatchAvailability, DatabaseError> {
        Ok(MatchAvailability {
            match_id: row.get("match_id"),
            player_id: row.get("player_id"),
            status: enum_from_db_string(&row.get::<String, _>("status"))?,
            commentaire: row.get("commentaire"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[async_trait]
impl AvailabilityRepository for PostgresAvailabilityRepository {
    async fn upsert(
        &self,
        match_id: Uuid,
        player_id: Uuid,
        status: AvailabilityStatus,
        commentaire: Option<String>,
    ) -> Result<MatchAvailability, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO match_disponibilites (match_id, player_id, status, commentaire, updated_at)
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (match_id, player_id) DO UPDATE SET
                status = EXCLUDED.status,
                commentaire = EXCLUDED.commentaire,
                updated_at = NOW()
            RETURNING match_id, player_id, status, commentaire, updated_at
            "#,
        )
        .bind(match_id)
        .bind(player_id)
        .bind(enum_to_db_string(&status))
        .bind(commentaire)
        .fetch_one(&*self.pool)
        .await?;
        Self::row_to_availability(row)
    }

    async fn get_for_match(&self, match_id: Uuid) -> Result<Vec<MatchAvailability>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT match_id, player_id, status, commentaire, updated_at
            FROM match_disponibilites
            WHERE match_id = $1
            ORDER BY updated_at DESC
            "#,
        )
        .bind(match_id)
        .fetch_all(&*self.pool)
        .await?;
        rows.into_iter().map(Self::row_to_availability).collect()
    }

    async fn get_for_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<MatchAvailability>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT match_id, player_id, status, commentaire, updated_at
            FROM match_disponibilites
            WHERE player_id = $1
            ORDER BY updated_at DESC
            "#,
        )
        .bind(player_id)
        .fetch_all(&*self.pool)
        .await?;
        rows.into_iter().map(Self::row_to_availability).collect()
    }
}

/// Helper de test : construit une dispo en mémoire.
pub fn availability_fixture(
    match_id: Uuid,
    player_id: Uuid,
    status: AvailabilityStatus,
) -> MatchAvailability {
    MatchAvailability {
        match_id,
        player_id,
        status,
        commentaire: None,
        updated_at: OffsetDateTime::now_utc(),
    }
}
