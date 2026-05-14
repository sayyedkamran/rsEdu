use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    entities::students::{self, ActiveModel},
    students::dto::{CreateStudentRequest, StudentResponse, UpdateStudentRequest},
};

fn to_response(student: students::Model) -> StudentResponse {
    StudentResponse {
        id: student.id,
        user_id: student.user_id,
        organization_id: student.organization_id,
        branch_id: student.branch_id,
        first_name: student.first_name,
        last_name: student.last_name,
        date_of_birth: student.date_of_birth.to_string(),
        gender: student.gender,
        roll_number: student.roll_number,
        admission_date: student.admission_date.to_string(),
        is_active: student.is_active,
    }
}

// POST /api/v1/students
pub async fn create_student(
    State(state): State<AppState>,
    Json(payload): Json<CreateStudentRequest>,
) -> Result<Json<StudentResponse>, (StatusCode, String)> {
    let date_of_birth = NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date_of_birth: {}", e)))?;

    let admission_date = NaiveDate::parse_from_str(&payload.admission_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid admission_date: {}", e)))?;

    let new_student = ActiveModel {
        user_id: ActiveValue::Set(payload.user_id),
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        first_name: ActiveValue::Set(payload.first_name),
        last_name: ActiveValue::Set(payload.last_name),
        date_of_birth: ActiveValue::Set(date_of_birth),
        gender: ActiveValue::Set(payload.gender),
        roll_number: ActiveValue::Set(payload.roll_number),
        admission_date: ActiveValue::Set(admission_date),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let student = new_student
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(student)))
}

// GET /api/v1/students
pub async fn get_students(
    State(state): State<AppState>,
) -> Result<Json<Vec<StudentResponse>>, (StatusCode, String)> {
    let students = students::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(students.into_iter().map(to_response).collect()))
}

// GET /api/v1/students/{id}
pub async fn get_student(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<StudentResponse>, (StatusCode, String)> {
    let student = students::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student not found".to_string()))?;

    Ok(Json(to_response(student)))
}

// PUT /api/v1/students/{id}
pub async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<Json<StudentResponse>, (StatusCode, String)> {
    let student = students::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student not found".to_string()))?;

    let mut active_model: ActiveModel = student.into();

    if let Some(first_name) = payload.first_name {
        active_model.first_name = ActiveValue::Set(first_name);
    }
    if let Some(last_name) = payload.last_name {
        active_model.last_name = ActiveValue::Set(last_name);
    }
    if let Some(roll_number) = payload.roll_number {
        active_model.roll_number = ActiveValue::Set(roll_number);
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated_student = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated_student)))
}

// DELETE /api/v1/students/{id}
pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let student = students::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student not found".to_string()))?;

    let active_model: ActiveModel = student.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}