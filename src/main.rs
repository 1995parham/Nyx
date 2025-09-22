mod database;
mod encryption;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://nyx_user:nyx_password@localhost:5432/nyx_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/encrypt", post(handlers::encrypt_content))
        .route("/decrypt/:key", get(handlers::decrypt_content))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    tracing::info!("Server running on http://0.0.0.0:{}", port);
    tracing::info!("Endpoints:");
    tracing::info!("  POST /encrypt - Encrypt content and get a key");
    tracing::info!("  GET /decrypt/:key - Decrypt content using the key");

    axum::serve(listener, app).await?;

    Ok(())
}