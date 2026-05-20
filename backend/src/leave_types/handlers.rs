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
    entities::leave_types::{self, ActiveModel},
    leave_types::dto::{CreateLeaveTypeRequest, LeaveTypeResponse, UpdateLeaveTypeRequest},
};

fn to_response(m: leave_types::Model) -> LeaveTypeResponse {
    LeaveTypeResponse {
        id: m.id,
        organization_id: m.organization_id,
        name: m.name,
        name_urdu: m.name_urdu,
        max_days_per_year: m.max_days_per_year,
        is_paid: m.is_paid,
        is_active: m.is_active,
    }
}

pub async fn create_leave_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateLeaveTypeRequest>,
) -> Result<Json<LeaveTypeResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "leave_type:create", Some(payload.organization_id), None).await?;

    let record = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        max_days_per_year: ActiveValue::Set(payload.max_days_per_year),
        is_paid: ActiveValue::Set(payload.is_paid),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_leave_types(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<LeaveTypeResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "leave_type:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = leave_types::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(leave_types::Column::OrganizationId.eq(org_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_leave_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveTypeResponse>, (StatusCode, String)> {
    let record = leave_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Leave type not found".to_string()))?;

    authorize(&*state.db, &claims, "leave_type:read", Some(record.organization_id), None).await?;

    Ok(Json(to_response(record)))
}

pub async fn update_leave_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLeaveTypeRequest>,
) -> Result<Json<LeaveTypeResponse>, (StatusCode, String)> {
    let record = leave_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Leave type not found".to_string()))?;

    authorize(&*state.db, &claims, "leave_type:update", Some(record.organization_id), None).await?;

    let mut active: ActiveModel = record.into();

    if let Some(name) = payload.name { active.name = ActiveValue::Set(name); }
    if let Some(name_urdu) = payload.name_urdu { active.name_urdu = ActiveValue::Set(Some(name_urdu)); }
    if let Some(max_days) = payload.max_days_per_year { active.max_days_per_year = ActiveValue::Set(max_days); }
    if let Some(is_paid) = payload.is_paid { active.is_paid = ActiveValue::Set(is_paid); }
    if let Some(is_active) = payload.is_active { active.is_active = ActiveValue::Set(is_active); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_leave_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let record = leave_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Leave type not found".to_string()))?;

    authorize(&*state.db, &claims, "leave_type:delete", Some(record.organization_id), None).await?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
