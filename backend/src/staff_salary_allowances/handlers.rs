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
    entities::staff_salary_allowances::{self, ActiveModel},
    staff_salary_allowances::dto::{CreateStaffSalaryAllowanceRequest, StaffSalaryAllowanceResponse, UpdateStaffSalaryAllowanceRequest},
};

fn to_response(m: staff_salary_allowances::Model) -> StaffSalaryAllowanceResponse {
    StaffSalaryAllowanceResponse {
        id: m.id,
        staff_salary_id: m.staff_salary_id,
        allowance_deduction_type_id: m.allowance_deduction_type_id,
        amount: m.amount,
        calculation_type: m.calculation_type,
        is_active: m.is_active,
    }
}

pub async fn create_staff_salary_allowance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffSalaryAllowanceRequest>,
) -> Result<Json<StaffSalaryAllowanceResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary_allowance:create", None, None).await?;

    let record = ActiveModel {
        staff_salary_id: ActiveValue::Set(payload.staff_salary_id),
        allowance_deduction_type_id: ActiveValue::Set(payload.allowance_deduction_type_id),
        amount: ActiveValue::Set(payload.amount),
        calculation_type: ActiveValue::Set(payload.calculation_type),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_salary_allowances(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffSalaryAllowanceResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary_allowance:read", None, None).await?;

    let records = staff_salary_allowances::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_salary_allowance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffSalaryAllowanceResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary_allowance:read", None, None).await?;

    let record = staff_salary_allowances::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary allowance not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_salary_allowance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffSalaryAllowanceRequest>,
) -> Result<Json<StaffSalaryAllowanceResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary_allowance:update", None, None).await?;

    let record = staff_salary_allowances::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary allowance not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(amount) = payload.amount { active.amount = ActiveValue::Set(amount); }
    if let Some(calc) = payload.calculation_type { active.calculation_type = ActiveValue::Set(calc); }
    if let Some(is_active) = payload.is_active { active.is_active = ActiveValue::Set(is_active); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_salary_allowance(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary_allowance:delete", None, None).await?;

    let record = staff_salary_allowances::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary allowance not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
