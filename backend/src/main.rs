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
mod organizations;
mod branches;
mod streams;
mod class_levels;
mod master_classes;
mod master_sections;
mod academic_years;

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
        // Organization routes
        .route("/api/v1/organizations", post(organizations::handlers::create_organization))
        .route("/api/v1/organizations", get(organizations::handlers::get_organizations))
        .route("/api/v1/organizations/{id}", get(organizations::handlers::get_organization))
        .route("/api/v1/organizations/{id}", put(organizations::handlers::update_organization))
        .route("/api/v1/organizations/{id}", delete(organizations::handlers::delete_organization))
        // Branch routes
        .route("/api/v1/branches", post(branches::handlers::create_branch))
        .route("/api/v1/branches", get(branches::handlers::get_branches))
        .route("/api/v1/branches/{id}", get(branches::handlers::get_branch))
        .route("/api/v1/branches/{id}", put(branches::handlers::update_branch))
        .route("/api/v1/branches/{id}", delete(branches::handlers::delete_branch))
        // Stream routes
        .route("/api/v1/streams", post(streams::handlers::create_stream))
        .route("/api/v1/streams", get(streams::handlers::get_streams))
        .route("/api/v1/streams/{id}", get(streams::handlers::get_stream))
        .route("/api/v1/streams/{id}", put(streams::handlers::update_stream))
        .route("/api/v1/streams/{id}", delete(streams::handlers::delete_stream))
        // Class level routes
        .route("/api/v1/class-levels", post(class_levels::handlers::create_class_level))
        .route("/api/v1/class-levels", get(class_levels::handlers::get_class_levels))
        .route("/api/v1/class-levels/{id}", get(class_levels::handlers::get_class_level))
        .route("/api/v1/class-levels/{id}", put(class_levels::handlers::update_class_level))
        .route("/api/v1/class-levels/{id}", delete(class_levels::handlers::delete_class_level))
        // Master class routes
        .route("/api/v1/master-classes", post(master_classes::handlers::create_master_class))
        .route("/api/v1/master-classes", get(master_classes::handlers::get_master_classes))
        .route("/api/v1/master-classes/{id}", get(master_classes::handlers::get_master_class))
        .route("/api/v1/master-classes/{id}", put(master_classes::handlers::update_master_class))
        .route("/api/v1/master-classes/{id}", delete(master_classes::handlers::delete_master_class))
        // Master section routes
        .route("/api/v1/master-sections", post(master_sections::handlers::create_master_section))
        .route("/api/v1/master-sections", get(master_sections::handlers::get_master_sections))
        .route("/api/v1/master-sections/{id}", get(master_sections::handlers::get_master_section))
        .route("/api/v1/master-sections/{id}", put(master_sections::handlers::update_master_section))
        .route("/api/v1/master-sections/{id}", delete(master_sections::handlers::delete_master_section))
        // Academic year routes
        .route("/api/v1/academic-years", post(academic_years::handlers::create_academic_year))
        .route("/api/v1/academic-years", get(academic_years::handlers::get_academic_years))
        .route("/api/v1/academic-years/{id}", get(academic_years::handlers::get_academic_year))
        .route("/api/v1/academic-years/{id}", put(academic_years::handlers::update_academic_year))
        .route("/api/v1/academic-years/{id}", delete(academic_years::handlers::delete_academic_year))

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