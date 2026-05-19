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
    entities::payment_methods::{self, ActiveModel},
    payment_methods::dto::{CreatePaymentMethodRequest, PaymentMethodResponse, UpdatePaymentMethodRequest},
};

fn to_response(pm: payment_methods::Model) -> PaymentMethodResponse {
    PaymentMethodResponse {
        id: pm.id,
        organization_id: pm.organization_id,
        name: pm.name,
        name_urdu: pm.name_urdu,
        requires_reference: pm.requires_reference,
        description: pm.description,
        is_active: pm.is_active,
    }
}

// POST /api/v1/payment-methods
pub async fn create_payment_method(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePaymentMethodRequest>,
) -> Result<Json<PaymentMethodResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "payment_method:create", Some(payload.organization_id), None).await?;

    let new_pm = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        requires_reference: ActiveValue::Set(payload.requires_reference),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let pm = new_pm
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(pm)))
}

// GET /api/v1/payment-methods
pub async fn get_payment_methods(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<PaymentMethodResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "payment_method:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = payment_methods::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(payment_methods::Column::OrganizationId.eq(org_id));
    }

    let payment_methods = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(payment_methods.into_iter().map(to_response).collect()))
}

// GET /api/v1/payment-methods/{id}
pub async fn get_payment_method(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<PaymentMethodResponse>, (StatusCode, String)> {
    let pm = payment_methods::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Payment method not found".to_string()))?;

    authorize(&*state.db, &claims, "payment_method:read", Some(pm.organization_id), None).await?;

    Ok(Json(to_response(pm)))
}

// PUT /api/v1/payment-methods/{id}
pub async fn update_payment_method(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePaymentMethodRequest>,
) -> Result<Json<PaymentMethodResponse>, (StatusCode, String)> {
    let pm = payment_methods::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Payment method not found".to_string()))?;

    authorize(&*state.db, &claims, "payment_method:update", Some(pm.organization_id), None).await?;

    let mut active_model: ActiveModel = pm.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(requires_reference) = payload.requires_reference {
        active_model.requires_reference = ActiveValue::Set(requires_reference);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/payment-methods/{id}
pub async fn delete_payment_method(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let pm = payment_methods::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Payment method not found".to_string()))?;

    authorize(&*state.db, &claims, "payment_method:delete", Some(pm.organization_id), None).await?;

    let active_model: ActiveModel = pm.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
