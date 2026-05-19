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
    entities::subjects::{self, ActiveModel},
    subjects::dto::{CreateSubjectRequest, SubjectResponse, UpdateSubjectRequest},
};

fn to_response(s: subjects::Model) -> SubjectResponse {
    SubjectResponse {
        id: s.id,
        organization_id: s.organization_id,
        stream_id: s.stream_id,
        name: s.name,
        name_urdu: s.name_urdu,
        code: s.code,
        medium: s.medium,
        description: s.description,
        is_active: s.is_active,
    }
}

// POST /api/v1/subjects
pub async fn create_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSubjectRequest>,
) -> Result<Json<SubjectResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "subject:create", Some(payload.organization_id), None).await?;

    let new_subject = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        stream_id: ActiveValue::Set(payload.stream_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        code: ActiveValue::Set(payload.code),
        medium: ActiveValue::Set(payload.medium),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let s = new_subject
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(s)))
}

// GET /api/v1/subjects
pub async fn get_subjects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SubjectResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "subject:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = subjects::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(subjects::Column::OrganizationId.eq(org_id));
    }

    let subjects = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(subjects.into_iter().map(to_response).collect()))
}

// GET /api/v1/subjects/{id}
pub async fn get_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<SubjectResponse>, (StatusCode, String)> {
    let s = subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Subject not found".to_string()))?;

    authorize(&*state.db, &claims, "subject:read", Some(s.organization_id), None).await?;

    Ok(Json(to_response(s)))
}

// PUT /api/v1/subjects/{id}
pub async fn update_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSubjectRequest>,
) -> Result<Json<SubjectResponse>, (StatusCode, String)> {
    let s = subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Subject not found".to_string()))?;

    authorize(&*state.db, &claims, "subject:update", Some(s.organization_id), None).await?;

    let mut active_model: ActiveModel = s.into();

    if let Some(stream_id) = payload.stream_id {
        active_model.stream_id = ActiveValue::Set(stream_id);
    }
    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(code) = payload.code {
        active_model.code = ActiveValue::Set(code);
    }
    if let Some(medium) = payload.medium {
        active_model.medium = ActiveValue::Set(medium);
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

// DELETE /api/v1/subjects/{id}
pub async fn delete_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let s = subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Subject not found".to_string()))?;

    authorize(&*state.db, &claims, "subject:delete", Some(s.organization_id), None).await?;

    let active_model: ActiveModel = s.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
