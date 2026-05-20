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
    entities::student_scholarships::{self, ActiveModel},
    student_scholarships::dto::{CreateStudentScholarshipRequest, StudentScholarshipResponse, UpdateStudentScholarshipRequest},
};

fn to_response(ss: student_scholarships::Model) -> StudentScholarshipResponse {
    StudentScholarshipResponse {
        id: ss.id,
        student_id: ss.student_id,
        scholarship_id: ss.scholarship_id,
        academic_year_id: ss.academic_year_id,
        start_date: ss.start_date.to_string(),
        end_date: ss.end_date.map(|d| d.to_string()),
        status: ss.status,
        requested_by: ss.requested_by,
        approved_by: ss.approved_by,
        remarks: ss.remarks,
        rejection_reason: ss.rejection_reason,
        is_active: ss.is_active,
    }
}

// POST /api/v1/student-scholarships
pub async fn create_student_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStudentScholarshipRequest>,
) -> Result<Json<StudentScholarshipResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_scholarship:create", None, None).await?;

    let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;

    let end_date = payload.end_date
        .as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;

    let new_ss = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        scholarship_id: ActiveValue::Set(payload.scholarship_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        status: ActiveValue::Set(payload.status),
        requested_by: ActiveValue::Set(payload.requested_by),
        approved_by: ActiveValue::Set(None),
        remarks: ActiveValue::Set(payload.remarks),
        rejection_reason: ActiveValue::Set(None),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let ss = new_ss
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(ss)))
}

// GET /api/v1/student-scholarships
pub async fn get_student_scholarships(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StudentScholarshipResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_scholarship:read", None, None).await?;

    let records = student_scholarships::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/student-scholarships/{id}
pub async fn get_student_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StudentScholarshipResponse>, (StatusCode, String)> {
    let ss = student_scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "student_scholarship:read", None, None).await?;

    Ok(Json(to_response(ss)))
}

// PUT /api/v1/student-scholarships/{id}
pub async fn update_student_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStudentScholarshipRequest>,
) -> Result<Json<StudentScholarshipResponse>, (StatusCode, String)> {
    let ss = student_scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "student_scholarship:update", None, None).await?;

    let mut active_model: ActiveModel = ss.into();

    if let Some(end_date_str) = payload.end_date {
        let date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;
        active_model.end_date = ActiveValue::Set(Some(date));
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(approved_by) = payload.approved_by {
        active_model.approved_by = ActiveValue::Set(Some(approved_by));
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }
    if let Some(rejection_reason) = payload.rejection_reason {
        active_model.rejection_reason = ActiveValue::Set(Some(rejection_reason));
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

// DELETE /api/v1/student-scholarships/{id}
pub async fn delete_student_scholarship(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ss = student_scholarships::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student scholarship not found".to_string()))?;

    authorize(&*state.db, &claims, "student_scholarship:delete", None, None).await?;

    let active_model: ActiveModel = ss.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
