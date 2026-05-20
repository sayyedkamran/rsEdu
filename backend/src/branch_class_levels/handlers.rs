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
    entities::branch_class_levels::{self, ActiveModel},
    branch_class_levels::dto::{CreateBranchClassLevelRequest, BranchClassLevelResponse, UpdateBranchClassLevelRequest},
};

fn to_response(bcl: branch_class_levels::Model) -> BranchClassLevelResponse {
    BranchClassLevelResponse {
        id: bcl.id,
        branch_id: bcl.branch_id,
        class_level_id: bcl.class_level_id,
        is_active: bcl.is_active,
    }
}

// POST /api/v1/branch-class-levels
pub async fn create_branch_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateBranchClassLevelRequest>,
) -> Result<Json<BranchClassLevelResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "branch_class_level:create", None, None).await?;

    let new_bcl = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        class_level_id: ActiveValue::Set(payload.class_level_id),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let bcl = new_bcl
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(bcl)))
}

// GET /api/v1/branch-class-levels
pub async fn get_branch_class_levels(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BranchClassLevelResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "branch_class_level:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = branch_class_levels::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(branch_class_levels::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/branch-class-levels/{id}
pub async fn get_branch_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<BranchClassLevelResponse>, (StatusCode, String)> {
    let bcl = branch_class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch class level not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_class_level:read", None, None).await?;

    Ok(Json(to_response(bcl)))
}

// PUT /api/v1/branch-class-levels/{id}
pub async fn update_branch_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBranchClassLevelRequest>,
) -> Result<Json<BranchClassLevelResponse>, (StatusCode, String)> {
    let bcl = branch_class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch class level not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_class_level:update", None, None).await?;

    let mut active_model: ActiveModel = bcl.into();

    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/branch-class-levels/{id}
pub async fn delete_branch_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let bcl = branch_class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch class level not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_class_level:delete", None, None).await?;

    let active_model: ActiveModel = bcl.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
