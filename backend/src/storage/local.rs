use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

use super::{StorageBackend, StorageConfig};

pub struct LocalStorage {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorage {
    pub fn new(config: &StorageConfig) -> Self {
        let base_path = config
            .local_path
            .clone()
            .unwrap_or_else(|| "./uploads".to_string());

        LocalStorage {
            base_path: PathBuf::from(base_path),
            base_url: config.base_url.clone(),
        }
    }

    pub async fn ensure_directory(&self, path: &str) -> Result<(), String> {
        let full_path = self.base_path.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        Ok(())
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn upload(
        &self,
        data: Vec<u8>,
        path: &str,
        _content_type: &str,
    ) -> Result<String, String> {
        // Ensure directory exists
        self.ensure_directory(path).await?;

        // Write file to disk
        let full_path = self.base_path.join(path);
        fs::write(&full_path, data)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        // Return public URL
        Ok(self.get_url(path))
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let full_path = self.base_path.join(path);
        if full_path.exists() {
            fs::remove_file(&full_path)
                .await
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        Ok(())
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    async fn exists(&self, path: &str) -> bool {
        let full_path = self.base_path.join(path);
        full_path.exists()
    }
}