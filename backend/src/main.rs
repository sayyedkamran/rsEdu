use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod config;
mod database;

#[derive(Clone)]
pub struct AppState {
    db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = config::Config::from_env();

    // Connect to database
    let db = database::connect(&config.database_url).await;
    let state = AppState {
        db: Arc::new(db),
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(state);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}