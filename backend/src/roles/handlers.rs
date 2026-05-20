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
    entities::roles::{self, ActiveModel},
    roles::dto::{CreateRoleRequest, RoleResponse, UpdateRoleRequest},
};

fn to_response(r: roles::Model) -> RoleResponse {
    RoleResponse {
        id: r.id,
        organization_id: r.organization_id,
        branch_id: r.branch_id,
        name: r.name,
        description: r.description,
        is_active: r.is_active,
    }
}

// POST /api/v1/roles
pub async fn create_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "role:create", payload.organization_id, payload.branch_id).await?;

    let new_role = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        name: ActiveValue::Set(payload.name),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let role = new_role
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(role)))
}

// GET /api/v1/roles
pub async fn get_roles(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<RoleResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "role:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = roles::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(roles::Column::OrganizationId.eq(org_id));
    }
    if let Some(branch_id) = scope.branch_id {
        query = query.filter(roles::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/roles/{id}
pub async fn get_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    let role = roles::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Role not found".to_string()))?;

    authorize(&*state.db, &claims, "role:read", role.organization_id, role.branch_id).await?;

    Ok(Json(to_response(role)))
}

// PUT /api/v1/roles/{id}
pub async fn update_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    let role = roles::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Role not found".to_string()))?;

    authorize(&*state.db, &claims, "role:update", role.organization_id, role.branch_id).await?;

    let mut active_model: ActiveModel = role.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
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

// DELETE /api/v1/roles/{id}
pub async fn delete_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let role = roles::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Role not found".to_string()))?;

    authorize(&*state.db, &claims, "role:delete", role.organization_id, role.branch_id).await?;

    let active_model: ActiveModel = role.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
