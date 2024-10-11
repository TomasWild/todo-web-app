use anyhow::{Context, Result};
use axum::{routing::get, Router};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use todo_web_app::{
    api::handler::health_check::health_check,
    config::Config,
    utils::app_state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::parse();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    let router = Router::new()
        .route("/health_check", get(health_check))
        .with_state(AppState::new(pool));

    let addr = std::net::SocketAddr::from((config.server_host, config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on: {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
