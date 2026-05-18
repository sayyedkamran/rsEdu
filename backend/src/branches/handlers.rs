use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::Utc;

use crate::{
    AppState,
    entities::branches::{self, ActiveModel},
    branches::dto::{CreateBranchRequest, BranchResponse, UpdateBranchRequest},
};

fn to_response(branch: branches::Model) -> BranchResponse {
    BranchResponse {
        id: branch.id,
        organization_id: branch.organization_id,
        name: branch.name,
        name_urdu: branch.name_urdu,
        code: branch.code,
        city_id: branch.city_id,
        area: branch.area,
        address_line: branch.address_line,
        postal_code: branch.postal_code,
        is_active: branch.is_active,
    }
}

// POST /api/v1/branches
pub async fn create_branch(
    State(state): State<AppState>,
    Json(payload): Json<CreateBranchRequest>,
) -> Result<Json<BranchResponse>, (StatusCode, String)> {
    let new_branch = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        code: ActiveValue::Set(payload.code),
        city_id: ActiveValue::Set(payload.city_id),
        area: ActiveValue::Set(payload.area),
        address_line: ActiveValue::Set(payload.address_line),
        postal_code: ActiveValue::Set(payload.postal_code),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let branch = new_branch
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(branch)))
}

// GET /api/v1/branches
pub async fn get_branches(
    State(state): State<AppState>,
) -> Result<Json<Vec<BranchResponse>>, (StatusCode, String)> {
    let branches = branches::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(branches.into_iter().map(to_response).collect()))
}

// GET /api/v1/branches/{id}
pub async fn get_branch(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<BranchResponse>, (StatusCode, String)> {
    let branch = branches::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch not found".to_string()))?;

    Ok(Json(to_response(branch)))
}

// PUT /api/v1/branches/{id}
pub async fn update_branch(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBranchRequest>,
) -> Result<Json<BranchResponse>, (StatusCode, String)> {
    let branch = branches::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch not found".to_string()))?;

    let mut active_model: ActiveModel = branch.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(code) = payload.code {
        active_model.code = ActiveValue::Set(code);
    }
    if let Some(city_id) = payload.city_id {
        active_model.city_id = ActiveValue::Set(Some(city_id));
    }
    if let Some(area) = payload.area {
        active_model.area = ActiveValue::Set(Some(area));
    }
    if let Some(address_line) = payload.address_line {
        active_model.address_line = ActiveValue::Set(Some(address_line));
    }
    if let Some(postal_code) = payload.postal_code {
        active_model.postal_code = ActiveValue::Set(Some(postal_code));
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

// DELETE /api/v1/branches/{id}
pub async fn delete_branch(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let branch = branches::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch not found".to_string()))?;

    let active_model: ActiveModel = branch.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}