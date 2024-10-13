use anyhow::{Context, Result};
use axum::Router;
use clap::Parser;
use todo_web_app::{
    api::route::build_router,
    config::Config,
    db::{setup_database, AppState},
    utils::init_environment::init_environment,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_environment();

    let config = Config::parse();
    let pool = setup_database(&config.database_url).await?;
    let app_state = AppState::new(pool);

    let router = build_router(app_state);
    let addr = std::net::SocketAddr::from((config.server_host, config.server_port));

    start_server(addr, router).await
}

/// Start the server and log startup info
async fn start_server(addr: std::net::SocketAddr, router: Router) -> Result<()> {
    tracing::info!("Listening on: {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind address")?;

    axum::serve(listener, router).await
        .context("Server failed")?;

    Ok(())
}