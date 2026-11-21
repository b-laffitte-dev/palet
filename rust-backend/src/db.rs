//! Database module for Palet Vendéen backend
//!
//! This module provides database connection management and error handling.

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

// Re-export repositories
pub use crate::repositories::{
    availability_repository::{AvailabilityRepository, PostgresAvailabilityRepository},
    championship_repository::{ChampionshipRepository, PostgresChampionshipRepository},
    club_repository::{ClubRepository, PostgresClubRepository},
    competition_repository::{CompetitionRepository, PostgresCompetitionRepository},
    division_repository::{DivisionRepository, PostgresDivisionRepository},
    match_repository::{MatchRepository, PostgresMatchRepository},
    news_repository::{NewsRepository, PostgresNewsRepository},
    notification_repository::{NotificationRepository, PostgresNotificationRepository},
    player_repository::{PlayerRepository, PostgresPlayerRepository},
    team_repository::{PostgresTeamRepository, TeamRepository},
    user_repository::{PostgresUserRepository, UserRepository},
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
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Pool error: {0}")]
    PoolError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(err) => DatabaseError::QueryError(err.to_string()),
            sqlx::Error::Io(err) => DatabaseError::ConnectionError(err.to_string()),
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
    pub matches: Arc<dyn MatchRepository + Send + Sync>,
    pub championships: Arc<dyn ChampionshipRepository + Send + Sync>,
    pub clubs: Arc<dyn ClubRepository + Send + Sync>,
    pub players: Arc<dyn PlayerRepository + Send + Sync>,
    pub teams: Arc<dyn TeamRepository + Send + Sync>,
    pub divisions: Arc<dyn DivisionRepository + Send + Sync>,
    pub users: Arc<dyn UserRepository + Send + Sync>,
    pub competitions: Arc<dyn CompetitionRepository + Send + Sync>,
    pub news: Arc<dyn NewsRepository + Send + Sync>,
    pub disponibilites: Arc<dyn AvailabilityRepository + Send + Sync>,
    pub notifications: Arc<dyn NotificationRepository + Send + Sync>,
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
            divisions: Arc::new(PostgresDivisionRepository::new(pool.clone())),
            users: Arc::new(PostgresUserRepository::new(pool.clone())),
            competitions: Arc::new(PostgresCompetitionRepository::new(pool.clone())),
            news: Arc::new(PostgresNewsRepository::new(pool.clone())),
            disponibilites: Arc::new(PostgresAvailabilityRepository::new(pool.clone())),
            notifications: Arc::new(PostgresNotificationRepository::new(pool.clone())),
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

    /// Apply SQL migrations from `./migrations` (sqlx::migrate!).
    ///
    /// In docker dev, the database is pre-seeded by dev-data/init-db.sh which
    /// loads the 0001 baseline itself: in that case the baseline is marked as
    /// already applied in `_sqlx_migrations` (with its real checksum) before
    /// running the migrator, so only the following migrations are executed.
    pub async fn initialize(&self) -> Result<(), DatabaseError> {
        adopt_seeded_baseline(&self.pool).await?;
        sqlx::migrate!("./migrations")
            .run(&*self.pool)
            .await
            .map_err(|e| DatabaseError::MigrationError(e.to_string()))?;
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

/// Mark migration 0001 (baseline) as applied when the database was pre-seeded
/// outside sqlx (docker-entrypoint-initdb.d loading 0001_baseline.sql directly):
/// the `clubs` table exists but `_sqlx_migrations` does not.
async fn adopt_seeded_baseline(pool: &PgPool) -> Result<(), DatabaseError> {
    let seeded: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM information_schema.tables
               WHERE table_schema = 'public' AND table_name = 'clubs'
           ) AND NOT EXISTS (
               SELECT 1 FROM information_schema.tables
               WHERE table_schema = 'public' AND table_name = '_sqlx_migrations'
           )"#,
    )
    .fetch_one(pool)
    .await?;
    if !seeded {
        return Ok(());
    }
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
            success BOOLEAN NOT NULL,
            checksum BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )"#,
    )
    .execute(pool)
    .await?;
    let migrator = sqlx::migrate!("./migrations");
    if let Some(baseline) = migrator.migrations.iter().find(|m| m.version == 1) {
        sqlx::query(
            r#"INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
               VALUES ($1, $2, true, $3, 0)
               ON CONFLICT (version) DO NOTHING"#,
        )
        .bind(baseline.version)
        .bind(&baseline.description)
        .bind(&baseline.checksum[..])
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Applique les seeds de dev (fichiers SQL du dossier `seed_dir`) si la table
/// `users` est vide. Appelé après les migrations, uniquement si `seed_dir` est
/// non vide et l'environnement n'est pas "production" (voir main.rs).
pub async fn seed_dev_data(pool: &PgPool, seed_dir: &str) -> Result<(), DatabaseError> {
    if seed_dir.is_empty() {
        return Ok(());
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM users")
        .fetch_one(pool)
        .await?;
    if count > 0 {
        return Ok(());
    }
    const SEED_FILES: [&str; 5] = [
        "players.sql",
        "teams.sql",
        "team_players.sql",
        "users_seed.sql",
        "news.sql",
    ];
    for name in SEED_FILES {
        let path = format!("{seed_dir}/{name}");
        let content = std::fs::read_to_string(&path)
            .map_err(|e| DatabaseError::MigrationError(format!("lecture de {path}: {e}")))?;
        let cleaned = content
            .lines()
            .filter(|l| !l.trim_start().starts_with('\\'))
            .collect::<Vec<_>>()
            .join("\n");
        sqlx::raw_sql(sqlx::AssertSqlSafe(cleaned))
            .execute(pool)
            .await
            .map_err(|e| DatabaseError::MigrationError(format!("seed {name}: {e}")))?;
        log::info!("seed appliqué: {name}");
    }
    Ok(())
}

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
        let sqlx_error = sqlx::Error::Io(std::io::Error::other("database error"));
        let error = DatabaseError::from(sqlx_error);
        assert!(matches!(error, DatabaseError::ConnectionError(_)));
    }
}
