use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::Utc;

use crate::{
    AppState,
    entities::users::{self, ActiveModel},
    entities::user_roles,
    entities::roles,
    auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest, GoogleLoginRequest},
        utils::{generate_token, hash_password, verify_password},
    },
};

// Helper to fetch role name and title from user_id
async fn get_user_role(
    db: &sea_orm::DatabaseConnection,
    user_id: i32,
) -> Result<(String, String, i32), (StatusCode, String)> {
    // Find user_role record
    let user_role = user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No role assigned to user".to_string()))?;

    // Find role
    let role = roles::Entity::find_by_id(user_role.role_id)
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Role not found".to_string()))?;

    // Use title if available, fallback to name
    let role_title = role.title.unwrap_or_else(|| role.name.clone());

    Ok((role.name, role_title, user_role.role_id))
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

    let (role_name, role_title, _) = get_user_role(&*state.db, user.id).await?;

    let token = generate_token(
        user.id,
        &user.email,
        &role_name,
        &role_title,
        Some(payload.organization_id),
        payload.branch_id,
        &state.jwt_secret,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
        role_title,
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

    let (role_name, role_title, _) = get_user_role(&*state.db, user.id).await?;

    let token = generate_token(
        user.id,
        &user.email,
        &role_name,
        &role_title,
        user.organization_id,
        user.branch_id,
        &state.jwt_secret,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
        role_title,
        organization_id: user.organization_id,
        branch_id: user.branch_id,
    }))
}

// POST /api/v1/auth/google
pub async fn google_login(
    State(state): State<AppState>,
    Json(payload): Json<GoogleLoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Check if user exists by google_id or email
    let existing_user = users::Entity::find()
        .filter(
            sea_orm::Condition::any()
                .add(users::Column::GoogleId.eq(&payload.google_id))
                .add(users::Column::Email.eq(&payload.email)),
        )
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = if let Some(mut found_user) = existing_user {
        // Update google_id if not set
        if found_user.google_id.is_none() {
            let mut active: ActiveModel = found_user.into();
            active.google_id = ActiveValue::Set(Some(payload.google_id.clone()));
            active.updated_at = ActiveValue::Set(Utc::now().into());
            active.update(&*state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        } else {
            found_user
        }
    } else {
        // Create new user with Google info
        let new_user = ActiveModel {
            username: ActiveValue::Set(payload.name.clone().unwrap_or_else(|| payload.email.clone())),
            email: ActiveValue::Set(payload.email.clone()),
            password_hash: ActiveValue::Set(String::new()),
            google_id: ActiveValue::Set(Some(payload.google_id.clone())),
            profile_picture_path: ActiveValue::Set(payload.picture.clone()),
            user_type: ActiveValue::Set(Some("guest".to_string())),
            is_active: ActiveValue::Set(true),
            created_at: ActiveValue::Set(Utc::now().into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
            ..Default::default()
        };

        new_user
            .insert(&*state.db)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    };

    // Get role - default to s_guest if no role assigned
    let (role_name, role_title) = match get_user_role(&*state.db, user.id).await {
        Ok((name, title, _)) => (name, title),
        Err(_) => ("s_guest".to_string(), "School Guest".to_string()),
    };

    let token = generate_token(
        user.id,
        &user.email,
        &role_name,
        &role_title,
        user.organization_id,
        user.branch_id,
        &state.jwt_secret,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
        role: role_name,
        role_title,
        organization_id: user.organization_id,
        branch_id: user.branch_id,
    }))
}