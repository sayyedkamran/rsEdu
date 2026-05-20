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
    entities::salary_payments::{self, ActiveModel},
    salary_payments::dto::{CreateSalaryPaymentRequest, SalaryPaymentResponse, UpdateSalaryPaymentRequest},
};

fn to_response(m: salary_payments::Model) -> SalaryPaymentResponse {
    SalaryPaymentResponse {
        id: m.id,
        staff_monthly_salary_id: m.staff_monthly_salary_id,
        staff_id: m.staff_id,
        amount_paid: m.amount_paid,
        payment_method_id: m.payment_method_id,
        reference_number: m.reference_number,
        payment_date: m.payment_date.to_string(),
        receipt_number: m.receipt_number,
        remarks: m.remarks,
        paid_by: m.paid_by,
    }
}

pub async fn create_salary_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSalaryPaymentRequest>,
) -> Result<Json<SalaryPaymentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_payment:create", None, None).await?;

    let payment_date = NaiveDate::parse_from_str(&payload.payment_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid payment_date: {}", e)))?;

    let record = ActiveModel {
        staff_monthly_salary_id: ActiveValue::Set(payload.staff_monthly_salary_id),
        staff_id: ActiveValue::Set(payload.staff_id),
        amount_paid: ActiveValue::Set(payload.amount_paid),
        payment_method_id: ActiveValue::Set(payload.payment_method_id),
        reference_number: ActiveValue::Set(payload.reference_number),
        payment_date: ActiveValue::Set(payment_date),
        receipt_number: ActiveValue::Set(payload.receipt_number),
        remarks: ActiveValue::Set(payload.remarks),
        paid_by: ActiveValue::Set(payload.paid_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_salary_payments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SalaryPaymentResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_payment:read", None, None).await?;

    let records = salary_payments::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_salary_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<SalaryPaymentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_payment:read", None, None).await?;

    let record = salary_payments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary payment not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_salary_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSalaryPaymentRequest>,
) -> Result<Json<SalaryPaymentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_payment:update", None, None).await?;

    let record = salary_payments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary payment not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(ref_num) = payload.reference_number { active.reference_number = ActiveValue::Set(Some(ref_num)); }
    if let Some(remarks) = payload.remarks { active.remarks = ActiveValue::Set(Some(remarks)); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_salary_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_payment:delete", None, None).await?;

    let record = salary_payments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary payment not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
