use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    entities::teachers::{self, ActiveModel},
    teachers::dto::{CreateTeacherRequest, TeacherResponse, UpdateTeacherRequest},
};

// Helper to convert entity Model to TeacherResponse
fn to_response(teacher: teachers::Model) -> TeacherResponse {
    TeacherResponse {
        id: teacher.id,
        user_id: teacher.user_id,
        first_name: teacher.first_name,
        last_name: teacher.last_name,
        father_name: teacher.father_name,
        date_of_birth: teacher.date_of_birth.to_string(),
        gender: teacher.gender,
        phone: teacher.phone,
        address: teacher.address,
        qualification: teacher.qualification,
        specialization: teacher.specialization,
        joining_date: teacher.joining_date.to_string(),
        cnic: teacher.cnic,
        is_active: teacher.is_active,
    }
}

// POST /api/v1/teachers
pub async fn create_teacher(
    State(state): State<AppState>,
    Json(payload): Json<CreateTeacherRequest>,
) -> Result<Json<TeacherResponse>, (StatusCode, String)> {
    let date_of_birth = NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date_of_birth: {}", e)))?;

    let joining_date = NaiveDate::parse_from_str(&payload.joining_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid joining_date: {}", e)))?;

    let new_teacher = ActiveModel {
        user_id: ActiveValue::Set(payload.user_id),
        first_name: ActiveValue::Set(payload.first_name),
        last_name: ActiveValue::Set(payload.last_name),
        father_name: ActiveValue::Set(payload.father_name),
        date_of_birth: ActiveValue::Set(date_of_birth),
        gender: ActiveValue::Set(payload.gender),
        phone: ActiveValue::Set(payload.phone),
        address: ActiveValue::Set(payload.address),
        qualification: ActiveValue::Set(payload.qualification),
        specialization: ActiveValue::Set(payload.specialization),
        joining_date: ActiveValue::Set(joining_date),
        cnic: ActiveValue::Set(payload.cnic),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let teacher = new_teacher
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(teacher)))
}

// GET /api/v1/teachers
pub async fn get_teachers(
    State(state): State<AppState>,
) -> Result<Json<Vec<TeacherResponse>>, (StatusCode, String)> {
    let teachers = teachers::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(teachers.into_iter().map(to_response).collect()))
}

// GET /api/v1/teachers/:id
pub async fn get_teacher(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TeacherResponse>, (StatusCode, String)> {
    let teacher = teachers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Teacher not found".to_string()))?;

    Ok(Json(to_response(teacher)))
}

// PUT /api/v1/teachers/:id
pub async fn update_teacher(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTeacherRequest>,
) -> Result<Json<TeacherResponse>, (StatusCode, String)> {
    let teacher = teachers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Teacher not found".to_string()))?;

    let mut active_model: ActiveModel = teacher.into();

    if let Some(first_name) = payload.first_name {
        active_model.first_name = ActiveValue::Set(first_name);
    }
    if let Some(last_name) = payload.last_name {
        active_model.last_name = ActiveValue::Set(last_name);
    }
    if let Some(father_name) = payload.father_name {
        active_model.father_name = ActiveValue::Set(Some(father_name));
    }
    if let Some(phone) = payload.phone {
        active_model.phone = ActiveValue::Set(Some(phone));
    }
    if let Some(address) = payload.address {
        active_model.address = ActiveValue::Set(Some(address));
    }
    if let Some(qualification) = payload.qualification {
        active_model.qualification = ActiveValue::Set(qualification);
    }
    if let Some(specialization) = payload.specialization {
        active_model.specialization = ActiveValue::Set(specialization);
    }
    if let Some(cnic) = payload.cnic {
        active_model.cnic = ActiveValue::Set(Some(cnic));
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated_teacher = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated_teacher)))
}

// DELETE /api/v1/teachers/:id
pub async fn delete_teacher(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let teacher = teachers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Teacher not found".to_string()))?;

    let active_model: ActiveModel = teacher.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}