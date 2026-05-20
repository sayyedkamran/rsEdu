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
    entities::student_enrollments::{self, ActiveModel},
    student_enrollments::dto::{CreateStudentEnrollmentRequest, StudentEnrollmentResponse, UpdateStudentEnrollmentRequest},
};

fn to_response(se: student_enrollments::Model) -> StudentEnrollmentResponse {
    StudentEnrollmentResponse {
        id: se.id,
        student_id: se.student_id,
        class_id: se.class_id,
        academic_year_id: se.academic_year_id,
        roll_number: se.roll_number,
        enrollment_date: se.enrollment_date.to_string(),
        status: se.status,
        remarks: se.remarks,
        enrolled_by: se.enrolled_by,
    }
}

// POST /api/v1/student-enrollments
pub async fn create_student_enrollment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStudentEnrollmentRequest>,
) -> Result<Json<StudentEnrollmentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_enrollment:create", None, None).await?;

    let enrollment_date = NaiveDate::parse_from_str(&payload.enrollment_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid enrollment_date: {}", e)))?;

    let new_se = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        class_id: ActiveValue::Set(payload.class_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        roll_number: ActiveValue::Set(payload.roll_number),
        enrollment_date: ActiveValue::Set(enrollment_date),
        status: ActiveValue::Set(payload.status),
        remarks: ActiveValue::Set(payload.remarks),
        enrolled_by: ActiveValue::Set(payload.enrolled_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let se = new_se
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(se)))
}

// GET /api/v1/student-enrollments
pub async fn get_student_enrollments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StudentEnrollmentResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_enrollment:read", None, None).await?;

    let records = student_enrollments::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/student-enrollments/{id}
pub async fn get_student_enrollment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StudentEnrollmentResponse>, (StatusCode, String)> {
    let se = student_enrollments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student enrollment not found".to_string()))?;

    authorize(&*state.db, &claims, "student_enrollment:read", None, None).await?;

    Ok(Json(to_response(se)))
}

// PUT /api/v1/student-enrollments/{id}
pub async fn update_student_enrollment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStudentEnrollmentRequest>,
) -> Result<Json<StudentEnrollmentResponse>, (StatusCode, String)> {
    let se = student_enrollments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student enrollment not found".to_string()))?;

    authorize(&*state.db, &claims, "student_enrollment:update", None, None).await?;

    let mut active_model: ActiveModel = se.into();

    if let Some(roll_number) = payload.roll_number {
        active_model.roll_number = ActiveValue::Set(Some(roll_number));
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/student-enrollments/{id}
pub async fn delete_student_enrollment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let se = student_enrollments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student enrollment not found".to_string()))?;

    authorize(&*state.db, &claims, "student_enrollment:delete", None, None).await?;

    let active_model: ActiveModel = se.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
