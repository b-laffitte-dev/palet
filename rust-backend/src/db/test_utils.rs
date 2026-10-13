//! Database test utilities
//!
//! This module provides utilities for testing database connectivity
//! and is isolated from the main application logic.

use super::Database;
use sqlx::postgres::PgPoolOptions;

/// Creates a test database connection
///
/// This function establishes a connection to the test database
/// and returns a Database instance for use in tests.
///
/// # Environment Variables
/// - `PALET__DATABASE_URL`: URL of the test database (default: postgres://palet:palet@localhost:5432/palet_test)
///
/// # Panics
/// Panics if the connection to the test database fails.
pub async fn create_test_db() -> Database {
    let db_url = std::env::var("PALET__DATABASE_URL")
        .unwrap_or_else(|_| "postgres://palet:palet@localhost:5432/palet_test".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to test database");

    Database::new(pool)
}

/// Creates a test database connection with custom connection pool settings
///
/// # Arguments
/// * `max_connections` - Maximum number of connections in the pool
///
/// # Environment Variables
/// - `PALET__DATABASE_URL`: URL of the test database (default: postgres://palet:palet@localhost:5432/palet_test)
pub async fn create_test_db_with_pool(max_connections: u32) -> Database {
    let db_url = std::env::var("PALET__DATABASE_URL")
        .unwrap_or_else(|_| "postgres://palet:palet@localhost:5432/palet_test".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
        .expect("Failed to connect to test database");

    Database::new(pool)
}
