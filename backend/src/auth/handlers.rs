use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::Utc;

use crate::{
    AppState,
    entities::users::{self, ActiveModel},
    entities::user_roles,
    entities::roles,
    auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest},
        utils::{generate_token, hash_password, verify_password},
    },
};

// Helper to fetch role name from user_id
async fn get_user_role(
    db: &sea_orm::DatabaseConnection,
    user_id: i32,
) -> Result<String, (StatusCode, String)> {
    // Find user_role record
    let user_role = user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No role assigned to user".to_string()))?;

    // Find role name
    let role = roles::Entity::find_by_id(user_role.role_id)
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Role not found".to_string()))?;

    Ok(role.name)
}

// POST /api/v1/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Hash the password
    let password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Create new user
    let new_user = ActiveModel {
        username: ActiveValue::Set(payload.username.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password_hash: ActiveValue::Set(password_hash),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let user = new_user
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Assign role to user
    let new_user_role = user_roles::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        role_id: ActiveValue::Set(payload.role_id),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    new_user_role
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Fetch role name
    let role_name = get_user_role(&*state.db, user.id).await?;

    // Generate JWT token
    let token = generate_token(user.id, &user.email, &role_name, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
    }))
}

// POST /api/v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Find user by email
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
    }

    // Check if user is active
    if !user.is_active {
        return Err((StatusCode::UNAUTHORIZED, "Account is disabled".to_string()));
    }

    // Fetch role name
    let role_name = get_user_role(&*state.db, user.id).await?;

    // Generate JWT token
    let token = generate_token(user.id, &user.email, &role_name, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
    }))
}