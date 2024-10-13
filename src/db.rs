use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};

const MAX_DB_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        AppState { pool }
    }
}

/// Setup database pool and run migrations
pub async fn setup_database(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(database_url)
        .await
        .context("Failed to connect to the database")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}