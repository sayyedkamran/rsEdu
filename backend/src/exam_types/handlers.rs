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
    entities::exam_types::{self, ActiveModel},
    exam_types::dto::{CreateExamTypeRequest, ExamTypeResponse, UpdateExamTypeRequest},
};

fn to_response(et: exam_types::Model) -> ExamTypeResponse {
    ExamTypeResponse {
        id: et.id,
        organization_id: et.organization_id,
        stream_id: et.stream_id,
        master_class_id: et.master_class_id,
        name: et.name,
        name_urdu: et.name_urdu,
        description: et.description,
        is_active: et.is_active,
    }
}

// POST /api/v1/exam-types
pub async fn create_exam_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateExamTypeRequest>,
) -> Result<Json<ExamTypeResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam_type:create", Some(payload.organization_id), None).await?;

    let new_et = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        stream_id: ActiveValue::Set(payload.stream_id),
        master_class_id: ActiveValue::Set(payload.master_class_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let et = new_et
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(et)))
}

// GET /api/v1/exam-types
pub async fn get_exam_types(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ExamTypeResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam_type:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = exam_types::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(exam_types::Column::OrganizationId.eq(org_id));
    }

    let exam_types = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(exam_types.into_iter().map(to_response).collect()))
}

// GET /api/v1/exam-types/{id}
pub async fn get_exam_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ExamTypeResponse>, (StatusCode, String)> {
    let et = exam_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam type not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_type:read", Some(et.organization_id), None).await?;

    Ok(Json(to_response(et)))
}

// PUT /api/v1/exam-types/{id}
pub async fn update_exam_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExamTypeRequest>,
) -> Result<Json<ExamTypeResponse>, (StatusCode, String)> {
    let et = exam_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam type not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_type:update", Some(et.organization_id), None).await?;

    let mut active_model: ActiveModel = et.into();

    if let Some(stream_id) = payload.stream_id {
        active_model.stream_id = ActiveValue::Set(Some(stream_id));
    }
    if let Some(master_class_id) = payload.master_class_id {
        active_model.master_class_id = ActiveValue::Set(Some(master_class_id));
    }
    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
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

// DELETE /api/v1/exam-types/{id}
pub async fn delete_exam_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let et = exam_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam type not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_type:delete", Some(et.organization_id), None).await?;

    let active_model: ActiveModel = et.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
