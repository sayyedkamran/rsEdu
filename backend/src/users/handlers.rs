use axum::{
    extract::{State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::Utc;

use crate::{
    AppState,
    auth::utils::{Claims, hash_password, verify_password},
    entities::users::{self, ActiveModel},
    users::dto::{
        ChangePasswordRequest, UpdateProfilePictureRequest,
        UpdateProfileRequest, UserProfileResponse,
    },
};

fn to_response(user: users::Model, base_url: &str) -> UserProfileResponse {
    let profile_picture_url = user.profile_picture_path.as_ref().map(|path| {
        format!("{}/{}", base_url.trim_end_matches('/'), path)
    });

    UserProfileResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        organization_id: user.organization_id,
        branch_id: user.branch_id,
        profile_picture_path: user.profile_picture_path,
        profile_picture_url,
        is_active: user.is_active,
    }
}

// GET /api/v1/users/me
pub async fn get_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let base_url = std::env::var("STORAGE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000/uploads".to_string());

    Ok(Json(to_response(user, &base_url)))
}

// PUT /api/v1/users/me
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let mut active_model: ActiveModel = user.into();

    if let Some(username) = payload.username {
        active_model.username = ActiveValue::Set(username);
    }
    if let Some(email) = payload.email {
        active_model.email = ActiveValue::Set(email);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let base_url = std::env::var("STORAGE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000/uploads".to_string());

    Ok(Json(to_response(updated, &base_url)))
}

// PUT /api/v1/users/me/password
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Check passwords match
    if payload.new_password != payload.confirm_password {
        return Err((
            StatusCode::BAD_REQUEST,
            "New password and confirm password do not match".to_string(),
        ));
    }

    // Check minimum length
    if payload.new_password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "New password must be at least 6 characters".to_string(),
        ));
    }

    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    // Verify current password
    let is_valid = verify_password(&payload.current_password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Current password is incorrect".to_string(),
        ));
    }

    // Hash new password
    let new_hash = hash_password(&payload.new_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let mut active_model: ActiveModel = user.into();
    active_model.password_hash = ActiveValue::Set(new_hash);
    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// PUT /api/v1/users/me/profile-picture
pub async fn update_profile_picture(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfilePictureRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID".to_string()))?;

    let user = users::Entity::find_by_id(user_id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let mut active_model: ActiveModel = user.into();
    active_model.profile_picture_path =
        ActiveValue::Set(Some(payload.profile_picture_path));
    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let base_url = std::env::var("STORAGE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000/uploads".to_string());

    Ok(Json(to_response(updated, &base_url)))
}