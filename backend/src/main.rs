use axum::{Router, middleware, routing::get, routing::post, routing::put, routing::delete};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod auth;
mod config;
mod database;
mod entities;
mod students;
mod provinces;
mod cities;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let db = database::connect(&config.database_url).await;
    let state = AppState {
        db: Arc::new(db),
        jwt_secret: config.jwt_secret,
    };

    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/register", post(auth::handlers::register))
        .route("/api/v1/auth/login", post(auth::handlers::login));

    let protected_routes = Router::new()
        // Student routes
        .route("/api/v1/students", post(students::handlers::create_student))
        .route("/api/v1/students", get(students::handlers::get_students))
        .route("/api/v1/students/{id}", get(students::handlers::get_student))
        .route("/api/v1/students/{id}", put(students::handlers::update_student))
        .route("/api/v1/students/{id}", delete(students::handlers::delete_student))
        // Province routes
        .route("/api/v1/provinces", post(provinces::handlers::create_province))
        .route("/api/v1/provinces", get(provinces::handlers::get_provinces))
        .route("/api/v1/provinces/{id}", get(provinces::handlers::get_province))
        .route("/api/v1/provinces/{id}", put(provinces::handlers::update_province))
        .route("/api/v1/provinces/{id}", delete(provinces::handlers::delete_province))
        // City routes
        .route("/api/v1/cities", post(cities::handlers::create_city))
        .route("/api/v1/cities", get(cities::handlers::get_cities))
        .route("/api/v1/cities/{id}", get(cities::handlers::get_city))
        .route("/api/v1/cities/{id}", put(cities::handlers::update_city))
        .route("/api/v1/cities/{id}", delete(cities::handlers::delete_city))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware::auth_middleware,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}