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
    entities::staff_leaves::{self, ActiveModel},
    staff_leaves::dto::{CreateStaffLeaveRequest, StaffLeaveResponse, UpdateStaffLeaveRequest},
};

fn to_response(m: staff_leaves::Model) -> StaffLeaveResponse {
    StaffLeaveResponse {
        id: m.id,
        staff_id: m.staff_id,
        branch_id: m.branch_id,
        leave_type_id: m.leave_type_id,
        start_date: m.start_date.to_string(),
        end_date: m.end_date.to_string(),
        days: m.days,
        reason: m.reason,
        status: m.status,
        approved_by: m.approved_by,
        rejection_reason: m.rejection_reason,
    }
}

pub async fn create_staff_leave(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffLeaveRequest>,
) -> Result<Json<StaffLeaveResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_leave:create", None, None).await?;

    let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;
    let end_date = NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;

    let record = ActiveModel {
        staff_id: ActiveValue::Set(payload.staff_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        leave_type_id: ActiveValue::Set(payload.leave_type_id),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        days: ActiveValue::Set(payload.days),
        reason: ActiveValue::Set(payload.reason),
        status: ActiveValue::Set("pending".to_string()),
        approved_by: ActiveValue::Set(None),
        rejection_reason: ActiveValue::Set(None),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_leaves(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffLeaveResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_leave:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff_leaves::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(staff_leaves::Column::BranchId.eq(branch_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_leave(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffLeaveResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_leave:read", None, None).await?;

    let record = staff_leaves::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff leave not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_leave(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffLeaveRequest>,
) -> Result<Json<StaffLeaveResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_leave:update", None, None).await?;

    let record = staff_leaves::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff leave not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(status) = payload.status { active.status = ActiveValue::Set(status); }
    if let Some(approved_by) = payload.approved_by { active.approved_by = ActiveValue::Set(Some(approved_by)); }
    if let Some(rejection_reason) = payload.rejection_reason { active.rejection_reason = ActiveValue::Set(Some(rejection_reason)); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_leave(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_leave:delete", None, None).await?;

    let record = staff_leaves::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff leave not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
