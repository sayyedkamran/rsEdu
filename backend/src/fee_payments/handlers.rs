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
    entities::fee_payments::{self, ActiveModel},
    fee_payments::dto::{CreateFeePaymentRequest, FeePaymentResponse, UpdateFeePaymentRequest},
};

fn to_response(fp: fee_payments::Model) -> FeePaymentResponse {
    FeePaymentResponse {
        id: fp.id,
        fee_bill_id: fp.fee_bill_id,
        student_id: fp.student_id,
        payment_method_id: fp.payment_method_id,
        amount_paid: fp.amount_paid,
        reference_number: fp.reference_number,
        payment_date: fp.payment_date.to_string(),
        receipt_number: fp.receipt_number,
        remarks: fp.remarks,
        received_by: fp.received_by,
    }
}

// POST /api/v1/fee-payments
pub async fn create_fee_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeePaymentRequest>,
) -> Result<Json<FeePaymentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_payment:create", None, None).await?;

    let payment_date = NaiveDate::parse_from_str(&payload.payment_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid payment_date: {}", e)))?;

    let new_fp = ActiveModel {
        fee_bill_id: ActiveValue::Set(payload.fee_bill_id),
        student_id: ActiveValue::Set(payload.student_id),
        payment_method_id: ActiveValue::Set(payload.payment_method_id),
        amount_paid: ActiveValue::Set(payload.amount_paid),
        reference_number: ActiveValue::Set(payload.reference_number),
        payment_date: ActiveValue::Set(payment_date),
        receipt_number: ActiveValue::Set(payload.receipt_number),
        remarks: ActiveValue::Set(payload.remarks),
        received_by: ActiveValue::Set(payload.received_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let fp = new_fp
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(fp)))
}

// GET /api/v1/fee-payments
pub async fn get_fee_payments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<FeePaymentResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_payment:read", None, None).await?;

    let fee_payments = fee_payments::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(fee_payments.into_iter().map(to_response).collect()))
}

// GET /api/v1/fee-payments/{id}
pub async fn get_fee_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<FeePaymentResponse>, (StatusCode, String)> {
    let fp = fee_payments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee payment not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_payment:read", None, None).await?;

    Ok(Json(to_response(fp)))
}

// PUT /api/v1/fee-payments/{id}
pub async fn update_fee_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFeePaymentRequest>,
) -> Result<Json<FeePaymentResponse>, (StatusCode, String)> {
    let fp = fee_payments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee payment not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_payment:update", None, None).await?;

    let mut active_model: ActiveModel = fp.into();

    if let Some(reference_number) = payload.reference_number {
        active_model.reference_number = ActiveValue::Set(Some(reference_number));
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/fee-payments/{id}
pub async fn delete_fee_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let fp = fee_payments::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee payment not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_payment:delete", None, None).await?;

    let active_model: ActiveModel = fp.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
