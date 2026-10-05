use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://palet:palet@localhost:5432/palet".to_string());
    PgPoolOptions::new().max_connections(10).connect(&url).await
}
