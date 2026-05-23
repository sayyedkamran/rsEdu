use async_trait::async_trait;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};

use super::{StorageBackend, StorageConfig};

pub struct GcsStorage {
    client: Client,
    bucket: String,
    base_url: String,
}

impl GcsStorage {
    pub async fn new(config: &StorageConfig) -> Result<Self, String> {
        let bucket = config
            .bucket
            .clone()
            .ok_or("STORAGE_BUCKET is required for GCS storage")?;

        // Build GCS client config
        let gcs_config = if let Some(credentials_path) = &config.credentials_path {
            // Use service account credentials file
            unsafe {
                std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", credentials_path);
            }
           
            ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| format!("Failed to authenticate with GCS: {}", e))?
        } else {
            // Use application default credentials
            ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| format!("Failed to authenticate with GCS: {}", e))?
        };

        let client = Client::new(gcs_config);

        Ok(GcsStorage {
            client,
            bucket,
            base_url: config.base_url.clone(),
        })
    }
}

#[async_trait]
impl StorageBackend for GcsStorage {
    async fn upload(
        &self,
        data: Vec<u8>,
        path: &str,
        content_type: &str,
    ) -> Result<String, String> {
        let upload_type = UploadType::Simple(Media::new(path.to_string()));

        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data,
                &upload_type,
            )
            .await
            .map_err(|e| format!("Failed to upload to GCS: {}", e))?;

        Ok(self.get_url(path))
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        self.client
            .delete_object(&DeleteObjectRequest {
                bucket: self.bucket.clone(),
                object: path.to_string(),
                ..Default::default()
            })
            .await
            .map_err(|e| format!("Failed to delete from GCS: {}", e))?;

        Ok(())
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    async fn exists(&self, path: &str) -> bool {
        self.client
            .get_object(&GetObjectRequest {
                bucket: self.bucket.clone(),
                object: path.to_string(),
                ..Default::default()
            })
            .await
            .is_ok()
    }
}