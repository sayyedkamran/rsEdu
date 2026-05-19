use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize},
    entities::exam_subjects::{self, ActiveModel},
    exam_subjects::dto::{CreateExamSubjectRequest, ExamSubjectResponse, UpdateExamSubjectRequest},
};

fn to_response(es: exam_subjects::Model) -> ExamSubjectResponse {
    ExamSubjectResponse {
        id: es.id,
        exam_id: es.exam_id,
        subject_id: es.subject_id,
        total_marks: es.total_marks,
        passing_marks: es.passing_marks,
        exam_date: es.exam_date.to_string(),
        exam_time: es.exam_time,
        venue: es.venue,
        is_active: es.is_active,
    }
}

// POST /api/v1/exam-subjects
pub async fn create_exam_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateExamSubjectRequest>,
) -> Result<Json<ExamSubjectResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam_subject:create", None, None).await?;

    let exam_date = NaiveDate::parse_from_str(&payload.exam_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid exam_date: {}", e)))?;

    let new_es = ActiveModel {
        exam_id: ActiveValue::Set(payload.exam_id),
        subject_id: ActiveValue::Set(payload.subject_id),
        total_marks: ActiveValue::Set(payload.total_marks),
        passing_marks: ActiveValue::Set(payload.passing_marks),
        exam_date: ActiveValue::Set(exam_date),
        exam_time: ActiveValue::Set(payload.exam_time),
        venue: ActiveValue::Set(payload.venue),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let es = new_es
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(es)))
}

// GET /api/v1/exam-subjects
pub async fn get_exam_subjects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ExamSubjectResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "exam_subject:read", None, None).await?;

    let exam_subjects = exam_subjects::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(exam_subjects.into_iter().map(to_response).collect()))
}

// GET /api/v1/exam-subjects/{id}
pub async fn get_exam_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ExamSubjectResponse>, (StatusCode, String)> {
    let es = exam_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam subject not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_subject:read", None, None).await?;

    Ok(Json(to_response(es)))
}

// PUT /api/v1/exam-subjects/{id}
pub async fn update_exam_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExamSubjectRequest>,
) -> Result<Json<ExamSubjectResponse>, (StatusCode, String)> {
    let es = exam_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam subject not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_subject:update", None, None).await?;

    let mut active_model: ActiveModel = es.into();

    if let Some(total_marks) = payload.total_marks {
        active_model.total_marks = ActiveValue::Set(total_marks);
    }
    if let Some(passing_marks) = payload.passing_marks {
        active_model.passing_marks = ActiveValue::Set(passing_marks);
    }
    if let Some(exam_date) = payload.exam_date {
        let date = NaiveDate::parse_from_str(&exam_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid exam_date: {}", e)))?;
        active_model.exam_date = ActiveValue::Set(date);
    }
    if let Some(exam_time) = payload.exam_time {
        active_model.exam_time = ActiveValue::Set(Some(exam_time));
    }
    if let Some(venue) = payload.venue {
        active_model.venue = ActiveValue::Set(Some(venue));
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

// DELETE /api/v1/exam-subjects/{id}
pub async fn delete_exam_subject(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let es = exam_subjects::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Exam subject not found".to_string()))?;

    authorize(&*state.db, &claims, "exam_subject:delete", None, None).await?;

    let active_model: ActiveModel = es.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
