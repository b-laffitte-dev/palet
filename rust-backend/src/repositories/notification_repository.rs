//! Repository des notifications in-app (table `notifications`).

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::AppNotification;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

#[async_trait]
pub trait NotificationRepository {
    /// Crée une notification (déclencheur : inscription confirmée, match vérifié…).
    async fn create(
        &self,
        user_id: Uuid,
        kind: &str,
        title: &str,
        message: &str,
        link: Option<&str>,
    ) -> Result<AppNotification, DatabaseError>;

    /// Liste les notifications d'un utilisateur (les plus récentes d'abord).
    async fn list(
        &self,
        user_id: Uuid,
        unread_only: bool,
        limit: i64,
    ) -> Result<Vec<AppNotification>, DatabaseError>;

    /// Nombre de notifications non lues.
    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DatabaseError>;

    /// Marque une notification comme lue (si elle appartient à l'utilisateur).
    async fn mark_read(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    /// Marque toutes les notifications de l'utilisateur comme lues.
    /// Retourne le nombre de notifications mises à jour.
    async fn mark_all_read(&self, user_id: Uuid) -> Result<i64, DatabaseError>;
}

pub struct PostgresNotificationRepository {
    pool: Arc<PgPool>,
}

impl PostgresNotificationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_notification(row: sqlx::postgres::PgRow) -> AppNotification {
        AppNotification {
            id: row.get("id"),
            user_id: row.get("user_id"),
            kind: row.get("kind"),
            title: row.get("title"),
            message: row.get("message"),
            link: row.get("link"),
            read: row.get("read"),
            created_at: row.get("created_at"),
        }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn create(
        &self,
        user_id: Uuid,
        kind: &str,
        title: &str,
        message: &str,
        link: Option<&str>,
    ) -> Result<AppNotification, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO notifications (id, user_id, kind, title, message, link, read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, false, NOW())
            RETURNING id, user_id, kind, title, message, link, read, created_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(kind)
        .bind(title)
        .bind(message)
        .bind(link)
        .fetch_one(&*self.pool)
        .await?;
        Ok(Self::row_to_notification(row))
    }

    async fn list(
        &self,
        user_id: Uuid,
        unread_only: bool,
        limit: i64,
    ) -> Result<Vec<AppNotification>, DatabaseError> {
        let rows = if unread_only {
            sqlx::query(
                r#"
                SELECT id, user_id, kind, title, message, link, read, created_at
                FROM notifications
                WHERE user_id = $1 AND read = false
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
            .bind(user_id)
            .bind(limit)
            .fetch_all(&*self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT id, user_id, kind, title, message, link, read, created_at
                FROM notifications
                WHERE user_id = $1
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
            .bind(user_id)
            .bind(limit)
            .fetch_all(&*self.pool)
            .await?
        };
        Ok(rows.into_iter().map(Self::row_to_notification).collect())
    }

    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DatabaseError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM notifications WHERE user_id = $1 AND read = false",
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await?;
        Ok(count)
    }

    async fn mark_read(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query(
            "UPDATE notifications SET read = true WHERE id = $1 AND user_id = $2 AND read = false",
        )
        .bind(id)
        .bind(user_id)
        .execute(&*self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn mark_all_read(&self, user_id: Uuid) -> Result<i64, DatabaseError> {
        let result =
            sqlx::query("UPDATE notifications SET read = true WHERE user_id = $1 AND read = false")
                .bind(user_id)
                .execute(&*self.pool)
                .await?;
        Ok(result.rows_affected() as i64)
    }
}

/// Helper de test : construit une notification en mémoire.
pub fn notification_fixture(user_id: Uuid, read: bool) -> AppNotification {
    AppNotification {
        id: Uuid::new_v4(),
        user_id,
        kind: "test".to_string(),
        title: "Titre".to_string(),
        message: "Message".to_string(),
        link: None,
        read,
        created_at: Utc::now(),
    }
}
