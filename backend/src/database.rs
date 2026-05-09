use sea_orm::{Database, DatabaseConnection};
use tracing::info;

pub async fn connect(database_url: &str) -> DatabaseConnection {
    info!("Connecting to database...");
    
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");
    
    info!("Database connected successfully");
    
    db
}