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
    entities::master_sections::{self, ActiveModel},
    master_sections::dto::{CreateMasterSectionRequest, MasterSectionResponse, UpdateMasterSectionRequest},
};

fn to_response(ms: master_sections::Model) -> MasterSectionResponse {
    MasterSectionResponse {
        id: ms.id,
        organization_id: ms.organization_id,
        name: ms.name,
        letter: ms.letter,
        name_urdu: ms.name_urdu,
        is_active: ms.is_active,
    }
}

// POST /api/v1/master-sections
pub async fn create_master_section(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateMasterSectionRequest>,
) -> Result<Json<MasterSectionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "master_section:create", Some(payload.organization_id), None).await?;

    let new_ms = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        letter: ActiveValue::Set(payload.letter),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let ms = new_ms
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(ms)))
}

// GET /api/v1/master-sections
pub async fn get_master_sections(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MasterSectionResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "master_section:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = master_sections::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(master_sections::Column::OrganizationId.eq(org_id));
    }

    let master_sections = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(master_sections.into_iter().map(to_response).collect()))
}

// GET /api/v1/master-sections/{id}
pub async fn get_master_section(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MasterSectionResponse>, (StatusCode, String)> {
    let ms = master_sections::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master section not found".to_string()))?;

    authorize(&*state.db, &claims, "master_section:read", Some(ms.organization_id), None).await?;

    Ok(Json(to_response(ms)))
}

// PUT /api/v1/master-sections/{id}
pub async fn update_master_section(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMasterSectionRequest>,
) -> Result<Json<MasterSectionResponse>, (StatusCode, String)> {
    let ms = master_sections::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master section not found".to_string()))?;

    authorize(&*state.db, &claims, "master_section:update", Some(ms.organization_id), None).await?;

    let mut active_model: ActiveModel = ms.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(letter) = payload.letter {
        active_model.letter = ActiveValue::Set(letter);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
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

// DELETE /api/v1/master-sections/{id}
pub async fn delete_master_section(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ms = master_sections::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master section not found".to_string()))?;

    authorize(&*state.db, &claims, "master_section:delete", Some(ms.organization_id), None).await?;

    let active_model: ActiveModel = ms.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}