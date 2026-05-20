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
    entities::student_guardians::{self, ActiveModel},
    student_guardians::dto::{CreateStudentGuardianRequest, StudentGuardianResponse, UpdateStudentGuardianRequest},
};

fn to_response(sg: student_guardians::Model) -> StudentGuardianResponse {
    StudentGuardianResponse {
        id: sg.id,
        student_id: sg.student_id,
        guardian_id: sg.guardian_id,
        is_primary_contact: sg.is_primary_contact,
        is_emergency_contact: sg.is_emergency_contact,
    }
}

// POST /api/v1/student-guardians
pub async fn create_student_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStudentGuardianRequest>,
) -> Result<Json<StudentGuardianResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_guardian:create", None, None).await?;

    let new_sg = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        guardian_id: ActiveValue::Set(payload.guardian_id),
        is_primary_contact: ActiveValue::Set(payload.is_primary_contact),
        is_emergency_contact: ActiveValue::Set(payload.is_emergency_contact),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let sg = new_sg
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(sg)))
}

// GET /api/v1/student-guardians
pub async fn get_student_guardians(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StudentGuardianResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_guardian:read", None, None).await?;

    let records = student_guardians::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/student-guardians/{id}
pub async fn get_student_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StudentGuardianResponse>, (StatusCode, String)> {
    let sg = student_guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "student_guardian:read", None, None).await?;

    Ok(Json(to_response(sg)))
}

// PUT /api/v1/student-guardians/{id}
pub async fn update_student_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStudentGuardianRequest>,
) -> Result<Json<StudentGuardianResponse>, (StatusCode, String)> {
    let sg = student_guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "student_guardian:update", None, None).await?;

    let mut active_model: ActiveModel = sg.into();

    if let Some(is_primary_contact) = payload.is_primary_contact {
        active_model.is_primary_contact = ActiveValue::Set(is_primary_contact);
    }
    if let Some(is_emergency_contact) = payload.is_emergency_contact {
        active_model.is_emergency_contact = ActiveValue::Set(is_emergency_contact);
    }

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/student-guardians/{id}
pub async fn delete_student_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let sg = student_guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "student_guardian:delete", None, None).await?;

    let active_model: ActiveModel = sg.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
