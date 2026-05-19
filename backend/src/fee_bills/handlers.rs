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
    entities::fee_bills::{self, ActiveModel},
    fee_bills::dto::{CreateFeeBillRequest, FeeBillResponse, UpdateFeeBillRequest},
};

fn to_response(fb: fee_bills::Model) -> FeeBillResponse {
    FeeBillResponse {
        id: fb.id,
        student_id: fb.student_id,
        branch_id: fb.branch_id,
        academic_year_id: fb.academic_year_id,
        fee_type_id: fb.fee_type_id,
        bill_month: fb.bill_month,
        bill_year: fb.bill_year,
        amount: fb.amount,
        discount_amount: fb.discount_amount,
        late_fee: fb.late_fee,
        carry_forward: fb.carry_forward,
        net_amount: fb.net_amount,
        amount_paid: fb.amount_paid,
        balance: fb.balance,
        status: fb.status,
        due_date: fb.due_date.to_string(),
        generated_at: fb.generated_at.to_string(),
    }
}

// POST /api/v1/fee-bills
pub async fn create_fee_bill(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeeBillRequest>,
) -> Result<Json<FeeBillResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_bill:create", None, None).await?;

    let due_date = NaiveDate::parse_from_str(&payload.due_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid due_date: {}", e)))?;

    let now = Utc::now();

    let new_fb = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        fee_type_id: ActiveValue::Set(payload.fee_type_id),
        bill_month: ActiveValue::Set(payload.bill_month),
        bill_year: ActiveValue::Set(payload.bill_year),
        amount: ActiveValue::Set(payload.amount),
        discount_amount: ActiveValue::Set(payload.discount_amount),
        late_fee: ActiveValue::Set(payload.late_fee),
        carry_forward: ActiveValue::Set(payload.carry_forward),
        net_amount: ActiveValue::Set(payload.net_amount),
        amount_paid: ActiveValue::Set(payload.amount_paid),
        balance: ActiveValue::Set(payload.balance),
        status: ActiveValue::Set(payload.status),
        due_date: ActiveValue::Set(due_date),
        generated_at: ActiveValue::Set(now.into()),
        created_at: ActiveValue::Set(now.into()),
        updated_at: ActiveValue::Set(now.into()),
        ..Default::default()
    };

    let fb = new_fb
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(fb)))
}

// GET /api/v1/fee-bills
pub async fn get_fee_bills(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<FeeBillResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_bill:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = fee_bills::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(fee_bills::Column::BranchId.eq(branch_id));
    }

    let fee_bills = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(fee_bills.into_iter().map(to_response).collect()))
}

// GET /api/v1/fee-bills/{id}
pub async fn get_fee_bill(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<FeeBillResponse>, (StatusCode, String)> {
    let fb = fee_bills::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee bill not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_bill:read", None, None).await?;

    Ok(Json(to_response(fb)))
}

// PUT /api/v1/fee-bills/{id}
pub async fn update_fee_bill(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFeeBillRequest>,
) -> Result<Json<FeeBillResponse>, (StatusCode, String)> {
    let fb = fee_bills::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee bill not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_bill:update", None, None).await?;

    let mut active_model: ActiveModel = fb.into();

    if let Some(discount_amount) = payload.discount_amount {
        active_model.discount_amount = ActiveValue::Set(discount_amount);
    }
    if let Some(late_fee) = payload.late_fee {
        active_model.late_fee = ActiveValue::Set(late_fee);
    }
    if let Some(carry_forward) = payload.carry_forward {
        active_model.carry_forward = ActiveValue::Set(carry_forward);
    }
    if let Some(net_amount) = payload.net_amount {
        active_model.net_amount = ActiveValue::Set(net_amount);
    }
    if let Some(amount_paid) = payload.amount_paid {
        active_model.amount_paid = ActiveValue::Set(amount_paid);
    }
    if let Some(balance) = payload.balance {
        active_model.balance = ActiveValue::Set(balance);
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(due_date) = payload.due_date {
        let date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid due_date: {}", e)))?;
        active_model.due_date = ActiveValue::Set(date);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/fee-bills/{id}
pub async fn delete_fee_bill(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let fb = fee_bills::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee bill not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_bill:delete", None, None).await?;

    let active_model: ActiveModel = fb.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
