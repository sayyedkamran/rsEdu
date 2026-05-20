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
    entities::branch_contacts::{self, ActiveModel},
    branch_contacts::dto::{CreateBranchContactRequest, BranchContactResponse, UpdateBranchContactRequest},
};

fn to_response(bc: branch_contacts::Model) -> BranchContactResponse {
    BranchContactResponse {
        id: bc.id,
        branch_id: bc.branch_id,
        contact_type: bc.contact_type,
        value: bc.value,
        has_whatsapp: bc.has_whatsapp,
        is_primary: bc.is_primary,
        is_active: bc.is_active,
    }
}

// POST /api/v1/branch-contacts
pub async fn create_branch_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateBranchContactRequest>,
) -> Result<Json<BranchContactResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "branch_contact:create", None, None).await?;

    let new_bc = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        contact_type: ActiveValue::Set(payload.contact_type),
        value: ActiveValue::Set(payload.value),
        has_whatsapp: ActiveValue::Set(payload.has_whatsapp),
        is_primary: ActiveValue::Set(payload.is_primary),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let bc = new_bc
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(bc)))
}

// GET /api/v1/branch-contacts
pub async fn get_branch_contacts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BranchContactResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "branch_contact:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = branch_contacts::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(branch_contacts::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/branch-contacts/{id}
pub async fn get_branch_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<BranchContactResponse>, (StatusCode, String)> {
    let bc = branch_contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch contact not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_contact:read", None, None).await?;

    Ok(Json(to_response(bc)))
}

// PUT /api/v1/branch-contacts/{id}
pub async fn update_branch_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBranchContactRequest>,
) -> Result<Json<BranchContactResponse>, (StatusCode, String)> {
    let bc = branch_contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch contact not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_contact:update", None, None).await?;

    let mut active_model: ActiveModel = bc.into();

    if let Some(contact_type) = payload.contact_type {
        active_model.contact_type = ActiveValue::Set(contact_type);
    }
    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
    }
    if let Some(has_whatsapp) = payload.has_whatsapp {
        active_model.has_whatsapp = ActiveValue::Set(has_whatsapp);
    }
    if let Some(is_primary) = payload.is_primary {
        active_model.is_primary = ActiveValue::Set(is_primary);
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

// DELETE /api/v1/branch-contacts/{id}
pub async fn delete_branch_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let bc = branch_contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Branch contact not found".to_string()))?;

    authorize(&*state.db, &claims, "branch_contact:delete", None, None).await?;

    let active_model: ActiveModel = bc.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
