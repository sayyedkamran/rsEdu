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
    entities::transfers::{self, ActiveModel},
    transfers::dto::{CreateTransferRequest, TransferResponse, UpdateTransferRequest},
};

fn to_response(t: transfers::Model) -> TransferResponse {
    TransferResponse {
        id: t.id,
        entity_type: t.entity_type,
        entity_id: t.entity_id,
        from_branch_id: t.from_branch_id,
        to_branch_id: t.to_branch_id,
        transfer_date: t.transfer_date.to_string(),
        reason: t.reason,
        requested_by: t.requested_by,
        approved_by: t.approved_by,
        status: t.status,
        rejection_reason: t.rejection_reason,
    }
}

// POST /api/v1/transfers
pub async fn create_transfer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTransferRequest>,
) -> Result<Json<TransferResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "transfer:create", None, None).await?;

    let transfer_date = NaiveDate::parse_from_str(&payload.transfer_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid transfer_date: {}", e)))?;

    let new_t = ActiveModel {
        entity_type: ActiveValue::Set(payload.entity_type),
        entity_id: ActiveValue::Set(payload.entity_id),
        from_branch_id: ActiveValue::Set(payload.from_branch_id),
        to_branch_id: ActiveValue::Set(payload.to_branch_id),
        transfer_date: ActiveValue::Set(transfer_date),
        reason: ActiveValue::Set(payload.reason),
        requested_by: ActiveValue::Set(payload.requested_by),
        approved_by: ActiveValue::Set(None),
        status: ActiveValue::Set("pending".to_string()),
        rejection_reason: ActiveValue::Set(None),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let t = new_t
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(t)))
}

// GET /api/v1/transfers
pub async fn get_transfers(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<TransferResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "transfer:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = transfers::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(transfers::Column::FromBranchId.eq(branch_id));
    }

    let transfers = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(transfers.into_iter().map(to_response).collect()))
}

// GET /api/v1/transfers/{id}
pub async fn get_transfer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<TransferResponse>, (StatusCode, String)> {
    let t = transfers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Transfer not found".to_string()))?;

    authorize(&*state.db, &claims, "transfer:read", None, None).await?;

    Ok(Json(to_response(t)))
}

// PUT /api/v1/transfers/{id}
pub async fn update_transfer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTransferRequest>,
) -> Result<Json<TransferResponse>, (StatusCode, String)> {
    let t = transfers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Transfer not found".to_string()))?;

    authorize(&*state.db, &claims, "transfer:update", None, None).await?;

    let mut active_model: ActiveModel = t.into();

    if let Some(transfer_date) = payload.transfer_date {
        let date = NaiveDate::parse_from_str(&transfer_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid transfer_date: {}", e)))?;
        active_model.transfer_date = ActiveValue::Set(date);
    }
    if let Some(reason) = payload.reason {
        active_model.reason = ActiveValue::Set(Some(reason));
    }
    if let Some(approved_by) = payload.approved_by {
        active_model.approved_by = ActiveValue::Set(Some(approved_by));
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(rejection_reason) = payload.rejection_reason {
        active_model.rejection_reason = ActiveValue::Set(Some(rejection_reason));
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/transfers/{id}
pub async fn delete_transfer(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let t = transfers::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Transfer not found".to_string()))?;

    authorize(&*state.db, &claims, "transfer:delete", None, None).await?;

    let active_model: ActiveModel = t.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
