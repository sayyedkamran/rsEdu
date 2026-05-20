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
    entities::fee_arrears::{self, ActiveModel},
    fee_arrears::dto::{CreateFeeArrearRequest, FeeArrearResponse, UpdateFeeArrearRequest},
};

fn to_response(fa: fee_arrears::Model) -> FeeArrearResponse {
    FeeArrearResponse {
        id: fa.id,
        student_id: fa.student_id,
        branch_id: fa.branch_id,
        academic_year_id: fa.academic_year_id,
        amount: fa.amount,
        reason: fa.reason,
        status: fa.status,
        entered_by: fa.entered_by,
        remarks: fa.remarks,
    }
}

// POST /api/v1/fee-arrears
pub async fn create_fee_arrear(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeeArrearRequest>,
) -> Result<Json<FeeArrearResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_arrear:create", None, None).await?;

    let new_fa = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        amount: ActiveValue::Set(payload.amount),
        reason: ActiveValue::Set(payload.reason),
        status: ActiveValue::Set(payload.status),
        entered_by: ActiveValue::Set(payload.entered_by),
        remarks: ActiveValue::Set(payload.remarks),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let fa = new_fa
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(fa)))
}

// GET /api/v1/fee-arrears
pub async fn get_fee_arrears(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<FeeArrearResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_arrear:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = fee_arrears::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(fee_arrears::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/fee-arrears/{id}
pub async fn get_fee_arrear(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<FeeArrearResponse>, (StatusCode, String)> {
    let fa = fee_arrears::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee arrear not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_arrear:read", None, None).await?;

    Ok(Json(to_response(fa)))
}

// PUT /api/v1/fee-arrears/{id}
pub async fn update_fee_arrear(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFeeArrearRequest>,
) -> Result<Json<FeeArrearResponse>, (StatusCode, String)> {
    let fa = fee_arrears::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee arrear not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_arrear:update", None, None).await?;

    let mut active_model: ActiveModel = fa.into();

    if let Some(amount) = payload.amount {
        active_model.amount = ActiveValue::Set(amount);
    }
    if let Some(reason) = payload.reason {
        active_model.reason = ActiveValue::Set(reason);
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
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

// DELETE /api/v1/fee-arrears/{id}
pub async fn delete_fee_arrear(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let fa = fee_arrears::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee arrear not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_arrear:delete", None, None).await?;

    let active_model: ActiveModel = fa.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
