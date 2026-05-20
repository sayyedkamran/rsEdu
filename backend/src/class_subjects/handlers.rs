use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::Utc;

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize},
    entities::class_subjects::{self, ActiveModel},
    class_subjects::dto::{CreateClassSubjectRequest, ClassSubjectResponse, UpdateClassSubjectRequest},
};

fn to_response(cs: class_subjects::Model) -> ClassSubjectResponse {
    ClassSubjectResponse {
        id: cs.id,
        class_id: cs.class_id,
        subject_id: cs.subject_id,
        staff_id: cs.staff_id,
        weekly_periods: cs.weekly_periods,
        is_active: cs.is_active,
    }
}

// POST /api/v1/class-subjects
pub async fn create_class_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClassSubjectRequest>,
) -> Result<Json<ClassSubjectResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_subject:create", None, None).await?;

    let new_cs = ActiveModel {
        class_id: ActiveValue::Set(payload.class_id),
        subject_id: ActiveValue::Set(payload.subject_id),
        staff_id: ActiveValue::Set(payload.staff_id),
        weekly_periods: ActiveValue::Set(payload.weekly_periods),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let cs = new_cs
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(cs)))
}

// GET /api/v1/class-subjects
pub async fn get_class_subjects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ClassSubjectResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_subject:read", None, None).await?;

    let records = class_subjects::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/class-subjects/{id}
pub async fn get_class_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ClassSubjectResponse>, (StatusCode, String)> {
    let cs = class_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class subject not found".to_string()))?;

    authorize(&*state.db, &claims, "class_subject:read", None, None).await?;

    Ok(Json(to_response(cs)))
}

// PUT /api/v1/class-subjects/{id}
pub async fn update_class_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateClassSubjectRequest>,
) -> Result<Json<ClassSubjectResponse>, (StatusCode, String)> {
    let cs = class_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class subject not found".to_string()))?;

    authorize(&*state.db, &claims, "class_subject:update", None, None).await?;

    let mut active_model: ActiveModel = cs.into();

    if let Some(staff_id) = payload.staff_id {
        active_model.staff_id = ActiveValue::Set(Some(staff_id));
    }
    if let Some(weekly_periods) = payload.weekly_periods {
        active_model.weekly_periods = ActiveValue::Set(Some(weekly_periods));
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

// DELETE /api/v1/class-subjects/{id}
pub async fn delete_class_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let cs = class_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class subject not found".to_string()))?;

    authorize(&*state.db, &claims, "class_subject:delete", None, None).await?;

    let active_model: ActiveModel = cs.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
