mod config;
mod database;
mod encryption;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    let config = AppConfig::load()?;

    tracing::info!("Loading configuration...");
    tracing::info!("Server will run on: {}", config.server_address());
    tracing::info!("Database URL: {}", config.database_url());
    tracing::info!("Max connections: {}", config.max_connections());
    tracing::info!("RSA key size: {} bits", config.key_size());

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections())
        .connect(config.database_url())
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/encrypt", post(handlers::encrypt_content))
        .route("/decrypt/:key", get(handlers::decrypt_content))
        .layer(CorsLayer::permissive())
        .with_state((pool, config.clone()));

    let listener = tokio::net::TcpListener::bind(config.server_address()).await?;

    tracing::info!("Server running on http://{}", config.server_address());
    tracing::info!("Endpoints:");
    tracing::info!("  POST /encrypt - Encrypt content and get a key");
    tracing::info!("  GET /decrypt/:key - Decrypt content using the key");

    axum::serve(listener, app).await?;

    Ok(())
}
