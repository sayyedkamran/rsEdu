use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use axum_extra::extract::Multipart;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub path: String,
}

// POST /api/v1/upload/logo
pub async fn upload_logo(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let content_type = field
            .content_type()
            .unwrap_or("image/png")
            .to_string();

        // Only allow images
        if !content_type.starts_with("image/") {
            return Err((
                StatusCode::BAD_REQUEST,
                "Only image files are allowed".to_string(),
            ));
        }

        // Get file extension from content type
        let extension = match content_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "png",
        };

        // Generate unique filename
        let filename = format!("{}.{}", Uuid::new_v4(), extension);
        let path = format!("logos/{}", filename);

        // Read file data
        let data = field
            .bytes()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
            .to_vec();

        // Upload to storage
        let url = state
            .storage
            .upload(data, &path, &content_type)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        return Ok(Json(UploadResponse { url, path }));
    }

    Err((StatusCode::BAD_REQUEST, "No file provided".to_string()))
}

// POST /api/v1/upload/profile-picture
pub async fn upload_profile_picture(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let content_type = field
            .content_type()
            .unwrap_or("image/png")
            .to_string();

        // Only allow images
        if !content_type.starts_with("image/") {
            return Err((
                StatusCode::BAD_REQUEST,
                "Only image files are allowed".to_string(),
            ));
        }

        // Get file extension
        let extension = match content_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "png",
        };

        // Generate unique filename
        let filename = format!("{}.{}", Uuid::new_v4(), extension);
        let path = format!("profiles/{}", filename);

        // Read file data
        let data = field
            .bytes()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
            .to_vec();

        // Upload to storage
        let url = state
            .storage
            .upload(data, &path, &content_type)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        return Ok(Json(UploadResponse { url, path }));
    }

    Err((StatusCode::BAD_REQUEST, "No file provided".to_string()))
}