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
) -> Result<(String, i32), (StatusCode, String)> {
    let user_role = user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No role assigned to user".to_string()))?;

    let role = roles::Entity::find_by_id(user_role.role_id)
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Role not found".to_string()))?;

    Ok((role.name, user_role.role_id))
}

// POST /api/v1/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let new_user = ActiveModel {
        username: ActiveValue::Set(payload.username.clone()),
        email: ActiveValue::Set(payload.email.clone()),
        password_hash: ActiveValue::Set(password_hash),
        organization_id: ActiveValue::Set(Some(payload.organization_id)),
        branch_id: ActiveValue::Set(payload.branch_id),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let user = new_user
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let new_user_role = user_roles::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        role_id: ActiveValue::Set(payload.role_id),
        organization_id: ActiveValue::Set(Some(payload.organization_id)),
        branch_id: ActiveValue::Set(payload.branch_id),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    new_user_role
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let (role_name, _) = get_user_role(&*state.db, user.id).await?;

    let token = generate_token(
        user.id,
        &user.email,
        &role_name,
        Some(payload.organization_id),
        payload.branch_id,
        &state.jwt_secret,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
        organization_id: user.organization_id,
        branch_id: user.branch_id,
    }))
}

// POST /api/v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
    }

    if !user.is_active {
        return Err((StatusCode::UNAUTHORIZED, "Account is disabled".to_string()));
    }

    let (role_name, _) = get_user_role(&*state.db, user.id).await?;

    let token = generate_token(
        user.id,
        &user.email,
        &role_name,
        user.organization_id,
        user.branch_id,
        &state.jwt_secret,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
        organization_id: user.organization_id,
        branch_id: user.branch_id,
    }))
}