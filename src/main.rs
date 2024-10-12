use anyhow::{Context, Result};
use axum::{routing::{get, post}, Router};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use todo_web_app::{
    api::handler::{
        health_check::health_check,
        todo_handler::{create_todo, delete_todo, find_all_todos, find_todo_by_id, update_todo},
    },
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

    sqlx::migrate!("./migrations").run(&pool).await?;

    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/api/v1/todos", post(create_todo).get(find_all_todos))
        .route("/api/v1/todos/{:id}", get(find_todo_by_id).put(update_todo).delete(delete_todo))
        .with_state(AppState::new(pool));

    let addr = std::net::SocketAddr::from((config.server_host, config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on: {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
