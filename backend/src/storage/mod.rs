use async_trait::async_trait;

pub mod local;
pub mod s3_compatible;
pub mod gcs;
pub mod factory;

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub local_path: Option<String>,
    pub base_url: String,
    pub endpoint: Option<String>,
    pub bucket: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub region: Option<String>,
    pub credentials_path: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StorageType {
    Local,
    S3Compatible, // covers AWS S3, Cloudflare R2, MinIO
    Gcs,
}

impl StorageConfig {
    pub fn from_env() -> Self {
        let storage_type = match std::env::var("STORAGE_TYPE")
            .unwrap_or_else(|_| "local".to_string())
            .as_str()
        {
            "s3" => StorageType::S3Compatible,
            "gcs" => StorageType::Gcs,
            _ => StorageType::Local,
        };

        StorageConfig {
            storage_type,
            local_path: std::env::var("STORAGE_LOCAL_PATH").ok(),
            base_url: std::env::var("STORAGE_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000/uploads".to_string()),
            endpoint: std::env::var("STORAGE_ENDPOINT").ok(),
            bucket: std::env::var("STORAGE_BUCKET").ok(),
            access_key: std::env::var("STORAGE_ACCESS_KEY").ok(),
            secret_key: std::env::var("STORAGE_SECRET_KEY").ok(),
            region: std::env::var("STORAGE_REGION").ok(),
            credentials_path: std::env::var("STORAGE_CREDENTIALS_PATH").ok(),
        }
    }
}

#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Upload a file and return its public URL
    async fn upload(
        &self,
        data: Vec<u8>,
        path: &str,
        content_type: &str,
    ) -> Result<String, String>;

    /// Delete a file by path
    async fn delete(&self, path: &str) -> Result<(), String>;

    /// Get public URL for a file
    fn get_url(&self, path: &str) -> String;

    /// Check if a file exists
    async fn exists(&self, path: &str) -> bool;
}