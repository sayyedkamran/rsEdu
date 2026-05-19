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
    entities::attendance::{self, ActiveModel},
    attendance::dto::{CreateAttendanceRequest, AttendanceResponse, UpdateAttendanceRequest},
};

fn to_response(a: attendance::Model) -> AttendanceResponse {
    AttendanceResponse {
        id: a.id,
        branch_id: a.branch_id,
        class_id: a.class_id,
        student_id: a.student_id,
        marked_by: a.marked_by,
        date: a.date.to_string(),
        status: a.status,
        remarks: a.remarks,
    }
}

// POST /api/v1/attendance
pub async fn create_attendance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAttendanceRequest>,
) -> Result<Json<AttendanceResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "attendance:create", None, None).await?;

    let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date: {}", e)))?;

    let new_attendance = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        class_id: ActiveValue::Set(payload.class_id),
        student_id: ActiveValue::Set(payload.student_id),
        marked_by: ActiveValue::Set(payload.marked_by),
        date: ActiveValue::Set(date),
        status: ActiveValue::Set(payload.status),
        remarks: ActiveValue::Set(payload.remarks),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let a = new_attendance
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(a)))
}

// GET /api/v1/attendance
pub async fn get_attendance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AttendanceResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "attendance:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = attendance::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(attendance::Column::BranchId.eq(branch_id));
    }

    let attendance = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(attendance.into_iter().map(to_response).collect()))
}

// GET /api/v1/attendance/{id}
pub async fn get_attendance_record(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<AttendanceResponse>, (StatusCode, String)> {
    let a = attendance::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Attendance record not found".to_string()))?;

    authorize(&*state.db, &claims, "attendance:read", None, None).await?;

    Ok(Json(to_response(a)))
}

// PUT /api/v1/attendance/{id}
pub async fn update_attendance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAttendanceRequest>,
) -> Result<Json<AttendanceResponse>, (StatusCode, String)> {
    let a = attendance::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Attendance record not found".to_string()))?;

    authorize(&*state.db, &claims, "attendance:update", None, None).await?;

    let mut active_model: ActiveModel = a.into();

    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }
    if let Some(marked_by) = payload.marked_by {
        active_model.marked_by = ActiveValue::Set(marked_by);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/attendance/{id}
pub async fn delete_attendance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let a = attendance::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Attendance record not found".to_string()))?;

    authorize(&*state.db, &claims, "attendance:delete", None, None).await?;

    let active_model: ActiveModel = a.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
