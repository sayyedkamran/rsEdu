use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::exams::{self, ActiveModel},
    exams::dto::{CreateExamRequest, ExamResponse, UpdateExamRequest},
};

fn to_response(e: exams::Model) -> ExamResponse {
    ExamResponse {
        id: e.id,
        branch_id: e.branch_id,
        class_id: e.class_id,
        academic_year_id: e.academic_year_id,
        exam_type_id: e.exam_type_id,
        name: e.name,
        name_urdu: e.name_urdu,
        start_date: e.start_date.to_string(),
        end_date: e.end_date.to_string(),
        is_active: e.is_active,
    }
}

// POST /api/v1/exams
pub async fn create_exam(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateExamRequest>,
) -> Result<Json<ExamResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam:create", None, None).await?;

    let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;
    let end_date = NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;

    let new_e = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        class_id: ActiveValue::Set(payload.class_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        exam_type_id: ActiveValue::Set(payload.exam_type_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let exam = new_e
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(exam)))
}

// GET /api/v1/exams
pub async fn get_exams(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ExamResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = exams::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(exams::Column::BranchId.eq(branch_id));
    }

    let exams = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(exams.into_iter().map(to_response).collect()))
}

// GET /api/v1/exams/{id}
pub async fn get_exam(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ExamResponse>, (StatusCode, String)> {
    let exam = exams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam not found".to_string()))?;

    authorize(&*state.db, &claims, "exam:read", None, None).await?;

    Ok(Json(to_response(exam)))
}

// PUT /api/v1/exams/{id}
pub async fn update_exam(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExamRequest>,
) -> Result<Json<ExamResponse>, (StatusCode, String)> {
    let exam = exams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam not found".to_string()))?;

    authorize(&*state.db, &claims, "exam:update", None, None).await?;

    let mut active_model: ActiveModel = exam.into();

    if let Some(class_id) = payload.class_id {
        active_model.class_id = ActiveValue::Set(class_id);
    }
    if let Some(academic_year_id) = payload.academic_year_id {
        active_model.academic_year_id = ActiveValue::Set(academic_year_id);
    }
    if let Some(exam_type_id) = payload.exam_type_id {
        active_model.exam_type_id = ActiveValue::Set(exam_type_id);
    }
    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(start_date) = payload.start_date {
        let date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;
        active_model.start_date = ActiveValue::Set(date);
    }
    if let Some(end_date) = payload.end_date {
        let date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;
        active_model.end_date = ActiveValue::Set(date);
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

// DELETE /api/v1/exams/{id}
pub async fn delete_exam(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let exam = exams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam not found".to_string()))?;

    authorize(&*state.db, &claims, "exam:delete", None, None).await?;

    let active_model: ActiveModel = exam.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
