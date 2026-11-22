//! User Repository - opérations de persistance des comptes et refresh tokens

use crate::db::DatabaseError;
use crate::models::user::{NewUser, RefreshToken, UserAccount};
use crate::repositories::enum_conv::{enum_from_db_string, enum_to_db_string};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, new_user: NewUser) -> Result<UserAccount, DatabaseError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserAccount>, DatabaseError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAccount>, DatabaseError>;
    async fn find_by_license(&self, license: &str) -> Result<Option<UserAccount>, DatabaseError>;
    async fn update_last_login(&self, id: Uuid) -> Result<(), DatabaseError>;
    async fn create_refresh_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: OffsetDateTime,
    ) -> Result<RefreshToken, DatabaseError>;
    async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, DatabaseError>;
    async fn revoke_refresh_token(&self, token: &str) -> Result<(), DatabaseError>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), DatabaseError>;
    async fn delete_expired_tokens(&self, now: OffsetDateTime) -> Result<u64, DatabaseError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_user(row: sqlx::postgres::PgRow) -> Result<UserAccount, DatabaseError> {
        let role_str: String = row.get("role");
        let status_str: String = row.get("status");
        Ok(UserAccount {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            role: enum_from_db_string(&role_str)?,
            club_id: row.get("club_id"),
            license_number: row.get("license_number"),
            category: row.get("category"),
            status: enum_from_db_string(&status_str)?,
            email_verified: row.get("email_verified"),
            last_login: row.get("last_login"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn row_to_token(row: sqlx::postgres::PgRow) -> RefreshToken {
        RefreshToken {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            expires_at: row.get("expires_at"),
            revoked_at: row.get("revoked_at"),
            created_at: row.get("created_at"),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, new_user: NewUser) -> Result<UserAccount, DatabaseError> {
        let email = new_user.email.clone();
        let now = OffsetDateTime::now_utc();
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, first_name, last_name, role, club_id, license_number, category, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'senior', 'active', $9, $9)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&new_user.email)
        .bind(&new_user.password_hash)
        .bind(&new_user.first_name)
        .bind(&new_user.last_name)
        .bind(enum_to_db_string(&new_user.role))
        .bind(new_user.club_id)
        .bind(&new_user.license_number)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("users_email_key") => {
                DatabaseError::ValidationError(format!("Un compte existe déjà pour {email}"))
            }
            other => DatabaseError::from(other),
        })?;
        Self::row_to_user(row)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserAccount>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&*self.pool)
            .await?;
        row.map(Self::row_to_user).transpose()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAccount>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;
        row.map(Self::row_to_user).transpose()
    }

    async fn find_by_license(&self, license: &str) -> Result<Option<UserAccount>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM users WHERE license_number = $1")
            .bind(license)
            .fetch_optional(&*self.pool)
            .await?;
        row.map(Self::row_to_user).transpose()
    }

    async fn update_last_login(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE users SET last_login = $1, updated_at = $1 WHERE id = $2")
            .bind(OffsetDateTime::now_utc())
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn create_refresh_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: OffsetDateTime,
    ) -> Result<RefreshToken, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&token)
        .bind(expires_at)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&*self.pool)
        .await?;
        Ok(Self::row_to_token(row))
    }

    async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM refresh_tokens WHERE token = $1")
            .bind(token)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(Self::row_to_token))
    }

    async fn revoke_refresh_token(&self, token: &str) -> Result<(), DatabaseError> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = $1 WHERE token = $2 AND revoked_at IS NULL",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(token)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = $1 WHERE user_id = $2 AND revoked_at IS NULL",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(user_id)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expired_tokens(&self, now: OffsetDateTime) -> Result<u64, DatabaseError> {
        let result = sqlx::query(
            "DELETE FROM refresh_tokens WHERE expires_at < $1 OR revoked_at IS NOT NULL",
        )
        .bind(now)
        .execute(&*self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}
