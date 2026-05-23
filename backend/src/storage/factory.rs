use std::sync::Arc;

use super::{
    gcs::GcsStorage,
    local::LocalStorage,
    s3_compatible::S3CompatibleStorage,
    StorageBackend, StorageConfig, StorageType,
};

pub async fn create_storage(config: &StorageConfig) -> Arc<dyn StorageBackend> {
    match config.storage_type {
        StorageType::Local => {
            let storage = LocalStorage::new(config);
            Arc::new(storage)
        }

        StorageType::S3Compatible => {
            let storage = S3CompatibleStorage::new(config)
                .await
                .expect("Failed to initialize S3 compatible storage");
            Arc::new(storage)
        }

        StorageType::Gcs => {
            let storage = GcsStorage::new(config)
                .await
                .expect("Failed to initialize Google Cloud Storage");
            Arc::new(storage)
        }
    }
}