use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::Utc;

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::permissions::{self, ActiveModel},
    permissions::dto::{CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest},
};

fn to_response(p: permissions::Model) -> PermissionResponse {
    PermissionResponse {
        id: p.id,
        organization_id: p.organization_id,
        branch_id: p.branch_id,
        name: p.name,
        description: p.description,
        module: p.module,
        is_active: p.is_active,
    }
}

// POST /api/v1/permissions
pub async fn create_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePermissionRequest>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "permission:create", payload.organization_id, payload.branch_id).await?;

    let new_permission = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        name: ActiveValue::Set(payload.name),
        description: ActiveValue::Set(payload.description),
        module: ActiveValue::Set(payload.module),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let permission = new_permission
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(permission)))
}

// GET /api/v1/permissions
pub async fn get_permissions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<PermissionResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "permission:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = permissions::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(permissions::Column::OrganizationId.eq(org_id));
    }
    if let Some(branch_id) = scope.branch_id {
        query = query.filter(permissions::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/permissions/{id}
pub async fn get_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    let permission = permissions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Permission not found".to_string()))?;

    authorize(&*state.db, &claims, "permission:read", permission.organization_id, permission.branch_id).await?;

    Ok(Json(to_response(permission)))
}

// PUT /api/v1/permissions/{id}
pub async fn update_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    let permission = permissions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Permission not found".to_string()))?;

    authorize(&*state.db, &claims, "permission:update", permission.organization_id, permission.branch_id).await?;

    let mut active_model: ActiveModel = permission.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
    }
    if let Some(module) = payload.module {
        active_model.module = ActiveValue::Set(module);
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/permissions/{id}
pub async fn delete_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let permission = permissions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Permission not found".to_string()))?;

    authorize(&*state.db, &claims, "permission:delete", permission.organization_id, permission.branch_id).await?;

    let active_model: ActiveModel = permission.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
