use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::Credentials,
    primitives::ByteStream,
    Client,
};

use super::{StorageBackend, StorageConfig};

pub struct S3CompatibleStorage {
    client: Client,
    bucket: String,
    base_url: String,
}

impl S3CompatibleStorage {
    pub async fn new(config: &StorageConfig) -> Result<Self, String> {
        let endpoint = config
            .endpoint
            .clone()
            .ok_or("STORAGE_ENDPOINT is required for S3 storage")?;

        let bucket = config
            .bucket
            .clone()
            .ok_or("STORAGE_BUCKET is required for S3 storage")?;

        let access_key = config
            .access_key
            .clone()
            .ok_or("STORAGE_ACCESS_KEY is required for S3 storage")?;

        let secret_key = config
            .secret_key
            .clone()
            .ok_or("STORAGE_SECRET_KEY is required for S3 storage")?;

        let region = config
            .region
            .clone()
            .unwrap_or_else(|| "auto".to_string());

        // Build credentials
        let credentials = Credentials::new(
            access_key,
            secret_key,
            None, // session token
            None, // expiry
            "rsedu-storage",
        );

        // Build S3 config
        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .force_path_style(true) // required for MinIO
            .build();

        let client = Client::from_conf(s3_config);

        Ok(S3CompatibleStorage {
            client,
            bucket,
            base_url: config.base_url.clone(),
        })
    }
}

#[async_trait]
impl StorageBackend for S3CompatibleStorage {
    async fn upload(
        &self,
        data: Vec<u8>,
        path: &str,
        content_type: &str,
    ) -> Result<String, String> {
        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(path)
            .body(body)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| format!("Failed to upload to S3: {}", e))?;

        Ok(self.get_url(path))
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await
            .map_err(|e| format!("Failed to delete from S3: {}", e))?;

        Ok(())
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    async fn exists(&self, path: &str) -> bool {
        self.client
            .head_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await
            .is_ok()
    }
}