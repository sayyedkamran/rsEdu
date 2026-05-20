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
    entities::staff_monthly_salary_details::{self, ActiveModel},
    staff_monthly_salary_details::dto::{CreateStaffMonthlySalaryDetailRequest, StaffMonthlySalaryDetailResponse, UpdateStaffMonthlySalaryDetailRequest},
};

fn to_response(m: staff_monthly_salary_details::Model) -> StaffMonthlySalaryDetailResponse {
    StaffMonthlySalaryDetailResponse {
        id: m.id,
        staff_monthly_salary_id: m.staff_monthly_salary_id,
        allowance_deduction_type_id: m.allowance_deduction_type_id,
        description: m.description,
        r#type: m.r#type,
        amount: m.amount,
        is_adhoc: m.is_adhoc,
        remarks: m.remarks,
    }
}

pub async fn create_staff_monthly_salary_detail(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffMonthlySalaryDetailRequest>,
) -> Result<Json<StaffMonthlySalaryDetailResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary_detail:create", None, None).await?;

    let record = ActiveModel {
        staff_monthly_salary_id: ActiveValue::Set(payload.staff_monthly_salary_id),
        allowance_deduction_type_id: ActiveValue::Set(payload.allowance_deduction_type_id),
        description: ActiveValue::Set(payload.description),
        r#type: ActiveValue::Set(payload.r#type),
        amount: ActiveValue::Set(payload.amount),
        is_adhoc: ActiveValue::Set(payload.is_adhoc),
        remarks: ActiveValue::Set(payload.remarks),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_monthly_salary_details(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffMonthlySalaryDetailResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary_detail:read", None, None).await?;

    let records = staff_monthly_salary_details::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_monthly_salary_detail(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffMonthlySalaryDetailResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary_detail:read", None, None).await?;

    let record = staff_monthly_salary_details::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary detail not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_monthly_salary_detail(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffMonthlySalaryDetailRequest>,
) -> Result<Json<StaffMonthlySalaryDetailResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary_detail:update", None, None).await?;

    let record = staff_monthly_salary_details::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary detail not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(desc) = payload.description { active.description = ActiveValue::Set(desc); }
    if let Some(amount) = payload.amount { active.amount = ActiveValue::Set(amount); }
    if let Some(is_adhoc) = payload.is_adhoc { active.is_adhoc = ActiveValue::Set(is_adhoc); }
    if let Some(remarks) = payload.remarks { active.remarks = ActiveValue::Set(Some(remarks)); }

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_monthly_salary_detail(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary_detail:delete", None, None).await?;

    let record = staff_monthly_salary_details::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary detail not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
