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
    entities::staff_loan_deductions::{self, ActiveModel},
    staff_loan_deductions::dto::{CreateStaffLoanDeductionRequest, StaffLoanDeductionResponse, UpdateStaffLoanDeductionRequest},
};

fn to_response(m: staff_loan_deductions::Model) -> StaffLoanDeductionResponse {
    StaffLoanDeductionResponse {
        id: m.id,
        staff_loan_id: m.staff_loan_id,
        staff_monthly_salary_id: m.staff_monthly_salary_id,
        amount_deducted: m.amount_deducted,
    }
}

pub async fn create_staff_loan_deduction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffLoanDeductionRequest>,
) -> Result<Json<StaffLoanDeductionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan_deduction:create", None, None).await?;

    let record = ActiveModel {
        staff_loan_id: ActiveValue::Set(payload.staff_loan_id),
        staff_monthly_salary_id: ActiveValue::Set(payload.staff_monthly_salary_id),
        amount_deducted: ActiveValue::Set(payload.amount_deducted),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_loan_deductions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffLoanDeductionResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan_deduction:read", None, None).await?;

    let records = staff_loan_deductions::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_loan_deduction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffLoanDeductionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan_deduction:read", None, None).await?;

    let record = staff_loan_deductions::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan deduction not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_loan_deduction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffLoanDeductionRequest>,
) -> Result<Json<StaffLoanDeductionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan_deduction:update", None, None).await?;

    let record = staff_loan_deductions::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan deduction not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(amount) = payload.amount_deducted { active.amount_deducted = ActiveValue::Set(amount); }

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_loan_deduction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan_deduction:delete", None, None).await?;

    let record = staff_loan_deductions::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan deduction not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
