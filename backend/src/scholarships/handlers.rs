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
    entities::scholarships::{self, ActiveModel},
    scholarships::dto::{CreateScholarshipRequest, ScholarshipResponse, UpdateScholarshipRequest},
};

fn to_response(s: scholarships::Model) -> ScholarshipResponse {
    ScholarshipResponse {
        id: s.id,
        organization_id: s.organization_id,
        fee_type_id: s.fee_type_id,
        name: s.name,
        name_urdu: s.name_urdu,
        coverage_type: s.coverage_type,
        value: s.value,
        description: s.description,
        is_active: s.is_active,
    }
}

// POST /api/v1/scholarships
pub async fn create_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateScholarshipRequest>,
) -> Result<Json<ScholarshipResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "scholarship:create", Some(payload.organization_id), None).await?;

    let new_s = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        fee_type_id: ActiveValue::Set(payload.fee_type_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        coverage_type: ActiveValue::Set(payload.coverage_type),
        value: ActiveValue::Set(payload.value),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let s = new_s
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(s)))
}

// GET /api/v1/scholarships
pub async fn get_scholarships(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ScholarshipResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "scholarship:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = scholarships::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(scholarships::Column::OrganizationId.eq(org_id));
    }

    let scholarships = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(scholarships.into_iter().map(to_response).collect()))
}

// GET /api/v1/scholarships/{id}
pub async fn get_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ScholarshipResponse>, (StatusCode, String)> {
    let s = scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "scholarship:read", Some(s.organization_id), None).await?;

    Ok(Json(to_response(s)))
}

// PUT /api/v1/scholarships/{id}
pub async fn update_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateScholarshipRequest>,
) -> Result<Json<ScholarshipResponse>, (StatusCode, String)> {
    let s = scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "scholarship:update", Some(s.organization_id), None).await?;

    let mut active_model: ActiveModel = s.into();

    if let Some(fee_type_id) = payload.fee_type_id {
        active_model.fee_type_id = ActiveValue::Set(Some(fee_type_id));
    }
    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(coverage_type) = payload.coverage_type {
        active_model.coverage_type = ActiveValue::Set(coverage_type);
    }
    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
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

// DELETE /api/v1/scholarships/{id}
pub async fn delete_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let s = scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "scholarship:delete", Some(s.organization_id), None).await?;

    let active_model: ActiveModel = s.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
