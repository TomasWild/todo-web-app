use axum::{routing::get, Router};
use clap::Parser;
use todo_web_app::{
    api::handler::health_check::health_check,
    config::Config,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::parse();

    let router = Router::new()
        .route("/health_check", get(health_check));

    let addr = std::net::SocketAddr::from((config.server_host, config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on: {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
