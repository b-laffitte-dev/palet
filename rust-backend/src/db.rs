//! Database module for Palet Vendeen backend
//!
//! This module provides database connection management and error handling.

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

// Re-export repositories
pub use crate::repositories::{
    championship_repository::{ChampionshipRepository, PostgresChampionshipRepository},
    club_repository::{ClubRepository, PostgresClubRepository},
    match_repository::{MatchRepository, PostgresMatchRepository},
    player_repository::{PlayerRepository, PostgresPlayerRepository},
    team_repository::{TeamRepository, PostgresTeamRepository},
};

// ============================================
// DATABASE ERROR
// ============================================

/// Error type for database operations
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Pool error: {0}")]
    PoolError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(err) => DatabaseError::QueryError(err.to_string()),
            sqlx::Error::Connection(err) => DatabaseError::ConnectionError(err.to_string()),
            sqlx::Error::PoolTimedOut => {
                DatabaseError::PoolError("Connection pool timed out".to_string())
            }
            sqlx::Error::PoolClosed => {
                DatabaseError::PoolError("Connection pool closed".to_string())
            }
            _ => DatabaseError::QueryError(error.to_string()),
        }
    }
}

impl From<String> for DatabaseError {
    fn from(error: String) -> Self {
        DatabaseError::QueryError(error)
    }
}

// ============================================
// DATABASE WRAPPER
// ============================================

/// Database wrapper for managing PostgreSQL connections and providing repository access
#[derive(Clone)]
pub struct Database {
    pool: Arc<PgPool>,
    pub matches: Arc<PostgresMatchRepository>,
    pub championships: Arc<PostgresChampionshipRepository>,
    pub clubs: Arc<PostgresClubRepository>,
    pub players: Arc<PostgresPlayerRepository>,
    pub teams: Arc<PostgresTeamRepository>,
}

impl Database {
    /// Create a new Database instance with all repositories
    pub fn new(pool: PgPool) -> Self {
        let pool = Arc::new(pool);

        Self {
            pool: pool.clone(),
            matches: Arc::new(PostgresMatchRepository::new(pool.clone())),
            championships: Arc::new(PostgresChampionshipRepository::new(pool.clone())),
            clubs: Arc::new(PostgresClubRepository::new(pool.clone())),
            players: Arc::new(PostgresPlayerRepository::new(pool.clone())),
            teams: Arc::new(PostgresTeamRepository::new(pool.clone())),
        }
    }

    /// Get the connection pool
    pub fn pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }

    /// Check database connection
    pub async fn check_connection(&self) -> Result<bool, DatabaseError> {
        let row = sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(&*self.pool)
            .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Initialize database schema (if migrations directory exists)
    pub async fn initialize(&self) -> Result<(), DatabaseError> {
        // Try to run migrations if migrations directory exists
        // For now, we'll skip this as we don't have migrations set up yet
        // In production, you would use sqlx::migrate!()
        Ok(())
    }

    /// Get database version
    pub async fn get_version(&self) -> Result<String, DatabaseError> {
        let version = sqlx::query_scalar::<_, String>("SELECT version()")
            .fetch_one(&*self.pool)
            .await?;
        Ok(version)
    }
}

// ============================================
// DATABASE INITIALIZATION
// ============================================

/// Initialize database connection pool from URL
pub async fn create_pool(database_url: &str) -> Result<PgPool, DatabaseError> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
}

/// Initialize database with all repositories
pub async fn initialize_database(database_url: &str) -> Result<Database, DatabaseError> {
    let pool = create_pool(database_url).await?;
    let database = Database::new(pool);
    database.initialize().await?;
    Ok(database)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_error_conversions() {
        // Test String to DatabaseError conversion
        let error = DatabaseError::from("test error".to_string());
        assert!(matches!(error, DatabaseError::QueryError(_)));

        // Test sqlx::Error to DatabaseError conversion
        let sqlx_error = sqlx::Error::Database("database error".into());
        let error = DatabaseError::from(sqlx_error);
        assert!(matches!(error, DatabaseError::QueryError(_)));
    }
}
